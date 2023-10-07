#![allow(clippy::eq_op)]

use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use directories::ProjectDirs;
use once_cell::sync::OnceCell;

static PLATFORM_DIRS: OnceCell<ProjectDirs> = OnceCell::new();
static INSTALLATION_DIRECTORY: OnceCell<PathBuf> = OnceCell::new();

fn platform_dirs() -> &'static ProjectDirs {
    PLATFORM_DIRS
        .get()
        .expect("platform directories are not initialized")
}

pub fn install_dir() -> &'static PathBuf {
    INSTALLATION_DIRECTORY
        .get()
        .expect("installation directory is not initialized")
}

pub fn initialize() -> Result<()> {
    let platform_directories = ProjectDirs::from("", "", "pyapp")
        .with_context(|| "unable to find platform directories")?;
    PLATFORM_DIRS
        .set(platform_directories)
        .expect("could not set platform directories");

    let install_dir_override = env::var(format!(
        "PYAPP_INSTALL_DIR_{}",
        project_name().to_uppercase()
    ))
    .unwrap_or_default();
    let installation_directory = if !install_dir_override.is_empty() {
        PathBuf::from(install_dir_override)
    } else {
        platform_dirs()
            .data_local_dir()
            .join(project_name())
            .join(distribution_id())
            .join(project_version())
    };
    INSTALLATION_DIRECTORY
        .set(installation_directory)
        .expect("could not set installation directory");

    Ok(())
}

fn decode_option(encoded: &'static str) -> String {
    String::from_utf8(
        STANDARD_NO_PAD
            .decode(encoded)
            .unwrap_or_else(|_| panic!("{} is not valid base64", encoded)),
    )
    .unwrap_or_else(|_| panic!("{} is not valid UTF-8", encoded))
}

pub fn embedded_distribution() -> &'static [u8] {
    // If this is empty, then the distribution will be downloaded at runtime
    include_bytes!("embed/distribution")
}

pub fn embedded_project() -> &'static [u8] {
    // If this is empty, then the project will be downloaded at runtime
    include_bytes!("embed/project")
}

fn installation_python_path() -> String {
    env!("PYAPP__INSTALLATION_PYTHON_PATH").into()
}

fn installation_site_packages_path() -> String {
    env!("PYAPP__INSTALLATION_SITE_PACKAGES_PATH").into()
}

pub fn exposed_command() -> String {
    env!("PYAPP__EXPOSED_COMMAND").into()
}

pub fn distribution_id() -> String {
    env!("PYAPP__DISTRIBUTION_ID").into()
}

pub fn python_isolation_flag() -> String {
    env!("PYAPP__PYTHON_ISOLATION_FLAG").into()
}

pub fn distribution_source() -> String {
    env!("PYAPP_DISTRIBUTION_SOURCE").into()
}

pub fn distribution_format() -> String {
    env!("PYAPP_DISTRIBUTION_FORMAT").into()
}

pub fn distribution_python_path() -> String {
    env!("PYAPP_DISTRIBUTION_PYTHON_PATH").into()
}

pub fn distribution_pip_available() -> bool {
    env!("PYAPP_DISTRIBUTION_PIP_AVAILABLE") == "1"
}

pub fn project_name() -> String {
    env!("PYAPP_PROJECT_NAME").into()
}

pub fn project_version() -> String {
    env!("PYAPP_PROJECT_VERSION").into()
}

pub fn project_dependency_file() -> String {
    decode_option(env!("PYAPP_PROJECT_DEPENDENCY_FILE"))
}

pub fn project_dependency_file_name() -> String {
    env!("PYAPP__PROJECT_DEPENDENCY_FILE_NAME").into()
}

pub fn project_embed_file_name() -> String {
    env!("PYAPP__PROJECT_EMBED_FILE_NAME").into()
}

pub fn exec_module() -> String {
    env!("PYAPP_EXEC_MODULE").into()
}

pub fn exec_code() -> String {
    if env!("PYAPP__EXEC_CODE_ENCODED") == "1" {
        decode_option(env!("PYAPP_EXEC_CODE"))
    } else {
        env!("PYAPP_EXEC_CODE").into()
    }
}

pub fn exec_script() -> String {
    decode_option(env!("PYAPP_EXEC_SCRIPT"))
}

pub fn exec_script_path() -> PathBuf {
    cache_dir()
        .join("scripts")
        .join(env!("PYAPP__EXEC_SCRIPT_ID"))
        .join(env!("PYAPP__EXEC_SCRIPT_NAME"))
}

pub fn pip_extra_args() -> String {
    env!("PYAPP_PIP_EXTRA_ARGS").into()
}

pub fn pip_allow_config() -> bool {
    env!("PYAPP_PIP_ALLOW_CONFIG") == "1"
}

pub fn pip_version() -> String {
    env!("PYAPP_PIP_VERSION").into()
}

pub fn pip_external() -> bool {
    env!("PYAPP_PIP_EXTERNAL") == "1"
}

pub fn full_isolation() -> bool {
    env!("PYAPP_FULL_ISOLATION") == "1"
}

pub fn upgrade_virtualenv() -> bool {
    env!("PYAPP_UPGRADE_VIRTUALENV") == "1"
}

pub fn skip_install() -> bool {
    env!("PYAPP_SKIP_INSTALL") == "1"
}

pub fn pass_location() -> bool {
    env!("PYAPP_PASS_LOCATION") == "1"
}

pub fn metadata_template() -> String {
    env!("PYAPP_METADATA_TEMPLATE").into()
}

pub fn python_path() -> PathBuf {
    install_dir().join(installation_python_path())
}

pub fn site_packages_path() -> PathBuf {
    install_dir().join(installation_site_packages_path())
}

pub fn cache_dir() -> PathBuf {
    platform_dirs().cache_dir().to_path_buf()
}

pub fn distributions_cache() -> PathBuf {
    cache_dir().join("distributions")
}

pub fn external_pip_cache() -> PathBuf {
    cache_dir().join("pip")
}

pub fn external_pip_zipapp() -> PathBuf {
    let pip_version = pip_version();
    let filename = if pip_version == "latest" {
        "pip.pyz".to_string()
    } else {
        format!("pip-{}.pyz", pip_version)
    };
    external_pip_cache().join(filename)
}
