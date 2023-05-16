import re
import sys
import tomllib
from itertools import zip_longest
from pathlib import Path

IGNORED = {'PYAPP_PROJECT_{}'}


def main():
    available_options = set(re.findall(r'"(PYAPP_[^_].+?)"', Path('build.rs').read_text('utf-8')))
    available_options -= IGNORED
    expected_options = sorted(available_options)
    defined_options = tomllib.loads(
        Path('Cargo.toml').read_text('utf-8')
    )['package']['metadata']['cross']['build']['env']['passthrough']

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
