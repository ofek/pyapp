# Configuration

-----

All configuration is done with environment variables.

## Project

There are 3 ways to configure runtime installation, none of which will occur when [disabled](#skipping-project-installation).

The project name and version must be known in all cases.

### Package index

The desired project name and version are configured with the `PYAPP_PROJECT_NAME` and `PYAPP_PROJECT_VERSION` options, respectively. The project name must adhere to [PEP 508](https://peps.python.org/pep-0508/#names) and will be normalized during builds according to [PEP 503](https://peps.python.org/pep-0503/#normalized-names).

#### Dependency file

You may install your project using a dependency file with the `PYAPP_PROJECT_DEPENDENCY_FILE` option which should be a local path to the file. In this mode, the project name and version have nothing to do with installation and are just used as metadata.

The following formats are supported:

| Extensions | Description |
| --- | --- |
| <code>.txt</code><br><code>.in</code> | This is the [requirements file format](https://pip.pypa.io/en/stable/reference/requirements-file-format/) |

### Embedding ### {: #project-embedding }

You may embed the project with the `PYAPP_PROJECT_PATH` option which should be a path to a wheel ending in `.whl` or a source distribution ending in `.tar.gz`.

!!! note
    The project name and version is automatically derived from the metadata files inside.

## Features (extras) ## {: #project-features }

You may set the `PYAPP_PROJECT_FEATURES` option to select [optional dependency groups](https://packaging.python.org/en/latest/specifications/dependency-specifiers/#extras) that would usually be passed to installers within square brackets after the package name e.g. `pkg[foo,bar]`. In that example, you would set `PYAPP_PROJECT_FEATURES` to `foo,bar`.

This also works when [embedding the project](#project-embedding).

## Execution mode

The following options are mutually exclusive:

| Option | Description |
| --- | --- |
| `PYAPP_EXEC_MODULE` | This is the name of the module to execute via `python -m <MODULE>` |
| `PYAPP_EXEC_SPEC` | This is an [object reference](https://packaging.python.org/en/latest/specifications/entry-points/#data-model) to execute e.g. `pkg.foo:cli` |
| `PYAPP_EXEC_CODE` | This is arbitrary code to run via `python -c <CODE>` (the spec option uses this internally) |
| `PYAPP_EXEC_SCRIPT` | This is a path to a script to embed in the binary and run |
| `PYAPP_EXEC_NOTEBOOK` | This is a path to a [Jupyter notebook](https://docs.jupyter.org/en/latest/) (`.ipynb` file) to embed in the binary and run |

If none are set then the `PYAPP_EXEC_MODULE` option will default to the value of `PYAPP_PROJECT_NAME` with hyphens replaced by underscores.

### GUI

If you are packaging a graphical user interface (GUI), you can set  `PYAPP_IS_GUI` to `true` or `1`.

On Windows, this will use `pythonw.exe` instead of `python.exe` to execute [the application](https://docs.python.org/3/using/windows.html#python-application), which avoids a console window from appearing. Running a GUI application with `pythonw.exe` means that all `stdout` and `stderr` output from your GUI will be discarded.

Otherwise, the application will execute as usual. PyApp will run your GUI by spawning a new process, such that the console window that calls runs the application terminates after successful spawning.

Even when `PYAPP_IS_GUI` is enabled you can still run the application from the command line. Furthermore, PyApp-specific logic (e.g. installation and setup) will still display a console window with status messages.

!!! note
    On macOS, the console by default does not automatically close when processes have terminated (however it can be closed manually without interferring with the GUI). The default console behavior [can be changed](https://stackoverflow.com/questions/5560167/osx-how-to-auto-close-terminal-window-after-the-exit-command-executed) in the user settings to close after the last process terminated successfully.

## Python distribution

### Known

Setting the `PYAPP_PYTHON_VERSION` option will determine the distribution used at runtime based on the environment at build time. If unset then the default will be the latest stable minor version of [CPython](#cpython).

#### CPython

| ID |
| --- |
| `3.7` |
| `3.8` |
| `3.9` |
| `3.10` |
| `3.11` |
| `3.12` |

The source for pre-built distributions is the [python-build-standalone](https://github.com/indygreg/python-build-standalone) project.

Some distributions have [variants](https://gregoryszorc.com/docs/python-build-standalone/main/running.html) that may be configured with the `PYAPP_DISTRIBUTION_VARIANT` option:

| Platform | Options |
| --- | --- |
| Linux | <ul><li><code>v1</code></li><li><code>v2</code></li><li><code>v3</code> (default)</li><li><code>v4</code></li></ul> |
| Windows | <ul><li><code>shared</code> (default)</li><li><code>static</code></li></ul> |

#### PyPy

| ID |
| --- |
| `pypy2.7` |
| `pypy3.9` |
| `pypy3.10` |

The source of distributions is the [PyPy](https://www.pypy.org) project.

### Custom

You may explicitly set the `PYAPP_DISTRIBUTION_SOURCE` option which overrides the [known](#known) distribution settings. The source must be a URL that points to an archived version of the desired Python distribution.

Setting this manually may require you to define extra metadata about the distribution that is required for correct [runtime behavior](runtime.md).

#### Format

The following formats are supported for the `PYAPP_DISTRIBUTION_FORMAT` option, with the default chosen based on the ending of the source URL:

| Format | Extensions | Description |
| --- | --- | --- |
| `tar|bzip2` | <ul><li><code>.tar.bz2</code></li><li><code>.bz2</code></li></ul> | A [tar file](https://en.wikipedia.org/wiki/Tar_(computing)) with [bzip2 compression](https://en.wikipedia.org/wiki/Bzip2) |
| `tar|gzip` | <ul><li><code>.tar.gz</code></li><li><code>.tgz</code></li></ul> | A [tar file](https://en.wikipedia.org/wiki/Tar_(computing)) with [gzip compression](https://en.wikipedia.org/wiki/Gzip) |
| `tar|zstd` | <ul><li><code>.tar.zst</code></li><li><code>.tar.zstd</code></li></ul> | A [tar file](https://en.wikipedia.org/wiki/Tar_(computing)) with [Zstandard compression](https://en.wikipedia.org/wiki/Zstd) |
| `zip` | <ul><li><code>.zip</code></li></ul> | A [ZIP file](https://en.wikipedia.org/wiki/ZIP_(file_format)) with [DEFLATE compression](https://en.wikipedia.org/wiki/Deflate) |

#### Python location

You may set the relative path to the Python executable after unpacking the archive with the `PYAPP_DISTRIBUTION_PYTHON_PATH` option. The default is `python.exe` on Windows and `bin/python3` on all other platforms.

#### Site packages location

You may set the relative path to the [`site-packages`](https://docs.python.org/3/library/site.html) directory after unpacking the archive with the `PYAPP_DISTRIBUTION_SITE_PACKAGES_PATH` option. The default is `Lib\site-packages` on Windows and `lib/python<ID>/site-packages` on all other platforms where `<ID>` is the [distribution ID](#known) is defined.

#### pip availability

You may indicate whether pip is already installed by setting the `PYAPP_DISTRIBUTION_PIP_AVAILABLE` option to `true` or `1`. This elides the check for installation when [upgraded virtual environments](#virtual-environments) are enabled.

### Embedding ### {: #distribution-embedding }

You may set the `PYAPP_DISTRIBUTION_EMBED` option to `true` or `1` to embed the distribution in the executable at build time to avoid fetching it at runtime.

You can set the `PYAPP_DISTRIBUTION_PATH` option to use a local path rather than fetching the source, which implicitly enables embedding. The local archive should be similar to the [default distributions](#python-distribution) in that there should be a Python interpreter ready for use.

## pip

These options have no effect when the project installation is [disabled](#skipping-project-installation).

### Externally managed

You may set the `PYAPP_PIP_EXTERNAL` option to `true` or `1` to use the [standalone](https://pip.pypa.io/en/stable/installation/#standalone-zip-application) versions of pip rather than whatever the distribution provides.

By default, the latest version is used. You may use a specific `X.Y.Z` version by setting the `PYAPP_PIP_VERSION` option.

!!! tip
    This provides a significant installation speed up when [full isolation](#full-isolation) is not enabled.

### Extra arguments

You may set the `PYAPP_PIP_EXTRA_ARGS` option to provide extra arguments to the [`pip install`](https://pip.pypa.io/en/stable/cli/pip_install/) command at runtime when installing or updating the project e.g. `--only-binary :all: --index-url URL`.

### Allowing configuration

You may set the `PYAPP_PIP_ALLOW_CONFIG` option to `true` or `1` to allow the use of environment variables and other configuration at runtime.

## Full isolation

You may set the `PYAPP_FULL_ISOLATION` option to `true` or `1` to provide each installation with a full copy of the distribution rather than a virtual environment.

## Virtual environments

When [full isolation](#full-isolation) is not enabled, you may set the `PYAPP_UPGRADE_VIRTUALENV` option to `true` or `1` to create virtual environments with [virtualenv](https://github.com/pypa/virtualenv) rather than the standard library's `venv` module.

## Skipping project installation

You may set the `PYAPP_SKIP_INSTALL` option to `true` or `1` to skip installing the project in the distribution. This allows for entirely predefined distributions and thus no network calls at runtime if used in conjunction with [distribution embedding](#distribution-embedding).

## Installation indicator

The environment variable that is used for [detection](runtime.md#detection) may be set to the path of the executable at runtime if you set the `PYAPP_PASS_LOCATION` option to `true` or `1`. This is useful if your application wishes to in some way manage itself.

## Management command

You may set the `PYAPP_SELF_COMMAND` option to override the default name (`self`) of the [management command group](runtime.md#commands). Setting this to `none` effectively disables the use of management commands.

When enabled, the value will be available at runtime as the `PYAPP_COMMAND_NAME` environment variable.

## Metadata template

You may set a custom template used to [output metadata](runtime.md#metadata) with the `PYAPP_METADATA_TEMPLATE` option which supports the following placeholders:

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
