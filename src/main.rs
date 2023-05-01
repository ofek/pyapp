mod app;
mod commands;
mod compression;
mod distribution;
mod network;
mod terminal;

use std::env;

use anyhow::Result;
use clap::Parser;

use crate::commands::cli::Cli;

fn main() -> Result<()> {
    app::initialize()?;

    match env::args().nth(1).as_deref() {
        Some("self") => Cli::parse().exec(),
        _ => {
            let installation_directory = app::installation_directory();
            let python = installation_directory.join(app::distribution_python_path());

            distribution::ensure_ready(&installation_directory, &python)?;
            distribution::run_project(&python)?;

            Ok(())
        }
    }
}
