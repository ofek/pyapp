# Runtime behavior

-----

## Initialization

Applications will bootstrap themselves on the first run. All subsequent invocations will only check if the installation directory exists and nothing else, to maximize CLI responsiveness.

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
    FULLISOLATION -- No --> VENV[[Create virtual environment]]
    FULLISOLATION -- Yes --> UNPACK[[Unpack distribution directly]]
    PROJEMBEDDED([Project embedded]) -- No --> PROJINDEX[[Install from package index]]
    PROJEMBEDDED -- Yes --> PROJEMBED[[Install from embedded data]]
    PROJINDEX --> MNG
    PROJEMBED --> MNG
    VENV --> PROJEMBEDDED
    UNPACK --> PROJEMBEDDED
    MNG -- No --> EXECUTE[[Execute project]]
    MNG -- Yes --> MNGCMD([Command invoked])
    MNGCMD -- No --> EXECUTE
    MNGCMD -- Yes --> MANAGE[[Run management command]]
```

## Execution

Projects are [executed](config.md#execution-mode) using [`execvp`](https://linux.die.net/man/3/execvp) on non-Windows systems, replacing the process.

To provide consistent behavior on each user's machine:

- Python runs projects in [isolated mode](https://docs.python.org/3/using/cmdline.html#cmdoption-I)
- When installing or upgrading projects, [pip](https://github.com/pypa/pip) uses [isolation](https://pip.pypa.io/en/stable/cli/pip/#cmdoption-isolated) ([by default](config.md#allowing-configuration))

## Detection

A single environment variable called `PYAPP` is injected with the value of `1` ([by default](config.md#installation-indicator)) when running applications and may be used to detect this mode of installation versus others.

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

#### Python

```
<EXE> self python
```

This directly invokes the installed Python.
