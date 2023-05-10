use std::env;
use std::fmt::Display;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use highway::PortableHash;
use regex::Regex;

const DEFAULT_PYTHON_VERSION: &str = "3.11";
const KNOWN_DISTRIBUTION_FORMATS: &[&str] = &["tar|gzip", "tar|zstd", "zip"];

// Python version in the form MAJOR.MINOR
// Target OS https://doc.rust-lang.org/reference/conditional-compilation.html#target_os
// Target arch https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch
// Target ABI https://doc.rust-lang.org/reference/conditional-compilation.html#target_env
// Variant e.g. shared/static for Windows, CPU optimization level for Linux
// URL for fetching the distribution
#[rustfmt::skip]
const DEFAULT_CPYTHON_DISTRIBUTIONS: &[(&str, &str, &str, &str, &str, &str)] = &[
    ("3.11", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "windows", "x86", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-i686-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.11", "windows", "x86", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-i686-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.11", "windows", "x86_64", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.11", "windows", "x86_64", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.11", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.11", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.11.1%2B20230116-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.10", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "windows", "x86", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-i686-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.10", "windows", "x86", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-i686-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.10", "windows", "x86_64", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.10", "windows", "x86_64", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.10", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.10", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.10.9%2B20230116-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.9", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "windows", "x86", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-i686-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.9", "windows", "x86", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-i686-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.9", "windows", "x86_64", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.9", "windows", "x86_64", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.9", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.9", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.9.16%2B20230116-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.8", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.8", "windows", "x86", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-i686-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.8", "windows", "x86", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-i686-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.8", "windows", "x86_64", "msvc", "shared",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-x86_64-pc-windows-msvc-shared-install_only.tar.gz"),
    ("3.8", "windows", "x86_64", "msvc", "static",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-x86_64-pc-windows-msvc-static-install_only.tar.gz"),
    ("3.8", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.8", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230116/cpython-3.8.16%2B20230116-x86_64-apple-darwin-install_only.tar.gz"),
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
    let selected_variant = {
        let mut variant = env::var("PYAPP_DISTRIBUTION_VARIANT").unwrap_or_default();
        if variant.is_empty() {
            if selected_platform == "windows" {
                variant = "shared".to_string();
            } else if selected_platform == "linux" {
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
    let selected_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

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
    if !command_name.is_empty() {
        set_runtime_variable(variable, &command_name);
    } else {
        set_runtime_variable(variable, "self");
    }
}

fn main() {
    set_runtime_variable("PYAPP_STARSHIP_PROMPT", "{project} v{version}");

    let project_name = check_environment_variable("PYAPP_PROJECT_NAME");
    set_runtime_variable("PYAPP_PROJECT_NAME", normalize_project_name(&project_name));

    let project_version = check_environment_variable("PYAPP_PROJECT_VERSION");
    set_runtime_variable("PYAPP_PROJECT_VERSION", project_version);

    let distribution_source = get_distribution_source();
    set_runtime_variable("PYAPP_DISTRIBUTION_SOURCE", &distribution_source);

    let mut h = PortableHash::default();
    distribution_source.hash(&mut h);
    set_runtime_variable("PYAPP__DISTRIBUTION_ID", h.finish());

    set_distribution_format(&distribution_source);
    set_python_path(&distribution_source);
    set_execution_mode();
    set_skip_install();
    set_indicator();
    set_self_command();

    let archive_path: PathBuf = [
        env::var("CARGO_MANIFEST_DIR").unwrap().as_str(),
        "src",
        "embed",
        "archive",
    ]
    .iter()
    .collect();

    if is_enabled("PYAPP_DISTRIBUTION_EMBED") {
        let bytes = reqwest::blocking::get(&distribution_source)
            .unwrap()
            .bytes()
            .unwrap();
        fs::write(&archive_path, bytes).unwrap();
    } else {
        // Ensure the file is empty as that is the heuristic used at runtime to
        // determine whether to fetch from the source
        fs::File::create(&archive_path).unwrap().set_len(0).unwrap();
    }
}
