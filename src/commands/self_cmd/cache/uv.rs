use std::fs;

use anyhow::Result;
use clap::Args;

use crate::app;

/// Manage the UV cache
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(short, long)]
    remove: bool,
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        if !app::uv_enabled() {
            println!("UV is not enabled");
            return Ok(());
        }

        let managed_uv = app::managed_uv();
        if !managed_uv.exists() {
            if self.remove {
                println!("Does not exist");
            }
            return Ok(());
        }

        if self.remove {
            fs::remove_file(managed_uv)?;
        } else {
            println!("{}", managed_uv.display());
        }

        Ok(())
    }
}
