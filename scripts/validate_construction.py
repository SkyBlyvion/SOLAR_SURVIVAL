#!/usr/bin/env python3
"""Validation graphique P5, binaire déjà compilé, dossier neuf et preuves conservées."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--mesa-immediate', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    output = args.output.resolve()
    output.mkdir()  # parent must exist; existing evidence must never be replaced
    binary = root / 'target/release/solar_survival'
    env = {k: v for k, v in os.environ.items() if not k.startswith('SOLAR_')}
    env.update(SOLAR_CONSTRUCTION_VALIDATE_DIR=str(output / 'captures'), RUST_LOG='info,solar_survival::player::surface=debug')
    if args.mesa_immediate:
        env['MESA_VK_WSI_PRESENT_MODE'] = 'immediate'
    sources = sorted([*root.glob('src/**/*.rs'), *root.glob('tests/**/*.rs'), root/'Cargo.toml', root/'Cargo.lock'])
    metadata = dict(binary_sha256=sha(binary), source_sha256={str(p.relative_to(root)): sha(p) for p in sources},
                    variables={k: v for k, v in env.items() if k.startswith(('SOLAR_', 'MESA_', 'RUST_LOG'))},
                    display=env.get('DISPLAY'), command=[str(binary)], environment=dict(os=os.uname().sysname) if hasattr(os, 'uname') else dict(os=os.name))
    (output/'environment.json').write_text(json.dumps(metadata, indent=2)+'\n')
    started = time.monotonic()
    with (output/'game.txt').open('w') as log:
        try:
            result = subprocess.run([str(binary)], cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, timeout=100)
            code = result.returncode
        except subprocess.TimeoutExpired:
            code = 'timeout'
    log = (output/'game.txt').read_text()
    record = dict(exit_code=code, seconds=time.monotonic()-started,
                  diagnostics=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*', log),
                  completion=re.findall(r'P5 terminé[^\n]*', log), captures=len(list((output/'captures').glob('*.png'))),
                  hashes_unchanged=sha(binary)==metadata['binary_sha256'] and all(sha(root/p)==h for p,h in metadata['source_sha256'].items()))
    (output/'results.json').write_text(json.dumps(record, indent=2)+'\n')
    print(json.dumps(record, indent=2))
    if code != 0 or record['diagnostics'] or not record['completion'] or record['captures'] != 16 or not record['hashes_unchanged']:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
