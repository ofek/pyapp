# Command line configuration

-----

## Installation indicator

The environment variable that is used for [detection](../runtime.md#detection) may be set to the path of the executable at runtime if you set the `PYAPP_PASS_LOCATION` option to `true` or `1`. This is useful if your application wishes to in some way manage itself.

## Management command

You may set the `PYAPP_SELF_COMMAND` option to override the default name (`self`) of the [management command group](../runtime.md#commands). Setting this to `none` effectively disables the use of management commands.

When enabled, the value will be available at runtime as the `PYAPP_COMMAND_NAME` environment variable.

## Metadata template

You may set a custom template used to [output metadata](../runtime.md#metadata) with the `PYAPP_METADATA_TEMPLATE` option which supports the following placeholders:

| Placeholder | Description |
| --- | --- |
| `{project}` | The normalized project name |
| `{version}` | The currently installed version of the project |

The default template is `{project} v{version}` if this option is unset.

This is useful for setting [custom commands](https://starship.rs/config/#custom-commands) for the [Starship](https://github.com/starship/starship) prompt. The following example configuration assumes that the built executable has been renamed to `foo`:

````toml
format = """
...
${custom.foo}\
...
$line_break\
...
$character"""

# <clipped>

[custom.foo]
command = "foo self metadata"
when = true
## Windows
# shell = ["cmd", "/C"]
## Other
# shell = ["sh", "--norc"]
````
