# Examples

-----

The following examples do not illustrate every possible combination of options but rather some common use cases.

## Single project basic

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |

## Single project embedded

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_PATH` | `./proj-X.Y.Z.dev0-py3-none-any.whl` |

## Dependency file basic

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |
| `PYAPP_PROJECT_DEPENDENCY_FILE` | `./requirements.txt` |

!!! note
    The [default execution](config/project.md#execution-mode) will be `python -m proj` at runtime.

## Dependency file with script

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |
| `PYAPP_PROJECT_DEPENDENCY_FILE` | `./requirements.txt` |
| `PYAPP_EXEC_SCRIPT` | `./script.py` |

## Execution with object reference

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |
| `PYAPP_EXEC_SPEC` | `proj.cli:main` |

## Specific known distribution

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |
| `PYAPP_PYTHON_VERSION` | `3.10` |

## Custom remote distribution basic

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |
| `PYAPP_DISTRIBUTION_SOURCE` | `https://foo.bar.baz/archive.tar.gz` |

## Custom remote distribution embedded

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |
| `PYAPP_DISTRIBUTION_SOURCE` | `https://foo.bar.baz/archive.tar.gz` |
| `PYAPP_DISTRIBUTION_EMBED` | `true` |

## Custom embedded local distribution

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |
| `PYAPP_DISTRIBUTION_PATH` | `./archive.tar.gz` |

## Offline installation

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_PATH` | `./proj-X.Y.Z-py3-none-any.whl` |
| `PYAPP_DISTRIBUTION_PATH` | `./archive.tar.gz` |
| `PYAPP_PIP_EXTRA_ARGS` | `--no-deps` |

## Reproducible installation with custom package index

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `proj` |
| `PYAPP_PROJECT_VERSION` | `X.Y.Z` |
| `PYAPP_PROJECT_DEPENDENCY_FILE` | `./requirements.txt` |
| `PYAPP_PIP_EXTRA_ARGS` | `--only-binary :all: --index-url URL` |
