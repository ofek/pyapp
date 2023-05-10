use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(bin_name = env!("PYAPP_PROJECT_NAME"), version, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = env!("PYAPP_SELF_COMMAND"))]
    SelfCmd(super::self_cmd::cli::Cli),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::SelfCmd(cli) => cli.exec(),
        }
    }
}
