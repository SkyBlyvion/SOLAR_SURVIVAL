#!/usr/bin/env python3
"""Comparaison P8/P9 sur le parcours P5 inchangé, sans captures ; binaire P8 avec ses assets propres."""
import argparse
import csv
import ctypes
import threading
import subprocess
import hashlib
import json
import os
from pathlib import Path
import re
import statistics
import time
from focus_diagnostic import run_focused

ROOT = Path(__file__).resolve().parent.parent

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--focus-x11',action='store_true',help='Autorisation explicite du créneau visible et du focus X11')
    args=parser.parse_args()
    if not args.focus_x11:
        parser.error('Parcours visible reporté : --focus-x11 exige un créneau explicitement demandé par l’utilisateur.')
    out=args.output.resolve();out.mkdir()
    binaries={'p8':args.baseline.resolve(),'p9-1':ROOT/'target/release/solar_survival','p9-2':ROOT/'target/release/solar_survival','p9-gpu':ROOT/'target/release/solar_survival'}
    files=[*ROOT.glob('src/**/*.rs'),*ROOT.glob('tests/**/*.rs'),*[p for p in ROOT.glob('assets/**/*') if p.is_file()],ROOT/'Cargo.toml',ROOT/'Cargo.lock']
    initial={str(p):sha(p) for p in [*files,*set(binaries.values())]}
    results={}
    for name,binary in binaries.items():
        env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
        env.update(SOLAR_CONSTRUCTION_VALIDATE_DIR=str(out/(name+'-scenario')),SOLAR_P5_TIMING_OUTPUT=str(out/(name+'.csv')),SOLAR_P5_TIMING_GPU=str(int(name.endswith('gpu'))),SOLAR_RESOURCE_SAMPLES=str(out/(name+'-resources.csv')),MESA_VK_WSI_PRESENT_MODE='immediate',RUST_LOG='info,solar_survival::player::surface=debug')
        started=time.monotonic()
        stop=threading.Event();focus_trace=[]
        def watch_focus():
            xlib=ctypes.CDLL('libX11.so.6');xlib.XOpenDisplay.restype=ctypes.c_void_p
            xlib.XGetInputFocus.argtypes=[ctypes.c_void_p,ctypes.POINTER(ctypes.c_ulong),ctypes.POINTER(ctypes.c_int)]
            xlib.XCloseDisplay.argtypes=[ctypes.c_void_p]
            display=xlib.XOpenDisplay(None)
            if not display:return
            game_window=None
            try:
                while not stop.wait(.5):
                    if game_window is None:
                        tree=subprocess.check_output(['xwininfo','-root','-tree'],text=True)
                        match=re.search(r'(0x[0-9a-f]+) "Solar Survival · 0.2 · Mars": \(\"solar_survival\"',tree)
                        if match:game_window=int(match.group(1),16)
                    window=ctypes.c_ulong();revert=ctypes.c_int();xlib.XGetInputFocus(display,ctypes.byref(window),ctypes.byref(revert))
                    focus_trace.append({'seconds':time.monotonic()-started,'game_focused':game_window==window.value,'focused_window':hex(window.value)})
            finally:xlib.XCloseDisplay(display)
        watcher=threading.Thread(target=watch_focus);watcher.start()
        try:
            with (out/(name+'.txt')).open('w') as log:
                code,focus=run_focused([str(binary)],env,log,100)
        finally:
            stop.set();watcher.join()
            (out/(name+'-focus.json')).write_text(json.dumps(focus_trace,indent=2)+'\n')
        log=(out/(name+'.txt')).read_text()
        record={'exit_code':code,'seconds':time.monotonic()-started,'focused_window':focus,'diagnostics':re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',log),'completion':bool(re.search(r'P5 terminé',log)),'binary_sha256':sha(binary)}
        results[name]={'run':record}
        (out/'comparison.json').write_text(json.dumps(results,indent=2)+'\n')
        print(json.dumps({name:record},indent=2),flush=True)
        if code or record['diagnostics'] or not record['completion']:raise SystemExit(1)
        groups={}
        with (out/(name+'.csv')).open() as data:
            for row in csv.DictReader(data):
                if row['phase'] not in ['built_rest','preview','streaming']:continue
                groups.setdefault(row['phase']+'/'+row['metric'],[]).append(float(row['value_ms']))
        for key,values in groups.items():
            values.sort();results[name][key]={'samples':len(values),'mean_ms':statistics.mean(values),'p95_ms':values[min(len(values)-1,int(len(values)*.95))],'max_ms':max(values)}
        (out/'comparison.json').write_text(json.dumps(results,indent=2)+'\n')
    concordance={'all_hashes_unchanged':all(sha(Path(p))==h for p,h in initial.items()),'initial_hashes':initial}
    (out/'concordance.json').write_text(json.dumps(concordance,indent=2)+'\n')
    if not concordance['all_hashes_unchanged']:raise SystemExit(1)

if __name__=='__main__':main()
