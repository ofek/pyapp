# Project configuration

-----

## Sources

There are 3 ways to configure runtime installation, none of which will occur when [disabled](installation.md#skipping-installation).

The project [identifier](#identifier) must be known in all cases.

### Identifier

The desired project name and version are configured with the `PYAPP_PROJECT_NAME` and `PYAPP_PROJECT_VERSION` options, respectively. The project name must adhere to [PEP 508](https://peps.python.org/pep-0508/#names) and will be normalized during builds according to [PEP 503](https://peps.python.org/pep-0503/#normalized-names).

When using only this method, the package will be installed from a package index like PyPI.

### Dependency file

You may install your project using a dependency file with the `PYAPP_PROJECT_DEPENDENCY_FILE` option which should be a local path to the file. In this mode, the project [identifier](#identifier) has nothing to do with installation and is just used as metadata.

The following formats are supported:

| Extensions | Description |
| --- | --- |
| <code>.txt</code><br><code>.in</code> | This is the [requirements file format](https://pip.pypa.io/en/stable/reference/requirements-file-format/) |

### Embedding

You may embed the project with the `PYAPP_PROJECT_PATH` option which should be a path to a wheel ending in `.whl` or a source distribution ending in `.tar.gz`.

!!! note
    The project [identifier](#identifier) is automatically derived from the metadata files inside.

## Features (extras) ## {: #features }

You may set the `PYAPP_PROJECT_FEATURES` option to select [optional dependency groups](https://packaging.python.org/en/latest/specifications/dependency-specifiers/#extras) that would usually be passed to installers within square brackets after the package name e.g. `pkg[foo,bar]`. In that example, you would set `PYAPP_PROJECT_FEATURES` to `foo,bar`.

This also works when [embedding the project](#embedding).

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

Otherwise, the application will execute as usual. PyApp will run your GUI by spawning a new process, such that the console window that runs the application terminates after successful spawning.

Even when `PYAPP_IS_GUI` is enabled you can still run the application from the command line. Furthermore, PyApp-specific logic (e.g. installation and setup) will still display a console window with status messages.

!!! note
    On macOS, the console by default does not automatically close when processes have terminated (however it can be closed manually without interferring with the GUI). The default console behavior [can be changed](https://stackoverflow.com/questions/5560167/osx-how-to-auto-close-terminal-window-after-the-exit-command-executed) in the user settings to close after the last process terminates successfully.
