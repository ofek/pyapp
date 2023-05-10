use std::io::Read;
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
