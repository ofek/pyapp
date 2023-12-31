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

    if let Some(env!("PYAPP_SELF_COMMAND")) = env::args().nth(1).as_deref() {
        match Cli::try_parse() {
            Ok(cli) => return cli.exec(),
            Err(err) => {
                if !err.use_stderr() {
                    err.exit();
                }
            }
        };
    };

    distribution::ensure_ready()?;
    distribution::run_project()?;

    Ok(())
}
