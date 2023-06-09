use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, ExitStatus};

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

pub fn pip_base_command(installation_directory: &Path) -> Command {
    let mut command = python_command(&app::python_path(installation_directory));
    if app::pip_external() {
        let external_pip = app::external_pip_zipapp();
        command.arg(external_pip.to_string_lossy().as_ref());
    } else {
        command.args(["-m", "pip"]);
    }

    command
}

pub fn pip_install_command(installation_directory: &Path) -> Command {
    let mut command = pip_base_command(installation_directory);

    command.args([
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
    let distributions_dir = app::distributions_cache();
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
        command.args(["-m", "venv"]);
        if app::pip_external() {
            command.arg("--without-pip");
        }
        command.arg(installation_directory.to_string_lossy().as_ref());
        process::wait_for(command, "Creating virtual environment".to_string())?;
    }

    Ok(())
}

fn install_project(installation_directory: &PathBuf) -> Result<()> {
    let install_target = format!("{} {}", app::project_name(), app::project_version());
    let binary_only = app::pip_extra_args().contains("--only-binary :all:")
        || app::pip_extra_args().contains("--only-binary=:all:");

    let mut command = pip_install_command(installation_directory);
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
        pip_install(command, wait_message)?
    } else {
        let wait_message = if binary_only {
            format!("Unpacking {}", install_target)
        } else {
            format!("Installing {}", install_target)
        };

        let dependency_file = app::project_dependency_file();
        if dependency_file.is_empty() {
            command.arg(format!("{}=={}", app::project_name(), app::project_version()).as_str());
            pip_install(command, wait_message)?
        } else {
            pip_install_dependency_file(&dependency_file, command, wait_message)?
        }
    };

    if !status.success() {
        fs::remove_dir_all(installation_directory).ok();
        println!("{}", output.trim_end());
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}

pub fn pip_install(command: Command, wait_message: String) -> Result<(ExitStatus, String)> {
    ensure_pip()?;
    process::wait_for(command, wait_message)
}

pub fn pip_install_dependency_file(
    dependency_file: &String,
    mut command: Command,
    wait_message: String,
) -> Result<(ExitStatus, String)> {
    let dir = tempdir().with_context(|| "unable to create temporary directory")?;
    let file_name = app::project_dependency_file_name();
    let temp_path = dir.path().join(file_name);

    let mut f = fs::File::create(&temp_path)
        .with_context(|| format!("unable to create temporary file: {}", &temp_path.display()))?;
    f.write(dependency_file.as_bytes()).with_context(|| {
        format!(
            "unable to write dependency file to temporary file: {}",
            &temp_path.display()
        )
    })?;

    command.args(["-r", temp_path.to_string_lossy().as_ref()]);

    ensure_pip()?;
    process::wait_for(command, wait_message)
}

fn ensure_pip() -> Result<()> {
    if !app::pip_external() {
        return Ok(());
    }

    let external_pip = app::external_pip_zipapp();
    if external_pip.is_file() {
        return Ok(());
    }

    let external_pip_cache = app::external_pip_cache();
    fs::create_dir_all(&external_pip_cache).with_context(|| {
        format!(
            "unable to create distribution cache {}",
            &external_pip_cache.display()
        )
    })?;

    let dir = tempdir().with_context(|| "unable to create temporary directory")?;
    let temp_path = dir.path().join("pip.pyz");

    let mut f = fs::File::create(&temp_path)
        .with_context(|| format!("unable to create temporary file: {}", &temp_path.display()))?;

    let pip_version = app::pip_version();
    let url = if pip_version == "latest" {
        "https://bootstrap.pypa.io/pip/pip.pyz".to_string()
    } else {
        format!(
            "https://bootstrap.pypa.io/pip/pip.pyz#/pip-{}.pyz",
            app::pip_version()
        )
    };

    network::download(
        &url,
        &mut f,
        external_pip.file_name().unwrap().to_str().unwrap(),
    )?;

    fs::rename(&temp_path, &external_pip).with_context(|| {
        format!(
            "unable to move {} to {}",
            &temp_path.display(),
            &external_pip.display()
        )
    })?;

    Ok(())
}
