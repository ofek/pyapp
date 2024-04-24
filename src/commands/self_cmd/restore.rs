use anyhow::Result;
use clap::Args;

use crate::distribution;

/// Restore the installation
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(self) -> Result<()> {
        super::remove::Cli {}.exec()?;
        distribution::ensure_ready()?;

        Ok(())
    }
}
