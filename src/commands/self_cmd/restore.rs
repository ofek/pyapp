use std::fs;

use anyhow::Result;
use clap::Args;

use crate::{app, distribution, terminal};

/// Restore the installation
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(self) -> Result<()> {
        let installation_directory = app::installation_directory();
        let python = installation_directory.join(app::distribution_python_path());

        if installation_directory.is_dir() {
            let spinner = terminal::spinner("Removing installation".to_string());
            let result = fs::remove_dir_all(&installation_directory);
            spinner.finish_and_clear();
            result?;
        }
        distribution::ensure_ready(&installation_directory, &python)?;

        Ok(())
    }
}
