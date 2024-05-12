use std::fs;

use anyhow::Result;
use clap::Args;

use crate::app;

/// Manage the distribution cache
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(short, long)]
    remove: bool,
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        let distributions_dir = app::distributions_cache();
        let distribution_file = distributions_dir.join(app::distribution_id());
        if !distribution_file.exists() {
            if self.remove {
                println!("Does not exist");
            }
            return Ok(());
        }

        if self.remove {
            fs::remove_file(distribution_file)?;
        } else {
            println!("{}", distribution_file.display());
        }

        Ok(())
    }
}
