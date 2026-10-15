#!/usr/bin/env python3
"""Inspection X11 de la seule fenêtre du jeu aux poses P9 déjà préparées, sans entrée clavier/souris."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import struct
import subprocess
import threading
import time
from PIL import Image
from focus_diagnostic import run_focused

ROOT=Path(__file__).resolve().parent.parent

def png_from_xwd(path):
    data=path.read_bytes();h=struct.unpack('>25I',data[:100])
    header,version,fmt,depth,width,height,offset,order,unit,bitorder,pad,bpp,stride,visual,red,green,blue,bits,entries,colors,*_=h
    if (version,fmt,depth,offset,order,bpp,red,green,blue)!=(7,2,24,0,0,32,0xff0000,0xff00,0xff):
        raise ValueError(f'Format XWD non pris en charge : {h}')
    start=header+colors*12
    image=Image.frombytes('RGB',(width,height),data[start:start+stride*height],'raw','BGRX',stride,1)
    image.save(path.with_suffix('.png'))
    return dict(width=width,height=height,sha256=hashlib.sha256(path.with_suffix('.png').read_bytes()).hexdigest())

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--output',type=Path,required=True);parser.add_argument('--binary',type=Path,required=True);parser.add_argument('--focus-x11',action='store_true');args=parser.parse_args()
    if not args.focus_x11:parser.error('Parcours visible reporté ; --focus-x11 exige un créneau explicitement demandé par l’utilisateur.')
    out=args.output.resolve();out.mkdir();binary=args.binary.resolve()
    env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
    env.update(SOLAR_P9_VALIDATE_DIR=str(out/'scenario'),SOLAR_P9_STAGE='save',SOLAR_P9_NO_CAPTURES='1',SOLAR_SAVE_ROOT=str(out/'profiles'),MESA_VK_WSI_PRESENT_MODE='immediate',RUST_LOG='info,solar_survival::player::surface=debug')
    errors=[];shots=[]
    def record():
        try:
            for phase,name in [(30,'panel'),(35,'battery'),(40,'generator'),(45,'tank')]:
                deadline=time.monotonic()+130
                while time.monotonic()<deadline:
                    log=(out/'game.txt').read_text()
                    if re.search(r'P9 phase '+str(phase)+r' t=',log):break
                    time.sleep(.05)
                else:raise TimeoutError(f'Phase {phase} absente')
                time.sleep(.75)
                tree=subprocess.check_output(['xwininfo','-root','-tree'],text=True)
                match=re.search(r'(0x[0-9a-f]+) "Solar Survival · 0.2 · Mars": \("solar_survival"',tree)
                if not match:raise RuntimeError('Fenêtre de jeu absente')
                path=out/(name+'.xwd');subprocess.run(['xwd','-silent','-id',match.group(1),'-out',str(path)],check=True)
                shots.append(dict(phase=phase,name=name,window=match.group(1)))
        except Exception as e:errors.append(repr(e))
    initial=hashlib.sha256(binary.read_bytes()).hexdigest();started=time.monotonic()
    with (out/'game.txt').open('w') as log:
        worker=threading.Thread(target=record);worker.start();code,focus=run_focused([str(binary)],env,log,150);worker.join(timeout=5)
    for shot in shots:shot.update(png_from_xwd(out/(shot['name']+'.xwd')))
    diagnostics=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',(out/'game.txt').read_text())
    result=dict(exit_code=code,seconds=time.monotonic()-started,focused_window=focus,errors=errors,diagnostics=diagnostics,shots=shots,binary_sha256=initial,binary_unchanged=hashlib.sha256(binary.read_bytes()).hexdigest()==initial)
    (out/'results.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
    if code or errors or diagnostics or len(shots)!=4 or not result['binary_unchanged']:raise SystemExit(1)

if __name__=='__main__':main()
