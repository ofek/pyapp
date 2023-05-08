use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage this application
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Restore(super::restore::Cli),
    Starship(super::starship::Cli),
    Update(super::update::Cli),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Restore(cli) => cli.exec(),
            Commands::Starship(cli) => cli.exec(),
            Commands::Update(cli) => cli.exec(),
        }
    }
}
