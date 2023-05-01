use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

use anyhow::{bail, Context, Result};
use tempfile::tempdir;

use crate::{app, compression, network, terminal};

pub fn run_project(python: &PathBuf) -> Result<()> {
    let mut command = Command::new(python);
    if app::exec_module().is_empty() {
        command.args(["-c", app::exec_code().as_str()]);
    } else {
        command.args(["-m", app::exec_module().as_str()]);
    }
    command.args(env::args().skip(1));

    let status = command.status()?;
    exit(status.code().unwrap_or(1));
}

pub fn ensure_ready(installation_directory: &PathBuf, python: &PathBuf) -> Result<()> {
    if !installation_directory.is_dir() {
        materialize(installation_directory)?;

        let mut command = pip_command(python);
        command.arg(format!("{}=={}", app::project_name(), app::project_version()).as_str());

        let spinner = terminal::spinner(format!(
            "Installing {} {}",
            app::project_name(),
            app::project_version()
        ));
        let result = command.output();
        spinner.finish_and_clear();

        let output = result?;
        if !output.status.success() {
            fs::remove_dir_all(installation_directory).ok();
            println!("{}", String::from_utf8_lossy(&output.stdout));
            exit(output.status.code().unwrap_or(1));
        }
    }

    Ok(())
}

pub fn pip_command(python: &PathBuf) -> Command {
    let mut command = Command::new(python);
    command.args([
        "-m",
        "pip",
        "install",
        "--isolated",
        "--disable-pip-version-check",
        "--no-warn-script-location",
    ]);
    command
}

pub fn materialize(installation_directory: &PathBuf) -> Result<()> {
    let distribution_file = app::cache_directory()
        .join("distributions")
        .join(app::distribution_id());

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

    compression::unpack(
        app::distribution_compression(),
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

    Ok(())
}
