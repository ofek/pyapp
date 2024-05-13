# PyApp

| | |
| --- | --- |
| CI/CD | [![CI - Test](https://github.com/ofek/pyapp/actions/workflows/test.yml/badge.svg)](https://github.com/ofek/pyapp/actions/workflows/test.yml) [![CD - Publish](https://github.com/ofek/pyapp/actions/workflows/publish.yml/badge.svg)](https://github.com/ofek/pyapp/actions/workflows/publish.yml) |
| Docs | [![Docs - Latest](https://github.com/ofek/pyapp/actions/workflows/docs-latest.yml/badge.svg)](https://github.com/ofek/pyapp/actions/workflows/docs-latest.yml) [![Docs - Dev](https://github.com/ofek/pyapp/actions/workflows/docs-dev.yml/badge.svg)](https://github.com/ofek/pyapp/actions/workflows/docs-dev.yml) |
| Project | [![Project - Version](https://img.shields.io/crates/v/pyapp)](https://crates.io/crates/pyapp) [![Project - Package downloads](https://img.shields.io/crates/d/pyapp?label=package%20downloads)](https://crates.io/crates/pyapp) [![Project - Repo downloads](https://img.shields.io/github/downloads/ofek/pyapp/total?label=repo%20downloads)](https://github.com/ofek/pyapp/releases) |
| Meta | [![Hatch project](https://img.shields.io/badge/%F0%9F%A5%9A-Hatch-4051b5.svg)](https://github.com/pypa/hatch) [![License - Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-9400d3.svg)](https://spdx.org/licenses/) [![GitHub Sponsors](https://img.shields.io/github/sponsors/ofek?logo=GitHub%20Sponsors&style=social)](https://github.com/sponsors/ofek) |

-----

PyApp is a wrapper for Python applications that bootstrap themselves at runtime.

<div align="center">
<table>
  <tr><th>You build</th></tr>
  <tr>
    <td>
      <img src="https://raw.githubusercontent.com/ofek/pyapp/master/docs/assets/images/example-build.gif" alt="PyApp example build" role="img">
    </td>
  </tr>
</table>

<table>
  <tr><th>User runs</th></tr>
  <tr>
    <td>
      <img src="https://raw.githubusercontent.com/ofek/pyapp/master/docs/assets/images/example-run.gif" alt="PyApp example run" role="img">
    </td>
  </tr>
</table>
</div>

See the [how-to](https://ofek.dev/pyapp/latest/how-to/) for a detailed example walkthrough.

## Features

- Easily build standalone binaries for every platform
- Optional management commands providing functionality such as self updates
- Extremely configurable runtime behavior allowing for targeting of different end users

## Documentation

The [documentation](https://ofek.dev/pyapp/) is made with [Material for MkDocs](https://github.com/squidfunk/mkdocs-material) and is hosted by [GitHub Pages](https://docs.github.com/en/pages).

## License

PyApp is distributed under the terms of any of the following licenses:

- [Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)
- [MIT](https://spdx.org/licenses/MIT.html)
