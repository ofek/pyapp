#![allow(clippy::eq_op)]

use anyhow::{Context, Result};
use clap::Args;

use crate::{distribution, process};

/// Directly invoke pip with the installed Python
#[derive(Args, Debug)]
#[command(hide = env!("PYAPP_EXPOSE_PIP") == "0", disable_help_flag = true)]
pub struct Cli {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        distribution::ensure_ready()?;
        distribution::ensure_installer_available()?;

        let mut command = distribution::pip_base_command();
        command.args(self.args);

        process::exec(command)
            .with_context(|| "pip execution failed, consider restoring project from scratch")
    }
}
