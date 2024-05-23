use std::env;
use std::fmt::Display;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::{Path, PathBuf};

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use highway::PortableHash;
use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;

const DEFAULT_PYTHON_VERSION: &str = "3.12";
const KNOWN_DISTRIBUTION_FORMATS: &[&str] = &["tar|bzip2", "tar|gzip", "tar|zstd", "zip"];
const DEFAULT_CPYTHON_SOURCE: &str =
    "https://github.com/indygreg/python-build-standalone/releases/download/";
const DEFAULT_PYPY_SOURCE: &str = "https://downloads.python.org/pypy/";

// Python version in the form MAJOR.MINOR
// Target OS https://doc.rust-lang.org/reference/conditional-compilation.html#target_os
// Target arch https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch
// Target ABI https://doc.rust-lang.org/reference/conditional-compilation.html#target_env
// Variant e.g. CPU optimization level for Linux
// URL for fetching the distribution
//
// See also https://llvm.org/doxygen/classllvm_1_1Triple.html
#[rustfmt::skip]
const DEFAULT_CPYTHON_DISTRIBUTIONS: &[(&str, &str, &str, &str, &str, &str)] = &[
    ("3.12", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.12", "linux", "armv7", "gnueabi", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-armv7-unknown-linux-gnueabi-install_only.tar.gz"),
    ("3.12", "linux", "armv7", "gnueabihf", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-armv7-unknown-linux-gnueabihf-install_only.tar.gz"),
    ("3.12", "linux", "powerpc64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-ppc64le-unknown-linux-gnu-install_only.tar.gz"),
    ("3.12", "linux", "s390x", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-s390x-unknown-linux-gnu-install_only.tar.gz"),
    ("3.12", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.12", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.12", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.12", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.12", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.12", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.12", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.12", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.12", "windows", "x86", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-i686-pc-windows-msvc-install_only.tar.gz"),
    ("3.12", "windows", "x86_64", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64-pc-windows-msvc-install_only.tar.gz"),
    ("3.12", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.12", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.12.3%2B20240415-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.11", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "armv7", "gnueabi", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-armv7-unknown-linux-gnueabi-install_only.tar.gz"),
    ("3.11", "linux", "armv7", "gnueabihf", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-armv7-unknown-linux-gnueabihf-install_only.tar.gz"),
    ("3.11", "linux", "powerpc64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-ppc64le-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "s390x", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-s390x-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230826/cpython-3.11.5%2B20230826-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.11", "windows", "x86", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-i686-pc-windows-msvc-install_only.tar.gz"),
    ("3.11", "windows", "x86_64", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64-pc-windows-msvc-install_only.tar.gz"),
    ("3.11", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.11", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.11.9%2B20240415-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.10", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "armv7", "gnueabi", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-armv7-unknown-linux-gnueabi-install_only.tar.gz"),
    ("3.10", "linux", "armv7", "gnueabihf", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-armv7-unknown-linux-gnueabihf-install_only.tar.gz"),
    ("3.10", "linux", "powerpc64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-ppc64le-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "s390x", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-s390x-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230826/cpython-3.10.13%2B20230826-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.10", "windows", "x86", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-i686-pc-windows-msvc-install_only.tar.gz"),
    ("3.10", "windows", "x86_64", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64-pc-windows-msvc-install_only.tar.gz"),
    ("3.10", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.10", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.10.14%2B20240415-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.9", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "armv7", "gnueabi", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-armv7-unknown-linux-gnueabi-install_only.tar.gz"),
    ("3.9", "linux", "armv7", "gnueabihf", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-armv7-unknown-linux-gnueabihf-install_only.tar.gz"),
    ("3.9", "linux", "powerpc64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-ppc64le-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "s390x", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-s390x-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230826/cpython-3.9.18%2B20230826-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64_v2-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64_v3-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "gnu", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64_v4-unknown-linux-gnu-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v2",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64_v2-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v3",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64_v3-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "linux", "x86_64", "musl", "v4",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64_v4-unknown-linux-musl-install_only.tar.gz"),
    ("3.9", "windows", "x86", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-i686-pc-windows-msvc-install_only.tar.gz"),
    ("3.9", "windows", "x86_64", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64-pc-windows-msvc-install_only.tar.gz"),
    ("3.9", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.9", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.9.19%2B20240415-x86_64-apple-darwin-install_only.tar.gz"),
    ("3.8", "linux", "aarch64", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.8.19%2B20240415-aarch64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86", "gnu", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20230826/cpython-3.8.17%2B20230826-i686-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86_64", "gnu", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.8.19%2B20240415-x86_64-unknown-linux-gnu-install_only.tar.gz"),
    ("3.8", "linux", "x86_64", "musl", "v1",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.8.19%2B20240415-x86_64-unknown-linux-musl-install_only.tar.gz"),
    ("3.8", "windows", "x86", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.8.19%2B20240415-i686-pc-windows-msvc-install_only.tar.gz"),
    ("3.8", "windows", "x86_64", "msvc", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.8.19%2B20240415-x86_64-pc-windows-msvc-install_only.tar.gz"),
    ("3.8", "macos", "aarch64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.8.19%2B20240415-aarch64-apple-darwin-install_only.tar.gz"),
    ("3.8", "macos", "x86_64", "", "",
        "https://github.com/indygreg/python-build-standalone/releases/download/20240415/cpython-3.8.19%2B20240415-x86_64-apple-darwin-install_only.tar.gz"),
    // Frozen
    ("3.7", "linux", "x86_64", "gnu", "", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-x86_64-unknown-linux-gnu-pgo-20200823T0036.tar.zst"),
    ("3.7", "linux", "x86_64", "musl", "", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-x86_64-unknown-linux-musl-noopt-20200823T0036.tar.zst"),
    ("3.7", "windows", "x86", "msvc", "", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-i686-pc-windows-msvc-shared-pgo-20200823T0159.tar.zst"),
    ("3.7", "windows", "x86_64", "msvc", "", "https://github.com/indygreg/python-build-standalone/releases/download/20200822/cpython-3.7.9-x86_64-pc-windows-msvc-shared-pgo-20200823T0118.tar.zst"),
    ("3.7", "macos", "x86_64", "", "", "https://github.com/indygreg/python-build-standalone/releases/download/20200823/cpython-3.7.9-x86_64-apple-darwin-pgo-20200823T2228.tar.zst"),
];

// See https://downloads.python.org/pypy/
#[rustfmt::skip]
const DEFAULT_PYPY_DISTRIBUTIONS: &[(&str, &str, &str, &str, &str)] = &[
    ("pypy3.10", "linux", "aarch64", "gnu",
        "https://downloads.python.org/pypy/pypy3.10-v7.3.15-aarch64.tar.bz2"),
    ("pypy3.10", "linux", "x86_64", "gnu",
        "https://downloads.python.org/pypy/pypy3.10-v7.3.15-linux64.tar.bz2"),
    ("pypy3.10", "windows", "x86_64", "msvc",
        "https://downloads.python.org/pypy/pypy3.10-v7.3.15-win64.zip"),
    ("pypy3.10", "macos", "aarch64", "",
        "https://downloads.python.org/pypy/pypy3.10-v7.3.15-macos_arm64.tar.bz2"),
    ("pypy3.10", "macos", "x86_64", "",
        "https://downloads.python.org/pypy/pypy3.10-v7.3.15-macos_x86_64.tar.bz2"),
    ("pypy3.9", "linux", "aarch64", "gnu",
        "https://downloads.python.org/pypy/pypy3.9-v7.3.15-aarch64.tar.bz2"),
    ("pypy3.9", "linux", "x86_64", "gnu",
        "https://downloads.python.org/pypy/pypy3.9-v7.3.15-linux64.tar.bz2"),
    ("pypy3.9", "windows", "x86_64", "msvc",
        "https://downloads.python.org/pypy/pypy3.9-v7.3.15-win64.zip"),
    ("pypy3.9", "macos", "aarch64", "",
        "https://downloads.python.org/pypy/pypy3.9-v7.3.15-macos_arm64.tar.bz2"),
    ("pypy3.9", "macos", "x86_64", "",
        "https://downloads.python.org/pypy/pypy3.9-v7.3.15-macos_x86_64.tar.bz2"),
    ("pypy2.7", "linux", "aarch64", "gnu",
        "https://downloads.python.org/pypy/pypy2.7-v7.3.15-aarch64.tar.bz2"),
    ("pypy2.7", "linux", "x86_64", "gnu",
        "https://downloads.python.org/pypy/pypy2.7-v7.3.15-linux64.tar.bz2"),
    ("pypy2.7", "windows", "x86_64", "msvc",
        "https://downloads.python.org/pypy/pypy2.7-v7.3.15-win64.zip"),
    ("pypy2.7", "macos", "aarch64", "",
        "https://downloads.python.org/pypy/pypy2.7-v7.3.15-macos_arm64.tar.bz2"),
    ("pypy2.7", "macos", "x86_64", "",
        "https://downloads.python.org/pypy/pypy2.7-v7.3.15-macos_x86_64.tar.bz2"),
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

fn is_explicitly_disabled(name: &str) -> bool {
    ["false", "0"].contains(&env::var(name).unwrap_or_default().as_str())
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

fn normalize_relative_path(path: &String) -> String {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        path.replace('/', "\\")
            .strip_prefix('\\')
            .unwrap_or(path)
            .strip_suffix('\\')
            .unwrap_or(path)
            .to_string()
    } else {
        path.strip_prefix('/')
            .unwrap_or(path)
            .strip_suffix('/')
            .unwrap_or(path)
            .to_string()
    }
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
    let python_version = env::var("PYAPP_PYTHON_VERSION").unwrap_or_default();
    if !python_version.is_empty() {
        return python_version;
    };

    DEFAULT_PYTHON_VERSION.to_string()
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
        if variant.is_empty()
            && selected_platform == "linux"
            && selected_arch == "x86_64"
            && selected_python_version != "3.7"
        {
            variant = "v3".to_string();
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
        // Force MinGW-w64 to use msvc
        } else if &abi == "gnu" && selected_platform == "windows" {
            abi = "msvc".to_string();
        };
        abi
    };

    for (python_version, platform, arch, abi, variant, url) in DEFAULT_CPYTHON_DISTRIBUTIONS {
        if python_version == &selected_python_version
            && platform == &selected_platform
            && arch == &selected_arch
            && abi == &selected_abi
            && variant == &selected_variant
        {
            return url.to_string();
        }
    }

    for (python_version, platform, arch, abi, url) in DEFAULT_PYPY_DISTRIBUTIONS {
        if python_version == &selected_python_version
            && platform == &selected_platform
            && arch == &selected_arch
            && abi == &selected_abi
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
        match Regex::new(&format!(r"(?m)^{item}: (\S+)"))
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

fn set_project_dependency_file(dependency_file: &str) {
    if dependency_file.is_empty() {
        set_runtime_variable("PYAPP_PROJECT_DEPENDENCY_FILE", "");
        set_runtime_variable("PYAPP__PROJECT_DEPENDENCY_FILE_NAME", "");
        return;
    }

    let path = PathBuf::from(dependency_file);
    if !path.is_file() {
        panic!("\n\nDependency file is not a file: {dependency_file}\n\n");
    }

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let contents = fs::read_to_string(dependency_file)
        .unwrap_or_else(|_| panic!("\n\nFailed to read dependency file {dependency_file}\n\n"));

    set_runtime_variable(
        "PYAPP_PROJECT_DEPENDENCY_FILE",
        STANDARD_NO_PAD.encode(contents),
    );
    set_runtime_variable("PYAPP__PROJECT_DEPENDENCY_FILE_NAME", file_name);
}

fn set_project() {
    let embed_path = embed_file("project");
    let local_path = env::var("PYAPP_PROJECT_PATH").unwrap_or_default();
    if !local_path.is_empty() {
        set_project_dependency_file("");

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

        let dependency_file = env::var("PYAPP_PROJECT_DEPENDENCY_FILE").unwrap_or_default();
        if dependency_file.is_empty() {
            set_project_dependency_file("");
        } else {
            set_project_dependency_file(&dependency_file);
        }

        set_runtime_variable("PYAPP__PROJECT_EMBED_FILE_NAME", "");
        truncate_embed_file(&embed_path);
    }
}

fn set_distribution() {
    let embed_path = embed_file("distribution");
    let mut hasher = PortableHash::default();

    let local_path = env::var("PYAPP_DISTRIBUTION_PATH").unwrap_or_default();
    if !local_path.is_empty()
        && !env::var("PYAPP_DISTRIBUTION_SOURCE")
            .unwrap_or_default()
            .is_empty()
    {
        panic!("\n\nBoth distribution path and source are set\n\n");
    }

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

        let mut file = File::open(&embed_path).unwrap();
        std::io::copy(&mut file, &mut hasher).unwrap();

        local_path
    } else if is_enabled("PYAPP_DISTRIBUTION_EMBED") {
        let distribution_source = get_distribution_source();
        let bytes = reqwest::blocking::get(&distribution_source)
            .unwrap()
            .bytes()
            .unwrap();
        fs::write(&embed_path, bytes).unwrap();

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
    set_site_packages_path(&distribution_source);
    set_distribution_pip_available(&distribution_source);

    let python_isolation_flag = if get_python_version() == "pypy2.7" {
        // https://docs.python.org/2/using/cmdline.html#cmdoption-e
        // https://docs.python.org/2/using/cmdline.html#cmdoption-s
        "-sE"
    } else {
        // https://docs.python.org/3/using/cmdline.html#cmdoption-I
        "-I"
    };
    set_runtime_variable("PYAPP__PYTHON_ISOLATION_FLAG", python_isolation_flag);
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
    } else if distribution_source.ends_with(".tar.bz2") || distribution_source.ends_with(".bz2") {
        set_runtime_variable(variable, "tar|bzip2");
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
    let distribution_variable = "PYAPP_DISTRIBUTION_PYTHON_PATH";
    let on_windows = env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows";
    let python_path = env::var(distribution_variable).unwrap_or_default();
    let mut relative_path = if !python_path.is_empty() {
        normalize_relative_path(&python_path)
    } else if !env::var("PYAPP_DISTRIBUTION_PATH")
        .unwrap_or_default()
        .is_empty()
    {
        panic!("\n\nThe following option must be set when embedding a custom distribution: {distribution_variable}\n\n");
    } else if distribution_source.starts_with(DEFAULT_CPYTHON_SOURCE) {
        if get_python_version() == "3.7" {
            if on_windows {
                r"python\install\python.exe".to_string()
            } else {
                "python/install/bin/python3".to_string()
            }
        } else if on_windows {
            r"python\python.exe".to_string()
        } else {
            "python/bin/python3".to_string()
        }
    } else if distribution_source.starts_with(DEFAULT_PYPY_SOURCE) {
        let directory = distribution_source
            .split('/')
            .last()
            .unwrap()
            .trim_end_matches(".tar.bz2")
            .trim_end_matches(".zip");
        if on_windows {
            format!(r"{}\pypy.exe", directory)
        } else {
            format!("{}/bin/pypy", directory)
        }
    } else if on_windows {
        "python.exe".to_string()
    } else {
        "bin/python3".to_string()
    };

    let path_prefix = env::var("PYAPP_DISTRIBUTION_PATH_PREFIX").unwrap_or_default();
    if !path_prefix.is_empty() {
        if on_windows {
            relative_path = format!(
                r"{}\{}",
                normalize_relative_path(&path_prefix),
                relative_path
            );
        } else {
            relative_path = format!(
                "{}/{}",
                normalize_relative_path(&path_prefix),
                relative_path
            );
        }
    }
    set_runtime_variable(distribution_variable, &relative_path);

    let installation_variable = "PYAPP__INSTALLATION_PYTHON_PATH";
    if is_enabled("PYAPP_FULL_ISOLATION") {
        set_runtime_variable(installation_variable, &relative_path);
    } else if on_windows {
        set_runtime_variable(installation_variable, r"Scripts\python.exe");
    } else {
        set_runtime_variable(installation_variable, "bin/python3");
    };
}

fn set_site_packages_path(distribution_source: &str) {
    let distribution_variable = "PYAPP_DISTRIBUTION_SITE_PACKAGES_PATH";
    let on_windows = env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows";
    let python_version = get_python_version();
    let site_packages_path = env::var(distribution_variable).unwrap_or_default();
    let mut relative_path = if !site_packages_path.is_empty() {
        normalize_relative_path(&site_packages_path)
    } else if distribution_source.starts_with(DEFAULT_CPYTHON_SOURCE) {
        if python_version == "3.7" {
            if on_windows {
                r"python\install\Lib\site-packages".to_string()
            } else {
                format!("python/install/lib/python{}/site-packages", python_version)
            }
        } else if on_windows {
            r"python\Lib\site-packages".to_string()
        } else {
            format!("python/lib/python{}/site-packages", python_version)
        }
    } else if distribution_source.starts_with(DEFAULT_PYPY_SOURCE) {
        let directory = distribution_source
            .split('/')
            .last()
            .unwrap()
            .trim_end_matches(".tar.bz2")
            .trim_end_matches(".zip");
        if python_version == "pypy2.7" {
            if on_windows {
                format!(r"{}\site-packages", directory)
            } else {
                format!("{}/site-packages", directory)
            }
        } else if on_windows {
            format!(r"{}\Lib\site-packages", directory)
        } else {
            format!("{}/lib/{}/site-packages", directory, python_version)
        }
    } else if on_windows {
        r"Lib\site-packages".to_string()
    } else {
        format!("lib/python{}/site-packages", python_version)
    };

    let path_prefix = env::var("PYAPP_DISTRIBUTION_PATH_PREFIX").unwrap_or_default();
    if !path_prefix.is_empty() {
        if on_windows {
            relative_path = format!(
                r"{}\{}",
                normalize_relative_path(&path_prefix),
                relative_path
            );
        } else {
            relative_path = format!(
                "{}/{}",
                normalize_relative_path(&path_prefix),
                relative_path
            );
        }
    }
    set_runtime_variable(distribution_variable, &relative_path);

    let installation_variable = "PYAPP__INSTALLATION_SITE_PACKAGES_PATH";
    if is_enabled("PYAPP_FULL_ISOLATION") {
        set_runtime_variable(installation_variable, &relative_path);
    } else if get_python_version() == "pypy2.7" {
        set_runtime_variable(installation_variable, "site-packages");
    } else if on_windows {
        set_runtime_variable(installation_variable, r"Lib\site-packages");
    } else {
        set_runtime_variable(
            installation_variable,
            format!("lib/python{}/site-packages", python_version),
        );
    };
}

fn set_distribution_pip_available(distribution_source: &str) {
    let variable = "PYAPP_DISTRIBUTION_PIP_AVAILABLE";
    if is_enabled(variable)
        // Enable if a default source is used and known to have pip installed already
        || (!distribution_source.is_empty()
            && !distribution_source.starts_with(DEFAULT_PYPY_SOURCE)
            && env::var("PYAPP_DISTRIBUTION_PATH")
                .unwrap_or_default()
                .is_empty()
            && env::var("PYAPP_DISTRIBUTION_SOURCE")
                .unwrap_or_default()
                .is_empty())
    {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_execution_mode() {
    let module_variable = "PYAPP_EXEC_MODULE";
    let module = env::var(module_variable).unwrap_or_default();

    let spec_variable = "PYAPP_EXEC_SPEC";
    let spec = env::var(spec_variable).unwrap_or_default();

    let code_variable = "PYAPP_EXEC_CODE";
    let code = env::var(code_variable).unwrap_or_default();

    let script_variable = "PYAPP_EXEC_SCRIPT";
    let script = env::var(script_variable).unwrap_or_default();

    let notebook_variable = "PYAPP_EXEC_NOTEBOOK";
    let notebook = env::var(notebook_variable).unwrap_or_default();

    // Set defaults
    set_runtime_variable(module_variable, "");
    set_runtime_variable(code_variable, "");
    set_runtime_variable("PYAPP__EXEC_CODE_ENCODED", "0");
    set_runtime_variable(script_variable, "");
    set_runtime_variable("PYAPP__EXEC_SCRIPT_NAME", "");
    set_runtime_variable("PYAPP__EXEC_SCRIPT_ID", "");
    set_runtime_variable(notebook_variable, "");
    set_runtime_variable("PYAPP__EXEC_NOTEBOOK_NAME", "");
    set_runtime_variable("PYAPP__EXEC_NOTEBOOK_ID", "");

    if [
        module.is_empty(),
        spec.is_empty(),
        code.is_empty(),
        script.is_empty(),
        notebook.is_empty(),
    ]
    .iter()
    .filter(|x| !(**x))
    .count()
        > 1
    {
        panic!("\n\nThe {module_variable}, {spec_variable}, {code_variable}, {script_variable}, and {notebook_variable} options are mutually exclusive\n\n");
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
        set_runtime_variable(code_variable, STANDARD_NO_PAD.encode(code));
        set_runtime_variable("PYAPP__EXEC_CODE_ENCODED", "1");
    } else if !script.is_empty() {
        let path = PathBuf::from(&script);
        if !path.is_file() {
            panic!("\n\nScript is not a file: {script}\n\n");
        }

        let file_name = path.file_name().unwrap().to_str().unwrap();
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("\n\nFailed to read script: {script}\n\n"));
        let mut hasher = PortableHash::default();
        hasher.write(contents.as_bytes());

        set_runtime_variable(script_variable, STANDARD_NO_PAD.encode(contents));
        set_runtime_variable("PYAPP__EXEC_SCRIPT_NAME", file_name);
        set_runtime_variable("PYAPP__EXEC_SCRIPT_ID", hasher.finish());
    } else if !notebook.is_empty() {
        let path = PathBuf::from(&notebook);
        if !path.is_file() {
            panic!("\n\nNotebook is not a file: {notebook}\n\n");
        }

        let file_name = path.file_name().unwrap().to_str().unwrap();
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("\n\nFailed to read notebook: {notebook}\n\n"));
        let mut hasher = PortableHash::default();
        hasher.write(contents.as_bytes());

        set_runtime_variable(notebook_variable, STANDARD_NO_PAD.encode(contents));
        set_runtime_variable("PYAPP__EXEC_NOTEBOOK_NAME", file_name);
        set_runtime_variable("PYAPP__EXEC_NOTEBOOK_ID", hasher.finish());
    } else {
        set_runtime_variable(
            module_variable,
            normalize_project_name(&env::var("PYAPP_PROJECT_NAME").unwrap_or_default())
                .replace('-', "_"),
        );
    }
}

fn set_is_gui() {
    let variable = "PYAPP_IS_GUI";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_isolation_mode() {
    let variable = "PYAPP_FULL_ISOLATION";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_upgrade_virtualenv() {
    let variable = "PYAPP_UPGRADE_VIRTUALENV";
    if is_enabled(variable) || get_python_version() == "pypy2.7" {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_pip_external() {
    let variable = "PYAPP_PIP_EXTERNAL";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_pip_version() {
    let variable = "PYAPP_PIP_VERSION";
    set_runtime_variable(variable, env::var(variable).unwrap_or("latest".to_string()));
}

fn set_pip_project_features() {
    let variable = "PYAPP_PROJECT_FEATURES";
    set_runtime_variable(variable, env::var(variable).unwrap_or_default());
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

fn set_uv_enabled() {
    let variable = "PYAPP_UV_ENABLED";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_uv_only_bootstrap() {
    let variable = "PYAPP_UV_ONLY_BOOTSTRAP";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
    } else {
        set_runtime_variable(variable, "0");
    }
}

fn set_uv_version() {
    let variable = "PYAPP_UV_VERSION";
    let version = env::var(variable).unwrap_or("any".to_string());
    set_runtime_variable(variable, version);

    let artifact_name = if !is_enabled("PYAPP_UV_ENABLED") {
        "".to_string()
    } else if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // Force MinGW-w64 to use msvc
        if env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "gnu" {
            format!(
                "uv-{}-pc-windows-msvc.zip",
                env::var("CARGO_CFG_TARGET_ARCH").unwrap()
            )
        } else {
            format!("uv-{}.zip", env::var("TARGET").unwrap())
        }
    } else {
        format!("uv-{}.tar.gz", env::var("TARGET").unwrap())
    };
    set_runtime_variable("PYAPP__UV_ARTIFACT_NAME", artifact_name);
}

fn set_skip_install() {
    let variable = "PYAPP_SKIP_INSTALL";
    if is_enabled(variable) {
        set_runtime_variable(variable, "1");
        if is_enabled("PYAPP_ALLOW_UPDATES") {
            set_runtime_variable("PYAPP_EXPOSE_UPDATE", "1");
        } else {
            set_runtime_variable("PYAPP_EXPOSE_UPDATE", "0");
        }
    } else {
        set_runtime_variable(variable, "0");
        set_runtime_variable("PYAPP_EXPOSE_UPDATE", "1");
    }
}

fn set_allow_updates() {
    let variable = "PYAPP_ALLOW_UPDATES";
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
        set_runtime_variable("PYAPP__EXPOSED_COMMAND", "");
    } else if !command_name.is_empty() {
        set_runtime_variable(variable, &command_name);
        set_runtime_variable("PYAPP__EXPOSED_COMMAND", &command_name);
    } else {
        set_runtime_variable(variable, "self");
        set_runtime_variable("PYAPP__EXPOSED_COMMAND", "self");
    }
}

fn set_exposed_command(path: &Path, command_name: &str, indicator: &Regex) {
    if !path.is_file() {
        return;
    }

    let command_path = path.to_str().unwrap();
    let command_source = fs::read_to_string(command_path).unwrap();
    if indicator.is_match(&command_source) {
        let variable = format!("PYAPP_EXPOSE_{}", command_name.to_uppercase());
        if is_enabled(&variable)
            || (is_enabled("PYAPP_EXPOSE_ALL_COMMANDS") && !is_explicitly_disabled(&variable))
        {
            set_runtime_variable(&variable, "1");
        } else {
            set_runtime_variable(&variable, "0");
        }
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

    for entry in fs::read_dir(&commands_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        set_exposed_command(
            &path,
            path.file_stem().unwrap().to_str().unwrap(),
            &indicator,
        );
    }

    let command_groups = ["cache"];
    for command_group in command_groups {
        set_exposed_command(
            &commands_dir.join(command_group).join("cli.rs"),
            command_group,
            &indicator,
        );
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
    set_is_gui();
    set_isolation_mode();
    set_upgrade_virtualenv();
    set_pip_external();
    set_pip_version();
    set_pip_project_features();
    set_pip_extra_args();
    set_pip_allow_config();
    set_uv_enabled();
    set_uv_only_bootstrap();
    set_uv_version();
    set_allow_updates();
    set_indicator();
    set_self_command();
    set_exposed_commands();
    set_metadata_template();

    // This must come last because it might override a command exposure
    set_skip_install();
}
