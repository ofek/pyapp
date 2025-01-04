# Python distribution configuration

-----

## Known

Setting the `PYAPP_PYTHON_VERSION` option will determine the distribution used at runtime based on the environment at build time. If unset then the default will be the latest stable minor version of [CPython](#cpython).

### CPython

| ID |
| --- |
| `3.7` |
| `3.8` |
| `3.9` |
| `3.10` |
| `3.11` |
| `3.12` |
| `3.13` |

The source for pre-built distributions is the [python-build-standalone](https://github.com/astral-sh/python-build-standalone) project.

#### Variants

Some distributions have [variants](https://gregoryszorc.com/docs/python-build-standalone/main/running.html) that may be configured. Options may be combined.

| Option | Platforms | Allowed values |
| --- | --- | --- |
| `PYAPP_DISTRIBUTION_VARIANT_CPU` | <ul><li>Linux</li></ul> | <ul><li><code>v1</code></li><li><code>v2</code></li><li><code>v3</code> (default)</li><li><code>v4</code></li></ul> |
| `PYAPP_DISTRIBUTION_VARIANT_GIL` | <ul><li>Linux</li><li>Windows</li><li>macOS</li></ul> | <ul><li><code>freethreaded</code></li></ul> |

### PyPy

| ID |
| --- |
| `pypy2.7` |
| `pypy3.9` |
| `pypy3.10` |

The source of distributions is the [PyPy](https://www.pypy.org) project.

## Custom

You may explicitly set the `PYAPP_DISTRIBUTION_SOURCE` option which overrides the [known](#known) distribution settings. The source must be a URL that points to an archived version of the desired Python distribution.

Setting this manually may require you to define extra metadata about the distribution that is required for correct [runtime behavior](../runtime.md).

### Format

The following formats are supported for the `PYAPP_DISTRIBUTION_FORMAT` option, with the default chosen based on the ending of the source URL:

| Format | Extensions | Description |
| --- | --- | --- |
| `tar|bzip2` | <ul><li><code>.tar.bz2</code></li><li><code>.bz2</code></li></ul> | A [tar file](https://en.wikipedia.org/wiki/Tar_(computing)) with [bzip2 compression](https://en.wikipedia.org/wiki/Bzip2) |
| `tar|gzip` | <ul><li><code>.tar.gz</code></li><li><code>.tgz</code></li></ul> | A [tar file](https://en.wikipedia.org/wiki/Tar_(computing)) with [gzip compression](https://en.wikipedia.org/wiki/Gzip) |
| `tar|zstd` | <ul><li><code>.tar.zst</code></li><li><code>.tar.zstd</code></li></ul> | A [tar file](https://en.wikipedia.org/wiki/Tar_(computing)) with [Zstandard compression](https://en.wikipedia.org/wiki/Zstd) |
| `zip` | <ul><li><code>.zip</code></li></ul> | A [ZIP file](https://en.wikipedia.org/wiki/ZIP_(file_format)) with [DEFLATE compression](https://en.wikipedia.org/wiki/Deflate) |

### Python location

You may set the relative path to the Python executable after unpacking the archive with the `PYAPP_DISTRIBUTION_PYTHON_PATH` option. The default is `python.exe` on Windows and `bin/python3` on all other platforms.

### Site packages location

You may set the relative path to the [`site-packages`](https://docs.python.org/3/library/site.html) directory after unpacking the archive with the `PYAPP_DISTRIBUTION_SITE_PACKAGES_PATH` option. The default is `Lib\site-packages` on Windows and `lib/python<ID>/site-packages` on all other platforms where `<ID>` is the defined [distribution ID](#known).

### Path prefix

If the [Python executable](#python-location) and the [`site-packages` directory](#site-packages-location) are at the default locations but nested under top-level directories, you may set the `PYAPP_DISTRIBUTION_PATH_PREFIX` option to the common prefix of the two paths to avoid having to manually set those options.

### pip availability

You may indicate whether pip is already installed by setting the `PYAPP_DISTRIBUTION_PIP_AVAILABLE` option to `true` or `1`. This elides the check for installation when [upgraded virtual environments](installation.md#virtual-environments) are enabled.

## Embedding

You may set the `PYAPP_DISTRIBUTION_EMBED` option to `true` or `1` to embed the distribution in the executable at build time to avoid fetching it at runtime.

You can set the `PYAPP_DISTRIBUTION_PATH` option to use a local path rather than fetching the source, which implicitly enables embedding. The local archive should be similar to the [default distributions](#known) in that there should be a Python interpreter ready for use.

## Full isolation

You may set the `PYAPP_FULL_ISOLATION` option to `true` or `1` to provide each installation with a full copy of the distribution rather than a virtual environment.
