# /// script
# dependencies = [
#   "httpx",
#   "packaging",
# ]
# ///
import os
from collections import defaultdict
from contextlib import suppress
from pathlib import Path

import httpx
from packaging.version import Version

RELEASES_URL = 'https://api.github.com/repos/indygreg/python-build-standalone/releases'
PLATFORMS = ('linux', 'windows', 'macos')


def remove_extensions(filename: str) -> str:
    for _ in range(2):
        filename, _ = os.path.splitext(filename)

    return filename


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
        if not name.endswith(('.tar.gz', '.tar.zst')):
            continue

        # Rely on the very latest artifact naming
        if not (name.endswith('-install_only_stripped.tar.gz') or 'freethreaded' in name):
            continue

        # Examples:
        # cpython-3.13.0+20241008-x86_64-pc-windows-msvc-install_only_stripped.tar.gz
        # cpython-3.13.0+20241008-x86_64-pc-windows-msvc-shared-install_only_stripped.tar.gz - deprecated
        # cpython-3.13.0+20241008-x86_64-pc-windows-msvc-shared-freethreaded+pgo-full.tar.zst - variants: freethreaded
        # cpython-3.13.0+20241008-x86_64-apple-darwin-install_only_stripped.tar.gz
        # cpython-3.13.0+20241008-aarch64-apple-darwin-install_only_stripped.tar.gz
        # cpython-3.13.0+20241008-aarch64-apple-darwin-freethreaded+pgo+lto-full.tar.zst - variants: freethreaded
        # cpython-3.13.0+20241008-x86_64-unknown-linux-musl-install_only_stripped.tar.gz
        # cpython-3.13.0+20241008-x86_64-unknown-linux-gnu-install_only_stripped.tar.gz
        # cpython-3.13.0+20241008-x86_64-unknown-linux-gnu-freethreaded+pgo+lto-full.tar.zst
        # cpython-3.13.0+20241008-x86_64_v2-unknown-linux-gnu-install_only_stripped.tar.gz - variants: v2
        # cpython-3.13.0+20241008-x86_64_v2-unknown-linux-gnu-freethreaded+pgo+lto-full.tar.zst - variants: v2, freethreaded
        impl, release_data, *remaining = remove_extensions(name).split('-')
        if impl != 'cpython':
            continue

        raw_version, _, release = release_data.partition('+')
        version = Version(raw_version)

        # Skip prereleases for now
        if version.pre is not None:
            continue

        variant_start = 3 if 'apple' in remaining else 4
        target_parts = remaining[:variant_start]
        variant_parts = remaining[variant_start:]
        for possible_variant in ('install_only_stripped', 'full'):
            with suppress(ValueError):
                variant_parts.remove(possible_variant)

        variants = variant_parts[0].split('+') if variant_parts else []

        # Windows no longer supports variants but `shared` is still shipped as an alias
        if 'windows' in target_parts and 'shared' in variants:
            continue

        arch = target_parts[0]
        abi = '' if 'apple' in target_parts else target_parts[3]
        minor_version_parts = (version.major, version.minor)
        date = int(release)

        variant_gil = 'freethreaded' if 'freethreaded' in variants else ''
        variant_cpu = ''
        if 'windows' in target_parts:
            os_name = 'windows'
        elif 'apple' in target_parts:
            os_name = 'macos'
        elif 'linux' in target_parts:
            os_name = 'linux'
            if '_v' in arch:
                arch, _, variant_cpu = arch.rpartition('_')
            # Set to v1 since disabling with an empty string would trigger the defaults
            elif arch == 'x86_64':
                variant_cpu = 'v1'
        else:
            raise ValueError(f'unknown platform: {name}')

        # https://doc.rust-lang.org/std/env/consts/constant.ARCH.html
        if arch == 'i686':
            arch = 'x86'
        elif arch == 'ppc64le':
            arch = 'powerpc64'

        distribution = (minor_version_parts, os_name, arch, abi, variant_cpu, variant_gil)
        distributions[distribution].append((((version.major, Version.minor, version.micro), date), url))

    flattened_distributions = defaultdict(list)
    for (
        minor_version_parts, os_name, arch, abi, variant_cpu, variant_gil
    ), data in sorted(distributions.items()):
        data.sort(key=lambda x: x[0])
        url = data[-1][1]
        minor_version = '.'.join(map(str, minor_version_parts))
        flattened_distributions[minor_version].append((os_name, arch, abi, variant_cpu, variant_gil, url))

    for minor_version, data in flattened_distributions.items():
        data.sort(key=lambda x: (PLATFORMS.index(x[0]), x[1], x[2], x[3], x[4], x[5]), reverse=True)
        for (os_name, arch, abi, variant_cpu, variant_gil, url) in data:
            lines.insert(insertion_index, f'        "{url}"),')
            lines.insert(insertion_index, f'    ("{minor_version}", "{os_name}", "{arch}", "{abi}", "{variant_cpu}", "{variant_gil}",')

    lines.append('')
    Path('build.rs').write_text('\n'.join(lines), encoding='utf-8')
    print('Done')


if __name__ == '__main__':
    main()
