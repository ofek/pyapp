#![allow(clippy::eq_op)]

use std::fs;

use anyhow::Result;
use clap::Args;

use crate::app;

/// Display metadata
#[derive(Args, Debug)]
#[command(hide = env!("PYAPP_EXPOSE_METADATA") == "0")]
pub struct Cli {}

impl Cli {
    pub fn exec(self) -> Result<()> {
        if !app::install_dir().is_dir() {
            return Ok(());
        }

        let site_packages = app::site_packages_path();

        let expected_prefix = format!("{}-", app::project_name().replace('-', "_"));
        let metadata_file = fs::read_dir(site_packages).ok().and_then(|entries| {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".dist-info")
                    && name.starts_with(&expected_prefix)
                    && name
                        .chars()
                        .nth(&expected_prefix.len() + 1)
                        .map(|c| c.is_numeric())
                        .is_some()
                {
                    return Some(entry.path().join("METADATA"));
                }
            }
            None
        });

        let metadata_file = if let Some(metadata_file) = metadata_file.filter(|p| p.is_file()) {
            metadata_file
        } else {
            return Ok(());
        };

        if let Ok(metadata) = fs::read_to_string(metadata_file) {
            for line in metadata.lines() {
                if line.starts_with("Version: ") {
                    println!(
                        "{}",
                        app::metadata_template()
                            .replace("{project}", &app::project_name())
                            .replace("{version}", line.split_at(9).1)
                    );
                    break;
                }
            }
        }

        Ok(())
    }
}
