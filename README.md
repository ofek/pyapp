# PyApp

[![CI - Test](https://github.com/ofek/pyapp/actions/workflows/test.yml/badge.svg)](https://github.com/ofek/pyapp/actions/workflows/test.yml)
[![CD - Publish](https://github.com/ofek/pyapp/actions/workflows/publish.yml/badge.svg)](https://github.com/ofek/pyapp/actions/workflows/publish.yml)
[![Project - Version](https://img.shields.io/crates/v/pyapp)](https://crates.io/crates/pyapp)
[![Project - Downloads](https://img.shields.io/crates/d/pyapp)](https://crates.io/crates/pyapp)
[![License - Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-9400d3.svg)](https://spdx.org/licenses/)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/ofek?logo=GitHub%20Sponsors&style=social)](https://github.com/sponsors/ofek)

-----

PyApp is a CLI wrapper for Python applications that bootstrap themselves at runtime. Each application is configured with environment variables at build time.

For a more streamlined workflow, consider using the built-in [app](https://hatch.pypa.io/latest/plugins/builder/app/) build target of [Hatch](https://github.com/pypa/hatch).

**Table of Contents**

- [Building](#building)
  - [Local repository](#local-repository)
  - [Installation](#installation)
- [Runtime behavior](#runtime-behavior)
  - [Initialization](#initialization)
  - [Detection](#detection)
  - [pip](#pip)
  - [Commands](#commands)
    - [Default](#default)
      - [Restore](#restore)
      - [Update](#update)
    - [Optional](#optional)
      - [Metadata](#metadata)
- [Configuration](#configuration)
  - [Project](#project)
    - [Package index](#package-index)
    - [Embedding](#embedding)
  - [Execution mode](#execution-mode)
  - [Python distribution](#python-distribution)
    - [Known](#known)
      - [CPython](#cpython)
    - [Custom](#custom)
      - [Format](#format)
      - [Python location](#python-location)
    - [Embedding](#embedding-1)
  - [pip](#pip-1)
    - [Extra arguments](#extra-arguments)
    - [Allowing configuration](#allowing-configuration)
  - [Skipping project installation](#skipping-project-installation)
  - [Installation indicator](#installation-indicator)
  - [Management command name](#management-command-name)
  - [Metadata template](#metadata-template)
- [Cross compilation](#cross-compilation)
- [TODO](#todo)
- [License](#license)

## Building

Before building your application, you must [configure](#configuration) your [project](#project) at the very least.

After you have done that, there are 2 ways to build your application.

### Local repository

Clone this repository then enter the cloned directory and run:

```
cargo build --release
```

The executable will be located at `target/release/pyapp.exe` if on Windows or `target/release/pyapp` otherwise. If a particular [target](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget) has been set (or if [cross](https://github.com/cross-rs/cross) is used since it always sets one), then the `release` directory will be nested one level deeper under `target/<TARGET>`.

### Installation

Select the directory in which to build the executable with the `--root` option and run:

```
cargo install pyapp --force --root <DIR>
```

The executable will be located at `<DIR>/bin/pyapp.exe` if on Windows or `<DIR>/bin/pyapp` otherwise.

***Note:*** If you want to cross compile using [cross](https://github.com/cross-rs/cross), this method of building is currently [unsupported](https://github.com/cross-rs/cross/issues/1215).

## Runtime behavior

### Initialization

On the first run of the application:

1. the distribution (if not [embedded](#embedding)) will be downloaded and cached
2. the distribution will be unpacked
3. the project will be installed

All subsequent invocations will only check if the installation directory exists and nothing else, to maximize CLI responsiveness.

### Detection

A single environment variable called `PYAPP` is injected with the value of `1` ([by default](#installation-indicator)) when running applications and may be used to detect this mode of installation versus others.

### pip

When installing or upgrading projects, [pip](https://github.com/pypa/pip) uses [isolation](https://pip.pypa.io/en/stable/cli/pip/#cmdoption-isolated) ([by default](#allowing-configuration)) to provide consistent behavior on each user's machine.

### Commands

Built applications have a single top-level command group named `self` ([by default](#management-command-name)) and all other invocations will be forwarded to your actual [execution logic](#execution-mode).

#### Default

These commands are always exposed.

##### Restore

```
<EXE> self restore
```

This will wipe the installation and start fresh.

##### Update

```
<EXE> self update
```

This will update the project to the latest available version in the currently used distribution.

#### Optional

These commands are hidden by default and each can be individually exposed by setting its corresponding `PYAPP_EXPOSE_<COMMAND>` option (e.g. `PYAPP_EXPOSE_METADATA`) to `true` or `1`.

##### Metadata

```
<EXE> self metadata
```

This displays [customized](#metadata-template) output based on a template.

## Configuration

All configuration is done with environment variables.

### Project

There are 2 ways to configure runtime installation, neither of which will occur when [disabled](#skipping-project-installation).

#### Package index

The desired project name and version are configured with the `PYAPP_PROJECT_NAME` and `PYAPP_PROJECT_VERSION` options, respectively. The project name must adhere to [PEP 508](https://peps.python.org/pep-0508/#names) and will be normalized during builds according to [PEP 503](https://peps.python.org/pep-0503/#normalized-names).

#### Embedding

You may embed the project with the `PYAPP_PROJECT_PATH` option which should be a path to a wheel ending in `.whl` or a source distribution ending in `.tar.gz`.

### Execution mode

The following options are mutually exclusive:

| Option | Description |
| --- | --- |
| `PYAPP_EXEC_MODULE` | This is the name of the module to execute via `python -m <MODULE>` |
| `PYAPP_EXEC_SPEC` | This is an [object reference](https://packaging.python.org/en/latest/specifications/entry-points/#data-model) to execute e.g. `pkg.foo:cli` |
| `PYAPP_EXEC_CODE` | This is arbitrary code to run via `python -c <CODE>` (the spec option uses this internally) |

If none are set then the `PYAPP_EXEC_MODULE` option will default to the value of `PYAPP_PROJECT_NAME` with hyphens replaced by underscores.

### Python distribution

#### Known

Setting the `PYAPP_PYTHON_VERSION` option will determine the distribution used at runtime based on the environment at build time. If unset then the default will be the latest stable minor version of [CPython](#cpython).

##### CPython

| ID |
| --- |
| `3.7` |
| `3.8` |
| `3.9` |
| `3.10` |
| `3.11` |

The source of distributions is the [python-build-standalone](https://github.com/indygreg/python-build-standalone) project.

Some distributions have [variants](https://gregoryszorc.com/docs/python-build-standalone/main/running.html) that may be configured with the `PYAPP_DISTRIBUTION_VARIANT` option:

| Platform | Options |
| --- | --- |
| Linux | <ul><li><code>v1</code></li><li><code>v2</code></li><li><code>v3</code> (default)</li><li><code>v4</code></li></ul> |
| Windows | <ul><li><code>shared</code> (default)</li><li><code>static</code></li></ul> |

#### Custom

You may explicitly set the `PYAPP_DISTRIBUTION_SOURCE` option which overrides the [known](#known) distribution settings. The source must be a URL that points to an archived version of the desired Python distribution.

Setting this manually may require you to define extra metadata about the distribution that is required for accurate [runtime behavior](#runtime-behavior).

##### Format

The following formats are supported for the `PYAPP_DISTRIBUTION_FORMAT` option, with the default chosen based on the ending of the source URL:

| Format | Extensions | Description |
| --- | --- | --- |
| `tar\|gzip` | <ul><li><code>.tar.gz</code></li><li><code>.tgz</code></li></ul> | A [tar file](https://en.wikipedia.org/wiki/Tar_(computing)) with [gzip compression](https://en.wikipedia.org/wiki/Gzip) |
| `tar\|zstd` | <ul><li><code>.tar.zst</code></li><li><code>.tar.zstd</code></li></ul> | A [tar file](https://en.wikipedia.org/wiki/Tar_(computing)) with [Zstandard compression](https://en.wikipedia.org/wiki/Zstd) |
| `zip` | <ul><li><code>.zip</code></li></ul> | A [ZIP file](https://en.wikipedia.org/wiki/ZIP_(file_format)) with [DEFLATE compression](https://en.wikipedia.org/wiki/Deflate) |

##### Python location

You may set the relative path to the Python executable after unpacking the archive with the `PYAPP_DISTRIBUTION_PYTHON_PATH` option. The default is `python.exe` on Windows and `bin/python3` on all other platforms.

#### Embedding

You may set the `PYAPP_DISTRIBUTION_EMBED` option to `true` or `1` to embed the distribution in the executable at build time to avoid fetching it at runtime. When distribution embedding is enabled, you can set the `PYAPP_DISTRIBUTION_PATH` option to use a local path rather than fetching the source.

### pip

These options have no effect when the project installation is [disabled](#skipping-project-installation).

#### Extra arguments

You may set the `PYAPP_PIP_EXTRA_ARGS` option to provide extra arguments to the [`pip install`](https://pip.pypa.io/en/stable/cli/pip_install/) command at runtime when installing or updating the project e.g. `--index-url URL --only-binary :all:`.

#### Allowing configuration

You may set the `PYAPP_PIP_ALLOW_CONFIG` option to `true` or `1` to allow the use of environment variables and other configuration at runtime.

### Skipping project installation

You may set the `PYAPP_SKIP_INSTALL` option to `true` or `1` to skip installing the project in the distribution. This allows for entirely predefined distributions and thus no network calls at runtime if used in conjunction with [embedding](#embedding).

### Installation indicator

The environment variable that is used for [detection](#detection) may be set to the path of the executable at runtime if you set the `PYAPP_PASS_LOCATION` option to `true` or `1`. This is useful if your application wishes to in some way manage itself.

### Management command name

You may set the `PYAPP_SELF_COMMAND` option to override the default name (`self`) of the [management command group](#commands), useful if you wish to have complete control of the interface. You may set this to `none` to effectively disable the use of management commands.

### Metadata template

You may set a custom template used to [output metadata](#metadata) with the `PYAPP_METADATA_TEMPLATE` option which supports the following placeholders:

| Placeholder | Description |
| --- | --- |
| `{project}` | The normalized project name |
| `{version}` | The currently installed version of the project |

The default template is `{project} v{version}` if this option is unset.

This is useful for setting [custom commands](https://starship.rs/config/#custom-commands) for the [Starship](https://github.com/starship/starship) prompt. The following example configuration assumes that the built executable has been renamed to `foo`:

````toml
format = """
...
${custom.foo}\
...
$line_break\
...
$character"""

# <clipped>

[custom.foo]
command = "foo self metadata"
when = true
## Windows
# shell = ["cmd", "/C"]
## Other
# shell = ["sh", "--norc"]
````

## Cross compilation

Configuration for [cross](https://github.com/cross-rs/cross) is validated by CI to ensure all known environment variable options are passed through to the containers.

When embedding the [project](#embedding) or the [distribution](#embedding-1) using a local path, you must use the [local repository](#local-repository) way of building and ensure that the configured files to embed reside within the repository and the options refer to relative paths.

## TODO

- Support PyPy [stable versions](https://www.pypy.org/download.html) and [nightlies](https://buildbot.pypy.org/nightly/)

## License

PyApp is distributed under the terms of any of the following licenses:

- [Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)
- [MIT](https://spdx.org/licenses/MIT.html)
