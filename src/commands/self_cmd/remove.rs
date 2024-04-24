use std::fs;

use anyhow::Result;
use clap::Args;

use crate::{app, terminal};

/// Remove the installation
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(self) -> Result<()> {
        if app::install_dir().is_dir() {
            let spinner = terminal::spinner("Removing installation".to_string());
            let result = fs::remove_dir_all(app::install_dir());
            spinner.finish_and_clear();
            result?;
        }

        Ok(())
    }
}
