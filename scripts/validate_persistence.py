#!/usr/bin/env python3
"""P6 : compilation préalable, sauvegarde et chargement dans deux processus, dossier neuf."""
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

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--mesa-immediate',action='store_true')
    parser.add_argument('--focus-x11',action='store_true')
    parser.add_argument('--small-window',action='store_true')
    parser.add_argument('--binary',type=Path)
    parser.add_argument('--cwd',type=Path)
    args=parser.parse_args()
    out=args.output.resolve(); out.mkdir()
    binary=(args.binary or ROOT/'target/release'/('solar_survival.exe' if os.name=='nt' else 'solar_survival')).resolve()
    paths=sorted([*ROOT.glob('src/**/*.rs'),*ROOT.glob('tests/**/*.rs'),*ROOT.glob('assets/shaders/*'),ROOT/'Cargo.toml',ROOT/'Cargo.lock'])
    meta=dict(binary_sha256=sha(binary),source_sha256={str(p.relative_to(ROOT)):sha(p) for p in paths},binary=str(binary),display=os.environ.get('DISPLAY'))
    (out/'environment.json').write_text(json.dumps(meta,indent=2)+'\n')
    env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
    env.update(SOLAR_SAVE_ROOT=str(out/'profiles'),RUST_LOG='info,solar_survival::player::surface=debug')
    if args.mesa_immediate: env['MESA_VK_WSI_PRESENT_MODE']='immediate'
    if args.small_window: env['SOLAR_P6_SMALL_WINDOW']='1'
    results=[]
    for stage,expected in [('save',16),('load',16)]:
        env.update(SOLAR_P6_STAGE=stage,SOLAR_P6_VALIDATE_DIR=str(out/stage))
        started=time.monotonic()
        with (out/(stage+'.txt')).open('w') as log:
            if args.focus_x11:
                if args.cwd: raise SystemExit('--cwd ne se combine pas avec --focus-x11')
                code,focus=run_focused([str(binary)],env,log,90)
            else:
                code=subprocess.run([str(binary)],cwd=args.cwd or ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=90).returncode; focus=None
        log=(out/(stage+'.txt')).read_text()
        record=dict(stage=stage,exit_code=code,seconds=time.monotonic()-started,focused_window=focus,captures=len(list((out/stage).glob('stage-*.png'))),diagnostics=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',log),completion=re.findall(r'P6 '+stage+r' terminé[^\n]*',log))
        results.append(record);(out/'results.json').write_text(json.dumps(results,indent=2)+'\n'); print(json.dumps(record,indent=2),flush=True)
        if code or record['diagnostics'] or not record['completion'] or record['captures']!=expected: raise SystemExit(1)
        if stage=='save':
            import shutil
            shutil.copytree(out/'profiles',out/'profiles-after-save')
    concordance=dict(binary_unchanged=sha(binary)==meta['binary_sha256'],sources_unchanged=all(sha(ROOT/p)==h for p,h in meta['source_sha256'].items()))
    (out/'concordance.json').write_text(json.dumps(concordance,indent=2)+'\n')
    if not all(concordance.values()): raise SystemExit(1)

if __name__=='__main__': main()
