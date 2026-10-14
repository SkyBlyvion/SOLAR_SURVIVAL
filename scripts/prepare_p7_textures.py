#!/usr/bin/env python3
"""Exports offline P7. Pillow et NumPy sont requis ici seulement, jamais dans le jeu.
P7TX = magic + largeur/hauteur/niveaux u32 LE + mipmaps RGBA8 contigus.
RGB filtré en linéaire puis réencodé sRGB ; alpha = donnée linéaire.
"""
from pathlib import Path
import hashlib
import json
import struct
import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / 'assets/textures'
MASTER = ROOT / 'art/p7-masters'


def linear(rgb):
    return np.where(rgb <= .04045, rgb / 12.92, ((rgb + .055) / 1.055) ** 2.4)


def srgb(rgb):
    return np.where(rgb <= .0031308, rgb * 12.92, 1.055 * np.maximum(rgb, 0) ** (1/2.4) - .055)


def export(name, image, repeat=True):
    image = image.convert('RGBA')
    w, h = image.size
    data = np.asarray(image, dtype=np.float32) / 255
    data[:, :, :3] = linear(data[:, :, :3])
    payload = bytearray()
    sizes = []
    while True:
        output = data.copy()
        output[:, :, :3] = srgb(output[:, :, :3])
        payload.extend(np.uint8(np.clip(np.rint(output * 255), 0, 255)).tobytes())
        height, width = data.shape[:2]
        sizes.append([width, height])
        if width == height == 1:
            break
        data = np.stack([np.asarray(Image.fromarray(data[:,:,i], mode='F').resize(
            (max(1, width//2), max(1, height//2)), Image.Resampling.BOX)) for i in range(4)], axis=2)
    path = OUT / (name + '.p7tex')
    path.write_bytes(struct.pack('<4sIII', b'P7TX', w, h, len(sizes)) + payload)
    return dict(file=path.name, width=w, height=h, mip_levels=len(sizes),
                format='Rgba8UnormSrgb', repeat=repeat, gpu_bytes=len(payload),
                sha256=hashlib.sha256(path.read_bytes()).hexdigest())


def panel(kind):
    im = Image.new('RGB', (512,512), (170,181,177) if kind=='wall' else (78,90,91))
    d = ImageDraw.Draw(im)
    # Dessin natif reproductible : panneau d'un mètre, joint/vis, usure mesurée artistiquement.
    d.rectangle((0,0,511,511), outline=(49,60,61), width=7)
    d.rectangle((13,13,498,498), outline=(123,137,135), width=2)
    if kind=='wall':
        d.rectangle((20,420,492,471), fill=(49,91,95))
        for x in [25,487]:
            for y in [25,400,487]:
                d.ellipse((x-4,y-4,x+4,y+4), fill=(57,67,67))
                d.line((x-2,y,x+2,y),fill=(175,180,176),width=1)
    else:
        for x in range(32,490,36):
            d.rectangle((x,24,x+12,488),fill=(111,123,123))
    rng=np.random.default_rng(713 if kind=='wall' else 714)
    arr=np.asarray(im).astype(np.int16)
    arr += rng.integers(-3,4,(512,512,1),dtype=np.int16)
    return Image.fromarray(np.uint8(np.clip(arr,0,255)))


def crate():
    im=Image.new('RGB',(512,512),(152,105,43)); d=ImageDraw.Draw(im)
    d.rectangle((0,0,511,511),outline=(37,43,46),width=32)
    d.rectangle((40,40,471,471),outline=(198,157,77),width=5)
    for y in [66,92,416,442]:
        d.line((54,y,458,y),fill=(82,67,45),width=6)
    for x in [40,472]:
        for y in [40,472]: d.ellipse((x-9,y-9,x+9,y+9),fill=(146,156,154))
    d.rounded_rectangle((113,160,399,348),radius=8,fill=(38,46,48))
    f=ImageFont.truetype(str(ROOT/'assets/fonts/DejaVuSans.ttf'),38)
    d.text((256,228),'ALLIAGE',font=f,anchor='mm',fill=(231,224,192))
    f=ImageFont.truetype(str(ROOT/'assets/fonts/DejaVuSans.ttf'),25)
    d.text((256,283),'06  /  E',font=f,anchor='mm',fill=(199,165,86))
    return im


def sign():
    # Rapport métrique 6:1, tout le texte dans les 512x128 de la plaque.
    im=Image.new('RGB',(512,128),(22,51,55));d=ImageDraw.Draw(im)
    d.rectangle((4,4,507,123),outline=(144,204,198),width=4)
    f=ImageFont.truetype(str(ROOT/'assets/fonts/DejaVuSans.ttf'),48)
    d.text((28,29),'O₂',font=f,fill=(233,241,223))
    f=ImageFont.truetype(str(ROOT/'assets/fonts/DejaVuSans.ttf'),30)
    d.text((128,50),'RECHARGE',font=f,fill=(233,241,223))
    return im


def main():
    OUT.mkdir(exist_ok=True)
    records=[]
    for name,source,size in [('earth','earth-january.jpg',(2048,1024)),('mars','mars.jpg',(1440,720))]:
        im=Image.open(MASTER/source).convert('RGB')
        records.append(export(name,im.resize(size,Image.Resampling.LANCZOS),False))
    im=Image.open(MASTER/'regolith-generated.png').convert('RGB').resize((1024,1024),Image.Resampling.LANCZOS)
    a=np.asarray(im,dtype=np.float32)
    # Export raccordé : crossfade périodique étroit de 32 px, sans génération en jeu.
    for axis in [0,1]:
        for i in range(32):
            weight=(1-i/32)*.5
            if axis==0:
                x,y=a[i].copy(),a[-1-i].copy();a[i]=x*(1-weight)+y*weight;a[-1-i]=y*(1-weight)+x*weight
            else:
                x,y=a[:,i].copy(),a[:,-1-i].copy();a[:,i]=x*(1-weight)+y*weight;a[:,-1-i]=y*(1-weight)+x*weight
    rgba=np.dstack([np.uint8(np.clip(a,0,255)),np.full((1024,1024),255,dtype=np.uint8)])
    records.append(export('regolith',Image.fromarray(rgba)))
    for name,im in [('wall',panel('wall')),('roof',panel('roof')),('crate',crate()),('oxygen-sign',sign())]:
        records.append(export(name,im,name in ['wall','roof']))
    from prepare_orbital_textures import prepare
    records.extend(prepare(export))
    meta=dict(textures=records,total_gpu_bytes=sum(x['gpu_bytes'] for x in records),budget_gpu_bytes=128*1024**2,
              masters={str(p.relative_to(ROOT)):dict(sha256=hashlib.sha256(p.read_bytes()).hexdigest(),dimensions=Image.open(p).size) for p in sorted(MASTER.glob('*')) if p.suffix in ['.jpg','.png']})
    (OUT/'manifest.json').write_text(json.dumps(meta,indent=2)+'\n')
    print(json.dumps(meta,indent=2))

if __name__=='__main__':main()
