mod app;
mod commands;
mod compression;
mod distribution;
mod fs_utils;
mod network;
mod process;
mod terminal;

use std::env;

use anyhow::Result;
use clap::Parser;

use crate::commands::cli::Cli;

fn main() -> Result<()> {
    app::initialize()?;

    match env::args().nth(1).as_deref() {
        Some(env!("PYAPP_SELF_COMMAND")) => Cli::parse().exec(),
        _ => {
            let installation_directory = app::installation_directory();
            distribution::ensure_ready(&installation_directory)?;
            distribution::run_project(&installation_directory)?;

            Ok(())
        }
    }
}
