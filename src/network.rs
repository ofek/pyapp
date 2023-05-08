use std::io::Write;

use anyhow::{Context, Result};

use crate::terminal;

pub fn download(url: &String, writer: impl Write, description: &str) -> Result<()> {
    let mut response =
        reqwest::blocking::get(url).with_context(|| format!("download failed: {}", url))?;

    let pb = terminal::io_progress_bar(
        format!("Downloading {}", description),
        response.content_length().unwrap_or(0),
    );
    response.copy_to(&mut pb.wrap_write(writer))?;
    pb.finish_and_clear();

    Ok(())
}
