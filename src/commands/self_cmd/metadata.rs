use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::app;

/// Display metadata
#[derive(Args, Debug)]
#[command(hide = env!("PYAPP_EXPOSE_METADATA") == "0")]
pub struct Cli {}

impl Cli {
    pub fn exec(self) -> Result<()> {
        let installation_directory = app::installation_directory();
        if !installation_directory.is_dir() {
            return Ok(());
        }

        let python = installation_directory.join(app::distribution_python_path());
        let site_packages: Option<PathBuf> = if cfg!(target_os = "windows") {
            (|| Some(python.parent()?.join("Lib").join("site-packages")))()
        } else {
            (|| {
                let lib_dir = python.parent()?.parent()?.join("lib");
                let version_dir =
                    fs::read_dir(lib_dir)
                        .ok()?
                        .filter_map(Result::ok)
                        .find(|entry| {
                            let file_name = entry.file_name().to_string_lossy().to_string();
                            file_name.starts_with("python") || file_name.starts_with("pypy")
                        })?;

                Some(version_dir.path().join("site-packages"))
            })()
        };

        if let Some(site_packages) = site_packages.filter(|p| p.is_dir()) {
            let metadata_file: Option<PathBuf> =
                fs::read_dir(site_packages).ok().and_then(|entries| {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.ends_with(".dist-info")
                            && name
                                .starts_with(&format!("{}-", app::project_name().replace('-', "_")))
                        {
                            return Some(entry.path().join("METADATA"));
                        }
                    }
                    None
                });

            if let Some(metadata_file) = metadata_file.filter(|p| p.is_file()) {
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
            }
        }

        Ok(())
    }
}
