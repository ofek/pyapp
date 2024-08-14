use std::io::Write;

use anyhow::{bail, Context, Result};
use std::io;
use ureq;

use crate::terminal;

pub fn download(url: &String, writer: impl Write, description: &str) -> Result<()> {
    let download = ureq::get(url)
        .call()
        .with_context(|| format!("download failed: {}", url))?;
    let download_size: u64 = download
        .header("Content-Length")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);
    let status = download.status();

    let pb = terminal::io_progress_bar(format!("Downloading {}", description), download_size);
    io::copy(&mut download.into_reader(), &mut pb.wrap_write(writer)).unwrap();
    pb.finish_and_clear();

    if status == 200 {
        Ok(())
    } else {
        bail!("download failed: {}, {}", status, url)
    }
}
