import re
import sys
import tomllib
from itertools import zip_longest
from pathlib import Path

IGNORED = {'PYAPP_EXPOSE_{}', 'PYAPP_PROJECT_{}'}


def main():
    defined_options = tomllib.loads(
        Path('Cargo.toml').read_text('utf-8')
    )['package']['metadata']['cross']['build']['env']['passthrough']
    available_options = set(re.findall(r'"(PYAPP_[^_].+?)"', Path('build.rs').read_text('utf-8')))
    available_options -= IGNORED

    root_commands = Path('src/commands/self_cmd')
    expose_option = re.compile(r'^#\[command\(hide = env!\("(PYAPP_EXPOSE_.+?)"\)', re.MULTILINE)
    for entry in root_commands.iterdir():
        if entry.is_file() and (match := expose_option.search(entry.read_text('utf-8'))):
            available_options.add(match.group(1))

    command_groups = ['cache']
    for command_group in command_groups:
        path = root_commands / command_group / 'cli.rs'
        if match := expose_option.search(path.read_text('utf-8')):
            available_options.add(match.group(1))

    expected_options = sorted(available_options)
    if defined_options != expected_options:
        left_padding = max(len(option) for option in expected_options)
        right_padding = max(len(option) for option in defined_options)
        print(f'{"Expected":{left_padding}} | Defined')
        print('-' * left_padding + '-+-' + '-' * right_padding)
        for expected, defined in zip_longest(expected_options, defined_options, fillvalue=''):
            print(f'{expected:{left_padding}} | {defined}')

        sys.exit(1)


if __name__ == '__main__':
    main()
