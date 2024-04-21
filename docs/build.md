# Building

-----

Before building your application, you must configure your [project](config/project.md) at the very least.

After you have done that, your application can be built using a [local copy](#local-repository) of this repository or via [installation](#installation) with Cargo.

!!! tip
    For a more streamlined workflow, consider using the built-in [app](https://hatch.pypa.io/latest/plugins/builder/app/) build target of [Hatch](https://github.com/pypa/hatch).

## Local repository

The recommended method of downloading the repository is via the `source` asset archives attached to each [release](https://github.com/ofek/pyapp/releases).

The latest release can always be downloaded at the following URLs:

- https://github.com/ofek/pyapp/releases/latest/download/source.tar.gz
- https://github.com/ofek/pyapp/releases/latest/download/source.zip

Specific releases may be downloaded at the following URLs, replacing `X.Y.Z` with the desired version:

- https://github.com/ofek/pyapp/releases/download/vX.Y.Z/source.tar.gz
- https://github.com/ofek/pyapp/releases/download/vX.Y.Z/source.zip

After unpacking the repository, enter the directory and run:

```
cargo build --release
```

The executable will be located at `target/release/pyapp.exe` if on Windows or `target/release/pyapp` otherwise. If a particular [target](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget) has been set (or when [cross compiling](#cross-compilation) since one will always be set), then the `release` directory will be nested one level deeper under `target/<TARGET>`.

## Installation

Select the directory in which to build the executable with the `--root` option and run:

```
cargo install pyapp --force --root <DIR>
```

The executable will be located at `<DIR>/bin/pyapp.exe` if on Windows or `<DIR>/bin/pyapp` otherwise.

Specific versions may be chosen with the `--version` flag.

***Note:*** If you want to [cross compile](#cross-compilation), this method of building is currently [unsupported](https://github.com/cross-rs/cross/issues/1215).

## Cross compilation

Configuration for [cross](https://github.com/cross-rs/cross) is validated by CI to ensure all known environment variable options are passed through to the containers.

When embedding the [project](config/project.md#embedding) or the [distribution](config/distribution.md#embedding) using a local path, you must use the [local repository](#local-repository) way of building and ensure that the configured files to embed reside within the repository and the options refer to relative paths.
