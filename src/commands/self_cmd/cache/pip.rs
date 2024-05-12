use std::fs;

use anyhow::Result;
use clap::Args;

use crate::app;

/// Manage the external pip cache
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(short, long)]
    remove: bool,
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        if !app::pip_external() {
            println!("External pip not enabled");
            return Ok(());
        }

        let external_pip = app::external_pip_zipapp();
        if !external_pip.exists() {
            if self.remove {
                println!("Does not exist");
            }
            return Ok(());
        }

        if self.remove {
            fs::remove_file(external_pip)?;
        } else {
            println!("{}", external_pip.display());
        }

        Ok(())
    }
}
