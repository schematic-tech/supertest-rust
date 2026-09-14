#!/usr/bin/env python3
"""Publish in dependency order; retry safely after a partial release."""
import json
import urllib.error
import urllib.request
from release import ROOT, VERSION, require_public_tag, run

require_public_tag()
for name in ('schematic-supertest-macros', 'schematic-supertest'):
    request = urllib.request.Request(f'https://crates.io/api/v1/crates/{name}/{VERSION}',
                                    headers={'User-Agent': 'schematic-supertest-release (https://github.com/schematic-tech/supertest-rust)'})
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            published = json.load(response)
        if published['version']['yanked']:
            raise SystemExit(f'{name} {VERSION} is yanked; choose a new version')
        print(f'{name} {VERSION} is already published; continuing')
    except urllib.error.HTTPError as error:
        if error.code != 404:
            raise
        # Cargo waits for registry availability before returning; the main crate
        # can then resolve the just-published macro crate.
        run('cargo', 'publish', '--locked', '--package', name)
