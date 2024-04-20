# Runtime behavior

-----

## Initialization

Applications will bootstrap themselves on the first run. All subsequent invocations will only check if the installation directory exists and nothing else, to maximize CLI responsiveness.

!!! note
    The following diagram shows the possible behavior at runtime. The nodes with rounded edges are conditions and those with jagged edges are actions.

    Most nodes are clickable and will take you to the relevant documentation.

```mermaid
flowchart TD
    INSTALLED([Installed]) -- No --> DISTCACHED([Distribution cached])
    INSTALLED -- Yes --> MNG([Management enabled])
    DISTCACHED -- No --> DISTEMBEDDED([Distribution embedded])
    DISTCACHED -- Yes --> FULLISOLATION([Full isolation])
    DISTEMBEDDED -- No --> DISTSOURCE[[Cache from source]]
    DISTEMBEDDED -- Yes --> DISTEXTRACT[[Cache from embedded data]]
    DISTSOURCE --> FULLISOLATION
    DISTEXTRACT --> FULLISOLATION
    FULLISOLATION -- No --> UVENABLED([UV enabled])
    UVENABLED -- No --> VENV[[Create virtual environment]]
    UVENABLED -- Yes --> UVCACHED([UV cached])
    UVCACHED -- No --> DOWNLOADUV[[Download UV]]
    UVCACHED -- Yes --> VENV
    DOWNLOADUV --> VENV
    FULLISOLATION -- Yes --> UNPACK[[Unpack distribution directly]]
    UNPACK --> UVENABLEDUNPACK([UV enabled])
    UVENABLEDUNPACK -- No --> EXTERNALPIP[[External pip]]
    UVENABLEDUNPACK -- Yes --> UVCACHEDUNPACK([UV cached])
    UVCACHEDUNPACK -- No --> DOWNLOADUVUNPACK[[Download UV]]
    EXTERNALPIP([External pip]) -- No --> PROJEMBEDDED([Project embedded])
    EXTERNALPIP -- Yes --> PIPCACHED([pip cached])
    PIPCACHED -- No --> DOWNLOADPIP[[Download pip]]
    PIPCACHED -- Yes --> PROJEMBEDDED([Project embedded])
    DOWNLOADPIP --> PROJEMBEDDED
    PROJEMBEDDED -- No --> DEPFILE([Dependency file])
    PROJEMBEDDED -- Yes --> PROJEMBED[[Install from embedded data]]
    DEPFILE -- No --> SINGLEPROJECT[[Install single project]]
    DEPFILE -- Yes --> DEPFILEINSTALL[[Install from dependency file]]
    UVCACHEDUNPACK -- Yes --> PROJEMBEDDED
    DOWNLOADUVUNPACK --> PROJEMBEDDED
    VENV --> EXTERNALPIP
    SINGLEPROJECT --> MNG
    DEPFILEINSTALL --> MNG
    PROJEMBED --> MNG
    MNG -- No --> EXECUTE[[Execute project]]
    MNG -- Yes --> MNGCMD([Command invoked])
    MNGCMD -- No --> EXECUTE
    MNGCMD -- Yes --> MANAGE[[Run management command]]
    click DISTEMBEDDED href "../config/#distribution-embedding"
    click FULLISOLATION href "../config/#full-isolation"
    click UVENABLED href "../config/#uv"
    click UVENABLEDUNPACK href "../config/#uv"
    click EXTERNALPIP href "../config/#externally-managed"
    click PROJEMBEDDED href "../config/#project-embedding"
    click DEPFILE href "../config/#dependency-file"
    click SINGLEPROJECT href "../config/#package-index"
    click DEPFILEINSTALL href "../config/#dependency-file"
    click PROJEMBED href "../config/#project-embedding"
    click MNG href "../config/#management-command"
    click MNGCMD href "../config/#management-command"
    click MANAGE href "#commands"
    click EXECUTE href "../config/#execution-mode"
```

## Execution

Projects are [executed](config.md#execution-mode) using [`execvp`](https://linux.die.net/man/3/execvp) on non-Windows systems, replacing the process.

To provide consistent behavior on each user's machine:

- Python runs projects in [isolated mode](https://docs.python.org/3/using/cmdline.html#cmdoption-I)
- When installing or upgrading projects, [pip](https://github.com/pypa/pip) uses [isolation](https://pip.pypa.io/en/stable/cli/pip/#cmdoption-isolated) ([by default](config.md#allowing-configuration))

## Detection

A single environment variable called `PYAPP` is injected with the value of `1` ([by default](config.md#installation-indicator)) when running applications and may be used to detect this mode of installation versus others.

## Location

The default location of your application's installation differs based on the operating system and can be overridden at runtime with the `PYAPP_INSTALL_DIR_<PROJECT_NAME>` environment variable where `<PROJECT_NAME>` is the uppercased version of the [project name](config.md#project).

## Commands

Built applications have a single top-level command group named `self` ([by default](config.md#management-command)) and all other invocations will be forwarded to your actual [execution logic](config.md#execution-mode).

### Default

These commands are always exposed.

#### Restore

```
<EXE> self restore
```

This will wipe the installation and start fresh.

#### Update

```
<EXE> self update
```

This will update the project to the latest available version in the currently used distribution.

### Optional

These commands are hidden by default and each can be individually exposed by setting its corresponding `PYAPP_EXPOSE_<COMMAND>` option (e.g. `PYAPP_EXPOSE_METADATA`) to `true` or `1`.

#### Metadata

```
<EXE> self metadata
```

This displays [customized](config.md#metadata-template) output based on a template.

#### pip

```
<EXE> self pip
```

This directly invokes pip with the installed Python.

#### Python

```
<EXE> self python
```

This directly invokes the installed Python.

#### Python path

```
<EXE> self python-path
```

This outputs the path to the installed Python.
