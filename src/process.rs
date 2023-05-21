use std::io::Read;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
#[cfg(windows)]
use std::process::exit;
use std::process::{Command, ExitStatus};

use anyhow::Result;

use crate::terminal;

pub fn wait_for(mut command: Command, message: String) -> Result<(ExitStatus, String)> {
    let (mut reader, writer_stdout) = os_pipe::pipe()?;
    let writer_stderr = writer_stdout.try_clone()?;
    command.stdout(writer_stdout);
    command.stderr(writer_stderr);

    let mut child = command.spawn()?;
    drop(command);

    let spinner = terminal::spinner(message);

    let mut output = String::new();
    let result: Result<ExitStatus> = {
        reader.read_to_string(&mut output)?;
        Ok(child.wait()?)
    };

    spinner.finish_and_clear();
    Ok((result?, output))
}

#[cfg(unix)]
pub fn exec(mut command: Command) -> Result<()> {
    Err(command.exec().into())
}

#[cfg(windows)]
pub fn exec(mut command: Command) -> Result<()> {
    let status = command.status()?;
    exit(status.code().unwrap_or(1));
}
