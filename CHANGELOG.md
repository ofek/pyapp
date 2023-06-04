# Changelog

-----

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

***Added:***

- Add the ability to externally manage pip with the `PYAPP_PIP_EXTERNAL` and `PYAPP_PIP_VERSION` options

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
