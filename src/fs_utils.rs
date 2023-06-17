use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn move_temp_file(temp_file: &PathBuf, destination: &PathBuf) -> Result<()> {
    if fs::rename(temp_file, destination).is_err() {
        // If on different devices / mountpoints, copy the file, then remove the temp file
        fs::copy(temp_file, destination).with_context(|| {
            format!(
                "unable to copy {} to {}",
                temp_file.display(),
                destination.display()
            )
        })?;
        fs::remove_file(temp_file)
            .with_context(|| format!("unable to remove temporary file {}", temp_file.display()))?;
    }

    Ok(())
}
