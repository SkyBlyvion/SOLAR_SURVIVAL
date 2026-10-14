#!/usr/bin/env python3
"""Planches de contrôle, sans transformer les captures originales."""
from pathlib import Path
from PIL import Image, ImageDraw
import argparse
p=argparse.ArgumentParser(); p.add_argument('directory', type=Path); a=p.parse_args()
out=a.directory/'inspection'; out.mkdir(exist_ok=False)
labels={
'save':['Accueil','Nouvelle aventure','Nom Alpha','Surface Alpha','Catalogue 12 unités','Placement','Construction','Rebasage/recharge','Alpha sauvegardé','Nom Bravo','Bravo neuf','Bravo sauvegardé','Choix des aventures','Alpha restauré','Recharge et sauvegarde','Liste après retour'],
'load':['Nouveau processus','Liste des aventures','Bravo sélectionné','Bravo chargé','HUD Bravo','Catalogue stock insuffisant','Choix Alpha','Alpha chargé','Sauvegarde espace','Chargement espace','Retour Mars','Échec sauvegardé','Échec chargé','R et réserve pleine','Profil corrompu','Erreur et Alpha conservé']}
for stage,names in labels.items():
    files=sorted((a.directory/stage).glob('stage-*.png'))
    assert len(files)==16, (stage,len(files))
    for start in range(0,16,4):
        sheet=Image.new('RGB',(1440,960),(18,18,18)); draw=ImageDraw.Draw(sheet)
        for i,f in enumerate(files[start:start+4]):
            im=Image.open(f).convert('RGB'); im.thumbnail((720,450))
            x=(i%2)*720; y=(i//2)*480
            sheet.paste(im,(x,y+30)); draw.text((x+10,y+8),f'{stage} {start+i:02} — {names[start+i]}',fill='white')
        sheet.save(out/f'{stage}-{start//4}.jpg',quality=92)
