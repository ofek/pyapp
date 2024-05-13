# PyApp

| | |
| --- | --- |
| CI/CD | [![CI - Test](https://github.com/ofek/pyapp/actions/workflows/test.yml/badge.svg){ loading=lazy .off-glb }](https://github.com/ofek/pyapp/actions/workflows/test.yml) [![CD - Publish](https://github.com/ofek/pyapp/actions/workflows/publish.yml/badge.svg){ loading=lazy .off-glb }](https://github.com/ofek/pyapp/actions/workflows/publish.yml) |
| Docs | [![Docs - Latest](https://github.com/ofek/pyapp/actions/workflows/docs-latest.yml/badge.svg){ loading=lazy .off-glb }](https://github.com/ofek/pyapp/actions/workflows/docs-latest.yml) [![Docs - Dev](https://github.com/ofek/pyapp/actions/workflows/docs-dev.yml/badge.svg){ loading=lazy .off-glb }](https://github.com/ofek/pyapp/actions/workflows/docs-dev.yml) |
| Project | [![Project - Version](https://img.shields.io/crates/v/pyapp){ loading=lazy .off-glb }](https://crates.io/crates/pyapp) [![Project - Package downloads](https://img.shields.io/crates/d/pyapp?label=package%20downloads){ loading=lazy .off-glb }](https://crates.io/crates/pyapp) [![Project - Repo downloads](https://img.shields.io/github/downloads/ofek/pyapp/total?label=repo%20downloads){ loading=lazy .off-glb }](https://github.com/ofek/pyapp/releases) |
| Meta | [![Hatch project](https://img.shields.io/badge/%F0%9F%A5%9A-Hatch-4051b5.svg){ loading=lazy .off-glb }](https://github.com/pypa/hatch) [![License - Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-9400d3.svg){ loading=lazy .off-glb }](https://spdx.org/licenses/) [![GitHub Sponsors](https://img.shields.io/github/sponsors/ofek?logo=GitHub%20Sponsors&style=social){ loading=lazy .off-glb }](https://github.com/sponsors/ofek) |

-----

PyApp is a wrapper for Python applications that bootstrap themselves at runtime.

<div align="center" markdown>

| You build |
| :---: |
| ![PyApp example build](assets/images/example-build.gif){ loading=lazy role="img" } |

| User runs |
| :---: |
| ![PyApp example run](assets/images/example-run.gif){ loading=lazy role="img" } |

</div>

See the [how-to](how-to.md) for a detailed example walkthrough.

## Features

- Easily build standalone binaries for every platform
- Optional management commands providing functionality such as self updates
- Extremely configurable runtime behavior allowing for targeting of different end users

## License

PyApp is distributed under the terms of any of the following licenses:

- [Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)
- [MIT](https://spdx.org/licenses/MIT.html)

## Navigation

Documentation for specific versions can be chosen by using the dropdown on the top of every page. The `dev` version reflects changes that have not yet been released.

Desktop readers can use special keyboard shortcuts:

| Keys | Action |
| --- | --- |
| <ul><li><kbd>,</kbd> (comma)</li><li><kbd>p</kbd></li></ul> | Navigate to the "previous" page |
| <ul><li><kbd>.</kbd> (period)</li><li><kbd>n</kbd></li></ul> | Navigate to the "next" page |
| <ul><li><kbd>/</kbd></li><li><kbd>s</kbd></li></ul> | Display the search modal |
