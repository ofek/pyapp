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
    click DISTEMBEDDED href "../config/distribution/#embedding"
    click FULLISOLATION href "../config/distribution/#full-isolation"
    click UVENABLED href "../config/installation/#uv"
    click UVENABLEDUNPACK href "../config/installation/#uv"
    click EXTERNALPIP href "../config/installation/#externally-managed"
    click PROJEMBEDDED href "../config/project/#embedding"
    click DEPFILE href "../config/project/#dependency-file"
    click SINGLEPROJECT href "../config/project/#identifier"
    click DEPFILEINSTALL href "../config/project/#dependency-file"
    click PROJEMBED href "../config/project/#embedding"
    click MNG href "../config/cli/#management-command"
    click MNGCMD href "../config/cli/#management-command"
    click MANAGE href "#commands"
    click EXECUTE href "../config/project/#execution-mode"
```

## Execution

Projects are [executed](config/project.md#execution-mode) using [`execvp`](https://linux.die.net/man/3/execvp) on non-Windows systems, replacing the process.

To provide consistent behavior on each user's machine:

- Python runs projects in [isolated mode](https://docs.python.org/3/using/cmdline.html#cmdoption-I)
- When installing or upgrading projects, [pip](https://github.com/pypa/pip) uses [isolation](https://pip.pypa.io/en/stable/cli/pip/#cmdoption-isolated) ([by default](config/installation.md#allowing-configuration))

## Detection

A single environment variable called `PYAPP` is injected with the value of `1` ([by default](config/cli.md#installation-indicator)) when running applications and may be used to detect this mode of installation versus others.

## Commands

Built applications have a single top-level command group named `self` ([by default](config/cli.md#management-command)) and all other invocations will be forwarded to your actual [execution logic](config/project.md#execution-mode).

### Default

These commands are always exposed.

#### Remove

```
<EXE> self remove
```

This will wipe the installation.

#### Restore

```
<EXE> self restore
```

This will wipe the installation and then reinstall.

#### Update

```
<EXE> self update
```

This will update the project to the latest available version in the currently used distribution.

### Optional

These commands are hidden by default and each can be individually exposed by setting its corresponding `PYAPP_EXPOSE_<COMMAND>` option (e.g. `PYAPP_EXPOSE_METADATA`) to `true` or `1`.

You can enable all of them at once by setting the `PYAPP_EXPOSE_ALL_COMMANDS` option to `true` or `1`. Individual commands that are explicitly disabled (`PYAPP_EXPOSE_<COMMAND>` set to `false` or `0`) will not be exposed.

#### Cache

```
<EXE> self cache [dist|pip|uv]
```

This is the command group for managing the cache. Each subcommand has a `-r`/`--remove` flag to remove the cached asset. Not passing that flag will display the location instead.

#### Metadata

```
<EXE> self metadata
```

This displays [customized](config/cli.md#metadata-template) output based on a template.

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
