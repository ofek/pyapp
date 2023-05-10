use std::fs::File;
use std::path::PathBuf;

use anyhow::{bail, Result};

use crate::terminal;

pub fn unpack(format: String, archive: &PathBuf, destination: &PathBuf) -> Result<()> {
    match format.as_ref() {
        "tar|gzip" => unpack_tar_gzip(archive, destination)?,
        "tar|zstd" => unpack_tar_zstd(archive, destination)?,
        "zip" => unpack_zip(archive, destination)?,
        _ => bail!("unsupported distribution format: {}", format),
    }

    Ok(())
}

fn unpack_tar_gzip(path: &PathBuf, destination: &PathBuf) -> Result<()> {
    let gz = flate2::read::GzDecoder::new(File::open(path)?);
    let mut archive = tar::Archive::new(gz);

    let spinner = terminal::spinner("Unpacking distribution (tar|gzip)".to_string());
    let result = archive.unpack(destination);
    spinner.finish_and_clear();
    result?;

    Ok(())
}

fn unpack_tar_zstd(path: &PathBuf, destination: &PathBuf) -> Result<()> {
    let zst = zstd::stream::read::Decoder::new(File::open(path)?)?;
    let mut archive = tar::Archive::new(zst);

    let spinner = terminal::spinner("Unpacking distribution (tar|zstd)".to_string());
    let result = archive.unpack(destination);
    spinner.finish_and_clear();
    result?;

    Ok(())
}

fn unpack_zip(path: &PathBuf, destination: &PathBuf) -> Result<()> {
    let mut archive = zip::ZipArchive::new(File::open(path)?)?;

    let spinner = terminal::spinner("Unpacking distribution (zip)".to_string());
    let result = archive.extract(destination);
    spinner.finish_and_clear();
    result?;

    Ok(())
}
