use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage the cache
#[derive(Args, Debug)]
#[command(hide = env!("PYAPP_EXPOSE_CACHE") == "0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Dist(super::dist::Cli),
    Pip(super::pip::Cli),
    Uv(super::uv::Cli),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Dist(cli) => cli.exec(),
            Commands::Pip(cli) => cli.exec(),
            Commands::Uv(cli) => cli.exec(),
        }
    }
}
