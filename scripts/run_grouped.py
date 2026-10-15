#!/usr/bin/env python3
"""Campagne explicite du diagnostic renderer/audio P10–P12, sans périphériques d'entrée simulés."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
from focus_diagnostic import run_focused

ROOT=Path(__file__).resolve().parent.parent

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--output',required=True,type=Path)
    p.add_argument('--height',type=int,choices=[720,900],default=900)
    p.add_argument('--mode',choices=['full','load','perf','cases','menus'],default='full')
    p.add_argument('--save-root',required=True,type=Path)
    p.add_argument('--allow-window-and-audio',action='store_true',required=True)
    args=p.parse_args()
    out=args.output.resolve()
    out.parent.mkdir(parents=True,exist_ok=True)
    binary=ROOT/'target/release/examples/grouped_validation'
    env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
    env.update(SOLAR_SAVE_ROOT=str(args.save_root.resolve()),SOLAR_RESOURCE_SAMPLES=str(out.with_suffix('.resources.csv')),MESA_VK_WSI_PRESENT_MODE='immediate',RUST_LOG='info,solar_survival::player::surface=debug')
    before=hashlib.sha256(binary.read_bytes()).hexdigest()
    start=time.monotonic()
    with out.with_suffix('.log').open('w') as log:
        code,focused=run_focused([str(binary),str(out),str(args.height),args.mode],env,log,720)
    log=out.with_suffix('.log').read_text()
    record=dict(mode=args.mode,height=args.height,exit_code=code,seconds=time.monotonic()-start,focused_window=focused,
                binary_sha256=before,binary_unchanged=hashlib.sha256(binary.read_bytes()).hexdigest()==before,
                captures=len(list(out.glob('*.png'))),
                diagnostics=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',log),
                done=(out/'assertions.txt').exists() and 'DONE' in (out/'assertions.txt').read_text())
    out.with_suffix('.result.json').write_text(json.dumps(record,indent=2)+'\n')
    print(json.dumps(record,indent=2),flush=True)
    raise SystemExit(0 if code==0 and record['done'] else 1)
if __name__=='__main__': main()
