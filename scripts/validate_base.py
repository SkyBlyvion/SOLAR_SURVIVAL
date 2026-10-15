#!/usr/bin/env python3
"""P9 : boucle graphique et restauration dans deux processus du binaire release déjà compilé."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
from focus_diagnostic import run_focused

ROOT = Path(__file__).resolve().parent.parent

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--mesa-immediate', action='store_true')
    parser.add_argument('--focus-x11', action='store_true')
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--cwd', type=Path)
    parser.add_argument('--allow-window', action='store_true', help='Créneau visible explicitement demandé ; une fenêtre peut prendre le focus via le bureau')
    args = parser.parse_args()
    if not (args.focus_x11 or args.allow_window):
        parser.error('Parcours visible reporté : autorisation de créneau requise (--allow-window ou --focus-x11).')
    out = args.output.resolve()
    out.mkdir()
    binary = (args.binary or ROOT/'target/release/solar_survival').resolve()
    sources = sorted([*ROOT.glob('src/**/*.rs'), *ROOT.glob('tests/**/*.rs'), *ROOT.glob('assets/**/*'), ROOT/'Cargo.toml', ROOT/'Cargo.lock'])
    meta = dict(binary=str(binary), binary_sha256=sha(binary), sources={str(p.relative_to(ROOT)):sha(p) for p in sources if p.is_file()}, display=os.environ.get('DISPLAY'), cwd=str(args.cwd or ROOT))
    (out/'environment.json').write_text(json.dumps(meta, indent=2)+'\n')
    env = {k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
    env.update(SOLAR_SAVE_ROOT=str(out/'profiles'), RUST_LOG='info,solar_survival::player::surface=debug')
    if args.mesa_immediate:
        env['MESA_VK_WSI_PRESENT_MODE'] = 'immediate'
    results = []
    for stage, expected in [('save',27), ('load',9)]:
        env.update(SOLAR_P9_STAGE=stage, SOLAR_P9_VALIDATE_DIR=str(out/stage), SOLAR_RESOURCE_SAMPLES=str(out/(stage+'-resources.csv')))
        started = time.monotonic()
        with (out/(stage+'.txt')).open('w') as log:
            if args.focus_x11:
                code, focused = run_focused([str(binary)], env, log, 170, cwd=args.cwd)
            else:
                code = subprocess.run([str(binary)], cwd=args.cwd or ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, timeout=170).returncode
                focused = None
        log = (out/(stage+'.txt')).read_text()
        record = dict(stage=stage, exit_code=code, seconds=time.monotonic()-started, focused_window=focused,
                      captures=len(list((out/stage).glob('stage-*.png'))),
                      diagnostics=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*', log),
                      completion=re.findall(r'P9 '+stage+r' terminé[^\n]*', log))
        results.append(record)
        (out/'results.json').write_text(json.dumps(results, indent=2)+'\n')
        print(json.dumps(record, indent=2), flush=True)
        if code or record['diagnostics'] or not record['completion'] or record['captures'] != expected:
            raise SystemExit(1)
        if stage == 'save':
            import shutil
            shutil.copytree(out/'profiles', out/'profiles-after-save')
    concordance = dict(binary_unchanged=sha(binary)==meta['binary_sha256'], sources_unchanged=all(sha(ROOT/p)==h for p,h in meta['sources'].items()))
    (out/'concordance.json').write_text(json.dumps(concordance, indent=2)+'\n')
    if not all(concordance.values()):
        raise SystemExit(1)

if __name__ == '__main__':
    main()
