use std::fs::File;
use std::path::Path;

use anyhow::{bail, Result};

use crate::terminal;

pub fn unpack(
    format: String,
    archive: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<()> {
    let wait_message = format!("Unpacking distribution ({})", format);
    match format.as_ref() {
        "tar|bzip2" => unpack_tar_bzip2(archive, destination, wait_message)?,
        "tar|gzip" => unpack_tar_gzip(archive, destination, wait_message)?,
        "tar|zstd" => unpack_tar_zstd(archive, destination, wait_message)?,
        "zip" => unpack_zip(archive, destination, wait_message)?,
        _ => bail!("unsupported distribution format: {}", format),
    }

    Ok(())
}

fn unpack_tar_bzip2(
    path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    wait_message: String,
) -> Result<()> {
    let bz = bzip2::read::BzDecoder::new(File::open(path)?);
    let mut archive = tar::Archive::new(bz);

    let spinner = terminal::spinner(wait_message);
    let result = archive.unpack(destination);
    spinner.finish_and_clear();
    result?;

    Ok(())
}

pub fn unpack_tar_gzip(
    path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    wait_message: String,
) -> Result<()> {
    let gz = flate2::read::GzDecoder::new(File::open(path)?);
    let mut archive = tar::Archive::new(gz);

    let spinner = terminal::spinner(wait_message);
    let result = archive.unpack(destination);
    spinner.finish_and_clear();
    result?;

    Ok(())
}

fn unpack_tar_zstd(
    path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    wait_message: String,
) -> Result<()> {
    let zst = zstd::stream::read::Decoder::new(File::open(path)?)?;
    let mut archive = tar::Archive::new(zst);

    let spinner = terminal::spinner(wait_message);
    let result = archive.unpack(destination);
    spinner.finish_and_clear();
    result?;

    Ok(())
}

pub fn unpack_zip(
    path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    wait_message: String,
) -> Result<()> {
    let mut archive = zip::ZipArchive::new(File::open(path)?)?;

    let spinner = terminal::spinner(wait_message);
    let result = archive.extract(destination);
    spinner.finish_and_clear();
    result?;

    Ok(())
}
