from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
import json,hashlib
root=Path(__file__).resolve().parent/'graphics'
out=root/'inspection';out.mkdir()
labels=['Arrivée, caisses','Refus : alliage insuffisant','Confirmation refusée','Stock : 6 alliage','Stock : 12 alliage','Refus : volume occupé','Fantôme vert tourné/déplacé','Annulation : stock inchangé','Nouvelle preview admise','Construit : stock 0','Intérieur et recharge','Contact mur','Après rebasage','Mars spatiale','Menu : preview annulée','Retour : recharge abri 2']
font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',17)
manifest=[]
for page in range(4):
    board=Image.new('RGB',(1440,960),(20,24,29));draw=ImageDraw.Draw(board)
    records=[]
    for slot in range(4):
        index=page*4+slot;p=root/'captures'/f'stage-{index:02}.png'
        x=(slot%2)*720;y=(slot//2)*480
        draw.text((x+8,y+5),f'{index:02} — {labels[index]}',font=font,fill=(245,245,245))
        im=Image.open(p).convert('RGB');im.thumbnail((720,450));board.paste(im,(x,y+30))
        records.append(dict(capture=p.name,label=labels[index],sha256=hashlib.sha256(p.read_bytes()).hexdigest()))
    name=f'sheet-{page:02}.jpg';board.save(out/name,quality=93)
    manifest.append(dict(sheet=name,captures=records))
(out/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
print('4 planches, 16 captures, dossier:',out)
