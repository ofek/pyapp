use std::fs;
use std::process::exit;

use anyhow::Result;
use clap::Args;

use crate::{app, distribution, terminal};

/// Install the latest version
#[derive(Args, Debug)]
#[command(hide = env!("PYAPP_EXPOSE_UPDATE") == "0")]
pub struct Cli {
    /// Allow pre-release and development versions
    #[arg(long)]
    pre: bool,

    /// Restore the installation to the default state before upgrading
    #[arg(short, long)]
    restore: bool,
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        if app::skip_install() && !app::allow_updates() {
            println!("Cannot update as installation is disabled");
            exit(1);
        }

        let existing_installation = app::install_dir().is_dir();
        if !existing_installation {
            distribution::materialize()?;
        } else if self.restore {
            let spinner = terminal::spinner("Removing installation".to_string());
            let result = fs::remove_dir_all(app::install_dir());
            spinner.finish_and_clear();
            result?;
            distribution::materialize()?;
        }

        let mut command = distribution::pip_install_command();
        if self.pre {
            command.arg("--pre");
        }
        command.arg("--upgrade");

        let wait_message = format!("Updating {}", app::project_name());
        let dependency_file = app::project_dependency_file();
        let (status, output) = if dependency_file.is_empty() {
            command.arg(app::project_name().as_str());
            distribution::pip_install(command, wait_message)?
        } else {
            distribution::pip_install_dependency_file(&dependency_file, command, wait_message)?
        };

        if !status.success() {
            if !existing_installation {
                fs::remove_dir_all(app::install_dir()).ok();
            }
            println!("{}", output.trim_end());
            exit(status.code().unwrap_or(1));
        }

        if !dependency_file.is_empty() {
            println!("Updated");
            return Ok(());
        }

        let mut existing_version: Option<&str> = None;
        let mut installed_version: Option<&str> = None;
        for line in output.lines() {
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
