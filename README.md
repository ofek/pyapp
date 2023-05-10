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
- [Runtime behavior](#runtime-behavior)
  - [Initialization](#initialization)
  - [Commands](#commands)
    - [Exposed](#exposed)
      - [Restore](#restore)
      - [Update](#update)
    - [Hidden](#hidden)
      - [Starship](#starship)
  - [Detection](#detection)
- [Configuration](#configuration)
  - [Project](#project)
  - [Execution mode](#execution-mode)
  - [Python distribution](#python-distribution)
    - [Known](#known)
      - [CPython](#cpython)
    - [Custom](#custom)
      - [Format](#format)
      - [Python location](#python-location)
    - [Embedding](#embedding)
  - [Skipping project installation](#skipping-project-installation)
  - [Installation indicator](#installation-indicator)
  - [Management command name](#management-command-name)
  - [Starship prompt](#starship-prompt)
- [TODO](#todo)
- [License](#license)

## Building

Select the directory in which to build your application with the `--root` option:

```
cargo install pyapp --force --root <DIR>
```

The executable will be located at `<DIR>/bin/pyapp.exe` if on Windows or `<DIR>/bin/pyapp` otherwise.

## Runtime behavior

### Initialization

On the first run of the application:

1. the distribution (if not [embedded](#embedding)) will be downloaded and cached
2. the distribution will be unpacked
3. the project will be installed

All subsequent invocations will only check if the unpacked distribution directory exists and nothing else, to maximize CLI responsiveness.

### Commands

Built applications have a single top-level command group named `self` ([by default](#management-command-name)) and all other invocations will be forwarded to your actual [execution logic](#execution-mode).

#### Exposed

##### Restore

```
<EXE> self restore
```

This will wipe the unpacked distribution and start fresh.

##### Update

```
<EXE> self update
```

This will update the project to the latest available version in the currently used distribution.

#### Hidden

##### Starship

```
<EXE> self starship
```

This displays [customized](#starship-prompt) output that may be used by the [Starship](https://github.com/starship/starship) prompt.

### Detection

A single environment variable called `PYAPP` is injected with the value of `1` when running applications and may be used to detect this mode of installation versus others.

## Configuration

All configuration is done with environment variables.

### Project

The desired project name and version are configured with the `PYAPP_PROJECT_NAME` and `PYAPP_PROJECT_VERSION` options, respectively. The project name must adhere to [PEP 508](https://peps.python.org/pep-0508/#names) and will be normalized during builds according to [PEP 503](https://peps.python.org/pep-0503/#normalized-names).

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

You may set the `PYAPP_DISTRIBUTION_EMBED` option to `true` or `1` to embed the distribution in the executable at build time to avoid fetching it at runtime.

### Skipping project installation

You may set the `PYAPP_SKIP_INSTALL` option to `true` or `1` to skip installing the project in the distribution. This allows for entirely predefined distributions and thus no network calls at runtime if used in conjunction with [embedding](#embedding).

### Installation indicator

The environment variable that is used for [detection](#detection) may be set to the path of the executable at runtime if you set the `PYAPP_PASS_LOCATION` option to `true` or `1`. This is useful if your application wishes to in some way manage itself.

### Management command name

You may set the `PYAPP_SELF_COMMAND` option to override the default name (`self`) of the [management command](#commands). This is useful if you wish to have complete control of the interface or to set it to a bogus value with the intention of not using it.

### Starship prompt

You may set a [custom command](https://starship.rs/config/#custom-commands) for the [Starship](https://github.com/starship/starship) prompt with the `PYAPP_STARSHIP_PROMPT` option which supports the following placeholders:

| Placeholder | Description |
| --- | --- |
| `{project}` | The normalized project name |
| `{version}` | The currently installed version of the project |

The default [output](#starship) is `{project} v{version}` if this option is unset.

The following example configuration assumes that the built executable has been renamed to `foo`:

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
command = "foo self starship"
when = true
## Windows
# shell = ["cmd", "/C"]
## Other
# shell = ["sh", "--norc"]
````

## TODO

- Support PyPy [stable versions](https://www.pypy.org/download.html) and [nightlies](https://buildbot.pypy.org/nightly/)
- Support `PYAPP_PIP_INDEX_URL` and `PYAPP_PIP_EXTRA_INDEX_URL` build time options that correspond to the `--index-url` and `--extra-index-url` flags of the `pip install` command, respectively
- Add a `PYAPP_PIP_EXTERNAL` build time option that indicates the distribution does not ship with `pip` and will use its [standalone installation](https://pip.pypa.io/en/stable/installation/#standalone-zip-application) (note that this may be the default behavior in future depending on feedback)

## License

PyApp is distributed under the terms of any of the following licenses:

- [Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)
- [MIT](https://spdx.org/licenses/MIT.html)
