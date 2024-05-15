# Installation configuration

-----

These options have no effect when the project installation is [disabled](#skipping-installation).

## UV

You may set the `PYAPP_UV_ENABLED` option to `true` or `1` to use [UV](https://github.com/astral-sh/uv) for virtual environment creation and project installation.

### Version ### {: #uv-version }

You may use a specific `X.Y.Z` version by setting the `PYAPP_UV_VERSION` option.

By default, a version of UV that has already been downloaded by a PyApp application is used. If UV has not yet been downloaded then the latest version is used.

### Only bootstrap

You may set the `PYAPP_UV_ONLY_BOOTSTRAP` option to `true` or `1` to only use UV for virtual environment creation and continue using pip for project installation.

## pip

These options have no effect when UV is [enabled](#uv).

### Externally managed

You may set the `PYAPP_PIP_EXTERNAL` option to `true` or `1` to use the [standalone](https://pip.pypa.io/en/stable/installation/#standalone-zip-application) versions of pip rather than whatever the distribution provides.

By default, the latest version is used. You may use a specific `X.Y.Z` version by setting the `PYAPP_PIP_VERSION` option.

!!! tip
    This provides a significant installation speed up when [full isolation](distribution.md#full-isolation) is not enabled.

### Allowing configuration

You may set the `PYAPP_PIP_ALLOW_CONFIG` option to `true` or `1` to allow the use of environment variables and other configuration at runtime.

### Virtual environments

When [full isolation](distribution.md#full-isolation) is not enabled, you may set the `PYAPP_UPGRADE_VIRTUALENV` option to `true` or `1` to create virtual environments with [virtualenv](https://github.com/pypa/virtualenv) rather than the standard library's `venv` module.

## Extra installer arguments

You may set the `PYAPP_PIP_EXTRA_ARGS` option to provide extra arguments to the [`pip install`](https://pip.pypa.io/en/stable/cli/pip_install/) (or [UV](#uv) equivalent) command at runtime when installing or updating the project e.g. `--only-binary :all: --extra-index-url URL`.

## Location

The default location of your application's installation differs based on the operating system and can be overridden at runtime with the `PYAPP_INSTALL_DIR_<PROJECT_NAME>` environment variable where `<PROJECT_NAME>` is the uppercased version of the [project name](project.md#identifier).

## Skipping installation

You may set the `PYAPP_SKIP_INSTALL` option to `true` or `1` to skip installing the project in the distribution. This allows for entirely predefined distributions and thus no network calls at runtime if used in conjunction with [distribution embedding](distribution.md#embedding).

When project installation is skipped, the `update` command will not be available. You may set the `PYAPP_ALLOW_UPDATES` option to `true` or `1` to expose the command anyway. Be sure to set the appropriate [project options](project.md) as configuring a prebuilt distribution does not require those.
