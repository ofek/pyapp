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
            distribution::ensure_ready()?;
            distribution::run_project()?;

            Ok(())
        }
    }
}
