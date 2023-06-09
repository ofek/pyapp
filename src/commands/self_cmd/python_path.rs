use anyhow::Result;
use clap::Args;

use crate::app;

/// Output the path to the installed Python
#[derive(Args, Debug)]
#[command(hide = env!("PYAPP_EXPOSE_PYTHON_PATH") == "0")]
pub struct Cli {}

impl Cli {
    pub fn exec(self) -> Result<()> {
        let installation_directory = app::installation_directory();
        println!("{}", app::python_path(&installation_directory).display());

        Ok(())
    }
}
