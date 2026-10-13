#!/usr/bin/env python3
"""Parcours cube bornés et régression historique, successifs, sans compilation."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import time

ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--mesa-immediate', action='store_true')
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    if (output / 'environment.json').exists() or (output / 'graphics-results.json').exists():
        parser.error('Choisir un dossier sans ancienne campagne graphique')
    binary = ROOT / 'target/release' / ('solar_survival.exe' if os.name == 'nt' else 'solar_survival')
    if not binary.exists():
        parser.error('Compiler le binaire release avant la campagne')
    sources = sorted(ROOT.glob('src/**/*.rs')) + sorted(ROOT.glob('tests/*.rs')) + sorted(ROOT.glob('examples/*.rs')) + sorted(ROOT.glob('assets/shaders/*')) + [ROOT/'Cargo.toml', ROOT/'Cargo.lock', Path(__file__).resolve()]
    metadata = dict(date_client='2026-10-13', timezone_client='Europe/Paris',
                    platform=platform.platform(), start_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                    rustc=subprocess.check_output(['rustc', '--version'], text=True).strip(),
                    display=os.environ.get('DISPLAY'), session_type=os.environ.get('XDG_SESSION_TYPE'),
                    source_sha256={str(p.relative_to(ROOT)): sha(p) for p in sources},
                    binary_sha256=sha(binary), mesa_immediate=args.mesa_immediate,
                    windows_validated=False, compilation_and_gpu_runs='sequential')
    (output/'environment.json').write_text(json.dumps(metadata, indent=2)+'\n')
    results = []
    for name, variable in [('offcenter', 'SOLAR_CUBE_OFFCENTER'), ('central-regression', 'SOLAR_CUBE_MIGRATION'),
                           ('p32-regression', 'SOLAR_CUBE_SURFACE'), ('historical', 'SOLAR_ROTATION_SMOKE_SECONDS')]:
        env = {k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
        env.update(RUST_LOG='info')
        if args.mesa_immediate:
            env['MESA_VK_WSI_PRESENT_MODE'] = 'immediate'
        cwd = ROOT
        if name == 'historical':
            cwd = output / name
            (cwd/'docs').mkdir(parents=True)
            env[variable] = '50'
            images = cwd / 'docs'
        else:
            env[variable] = '1'
            images = output/name
            env['SOLAR_CUBE_VALIDATE_DIR'] = str(images)
        started = time.monotonic()
        print('Lancement', name, flush=True)
        code = None
        failure = None
        with (output/(name+'.txt')).open('w') as log:
            try:
                code = subprocess.run([str(binary)], cwd=cwd, env=env, stdout=log,
                                      stderr=subprocess.STDOUT, timeout=160).returncode
            except subprocess.TimeoutExpired:
                failure = 'timeout 160 s ; processus arrêté par le lanceur'
        content = (output/(name+'.txt')).read_text()
        result = dict(name=name, command=[str(binary)], cwd=str(cwd),
                      variables={k:v for k,v in env.items() if k.startswith('SOLAR_') or k in ('RUST_LOG','MESA_VK_WSI_PRESENT_MODE')},
                      exit_code=code, failure=failure, seconds=time.monotonic()-started,
                      captures=len(list(images.glob('*.png'))),
                      diagnostics=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*', content),
                      completion=re.findall(r'P3\.[234] terminé[^\n]*', content))
        results.append(result)
        (output/'graphics-results.json').write_text(json.dumps(results, indent=2)+'\n')
        print(name, code, result['captures'], result['completion'], flush=True)
        if code != 0 or result['diagnostics']:
            raise SystemExit('Examiner la campagne interrompue ou ses diagnostics avant de relancer')


if __name__ == '__main__':
    main()
