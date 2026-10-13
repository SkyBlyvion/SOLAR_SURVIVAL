#!/usr/bin/env python3
"""Caméra/ciel : scénario ciblé release, dossier neuf, sources et captures conservées."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import time
from focus_diagnostic import run_focused

ROOT = Path(__file__).resolve().parent.parent

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--mesa-immediate',action='store_true')
    parser.add_argument('--focus-x11',action='store_true')
    parser.add_argument('--construction',action='store_true')
    args=parser.parse_args()
    out=args.output.resolve();out.mkdir()
    binary=ROOT/'target/release'/('solar_survival.exe' if os.name=='nt' else 'solar_survival')
    sources=sorted([*ROOT.glob('src/**/*.rs'),*ROOT.glob('tests/**/*.rs'),*ROOT.glob('assets/shaders/*.wgsl'),ROOT/'Cargo.toml',ROOT/'Cargo.lock'])
    meta=dict(binary_sha256=sha(binary),source_sha256={str(p.relative_to(ROOT)):sha(p) for p in sources},options=vars(args)|dict(output=str(out)),display=os.environ.get('DISPLAY'))
    (out/'environment.json').write_text(json.dumps(meta,indent=2)+'\n')
    env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
    env.update(RUST_LOG='info,solar_survival::player::surface=debug', **{('SOLAR_CONSTRUCTION_VALIDATE_DIR' if args.construction else 'SOLAR_SKY_VALIDATE_DIR'):str(out/'captures')})
    if args.mesa_immediate:env['MESA_VK_WSI_PRESENT_MODE']='immediate'
    started=time.monotonic()
    with (out/'game.txt').open('w') as log:
        if args.focus_x11: code,focus=run_focused([str(binary)],env,log,100)
        else:
            import subprocess
            code=subprocess.run([str(binary)],cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=100).returncode
            focus=None
    log=(out/'game.txt').read_text()
    result=dict(exit_code=code,seconds=time.monotonic()-started,focused_window=focus,
        diagnostics=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',log),
        completion=re.findall(r'(?:SKY|P5) terminé[^\n]*',log),captures=len(list((out/'captures').glob('*.png'))),
        hashes_unchanged=sha(binary)==meta['binary_sha256'] and all(sha(ROOT/p)==h for p,h in meta['source_sha256'].items()))
    (out/'results.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
    if code or result['diagnostics'] or not result['completion'] or result['captures']!=(16 if args.construction else 24) or not result['hashes_unchanged']:raise SystemExit(1)

if __name__=='__main__':main()
