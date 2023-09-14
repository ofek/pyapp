# Changelog

-----

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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
