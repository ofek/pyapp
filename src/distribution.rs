use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, ExitStatus};

use anyhow::{bail, Context, Result};
use fs4::FileExt;
use tempfile::tempdir;

use crate::{app, compression, fs_utils, network, process};

#[cfg(windows)]
const PATH_SEPARATOR: char = ';';

#[cfg(not(windows))]
const PATH_SEPARATOR: char = ':';

fn apply_env_vars(command: &mut Command) {
    let python_path = app::python_path();
    let python_dir = python_path.parent().unwrap();
    let exe_paths = if app::full_isolation() && cfg!(windows) {
        format!(
            "{}{}{}",
            python_dir.to_string_lossy(),
            PATH_SEPARATOR,
            python_dir.join("Scripts").to_string_lossy(),
        )
    } else {
        format!("{}", python_dir.to_string_lossy())
    };
    match env::var_os("PATH") {
        Some(path) => {
            command.env(
                "PATH",
                format!("{}{}{}", exe_paths, PATH_SEPARATOR, path.to_string_lossy()),
            );
        }
        None => {
            command.env("PATH", exe_paths);
        }
    }

    if !app::full_isolation() {
        command.env("VIRTUAL_ENV", python_dir.parent().unwrap());
    } else if app::uv_as_installer() {
        command.env("UV_SYSTEM_PYTHON", "true");
    }

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
}

pub fn python_command(python: &impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(python);
    apply_env_vars(&mut command);
    command.arg(app::python_isolation_flag());

    command
}

fn uv_command() -> Command {
    let mut command = Command::new(app::managed_uv());
    apply_env_vars(&mut command);

    command
}

pub fn run_project() -> Result<()> {
    let mut command = python_command(&app::python_path());

    #[cfg(windows)]
    {
        if app::is_gui() {
            command = python_command(&app::pythonw_path());
        }
    }

    apply_env_vars(&mut command);
    if !app::exec_code().is_empty() {
        command.args(["-c", app::exec_code().as_str()]);
    } else if !app::exec_module().is_empty() {
        command.args(["-m", app::exec_module().as_str()]);
    } else if !app::exec_script().is_empty() {
        let script_path = app::exec_script_path();
        if !script_path.is_file() {
            let script_directory = script_path.parent().unwrap();
            fs::create_dir_all(script_directory).with_context(|| {
                format!(
                    "unable to create script cache directory {}",
                    &script_directory.display()
                )
            })?;
            fs::write(&script_path, app::exec_script()).with_context(|| {
                format!("unable to write project script {}", &script_path.display())
            })?;
        }
        command.arg(script_path);
    } else {
        let notebook_path = app::exec_notebook_path();
        if !notebook_path.is_file() {
            let notebook_directory = notebook_path.parent().unwrap();
            fs::create_dir_all(notebook_directory).with_context(|| {
                format!(
                    "unable to create notebook cache directory {}",
                    &notebook_directory.display()
                )
            })?;
            fs::write(&notebook_path, app::exec_notebook()).with_context(|| {
                format!(
                    "unable to write project notebook {}",
                    &notebook_path.display()
                )
            })?;
        }
        command
            .arg("-m")
            .arg("notebook")
            .arg(notebook_path.to_str().unwrap());
    }
    command.args(env::args().skip(1));

    process::exec(command)
        .with_context(|| "project execution failed, consider restoring from scratch")
}

pub fn ensure_ready() -> Result<()> {
    let lock_path = app::installation_lock();
    let lock_file = fs_utils::acquire_lock(&lock_path)?;

    if !app::install_dir().is_dir() {
        materialize()?;

        if !app::skip_install() {
            install_project()?;
        }
    }

    lock_file
        .unlock()
        .with_context(|| format!("unable to release lock file {}", lock_path.display()))
}

pub fn pip_base_command() -> Command {
    if app::uv_as_installer() {
        let mut command = uv_command();
        command.arg("pip");
        command
    } else {
        let mut command = python_command(&app::python_path());
        if app::pip_external() {
            let external_pip = app::external_pip_zipapp();
            command.arg(external_pip.to_string_lossy().as_ref());
        } else {
            command.args(["-m", "pip"]);
        }
        command
    }
}

pub fn pip_install_command() -> Command {
    let mut command = pip_base_command();

    if app::uv_as_installer() {
        command.arg("install");
    } else {
        command.args([
            "install",
            "--disable-pip-version-check",
            "--no-warn-script-location",
        ]);
        if !app::pip_allow_config() {
            command.arg("--isolated");
        }
    }
    command.args(
        app::pip_extra_args()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>(),
    );

    command
}

pub fn materialize() -> Result<()> {
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

        fs_utils::move_temp_file(&temp_path, &distribution_file)?;
    }

    if app::full_isolation() {
        compression::unpack(
            app::distribution_format(),
            &distribution_file,
            app::install_dir(),
        )
        .or_else(|err| {
            fs::remove_dir_all(app::install_dir()).ok();
            bail!(
                "unable to unpack to {}\n{}",
                &app::install_dir().display(),
                err
            );
        })?;

        if !app::skip_install() {
            ensure_base_pip(app::install_dir())?;
        }
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

        let python_path = unpacked_distribution.join(app::distribution_python_path());
        let mut command = if app::uv_enabled() {
            ensure_uv_available()?;
            let mut command = uv_command();
            command.args(["venv", "--python", python_path.to_string_lossy().as_ref()]);
            if app::uv_only_bootstrap() {
                command.arg("--seed");
            }

            command
        } else {
            let mut command = python_command(&python_path);
            if app::upgrade_virtualenv() {
                ensure_base_pip(&unpacked_distribution)?;

                let mut upgrade_command =
                    python_command(&unpacked_distribution.join(app::distribution_python_path()));
                upgrade_command.args([
                    "-m",
                    "pip",
                    "install",
                    "--upgrade",
                    "--isolated",
                    "--disable-pip-version-check",
                    "--no-warn-script-location",
                    "virtualenv",
                ]);
                let (status, output) =
                    run_setup_command(upgrade_command, "Upgrading virtualenv".to_string())?;
                check_setup_status(status, output)?;

                command.args(["-m", "virtualenv"]);
                if app::pip_external() {
                    command.arg("--no-pip");
                }
            } else {
                command.args(["-m", "venv"]);
                if app::pip_external() {
                    command.arg("--without-pip");
                }
            }

            command
        };

        command.arg(app::install_dir().to_string_lossy().as_ref());
        let (status, output) =
            run_setup_command(command, "Creating virtual environment".to_string())?;
        check_setup_status(status, output)?;
    }

    Ok(())
}

fn install_project() -> Result<()> {
    let install_target = format!("{} {}", app::project_name(), app::project_version());
    let binary_only = app::pip_extra_args().contains("--only-binary :all:")
        || app::pip_extra_args().contains("--only-binary=:all:");

    let mut command = pip_install_command();
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

        command.arg(apply_project_features(
            temp_path.to_string_lossy().as_ref().to_string(),
        ));

        let wait_message = if binary_only && file_name.ends_with(".whl") {
            format!("Unpacking {}", install_target)
        } else {
            format!("Installing {}", install_target)
        };
        pip_install(command, wait_message)
    } else {
        let wait_message = if binary_only {
            format!("Unpacking {}", install_target)
        } else {
            format!("Installing {}", install_target)
        };

        let dependency_file = app::project_dependency_file();
        if dependency_file.is_empty() {
            command.arg(format!(
                "{}=={}",
                apply_project_features(app::project_name()),
                app::project_version()
            ));
            pip_install(command, wait_message)
        } else {
            pip_install_dependency_file(&dependency_file, command, wait_message)
        }
    }?;
    check_setup_status(status, output)?;

    Ok(())
}

pub fn pip_install(command: Command, wait_message: String) -> Result<(ExitStatus, String)> {
    ensure_installer_available()?;
    run_setup_command(command, wait_message)
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

    ensure_installer_available()?;
    run_setup_command(command, wait_message)
}

fn ensure_base_pip(distribution_directory: &Path) -> Result<()> {
    if app::distribution_pip_available() || app::uv_enabled() {
        return Ok(());
    }

    let mut command = python_command(&distribution_directory.join(app::distribution_python_path()));
    command.args(["-m", "ensurepip"]);

    run_setup_command(command, "Validating pip".to_string())?;
    Ok(())
}

pub fn ensure_installer_available() -> Result<()> {
    if app::uv_as_installer() {
        ensure_uv_available()?;
    } else if app::pip_external() {
        let external_pip = app::external_pip_zipapp();
        let lock_path =
            app::installer_lock("pip", external_pip.file_name().unwrap().to_str().unwrap());
        let lock_file = fs_utils::acquire_lock(&lock_path)?;
        if external_pip.is_file() {
            return lock_file
                .unlock()
                .with_context(|| format!("unable to release lock file {}", lock_path.display()));
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

        let mut f = fs::File::create(&temp_path).with_context(|| {
            format!("unable to create temporary file: {}", &temp_path.display())
        })?;

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

        fs_utils::move_temp_file(&temp_path, &external_pip)?;

        lock_file
            .unlock()
            .with_context(|| format!("unable to release lock file {}", lock_path.display()))?;
    }

    Ok(())
}

fn ensure_uv_available() -> Result<()> {
    let uv_version = app::uv_version();
    let lock_path = app::installer_lock("uv", &uv_version);
    let lock_file = fs_utils::acquire_lock(&lock_path)?;

    let managed_uv = app::managed_uv();
    if managed_uv.is_file() {
        return lock_file
            .unlock()
            .with_context(|| format!("unable to release lock file {}", lock_path.display()));
    }

    let managed_uv_cache = app::managed_uv_cache();
    fs::create_dir_all(&managed_uv_cache)
        .with_context(|| format!("unable to create UV cache {}", &managed_uv_cache.display()))?;

    let dir = tempdir().with_context(|| "unable to create temporary directory")?;
    let artifact_name = app::uv_artifact_name();
    let temp_path = dir.path().join(&artifact_name);

    let mut f = fs::File::create(&temp_path)
        .with_context(|| format!("unable to create temporary file: {}", &temp_path.display()))?;

    let url = if uv_version == "any" {
        format!(
            "https://github.com/astral-sh/uv/releases/latest/download/{}",
            &artifact_name,
        )
    } else {
        format!(
            "https://github.com/astral-sh/uv/releases/download/{}/{}",
            &uv_version, &artifact_name,
        )
    };

    network::download(&url, &mut f, "UV")?;

    if artifact_name.ends_with(".zip") {
        compression::unpack_zip(temp_path, dir.path(), "Unpacking UV".to_string())
    } else {
        compression::unpack_tar_gzip(temp_path, dir.path(), "Unpacking UV".to_string())
    }
    .or_else(|err| {
        bail!("unable to unpack to {}\n{}", dir.path().display(), err);
    })?;

    let uv_file_name = managed_uv.file_name().unwrap();
    let mut binary_path = dir.path().join(uv_file_name);

    // Binary is nested in a directory with the same name as the artifact stem
    if !binary_path.is_file() {
        binary_path = dir
            .path()
            .join(
                artifact_name
                    .trim_end_matches(".zip")
                    .trim_end_matches(".tar.gz"),
            )
            .join(uv_file_name);
    }

    fs_utils::move_temp_file(&binary_path, &managed_uv)?;

    lock_file
        .unlock()
        .with_context(|| format!("unable to release lock file {}", lock_path.display()))
}

fn run_setup_command(command: Command, message: String) -> Result<(ExitStatus, String)> {
    let program = command.get_program().to_string_lossy().to_string();
    let (status, output) = process::wait_for(command, message).with_context(|| {
        format!(
            "could not run program, verify distribution build metadata options: {}",
            &program
        )
    })?;

    Ok((status, output))
}

fn check_setup_status(status: ExitStatus, output: String) -> Result<()> {
    if !status.success() {
        fs::remove_dir_all(app::install_dir()).ok();
        println!("{}", output.trim_end());
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}

fn apply_project_features(install_target: String) -> String {
    if app::pip_project_features().is_empty() {
        install_target
    } else {
        format!("{install_target}[{}]", app::pip_project_features())
    }
}
