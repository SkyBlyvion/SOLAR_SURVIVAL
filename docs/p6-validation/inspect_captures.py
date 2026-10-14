from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
import hashlib,json,sys
root=Path(sys.argv[1]).resolve();out=root/'inspection';out.mkdir()
labels=['Arrivée','Stock 12','Placement vert','Abri construit','Recharge après rebasage','Menu avant sauvegarde','Sauvegarde réussie','Fichier sauvegardé','Menu au redémarrage','Session chargée','Pose/stocks restaurés','Recharge après reprise','Sauvegarde en espace','Chargement en espace','Retour sur Mars','Échec sauvegardé','Échec chargé','R après chargement','Fichier corrompu refusé','Partie conservée après refus']
files=[*sorted((root/'save').glob('stage-*.png')),*sorted((root/'load').glob('stage-*.png'))]
assert len(files)==len(labels)==20
font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',17)
manifest=[]
for page in range(5):
    board=Image.new('RGB',(1440,960),(20,24,29));draw=ImageDraw.Draw(board);records=[]
    for slot in range(4):
        index=page*4+slot;p=files[index];x=slot%2*720;y=slot//2*480
        draw.text((x+8,y+5),f'{p.parent.name}/{p.stem} — {labels[index]}',font=font,fill=(245,245,245))
        im=Image.open(p).convert('RGB');im.thumbnail((720,450));board.paste(im,(x,y+30))
        records.append(dict(capture=str(p.relative_to(root)),label=labels[index],sha256=hashlib.sha256(p.read_bytes()).hexdigest()))
    name=f'board-{page:02}.jpg';board.save(out/name,quality=93);manifest.append(dict(board=name,captures=records))
(out/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
print(out)
