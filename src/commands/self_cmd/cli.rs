use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage this application
#[derive(Args, Debug)]
#[command(disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Metadata(super::metadata::Cli),
    Pip(super::pip::Cli),
    Python(super::python::Cli),
    PythonPath(super::python_path::Cli),
    Remove(super::remove::Cli),
    Restore(super::restore::Cli),
    Update(super::update::Cli),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Metadata(cli) => cli.exec(),
            Commands::Pip(cli) => cli.exec(),
            Commands::Python(cli) => cli.exec(),
            Commands::PythonPath(cli) => cli.exec(),
            Commands::Remove(cli) => cli.exec(),
            Commands::Restore(cli) => cli.exec(),
            Commands::Update(cli) => cli.exec(),
        }
    }
}
