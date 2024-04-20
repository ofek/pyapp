# /// script
# dependencies = [
#   "httpx",
# ]
# ///
import os
from collections import defaultdict
from pathlib import Path

import httpx

RELEASES_URL = 'https://api.github.com/repos/indygreg/python-build-standalone/releases'
PLATFORMS = ('linux', 'windows', 'macos')


def get_assets():
    token = os.environ.get('GH_TOKEN')
    if not token:
        raise OSError('GH_TOKEN not set')

    headers = {'Authorization': f'Bearer {token}', 'X-GitHub-Api-Version': '2022-11-28'}

    page = 1
    while True:
        response = httpx.get(RELEASES_URL, headers=headers, timeout=60, params={'page': page})
        releases = response.json()
        if not releases:
            break

        for release in releases:
            for asset in release['assets']:
                yield asset['name'], asset['browser_download_url']

        page += 1


def main():
    print('Updating distributions...')

    lines = Path('build.rs').read_text('utf-8').splitlines()
    start = end = -1
    for i, line in enumerate(lines):
        if line.startswith('const DEFAULT_CPYTHON_DISTRIBUTIONS'):
            start = i
        elif start != -1 and line.strip() == '// Frozen':
            end = i
            break
    else:
        raise ValueError('could not parse build.rs')

    insertion_index = start + 1
    del lines[insertion_index:end]

    distributions = defaultdict(list)
    for name, url in get_assets():
        # https://gregoryszorc.com/docs/python-build-standalone/main/distributions.html#install-only-archive
        if not name.endswith('-install_only.tar.gz'):
            continue

        original_name = name
        name = name.replace('-install_only.tar.gz', '').replace('cpython-', '')

        # https://gregoryszorc.com/docs/python-build-standalone/main/running.html
        weights, _, triple = name.partition('-')
        arch, _, platform = triple.partition('-')
        version, _, date = weights.partition('+')
        version_parts = tuple(map(int, version.split('.')))
        minor_version_parts = version_parts[:2]
        date = int(date)

        abi = ''
        variant = ''
        if 'windows' in platform:
            os_name = 'windows'

            _, _, abi = platform.rpartition('-')
            if abi in ('shared', 'static'):
                continue
        elif 'apple' in platform:
            os_name = 'macos'
        elif 'linux' in platform:
            os_name = 'linux'
            _, _, abi = platform.rpartition('-')
            if '_v' in arch:
                arch, _, variant = arch.rpartition('_')
            # Set to v1 since disabling with an empty string would trigger the defaults
            elif arch == 'x86_64':
                variant = 'v1'
        else:
            raise ValueError(f'unknown platform: {original_name}')

        # https://doc.rust-lang.org/std/env/consts/constant.ARCH.html
        if arch == 'i686':
            arch = 'x86'
        elif arch == 'ppc64le':
            arch = 'powerpc64'

        distribution = (minor_version_parts, os_name, arch, abi, variant)
        distributions[distribution].append(((version_parts, date), url))

    flattened_distributions = defaultdict(list)
    for (minor_version_parts, os_name, arch, abi, variant), data in sorted(distributions.items()):
        data.sort(key=lambda x: x[0])
        url = data[-1][1]
        minor_version = '.'.join(map(str, minor_version_parts))
        flattened_distributions[minor_version].append((os_name, arch, abi, variant, url))

    for minor_version, data in flattened_distributions.items():
        data.sort(key=lambda x: (PLATFORMS.index(x[0]), x[1], x[2], x[3], x[4]), reverse=True)
        for (os_name, arch, abi, variant, url) in data:
            lines.insert(insertion_index, f'        "{url}"),')
            lines.insert(insertion_index, f'    ("{minor_version}", "{os_name}", "{arch}", "{abi}", "{variant}",')

    lines.append('')
    Path('build.rs').write_text('\n'.join(lines), encoding='utf-8')
    print('Done')


if __name__ == '__main__':
    main()
