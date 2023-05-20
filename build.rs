use std::env;
use std::fmt::Display;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::PathBuf;

use highway::PortableHash;
use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;

const DEFAULT_PYTHON_VERSION: &str = "3.11";
const KNOWN_DISTRIBUTION_FORMATS: &[&str] = &["tar|gzip", "tar|zstd", "zip"];

// Python version in the form MAJOR.MINOR
// Target OS https://doc.rust-lang.org/reference/conditional-compilation.html#target_os
// Target arch https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch
// Target ABI https://doc.rust-lang.org/reference/conditional-compilation.html#target_env
// Variant e.g. shared/static for Windows, CPU optimization level for Linux
// URL for fetching the distribution
//
// See also https://llvm.org/doxygen/classllvm_1_1Triple.html
#[rustfmt::skip]
const DEFAULT_CPYTHON_DISTRIBUTIONS: &[(&str, &str, &str, &str, &str, &str)] = &[
    ("3.11", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "ppc64le", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-ppc64le-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "windows", "x86", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-i686-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.11", "windows", "x86", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-i686-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.11", "windows", "x86_64", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.11", "windows", "x86_64", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.11", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.11", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.11.3%2B20230507-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.10", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "ppc64le", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-ppc64le-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "windows", "x86", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-i686-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.10", "windows", "x86", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-i686-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.10", "windows", "x86_64", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.10", "windows", "x86_64", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.10", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.10", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.10.11%2B20230507-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.9", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "ppc64le", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-ppc64le-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "windows", "x86", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-i686-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.9", "windows", "x86", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-i686-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.9", "windows", "x86_64", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.9", "windows", "x86_64", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.9", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.9", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.9.16%2B20230507-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.8", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.8", "windows", "x86", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-i686-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.8", "windows", "x86", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-i686-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.8", "windows", "x86_64", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-x86_64-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.8", "windows", "x86_64", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-x86_64-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.8", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.8", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230507/cpython-3.8.16%2B20230507-x86_64-apple-darwin-install_only.tar.gz"),
    // Frozen
    ("3.7", "linux", "x86_64", "gnu", "", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-x86_64-unknown-linux-gnu-pgo-20200823T0036.tar.zst"),
    ("3.7", "linux", "x86_64", "musl", "", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-x86_64-unknown-linux-musl-noopt-20200823T0036.tar.zst"),
    ("3.7", "windows", "x86", "msvc", "shared", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-i686-pc-windows-msvc-shared-pgo-20200823T0159.tar.zst"),
    ("3.7", "windows", "x86", "msvc", "static", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-i686-pc-windows-msvc-static-noopt-20200823T0221.tar.zst"),
    ("3.7", "windows", "x86_64", "msvc", "shared", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-x86_64-pc-windows-msvc-shared-pgo-20200823T0118.tar.zst"),
    ("3.7", "windows", "x86_64", "msvc", "static", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-x86_64-pc-windows-msvc-static-noopt-20200823T0153.tar.zst"),
    ("3.7", "macos", "x86_64", "", "", "https://github.com/indygreg/python-build-standalone/releases/download/20200823/cpython-3.7.9-x86_64-apple-darwin-pgo-20200823T2228.tar.zst"),
];

fn set_runtime_variable(name: &str, value: impl Display) {
    println!("cargo:rustc-env={}={}", name, value)
}

fn check_environment_variable(name: &str) -> String {
    let value = env::var(name).unwrap_or_default();
    if value.is_empty() && env::var("DEBUG").unwrap() == "false" {
        panic!("\n\n{name} environment variable is not set\n\n");
    }
    value
}

fn is_enabled(name: &str) -> bool {
    ["true", "1"].contains(&env::var(name).unwrap_or_default().as_str())
}

fn normalize_project_name(name: &String) -> String {
    // https://peps.python.org/pep-0508/#names
    if !Regex::new(r"^([[:alnum:]]|[[:alnum:]][[:alnum:]._-]*[[:alnum:]])$")
        .unwrap()
        .is_match(name)
        && env::var("DEBUG").unwrap() == "false"
    {
        panic!(
            "\n\nInvalid project name `{name}`; must only contain ASCII letters/digits, underscores, \
            hyphens, and periods, and must begin and end with ASCII letters/digits.\n\n"
        );
    }

    // https://peps.python.org/pep-0503/#normalized-names
    Regex::new(r"[-_.]+")
        .unwrap()
        .replace_all(name, "-")
        .to_lowercase()
}

fn embed_file(name: &str) -> PathBuf {
    [
        env::var("CARGO_MANIFEST_DIR").unwrap().as_str(),
        "src",
        "embed",
        name,
    ]
    .iter()
    .collect()
}

fn truncate_embed_file(path: &PathBuf) {
    // Ensure the file is empty as that is the heuristic used at runtime to
    // determine whether to make network calls
    fs::File::create(path).unwrap().set_len(0).unwrap();
}

fn get_python_version() -> String {
    env::var("PYAPP_PYTHON_VERSION").unwrap_or(DEFAULT_PYTHON_VERSION.to_string())
}

fn get_distribution_source() -> String {
    let distribution_source = env::var("PYAPP_DISTRIBUTION_SOURCE").unwrap_or_default();
    if !distribution_source.is_empty() {
        return distribution_source;
    };

    let selected_python_version = get_python_version();

    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let selected_platform = match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => "windows",
        "macos" | "ios" => "macos",
        _ => "linux",
    }
    .to_string();
    let selected_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let selected_variant = {
        let mut variant = env::var("PYAPP_DISTRIBUTION_VARIANT").unwrap_or_default();
        if variant.is_empty() {
            if selected_platform == "windows" {
                variant = "shared".to_string();
            } else if selected_platform == "linux" && selected_arch == "x86_64" {
                variant = "v3".to_string();
            }
        };
        variant
    };
    let selected_abi = {
        let mut abi = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
        if abi.is_empty() {
            if selected_platform == "windows" {
                abi = "msvc".to_string();
            } else if selected_platform == "linux" {
                abi = "gnu".to_string();
            }
        };
        abi
    };

    for (python_version, platform, arch, abi, variant, url) in DEFAULT_CPYTHON_DISTRIBUTIONS.iter()
    {
        if python_version == &selected_python_version
            && platform == &selected_platform
            && arch == &selected_arch
            && abi == &selected_abi
            && variant == &selected_variant
        {
            return url.to_string();
        }
    }

    panic!(
        "\n\nNo default distribution source found\nPython version: {}\nPlatform: {}\nArchitecture: {}\nABI: {}\nVariant: {}\n\n",
        selected_python_version, selected_platform, selected_arch, selected_abi, selected_variant
    );
}

fn set_project_from_metadata(metadata: &str, file_name: &str) {
    for item in ["Name", "Version"] {
        match Regex::new(&format!("(?m)^{item}: (.+)$"))
            .unwrap()
            .captures(metadata)
        {
            Some(captures) => {
                let value = if item == "Name" {
                    normalize_project_name(&captures[1].to_string())
                } else {
                    captures[1].to_string()
                };
                set_runtime_variable(&format!("PYAPP_PROJECT_{}", item.to_uppercase()), value);
            }
            None => {
                panic!("\n\nFailed to parse metadata {item} in {file_name}\n\n");
            }
        }
    }
}

fn set_project() {
    let embed_path = embed_file("project");
    let local_path = env::var("PYAPP_PROJECT_PATH").unwrap_or_default();
    if !local_path.is_empty() {
        let path = PathBuf::from(&local_path);
        if !path.is_file() {
            panic!("\n\nProject path is not a file: {local_path}\n\n");
        }
        fs::copy(&local_path, &embed_path).unwrap_or_else(|_| {
            panic!(
                "\n\nFailed to copy project's archive from {local_path} to {embed_path}\n\n",
                embed_path = embed_path.display()
            )
        });

        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".whl") {
            let mut archive = zip::ZipArchive::new(File::open(embed_path).unwrap()).unwrap();

            // *.dist-info/ comes last by convention
            for i in (0..archive.len()).rev() {
                let mut file = archive.by_index(i).unwrap();
                let entry_path = file.enclosed_name().unwrap().to_string_lossy().to_string();
                if entry_path.ends_with(".dist-info/METADATA") {
                    let mut metadata = String::new();
                    file.read_to_string(&mut metadata).unwrap();

                    set_project_from_metadata(&metadata, &entry_path);
                    set_runtime_variable("PYAPP__PROJECT_EMBED_FILE_NAME", file_name);
                    return;
                }
            }
        } else if file_name.ends_with(".tar.gz") {
            let gz = flate2::read::GzDecoder::new(File::open(embed_path).unwrap());
            let mut archive = tar::Archive::new(gz);

            for file in archive.entries().unwrap() {
                let mut file = file.unwrap();
                let entry_path = file.path().unwrap().to_string_lossy().to_string();
                if entry_path.ends_with("/PKG-INFO") && entry_path.matches('/').count() == 1 {
                    let mut metadata = String::new();
                    file.read_to_string(&mut metadata).unwrap();

                    set_project_from_metadata(&metadata, &entry_path);
                    set_runtime_variable("PYAPP__PROJECT_EMBED_FILE_NAME", file_name);
                    return;
                }
            }
        } else {
            panic!("\n\nUnsupported project archive format: {file_name}\n\n");
        }

        panic!("\n\nUnable to find project metadata in {file_name}\n\n");
    } else {
        let project_name = check_environment_variable("PYAPP_PROJECT_NAME");
        set_runtime_variable("PYAPP_PROJECT_NAME", normalize_project_name(&project_name));

        let project_version = check_environment_variable("PYAPP_PROJECT_VERSION");
        set_runtime_variable("PYAPP_PROJECT_VERSION", project_version);

        set_runtime_variable("PYAPP__PROJECT_EMBED_FILE_NAME", "");
        truncate_embed_file(&embed_path);
    }
}

fn set_distribution() {
    let embed_path = embed_file("distribution");
    let mut hasher = PortableHash::default();

    let distribution_source = if is_enabled("PYAPP_DISTRIBUTION_EMBED") {
        let local_path = env::var("PYAPP_DISTRIBUTION_PATH").unwrap_or_default();
        let distribution_source = if !local_path.is_empty() {
            let path = PathBuf::from(&local_path);
            if !path.is_file() {
                panic!("\n\nDistribution path is not a file: {local_path}\n\n");
            }
            fs::copy(&local_path, &embed_path).unwrap_or_else(|_| {
                panic!(
                    "\n\nFailed to copy distribution's archive from {local_path} to {embed_path}\n\n",
                    embed_path = embed_path.display()
                )
            });

            "".to_string()
        } else {
            let distribution_source = get_distribution_source();
            let bytes = reqwest::blocking::get(&distribution_source)
                .unwrap()
                .bytes()
                .unwrap();
            fs::write(&embed_path, bytes).unwrap();

            distribution_source
        };

        let mut file = File::open(&embed_path).unwrap();
        std::io::copy(&mut file, &mut hasher).unwrap();

        distribution_source
    } else {
        truncate_embed_file(&embed_path);
        let distribution_source = get_distribution_source();
        distribution_source.hash(&mut hasher);

        distribution_source
    };

    set_runtime_variable("PYAPP_DISTRIBUTION_SOURCE", &distribution_source);
    set_runtime_variable("PYAPP__DISTRIBUTION_ID", hasher.finish());

    set_distribution_format(&distribution_source);
    set_python_path(&distribution_source);
}

fn set_distribution_format(distribution_source: &String) {
    let variable = "PYAPP_DISTRIBUTION_FORMAT";
    let distribution_format = env::var(variable).unwrap_or_default();
    if !distribution_format.is_empty() {
        if KNOWN_DISTRIBUTION_FORMATS.contains(&distribution_format.as_str()) {
            set_runtime_variable(variable, &distribution_format);
        } else {
            panic!("\n\nUnknown distribution format: {distribution_format}\n\n");
        }
    } else if distribution_source.ends_with(".tar.gz") || distribution_source.ends_with(".tgz") {
        set_runtime_variable(variable, "tar|gzip");
    } else if distribution_source.ends_with(".tar.zst")
        || distribution_source.ends_with(".tar.zstd")
    {
        set_runtime_variable(variable, "tar|zstd");
    } else if distribution_source.ends_with(".zip") {
        set_runtime_variable(variable, "zip");
    } else {
        panic!("\n\nUnable to determine format for distribution source: {distribution_source}\n\n");
    }
}

fn set_python_path(distribution_source: &str) {
    let variable = "PYAPP_DISTRIBUTION_PYTHON_PATH";
    let on_windows = env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows";
    let relative_path = env::var(variable).unwrap_or_default();
    if !relative_path.is_empty() {
        set_runtime_variable(variable, &relative_path);
    } else if distribution_source
        .starts_with("https://github.com/indygreg/python-build-standalone/releases/download/")
    {
        if get_python_version() == "3.7" {
            if on_windows {
                set_runtime_variable(variable, r"python\install\python.exe");
            } else {
                set_runtime_variable(variable, "python/install/bin/python3");
            }
        } else if on_windows {
            set_runtime_variable(variable, r"python\python.exe");
        } else {
            set_runtime_variable(variable, "python/bin/python3");
        }
    } else if on_windows {
        set_runtime_variable(variable, "python.exe");
    } else {
        set_runtime_variable(variable, "bin/python3");
    }
}

fn set_execution_mode() {
    let module_variable = "PYAPP_EXEC_MODULE";
    let module = env::var(module_variable).unwrap_or_default();

    let spec_variable = "PYAPP_EXEC_SPEC";
    let spec = env::var(spec_variable).unwrap_or_default();

    let code_variable = "PYAPP_EXEC_CODE";
    let code = env::var(code_variable).unwrap_or_default();

    // Set defaults
    set_runtime_variable(module_variable, "");
    set_runtime_variable(code_variable, "");

    if [module.is_empty(), spec.is_empty(), code.is_empty()]
        .iter()
        .filter(|x| !(**x))
        .count()
        > 1
    {
        panic!("\n\nThe {module_variable}, {spec_variable}, and {code_variable} options are mutually exclusive\n\n");
    } else if !module.is_empty() {
        set_runtime_variable(module_variable, &module);
    } else if !spec.is_empty() {
        // https://packaging.python.org/en/latest/specifications/entry-points/#data-model
        let (module, object) = spec.split_once(':').unwrap();
        set_runtime_variable(
            code_variable,
            format!("import {module};{module}.{object}()"),
        );
    } else if !code.is_empty() {
        set_runtime_variable(code_variable, &code);
    } else {
        set_runtime_variable(
            module_variable,
            normalize_project_name(&env::var("PYAPP_PROJECT_NAME").unwrap_or_default())
                .replace('-', "_"),
        );
    }
}

fn set_pip_extra_args() {
    let variable = "PYAPP_PIP_EXTRA_ARGS";
    set_runtime_variable(variable, env::var(variable).unwrap_or_default());
}

fn set_pip_allow_config() {
    let variable = "PYAPP_PIP_ALLOW_CONFIG";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_skip_install() {
    let variable = "PYAPP_SKIP_INSTALL";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_indicator() {
    let variable = "PYAPP_PASS_LOCATION";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_self_command() {
    let variable = "PYAPP_SELF_COMMAND";
    let command_name = env::var(variable).unwrap_or_default();
    if command_name == "none" {
        set_runtime_variable(
            variable,
            Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
        );
    } else if !command_name.is_empty() {
        set_runtime_variable(variable, &command_name);
    } else {
        set_runtime_variable(variable, "self");
    }
}

fn set_exposed_commands() {
    let indicator = Regex::new(r"(?m)^#\[command\(hide = env!").unwrap();
    let commands_dir: PathBuf = [
        env::var("CARGO_MANIFEST_DIR").unwrap().as_str(),
        "src",
        "commands",
        "self_cmd",
    ]
    .iter()
    .collect();

    for entry in fs::read_dir(commands_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let command_name = path.file_stem().unwrap().to_str().unwrap();
        let command_path = path.to_str().unwrap();
        let command_source = fs::read_to_string(command_path).unwrap();
        if indicator.is_match(&command_source) {
            let variable = format!("PYAPP_EXPOSE_{}", command_name.to_uppercase());
            if is_enabled(&variable) {
                set_runtime_variable(&variable, "1");
            } else {
                set_runtime_variable(&variable, "0");
            }
        }
    }
}

fn set_metadata_template() {
    let variable = "PYAPP_METADATA_TEMPLATE";
    let metadata_template = env::var(variable).unwrap_or_default();
    if !metadata_template.is_empty() {
        set_runtime_variable(variable, &metadata_template);
    } else {
        set_runtime_variable(variable, "{project} v{version}");
    }
}

fn main() {
    set_project();
    set_distribution();
    set_execution_mode();
    set_pip_extra_args();
    set_pip_allow_config();
    set_skip_install();
    set_indicator();
    set_self_command();
    set_exposed_commands();
    set_metadata_template();
}
