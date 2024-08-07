use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use fs4::fs_std::FileExt;

use crate::terminal;

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

pub fn acquire_lock(file_path: &PathBuf) -> Result<fs::File> {
    let locks_dir = file_path.parent().unwrap();
    fs::create_dir_all(locks_dir)
        .with_context(|| format!("unable to create lock directory {}", &locks_dir.display()))?;

    let lock_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .with_context(|| format!("unable to open lock file {}", file_path.display()))?;

    if lock_file.try_lock_exclusive().is_err() {
        let spinner = terminal::spinner("Waiting on shared resource".to_string());
        let result = lock_file.lock_exclusive();
        spinner.finish_and_clear();
        result.with_context(|| format!("unable to acquire lock file {}", file_path.display()))?;
    }

    Ok(lock_file)
}
