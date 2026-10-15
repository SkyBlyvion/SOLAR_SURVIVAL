#!/usr/bin/env python3
"""Archive portable du build natif déjà compilé ; assets et licences inclus, dossier neuf."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import tarfile
import zipfile

ROOT = Path(__file__).resolve().parent.parent

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    system = platform.system()
    if system not in ('Linux', 'Windows'):
        parser.error('Packaging pris en charge sur Linux ou Windows natif')
    out = args.output.resolve()
    archive = out.with_name(out.name + ('.zip' if system == 'Windows' else '.tar.gz'))
    if out.exists() or archive.exists():
        parser.error('Dossier et archive doivent être nouveaux')
    binary = ROOT / 'target/release' / ('solar_survival.exe' if system == 'Windows' else 'solar_survival')
    if not binary.is_file():
        parser.error('Compiler d’abord cargo build --release --locked sur cet OS')
    out.mkdir(parents=True)
    shutil.copy2(binary, out / binary.name)
    shutil.copytree(ROOT / 'assets', out / 'assets')
    shutil.copy2(ROOT / 'LICENSE', out / 'LICENSE')
    metadata = json.loads(subprocess.check_output(['cargo','metadata','--locked','--offline','--format-version','1'],cwd=ROOT,text=True))
    third_party = []
    for package in metadata['packages']:
        if package['name'] == 'solar_survival': continue
        source = Path(package['manifest_path']).parent
        texts = [p for p in source.iterdir() if p.is_file() and p.name.upper().startswith(('LICENSE','COPYING','COPYRIGHT'))]
        if package.get('license_file'):
            explicit = source / package['license_file']
            if explicit.is_file() and explicit not in texts: texts.append(explicit)
        destination = out / 'licenses' / (package['name']+'-'+package['version'])
        if texts:
            destination.mkdir(parents=True)
            for text in texts: shutil.copy2(text,destination/text.name)
        third_party.append(dict(name=package['name'],version=package['version'],license=package.get('license'),files=[str(destination.relative_to(out)/p.name) for p in texts],repository=package.get('repository')))
    (out/'third-party.json').write_text(json.dumps(third_party,indent=2)+'\n')
    if system == 'Linux':
        launcher=out/'lancer.sh'
        launcher.write_text('#!/bin/sh\ncd -- "$(dirname -- "$0")" || exit 1\nexec ./solar_survival "$@"\n')
        launcher.chmod(0o755)
    else:
        (out/'lancer.cmd').write_bytes(b'@echo off\r\ncd /d "%~dp0"\r\nsolar_survival.exe\r\n')
    (out / 'LIRE-MOI.txt').write_text(
        'Solar Survival 0.2 · P8 relief, horizons et atmosphère martienne\n\n'
        'Ubuntu : lancer ./solar_survival depuis ce dossier. Pilote Vulkan/X11 requis.\n'
        'Windows 11 : lancer solar_survival.exe depuis ce dossier. Pilote graphique récent requis.\n'
        'Les résultats Ubuntu ne valent pas validation Windows.\n\n'
        'Nouvelle aventure, saisir un nom, Créer ; puis M : Mars. Clic droit, WASD/ZQSD, Shift, Espace.\n'
        'P8 : relief artistique, paysage lointain, ciel jour/nuit, halo bleu, voiles de glace et tempêtes.\n'
        'P/T : figer ou accélérer le calendrier et la météo ; H : télémétrie atmosphérique.\n'
        'Sauvegardes au format 2 seulement ; anciennes aventures incompatibles, sans migration.\n'
        'E : récolter ; B : catalogue, puis Prévisualiser ; flèches et J/L ; Entrée confirmer, X annuler.\n'
        'Menu : Nouvelle aventure avec nom, Charger/F9 pour choisir, Sauvegarder/F5 pour le profil actif.\n'
        'Plusieurs aventures : saves/Nom/session.solar dans le dossier de travail.\n'
        'SOLAR_SAVE_ROOT permet de choisir un dossier parent ailleurs. Quitter ne sauvegarde pas automatiquement.\n'
        'La copie précédente est conservée en .bak ; deux instances simultanées ne sont pas prises en charge.\n'
        'Conserver assets à côté du programme. Crédits des textures : assets/textures/CREDITS.md.\nNASA/JPL : cartes orbitales et données du catalogue.\n'
        'Police DejaVu : licence dans assets/fonts. Jeu MIT ; Bevy MIT/Apache-2.0.\n', encoding='utf-8')
    metadata = dict(platform=system, binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                    files={str(p.relative_to(out)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(out.rglob('*')) if p.is_file()})
    (out / 'manifest.json').write_text(json.dumps(metadata, indent=2)+'\n')
    if system == 'Windows':
        with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as z:
            for p in out.rglob('*'):
                if p.is_file(): z.write(p, out.name + '/' + p.relative_to(out).as_posix())
    else:
        with tarfile.open(archive, 'w:gz') as tar:
            tar.add(out, arcname=out.name)
    print(json.dumps(dict(folder=str(out), archive=str(archive), platform=system, bytes=archive.stat().st_size), indent=2))

if __name__ == '__main__':
    main()
