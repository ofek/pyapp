# How-to

-----

What follows is a short example showing the end-to-end experience from building an application to running the application as a user.

## Install Rust

Follow [the instructions](https://www.rust-lang.org/tools/install) to install Rust and make sure the package manager Cargo is on your PATH.

## Get PyApp

In order to [build](build.md) applications with PyApp, you must first download the source code. Here we will download the latest release.

/// tab | Linux/macOS
1. `curl https://github.com/ofek/pyapp/releases/latest/download/source.tar.gz -Lo pyapp-source.tar.gz`
2. `tar -xzf pyapp-source.tar.gz`
3. `mv pyapp-v* pyapp-latest`
4. `cd pyapp-latest`
///

/// tab | Windows
1. `Invoke-WebRequest https://github.com/ofek/pyapp/releases/latest/download/source.zip -OutFile pyapp-source.zip`
2. `7z x pyapp-source.zip`
3. `mv pyapp-v* pyapp-latest`
4. `cd pyapp-latest`
///

## Configuration

You must [configure](config/project.md) the binaries PyApp produces with environment variables. There are [many ways](examples.md) to configure applications but here we will define a single package to install from PyPI at a specific version:

| Option | Value |
| --- | --- |
| `PYAPP_PROJECT_NAME` | `cowsay` |
| `PYAPP_PROJECT_VERSION` | `6.0` |

## Building

Run:

```
cargo build --release
```

The executable will be located at `target/release/pyapp.exe` if on Windows or `target/release/pyapp` otherwise.

## Distribution

Be sure to rename the binary to the name of the application (and make it executable on non-Windows systems):

/// tab | Linux/macOS
```
mv target/release/pyapp cowsay && chmod +x cowsay
```
///

/// tab | Windows
```
mv target\release\pyapp.exe cowsay
```
///

## Runtime

After you have distributed the binary to the user, they can execute it directly:

```
$ ./cowsay -t 'Hello, World!'
  _____________
| Hello, World! |
  =============
             \
              \
                ^__^
                (oo)\_______
                (__)\       )\/\
                    ||----w |
                    ||     ||
```
