use std::fs;
use std::process::exit;

use anyhow::Result;
use clap::Args;

use crate::{app, distribution, terminal};

/// Install the latest version
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(self) -> Result<()> {
        let installation_directory = app::installation_directory();
        let python = installation_directory.join(app::distribution_python_path());

        let existing_installation = installation_directory.is_dir();
        if !existing_installation {
            distribution::materialize(&installation_directory)?;
        }

        let mut command = distribution::pip_command(&python);
        command.args(["--upgrade", app::project_name().as_str()]);

        let spinner = terminal::spinner(format!("Updating {}", app::project_name()));
        let result = command.output();
        spinner.finish_and_clear();

        let output = result?;
        if !output.status.success() {
            if !existing_installation {
                fs::remove_dir_all(&installation_directory).ok();
            }

            println!("{}", String::from_utf8_lossy(&output.stdout));
            exit(output.status.code().unwrap_or(1));
        }

        let install_output = String::from_utf8_lossy(&output.stdout);
        let mut existing_version: Option<&str> = None;
        let mut installed_version: Option<&str> = None;
        for line in install_output.lines() {
            if line.starts_with(
                format!("Requirement already satisfied: {} in", app::project_name()).as_str(),
            ) {
                if let Some(version) = line.rsplit(' ').next() {
                    existing_version.replace(version);
                }
            } else if line.starts_with("Successfully installed") {
                if let Some(package) = line
                    .split(' ')
                    .skip(2)
                    .find(|s| s.starts_with(format!("{}-", app::project_name()).as_str()))
                {
                    let (_, version) = package.split_at(app::project_name().len() + 1);
                    installed_version.replace(version);
                }
                break;
            }
        }

        if let Some(version) = installed_version {
            println!("Updated to {}", version);
        } else if let Some(version) = existing_version {
            println!(
                "The latest version ({}) is already installed",
                &version[1..version.len() - 1]
            );
        } else {
            println!("Updated");
        }

        Ok(())
    }
}
