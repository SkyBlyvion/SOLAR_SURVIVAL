# Textures P7 — crédits, sources et conditions

**Chef de projet et orchestrateur : SkyBlyvion.**

Exports préparés le 14 octobre 2026. Jeu autonome, aucun téléchargement à l'exécution.
Les dimensions, SHA-256 maîtres/exports et octets RGBA8 avec tous les mipmaps sont
consignés dans `manifest.json`. Les maîtres sont conservés dans `art/p7-masters`
dans le dépôt ; ils ne sont pas distribués avec le jeu.

## Terre

NASA Earth Observatory, Blue Marble Next Generation, composite **janvier 2004**.
[Page source](https://science.nasa.gov/earth/earth-observatory/blue-marble-next-generation/base-map/),
[fichier JPEG officiel](https://assets.science.nasa.gov/content/dam/science/esd/eo/images/bmng/bmng-base/january/world.200401.3x5400x2700.jpg).
Maître 5 400×2 700 (8 km/pixel annoncé), export 2 048×1 024.
Carte globale équirectangulaire, nord en haut, -180° à gauche, +180° à droite,
équateur à mi-hauteur. Janvier statique, aucune simulation saisonnière ni nuage ajouté.
Réduction Lanczos RGB, mipmaps BOX en lumière linéaire. Aucune recolorisation.
[Conditions NASA pour les images](https://www.nasa.gov/nasa-brand-center/images-and-media/).
Crédit conservé NASA Earth Observatory ; aucun logo, personne ou soutien institutionnel revendiqué.

## Mars

**NASA/Jet Propulsion Laboratory & Caltech**, images Viking traitées à l'USGS,
base de cartes planétaires JPL/Caltech.
[Page source](https://science.nasa.gov/3d-resources/mars/),
[fichier JPEG officiel Mars.jpg](https://assets.science.nasa.gov/content/dam/science/cds/3d/resources/image/mars/Mars.jpg).
Maître et export 1 440×720, sans agrandissement. Carte globale pour modèles 3D,
interprétée équirectangulaire, nord en haut. Méridien zéro au centre retenu comme
convention visuelle de cette intégration (repères reconnaissables, pas géoréférencement certifié).
La fiche ne fournit pas de métadonnées IAU/projection détaillées. Couleur de la carte
publiée conservée : composite traité, pas une mesure d'albédo spectral ni une preuve
photographique de la couleur naturelle du site joueur.
[Conditions JPL](https://www.jpl.nasa.gov/jpl-image-use-policy/) : utilisation avec crédit,
sans faire croire à une approbation NASA/JPL/Caltech ; exceptions pour matériel tiers,
logos et personnes décrites sur la page. Aucun contenu de ces exceptions dans cette carte.

Les deux cartes ont +Y pour nord, longitude zéro sur +X, longitudes est vers -Z.
Le shader utilise atan2(-z,x) et acos(y), U répétée/V bornée, dérivées analytiques
pour le filtrage de la couture. Axes et phase du catalogue demeurent conventionnels,
pas une orientation IAU ni un atterrissage géographique exact.

## Proximité : créations artistiques

- **regolith** : création avec l'outil intégré OpenAI ImageGen, prompt conservé dans
  `art/p7-masters/PROMPT.md`. Maître 1 254² reçu, export 1 024², raccord périodique
  étroit de 32 pixels. Projection triplanaire, répétition de **4 m** dans les axes
  corporels. RGB en sRGB, alpha opaque. Aucun microrelief ni carte de normales ajoutés dans le shader final,
  aucune carte de relief scientifique ou collision nouvelle.
- **wall/roof** : panneaux industriels dessinés dans le script de préparation, 512²,
  répétition métrique 1 m ; joints, vis, bande teal et métal mat. Création Solar Survival,
  licence MIT du dépôt. Les quatre meshes UV des pièces sont partagés entre abris.
- **crate** : dessin Solar Survival, 512², une face de caisse (1×0,8×1 m),
  « ALLIAGE / 06 / E », licence MIT. La caisse n'a toujours pas de collider de déplacement.
- **oxygen-sign** : dessin Solar Survival, 512×128, plaque 2,4×0,6 m au-dessus de
  la porte, « O₂ RECHARGE », licence MIT. Présentation unlit, aucun service/collider ajouté.
  Police DejaVu Sans, licence dans `assets/fonts/LICENSE-DejaVu.txt`.

La palette martienne est guidée par la comparaison Curiosity Mastcam Sol 19 :
[NASA/JPL-Caltech/MSSS — Raw, Natural and White-Balanced Views](https://science.nasa.gov/resource/raw-natural-and-white-balanced-views-of-martian-terrain/).
Image consultée comme référence, **non incluse dans les textures**. L'estimation
« natural » centrale est distincte de la balance des blancs terrestre à droite.
Les textures proches et leur microrelief ne représentent pas une reconstruction mesurée.

## Reproduction et format

`scripts/prepare_p7_textures.py` + Pillow/NumPy, hors exécution uniquement.
Les versions du premier export : Pillow/NumPy relevées dans les preuves P7.
En-tête `.p7tex` : `P7TX`, largeur/hauteur/nombre de niveaux u32 little-endian,
puis tous les mipmaps RGBA8 contigus du plus grand au 1×1, dimensions divisées par
2 avec plancher et minimum 1. RGB filtré en linéaire puis réencodé sRGB, alpha linéaire.
Format chargé **Rgba8UnormSrgb**, trilinear/aniso ×8. Aucun nouveau décodeur/dépendance Rust.
Les fichiers bruts sont plus gros sur disque qu'un PNG, mais conservent les mipmaps
précalculés et rendent les octets GPU contrôlables ; l'archive tar.gz les compresse.


## Six autres planètes — extension demandée par SkyBlyvion

| Planète | Fichier distribué | Source officielle et nature | Crédit |
|---|---|---|---|
| Mercure | mercury, 1024×512 | [Modèle NASA](https://science.nasa.gov/resource/mercury-3d-model/), atlas PNG 2048×1536 extrait du GLB. Conversion en carte équirectangulaire par rayons/triangles et UV du mesh, hors jeu ; toutes les directions couvertes | NASA Visualization Technology Applications and Development (VTAD) |
| Vénus | venus, 1440×720 | [Modèle NASA](https://science.nasa.gov/resource/venus-3d-model/), PNG de nuages extrait du GLB, dimensions conservées. Représentation VTAD, pas la carte radar Magellan colorisée et pas une mesure de couleur naturelle | NASA VTAD |
| Jupiter | jupiter, 720×360 | [Texture NASA](https://science.nasa.gov/3d-resources/jupiter/), assemblage Voyager selon la fiche ; atmosphère visible statique | JPL & Caltech |
| Saturne | saturn, 720×360 | [Texture NASA](https://science.nasa.gov/3d-resources/saturn/), **fictionnelle** selon la fiche ; bandes atmosphériques artistiques, pas une mosaïque d'observations | NASA/JPL-Caltech, base de cartes planétaires |
| Uranus | uranus, 1024×512 | [Modèle NASA](https://science.nasa.gov/resource/uranus-3d-model/), PNG extrait du GLB. Nuages/brume VTAD, dimension conservée, pas une cartographie mesurée | NASA VTAD |
| Neptune | neptune, 720×360 | [Texture NASA](https://science.nasa.gov/3d-resources/neptune/), **fictionnelle**, nuages peints ; teinte de l'illustration conservée, pas une estimation de couleur naturelle | **Don Davis & JPL/Caltech** |

Aucun agrandissement des cartes de 720×360. La sphère physique et le shader Lambert
restent communs ; aucun modèle NASA, rayon du GLB ou animation de nuages n'est
chargé en jeu. Les anneaux de Saturne restent le mesh et la palette artistiques
existants. Soleil et Lune restent procéduraux : les huit planètes sont texturées.

Le rééchantillonnage de Mercure reprend le repère Y-up du mesh VTAD, zéro +X/est −Z
choisis pour le jeu ; pas de longitude géographique certifiée. Les autres cartes
sont interprétées équirectangulaires (nord en haut), sans phase IAU ajoutée.
Les bandes des géantes sont attachées à leur rotation conventionnelle. Uranus
conserve la convention de rotation rétrograde du catalogue ; le PNG n'impose pas
la pose entière du modèle NASA. Ces cartes sont des représentations, pas une
simulation météorologique actuelle ni des surfaces solides pour les géantes.

Le [portail NASA 3D](https://science.nasa.gov/3d-resources/) indique que ses assets
sont disponibles gratuitement au téléchargement et à l'usage et renvoie aux
[conditions NASA](https://www.nasa.gov/nasa-brand-center/images-and-media/).
Les crédits spécifiques ci-dessus sont conservés, avec les conditions JPL déjà
liées ; aucun soutien institutionnel ni logo revendiqué. Les images originales
ne sont pas relicenciées sous la licence MIT du code du jeu. Neptune conserve
explicitement le nom du créateur Don Davis.

Fichiers officiels téléchargés :
- [Mercury GLB](https://assets.science.nasa.gov/content/dam/science/psd/solar/2023/09/m/Mercury_1_4878.glb)
- [Venus GLB](https://assets.science.nasa.gov/content/dam/science/psd/solar/2023/09/v/Venus_1_12103.glb)
- [Uranus GLB](https://assets.science.nasa.gov/content/dam/science/psd/solar/2023/09/u/Uranus_1_51118.glb)
- [Jupiter JPEG](https://assets.science.nasa.gov/content/dam/science/cds/3d/resources/image/jupiter/Jupiter.jpg)
- [Saturn JPEG](https://assets.science.nasa.gov/content/dam/science/cds/3d/resources/image/saturn/Saturn.jpg)
- [Neptune JPEG](https://assets.science.nasa.gov/content/dam/science/cds/3d/resources/image/neptune/Neptune.jpg)

Dates et SHA-256 des GLB/JPEG : `art/p7-masters/ORBITAL_SOURCES.json` ; images
extraites et exports : `manifest.json`. `prepare_orbital_textures.py` est appelé
par la préparation P7 ; Python/NumPy/Pillow restent des outils hors jeu uniquement.
Total final des **13 images**, mipmaps RGBA8 sRGB inclus : **42 118 856 octets,
40,168 Mio**, sous le budget P7 de 128 Mio ; allocations/staging pilote exclus.
