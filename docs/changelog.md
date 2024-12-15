# Changelog

-----

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.25.0 - 2024-12-15

***Added:***

- Update default CPython distributions to 20241206
- Enable LTO for releases
- Update dependencies

## 0.24.0 - 2024-10-13

***Changed:***

- The `PYAPP_DISTRIBUTION_VARIANT` has been renamed to `PYAPP_DISTRIBUTION_VARIANT_CPU` although the former is still supported for backwards compatibility

***Added:***

- Support Python 3.13 and set as the default version
- Add `PYAPP_DISTRIBUTION_VARIANT_GIL` option (Python 3.13+)
- Update default CPython distributions to 20241008
- Update default PyPy distributions to 7.3.17
- Update dependencies

## 0.23.0 - 2024-08-03

***Added:***

- Add `PYAPP_UV_SOURCE` option

## 0.22.0 - 2024-05-26

***Added:***

- Add `PYAPP_EXPOSE_ALL_COMMANDS` option
- Update dependencies

## 0.21.1 - 2024-05-15

***Fixed:***

- Fix the `PYAPP_DISTRIBUTION_PATH_PREFIX` option

## 0.21.0 - 2024-05-15

***Added:***

- Add `PYAPP_DISTRIBUTION_PATH_PREFIX` option for easier configuring of custom distribution internal paths
- Add `PYAPP_ALLOW_UPDATES` option for enabling the `update` management command when project installation is skipped

***Fixed:***

- Properly hide the `update` management command when skipping project installation

## 0.20.1 - 2024-05-14

***Fixed:***

- Properly handle failed downloads

## 0.20.0 - 2024-05-13

***Added:***

- Add `cache` management command
- Update dependencies

***Fixed:***

- Bootstrapping is now safe across multiple processes (a shared resource message is displayed if another process is already bootstrapping)
- The `pip` management command is now resilient to cache removal
- Management commands now properly support the `-h`/`--help` flag

## 0.19.0 - 2024-04-24

***Added:***

- Add `remove` management command
- Update dependencies

***Fixed:***

- Fix UV and the `VIRTUAL_ENV` environment variable on non-Windows systems

## 0.18.0 - 2024-04-22

***Added:***

- Update PyPy distributions to 7.3.15

***Fixed:***

- Fix resolution for legacy 3.7 builds

## 0.17.0 - 2024-04-21

***Added:***

- Support using UV for virtual environment creation and project installation
- The PATH environment variable is now updated to include the installation's directory of executables
- Update default distributions to 20240415

***Fixed:***

- Dependencies are now locked

## 0.16.0 - 2024-03-24

***Added:***

- Add `PYAPP_IS_GUI` option to support graphical applications

## 0.15.1 - 2024-03-03

***Fixed:***

- Fix reading metadata with Windows line endings during build time from embedded distributions

## 0.15.0 - 2024-03-01

***Added:***

- Update default distributions to 20240224
- Update default Python version to 3.12
- Statically link the C runtime on Windows
- Add `PYAPP_PROJECT_FEATURES` option for selecting extras
- Add new execution mode option `PYAPP_EXEC_NOTEBOOK` for running Jupyter notebooks

***Fixed:***

- Properly resolve correct default distributions on MinGW-w64
- Fix embedding custom distributions
- Ignore nonexistent variant options for 3.7 distributions

## 0.14.0 - 2024-01-21

***Added:***

- Update default distributions to 20240107

## 0.13.0 - 2023-12-31

***Added:***

- Allow for forwarding of unknown management commands e.g. if apps have their own `self` commands

***Fixed:***

- Remove patch for powerpc64le now that the transitive dependency `ring` is fixed

## 0.12.0 - 2023-10-07

***Added:***

- Update default distributions to 20231002, adding support for Python 3.12

## 0.11.1 - 2023-09-14

***Fixed:***

- Fix the Python path for the `pypy2.7` distribution

## 0.11.0 - 2023-09-07

***Added:***

- Update default distributions to 20230826
- Build releases with codegen-units=1

## 0.10.1 - 2023-06-26

***Fixed:***

- Fix regression in the `PYAPP_EXEC_SPEC` option

## 0.10.0 - 2023-06-26

***Added:***

- Add `PYAPP_EXEC_SCRIPT` option for executing a project using a script
- Add support for overriding the installation directory
- Make the `PYAPP_DISTRIBUTION_PATH` option implicitly enable `PYAPP_DISTRIBUTION_EMBED`

***Fixed:***

- Properly handle cases where options contain line feed characters

## 0.9.0 - 2023-06-21

***Changed:***

- Custom distributions should now define the relative path to the `site-packages` directory

***Added:***

- Add support for PyPy distributions
- Add the `PYAPP_UPGRADE_VIRTUALENV` option to create virtual environments with `virtualenv` rather than the stdlib's `venv`
- Add support for custom distributions with `bzip2` compression

***Fixed:***

- Properly handle cases where temporary files are on different filesystems
- Fix regression in the `metadata` management command on Windows
- Improve error messages when running binaries that were misconfigured

## 0.8.0 - 2023-06-09

***Added:***

- Add the ability to externally manage pip with the `PYAPP_PIP_EXTERNAL` and `PYAPP_PIP_VERSION` options
- Allow for project installation with a dependency file using the `PYAPP_PROJECT_DEPENDENCY_FILE` option
- Add management command to directly invoke pip with the installed Python
- Add management command to output the path to the installed Python

***Fixed:***

- Fix builds for PowerPC64

## 0.7.0 - 2023-05-24

***Changed:***

- Installations use virtual environments by default; the previous behavior can be enabled with the `PYAPP_FULL_ISOLATION` option

***Added:***

- Update default CPython distributions
- Add `-r`/`--restore` flag to the `update` command
- Allow for disabling of management commands
- Add ability to expose optional commands
- Add optional command to directly invoke the installed Python
- Run Python in isolated mode
- Execute projects with `execvp` on non-Windows systems
- When the management command is enabled its name is available at runtime via an environment variable
- Add different installation wait message for when there is a guarantee of no side effects (e.g. pip's `--only-binary :all:`)

## 0.6.0 - 2023-05-16

***Added:***

- Add `PYAPP_PROJECT_PATH` option to embed the project for installation at runtime
- Add `PYAPP_DISTRIBUTION_PATH` option to embed the distribution from a local path rather than fetching the source

***Fixed:***

- Properly handle distributions packed as ZIP files

## 0.5.0 - 2023-05-11

***Added:***

- Strip symbols from release builds

***Fixed:***

- Properly pass through all required environment variable options to cross compilation images
- Properly allow configuration of the template used for the metadata command

## 0.4.0 - 2023-05-11

***Changed:***

- Rename `PYAPP_STARSHIP_PROMPT` option to `PYAPP_METADATA_TEMPLATE`

***Added:***

- Add `PYAPP_PIP_EXTRA_ARGS` option to provide extra `pip install` arguments
- Add `PYAPP_PIP_ALLOW_CONFIG` option to allow runtime configuration of `pip`
- Add configuration for correct cross compilation

## 0.3.1 - 2023-05-10

***Fixed:***

- Fix default distribution detection for Linux on architectures other than x86_64

## 0.3.0 - 2023-05-10

***Changed:***

- Rename `PYAPP_DISTRIBUTION_COMPRESSION` option to `PYAPP_DISTRIBUTION_FORMAT`

***Added:***

- Add `--pre` flag to the `self update` command to allow pre-release and development versions
- Add environment variable for detection
- Add `PYAPP_SELF_COMMAND` option to control the name of the management command
- Add `PYAPP_SKIP_INSTALL` option to skip project installation
- Remove dependence on OpenSSL

***Fixed:***

- Properly display error messages from `pip install` commands
- Fix project version reading for the metadata command on non-Windows systems

## 0.2.0 - 2023-05-07

This is the initial public release.
