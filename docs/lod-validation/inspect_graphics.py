from pathlib import Path
from PIL import Image,ImageDraw
import json,hashlib,sys
root=Path(sys.argv[1]);out=root/'inspection';out.mkdir()
manifest=[]
for folder in root.glob('*-captures'):
 images=sorted(folder.glob('*.png'))
 for start in range(0,len(images),4):
  chosen=images[start:start+4];sheet=Image.new('RGB',(1440,960),'#162029');draw=ImageDraw.Draw(sheet)
  records=[]
  for i,path in enumerate(chosen):
   im=Image.open(path).convert('RGB');im.thumbnail((720,450))
   x=(i%2)*720;y=(i//2)*480
   draw.text((x+12,y+8),folder.name+'/'+path.name,fill='white')
   sheet.paste(im,(x,y+30));records.append(dict(path=str(path.relative_to(root)),sha256=hashlib.sha256(path.read_bytes()).hexdigest()))
  name=f'{folder.name}-{start//4:02}.jpg';sheet.save(out/name,quality=94);manifest.append(dict(sheet=name,captures=records))
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
