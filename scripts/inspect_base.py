#!/usr/bin/env python3
"""Planches locales de vérification des captures P9, sans modifier les originaux."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();args.output.mkdir()
    paths=sorted(args.input.glob('save/stage-*.png'))+sorted(args.input.glob('load/stage-*.png'))
    font=ImageFont.truetype(str(ROOT/'assets/fonts/DejaVuSans.ttf'),16)
    sheets=[]
    for batch in range(0,len(paths),9):
        sheet=Image.new('RGB',(1440,990),(22,25,27));draw=ImageDraw.Draw(sheet)
        group=paths[batch:batch+9]
        for index,path in enumerate(group):
            x=(index%3)*480;y=(index//3)*330
            image=Image.open(path).convert('RGB');image.thumbnail((480,300));sheet.paste(image,(x,y+28))
            draw.text((x+8,y+5),path.parent.name+'/'+path.name,font=font,fill='white')
        target=args.output/f'sheet-{batch//9:02}.jpg';sheet.save(target,quality=92)
        sheets.append({'path':str(target),'captures':[{'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in group]})
    (args.output/'manifest.json').write_text(json.dumps({'capture_count':len(paths),'sheets':sheets},indent=2)+'\n')
    print(json.dumps({'capture_count':len(paths),'sheets':len(sheets)}))

if __name__=='__main__':main()
