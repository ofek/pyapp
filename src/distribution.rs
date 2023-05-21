use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use anyhow::{bail, Context, Result};
use tempfile::tempdir;

use crate::{app, compression, network, process};

pub fn python_command(python: &PathBuf) -> Command {
    let mut command = Command::new(python);

    // https://docs.python.org/3/using/cmdline.html#cmdoption-I
    command.arg("-I");

    command
}

pub fn run_project(installation_directory: &Path) -> Result<()> {
    let mut command = python_command(&app::python_path(installation_directory));

    if app::exec_module().is_empty() {
        command.args(["-c", app::exec_code().as_str()]);
    } else {
        command.args(["-m", app::exec_module().as_str()]);
    }
    command.args(env::args().skip(1));

    if !app::pass_location() {
        command.env("PYAPP", "1");
    } else if let Ok(exe_path) = env::current_exe() {
        command.env("PYAPP", exe_path);
    } else {
        command.env("PYAPP", "");
    }

    if !app::exposed_command().is_empty() {
        command.env("PYAPP_COMMAND_NAME", app::exposed_command());
    }

    process::exec(command)
        .with_context(|| "project execution failed, consider restoring from scratch")
}

pub fn ensure_ready(installation_directory: &PathBuf) -> Result<()> {
    if !installation_directory.is_dir() {
        materialize(installation_directory)?;

        if !app::skip_install() {
            install_project(installation_directory)?;
        }
    }

    Ok(())
}

pub fn pip_command(installation_directory: &Path) -> Command {
    let mut command = python_command(&app::python_path(installation_directory));
    command.args([
        "-m",
        "pip",
        "install",
        "--disable-pip-version-check",
        "--no-warn-script-location",
    ]);
    if !app::pip_allow_config() {
        command.arg("--isolated");
    }
    command.args(
        app::pip_extra_args()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>(),
    );
    command
}

pub fn materialize(installation_directory: &PathBuf) -> Result<()> {
    let distributions_dir = app::cache_directory().join("distributions");
    let distribution_file = distributions_dir.join(app::distribution_id());

    if !distribution_file.is_file() {
        let distribution_source = app::distribution_source();
        let distributions_dir = distribution_file.parent().unwrap();
        fs::create_dir_all(distributions_dir).with_context(|| {
            format!(
                "unable to create distribution cache {}",
                &distributions_dir.display()
            )
        })?;

        let dir = tempdir().with_context(|| "unable to create temporary directory")?;
        let temp_path = dir.path().join(app::distribution_id());

        let mut f = fs::File::create(&temp_path).with_context(|| {
            format!("unable to create temporary file: {}", &temp_path.display())
        })?;

        // The embedded distribution goes through the same process to become a file because
        // the ZIP archive API requires the Seek trait for the input stream
        if !app::embedded_distribution().is_empty() {
            f.write(app::embedded_distribution()).with_context(|| {
                format!(
                    "unable to write embedded distribution to temporary file: {}",
                    &temp_path.display()
                )
            })?;
        } else {
            network::download(&distribution_source, &mut f, "distribution")?;
        }

        fs::rename(&temp_path, &distribution_file).with_context(|| {
            format!(
                "unable to move {} to {}",
                &temp_path.display(),
                &distribution_file.display()
            )
        })?;
    }

    if app::full_isolation() {
        compression::unpack(
            app::distribution_format(),
            &distribution_file,
            installation_directory,
        )
        .or_else(|err| {
            fs::remove_dir_all(installation_directory).ok();
            bail!(
                "unable to unpack to {}\n{}",
                &installation_directory.display(),
                err
            );
        })?;
    } else {
        let unpacked_distribution = distributions_dir.join(format!("_{}", app::distribution_id()));
        if !unpacked_distribution.is_dir() {
            compression::unpack(
                app::distribution_format(),
                &distribution_file,
                &unpacked_distribution,
            )
            .or_else(|err| {
                fs::remove_dir_all(&unpacked_distribution).ok();
                bail!(
                    "unable to unpack to {}\n{}",
                    &unpacked_distribution.display(),
                    err
                );
            })?;
        }

        let mut command =
            python_command(&unpacked_distribution.join(app::distribution_python_path()));
        command.args([
            "-m",
            "venv",
            installation_directory.to_string_lossy().as_ref(),
        ]);
        process::wait_for(command, "Creating virtual environment".to_string())?;
    }

    Ok(())
}

fn install_project(installation_directory: &PathBuf) -> Result<()> {
    let install_target = format!("{} {}", app::project_name(), app::project_version());
    let binary_only = app::pip_extra_args().contains("--only-binary :all:")
        || app::pip_extra_args().contains("--only-binary=:all:");

    let mut command = pip_command(installation_directory);
    let (status, output) = if !app::embedded_project().is_empty() {
        let dir = tempdir().with_context(|| "unable to create temporary directory")?;
        let file_name = app::project_embed_file_name();
        let temp_path = dir.path().join(&file_name);

        let mut f = fs::File::create(&temp_path).with_context(|| {
            format!("unable to create temporary file: {}", &temp_path.display())
        })?;
        f.write(app::embedded_project()).with_context(|| {
            format!(
                "unable to write embedded project to temporary file: {}",
                &temp_path.display()
            )
        })?;

        command.arg(temp_path.to_string_lossy().as_ref());

        let wait_message = if binary_only && file_name.ends_with(".whl") {
            format!("Unpacking {}", install_target)
        } else {
            format!("Installing {}", install_target)
        };
        process::wait_for(command, wait_message)?
    } else {
        command.arg(format!("{}=={}", app::project_name(), app::project_version()).as_str());

        let wait_message = if binary_only {
            format!("Unpacking {}", install_target)
        } else {
            format!("Installing {}", install_target)
        };
        process::wait_for(command, wait_message)?
    };

    if !status.success() {
        fs::remove_dir_all(installation_directory).ok();
        println!("{}", output.trim_end());
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}
