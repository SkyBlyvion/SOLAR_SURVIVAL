# Prochain chat Solar Survival

**Caméra unique et ciel local réalisés/testés sur Ubuntu**, 13 octobre 2026. Le premier rendu a été rejeté par l’utilisateur puis repris : étoiles fines irrégulières, palette poussiéreuse plus claire, halo discret, sol mat supprimant une bande claire au crépuscule. **Prochaine fonctionnalité : P6 sauvegarde/chargement**, avec prise en compte de tout nouveau retour esthétique utilisateur. Aucune sauvegarde disque actuellement.

## Prompt autonome de reprise

Nous poursuivons Solar Survival dans `/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL`. Lis AGENTS.md, README.md, docs/ASTRONOMY.md, docs/CUBE_SPHERE.md, docs/GLOBAL_TERRAIN.md, docs/SURFACE_SKY.md, docs/SURVIVAL.md, docs/CONSTRUCTION.md, docs/VALIDATION.md, docs/ROADMAP.md, docs/PERFORMANCE.md et cette passation. Examine Git et les propriétaires du code, puis avance vers une fonctionnalité complète, jouable, avant d’actualiser documents/validation/roadmap/passation.

Départ du chat caméra : **HEAD 43f163b**, phase LOD commitée ; seuls NEXT_CHAT et ROADMAP avaient été modifiés après ce bilan. Copies exactes de leur état d’entrée : `docs/sky-validation/NEXT_CHAT-before.md` et `ROADMAP-before.md`. **La refonte caméra/ciel et ses preuves restent non commitées à la fin de cette passe**. Examiner Git à la reprise ; ne supprimer/écraser aucune preuve historique. `docs/lod-validation/NEXT_CHAT-before.md` conserve la passation encore antérieure au LOD.

## Contraintes et méthode

Rust stable édition 2024, application 0.2, Bevy **exactement =0.19.1**. Cargo.toml/Cargo.lock inchangés pendant cette refonte, aucune dépendance ajoutée. Ubuntu Linux et Windows 11 ; aucun résultat Linux ne valide Windows. Progrès par fonctionnalités complètes, tests progressifs adaptés au changement réel, pas de replay systématique des campagnes anciennes. Les restrictions historiques de chaînes fixes P3.5 ne limitent pas Mars mondiale.

SI/1:1, positions/rotations/temps orbital et transformations corporelles en DVec3/DQuat/f64. Soustraire l’origine corporelle en f64 avant conversion explicite au proxy f32. Simulation, rendu, navigation, physique, UI, survie, terrain distincts ; ne pas grossir main.rs ou créer des abstractions futures sans fonctionnalité. Les proxies ne modifient pas les données physiques.

GameSet Input→Simulation→Navigation→Rendering→Interface ; Oxygen après navigate, Construction après Oxygen. Orbites Time<Real>, physique locale 1/120 s avec plafond 0,25 s/frame ; oxygène delta réel complet ; souris déjà intégrée sans delta time. Visites spatiales translatoires, surface tournante avec Mars, joueur entraîné en l’air ; forces d’inertie omises. Pas d’orientation IAU complète, éphéméride exacte ou relief mesuré.

## État réel : caméra et ciel

**Une seule `camera::GameCamera` / Camera3d**, même entité entre espace et surface. Navigation publie translation nulle et rotation globale en espace/visée locale en surface. Rendering adapte projection, couche 0/1, ambiance et brouillard avant extraction. near/far configurés : 0,001/30 000 en espace ; 0,05/1 000 m en surface. FOV/MSAA/HDR et toutes les racines UI ciblent cette caméra. Bloom conservé uniquement en espace, restauré à la transition ; halo de surface dans le shader. `SpaceCamera`/`SurfaceCamera` n’existent plus.

En espace : proxies compressés des planètes, Soleil, anneaux et ancien champ triangulaire d’étoiles conservés. En surface : terrain/objets en mètres et `rendering::sky` dans la même passe opaque. Un triangle couvrant l’écran, reverse-Z=0, sans écriture de profondeur ; terrain/parois/toits le masquent. Aucune caméra cachée, render target intermédiaire ou capture de ciel par frame. Une seule caméra ne signifie pas une seule passe GPU.

Panorama **4 096×2 048 RGBA8, 32 Mio**, statique, 2 400 étoiles artistiques irrégulières (SplitMix64, graine fixe), éclats variés sans grosses couronnes. Il diffère du champ spatial historique. Rotation du panorama vers les axes inertiels par Q_Mars×Q_site, pas de parallaxe de marche. Les neuf autres corps que Mars emploient positions/rayons du catalogue : soustraction f64 depuis les yeux, direction et `asin(R/d)` convertis en f32 seulement. Disque solaire physique, points pour les corps subpixel, disques Lambert lisses si résolus ; ordre lointain→proche f64. Uniforme 608 octets, dix entrées bornées dont neuf utilisées. Mesh/image/matériau réutilisés aux rotations, rebasages et transitions.

Palette diurne/nocturne et halo atmosphérique **artistiques**, liés à la hauteur du Soleil ; direction solaire commune au modèle de lumière locale. Horizon radial idéal : fond de sol sous la tangente sans support physique, pas de dépression d’altitude/refraction/relief lointain. Brouillard **40–60 m** au lieu de 70–125 m, avant le disque de couverture certifié 66 m : visibilité lointaine explicitement réduite. Sol mat `reflectance=0`, couleur/rugosité/lumière directe conservées. Le contraste sol/ciel et l’horizon simplifié restent visibles ; aucune planète entière/horizon global géométrique ajouté. Points/éclats/phases/halos sont des approximations de présentation, pas une photométrie ou un modèle atmosphérique mesurés.

Les marqueurs UI utilisent le FOV et la direction de visée globale ; en surface ils sont filtrés sous l’horizon et par les boîtes d’abri/obstacle sur 120 m. Pas d’occlusion des collines historiques ou du texte par pixel. Les vrais disques/étoiles utilisent la profondeur locale. Le retour utilisateur initial a été traité ; ne pas présenter le rendu révisé comme une approbation esthétique déjà donnée par l’utilisateur.

Propriétaires à examiner : `src/camera/mod.rs`, `src/rendering/sky.rs`, `terrain.rs`, `planets.rs`, `src/ui/{mod,menu,survival,construction}.rs`, `assets/shaders/surface_sky.wgsl`, `src/config/{sky_validation,profiling}.rs`. Détails/paramètres : [SURFACE_SKY](SURFACE_SKY.md).

## Terrain mondial à préserver

M normal ouvre Mars sphérique traversable sur les six faces/arêtes/coins. Clés niveau spatial 15, voisinage de graphe ≤2, ≤13 patches ; **LOD de résolution choisi selon distance 32/16/8** (1 089/413/205 sommets, 2 048/696/280 triangles). Ce niveau de clés n’est pas un quadtree. Pas de relief mondial.

Sélection à l’entrée et au rebasage horizontal de 64 m depuis l’ancre radiale des pieds. Distance conservative au patch entier (centre moins demi-diagonale projetée bornée). Fin ≤80 m, conservé jusqu’à 110 ; moyen ≤160, conservé jusqu’à 200 ; grossier au-delà. Aucun mesh/sélection au repos, pendant la visée, rotation ou menu. Bords à 32 segments de PatchKey::vertex ; éventails des cellules de bord grossières. Polylignes f64 exactement communes, même inversées entre faces/coins ; pas de jupe/T-junction. Flèche du changement lointain <0,1 mm sur cette sphère, à réévaluer avant relief.

Disque de requêtes 66 m ; joueur demi-largeur ≤1 m, construction ≤5 m, triangles fins et extrema exacts, certificat d’aire 10⁻⁹ m². Index XZ 8 m limité à ±72 m ; maxima sur 150 directions : 361 cases/10 613 références/5 375 triangles. Transport minimal indépendant des faces ; poses/vitesse/visée f64 conservées. Boîte joueur alignée sur la carte, OBB exactes des obstacles/bâtiments dans le corps.

Arc communs à clé/détail identiques, mêmes entités/handles lors d’un changement de détail ; contenu remplacé seulement à révision. Slots recyclés, assets libérés en espace. Plafonds conservateurs ≤13 meshes/14 157 sommets/26 624 triangles ; ≤26 occurrences CPU/53 248 triangles transitoires, géométries partagées. Streaming synchrone borné, pas de RAM/VRAM exhaustive ou mesure individuelle de l’upload GPU. `SOLAR_TERRAIN_LOD=fixed` conserve le fournisseur fin précédent pour A/B ; `SOLAR_LOD_DEBUG=1` colore les détails. [GLOBAL_TERRAIN](GLOBAL_TERRAIN.md).

## P4/P5 et données autoritaires

Oxygène 180 s, consommation 1 s/s réel dehors, recharge +30 s/s dans l’abri, alerte ≤30 s. Zéro verrouille l’échec ; R revient au premier abri plein, stock/constructions/caisses conservés. Menu suspend réserve/recharge, calendrier et gravité continuent ; espace conserve réserve/échec sans recharge. H garde HUD vital/P5. Retour M au site de première arrivée, potentiellement nocturne.

Premier OxygenShelter six mètres devant le départ, six pièces/contact/service communs, porte 2,4 m côté +Z, terrain comme sol. `Buildings` possède BuildingId monotone et poses corporelles f64 fixes, indépendantes des patches/JD/origine. Tous les huit coins du joueur doivent tenir dans le volume utile pour recharger ; pas à travers mur/toit. Six BodyBox par abri, publication seulement à installation ; proxies à 120 m, cube partagé.

Huit caisses artistiques aux x=±8 ; z=0/−6/−12/−18 m au premier site. E à 2,5 m récolte six alliage, épuisement sans doublon/réapparition ; segment yeux→caisse bloqué par solide interdit. Aucun contact de déplacement pour les dépôts. Stock initial vide, **12 alliage par abri**, 48 unités finies : quatre constructions supplémentaires.

B ouvre le fantôme du même abri, suivi par visée/marche ; flèches 4 m/s, distance 4–24 m, latéral ±16 ; J/L 1 rad/s. Vert/rouge avec motif ; Entrée revalide/installe/déduit une fois, X/B/Échap annulent sans coût. Menu/espace/échec annulent, oxygène indépendant. Carré de demi-largeur 4,5 m entièrement couvert et dans le disque 66 m, variation ≤0,4 m, centre au milieu des extrema. Enveloppe OBB 6,3×3,95×6,3 m, SAT 15 axes/marge 0,2 m contre joueur, abris, obstacles et caisses actives. Politique conservative, limite 32 abris, pas d’index spéculatif. Confirm réutilise Buildings::install_shelter/publish_colliders. Toutes ces données survivent aux retours et au LOD, **en mémoire seulement**. [CONSTRUCTION](CONSTRUCTION.md), [SURVIVAL](SURVIVAL.md).

## Validations et mesures de cette passe

**120 tests réussis** (40 internes, les 115 antérieurs conservés). fmt/check/check-all-targets/test/Clippy/release --locked réussis ; dernier build release **145,584 s**. Cinq tests nouveaux : directions/rayons f64, trois JD/projection, rebasage/directions inertiels, assets réutilisés, même caméra/projections/couches aux quatre sorties. Options/menu/horloges/P4/P5 et régressions terrain restent réussis.

Deux campagnes finales code 0 sans ERROR/WARN/panic, sources/shaders/binaire inchangés : `sky-matte` **72,416 s**, 24 captures ; `p5-matte` **59,373 s**, 16 captures. **40 captures finales inspectées** via dix planches, nuit/crépuscule également originales. Identité de caméra conservée et une caméra extraite ; toit/paroi masquent le ciel, jour/nuit/crépuscule, visée, course/rebasage, P/T, menu/reprise, M/G/O/U, échec/R et deux autres poses JD/site. P5 récolte réelle, preview/refus/annulation/confirmation/recharge/mur/retours conservés. Les préparations de pose/JD/réserve sont explicites ; pas d’injection d’alliage ou installation directe P5. Pas de vidéo de chaque frame, nouveau cycle P2/suspension/DPI ou essai de périphériques OS.

**Sept profils release réussis**, avant puis après, même matériel/1440×900/MSAA ×4/parcours et vrai streaming, sans compilation ni capture dans les fenêtres. Deux répétitions sans recorder : repos **51,038/50,983→29,313/29,096 ms** (~20→34 FPS), course **50,735/50,690→28,875/28,965 ms**. Gain de la **refonte complète**, pas attribution à une seule ligne ou au LOD ; images pas strictement identiques (ciel, sol mat, portée de brume). Contrôle spatial ~27,3–27,8→28,2–28,3 ms ; limites de dérive explicites.

Recorder : repos 29,867 ms après, opaque **7,799 ms** (avant span partagé 3,375), tonemapping 2,085 ms. `msaa_writeback` ancien 9,629 ms non observé en surface finale ; bloom absent en surface, conservé en espace. Spans asynchrones **non additionnables**. Diagnostic no_sky 24,041 ms repos/23,934 course, retire le fond ; écart ~5 ms de frame, pas coût GPU isolé ni réglage livré. Remplacements mesurés CPU **0,975/0,984→0,924/1,052 ms**, coûts similaires, cinq clés renouvelées dans la fenêtre ; aucun nouveau gain terrain. La remise aux assets ne mesure pas l’upload GPU effectif. [PERFORMANCE](PERFORMANCE.md), [comparison.json](sky-validation/comparison.json).

Environnement : Ubuntu 24.04.5 LTS/X11 DISPLAY=:1, Rust 1.99.0, i7-6700K/GTX 1060 3 Go/NVK/Mesa 25.2.8. **Windows natif et RTX 4050/Core i5 non validés** ; modèle CPU/RAM/pilote/réglages à relever. Les essais intermédiaires, y compris premier diagnostic code 101 (ordre de lecture du menu), premier ciel rejeté et étapes de bande claire, sont conservés dans `docs/sky-validation/NOTES.md`. Le nom de certains dossiers intermédiaires contient final/verified : les références finales de ce bilan sont **final-matte, sky-matte, p5-matte et after**.

## Objectif suivant : P6 sauvegarde/chargement

Livrer une boucle complète permettant de quitter l’application et retrouver la session jouable : bâtiments construits/identités/poses corporelles, dépôts épuisés, stock, oxygène/échec, site de retour, pose/joueur et état temporel/options nécessaires. Examiner les propriétaires actuels avant de choisir un **format local versionné**, une écriture robuste et une restauration validée. Les meshes/index/entités/Transform ne sont pas des données autoritaires à sauvegarder : reconstruire les caches depuis les poses f64. Même abri, service et transaction P5 ; pas de ressources régénérées au chargement ni paiement doublé.

Rendre sauvegarder/charger visibles et utilisables depuis le menu, avec messages clairs. Tester roundtrip après récolte/construction/rebasage/retour spatial/échec ; **redémarrage réel de l’application** ; fichier absent/corrompu/incompatible et absence de mutation partielle en cas de refus. Préserver l’entité caméra unique, reconstruire uniquement ses paramètres/projection depuis l’état restauré, sans sauvegarder des handles GPU. Ne pas changer Bevy, l’échelle, les horloges ou le LOD pour la persistance. Ajouter une dépendance seulement si nécessaire, compatibilité vérifiée.

P6 prévoit ensuite réglages complémentaires, packaging Ubuntu/Windows et **validation native Windows 11**. Sans hôte Windows, achever la fonctionnalité locale et consigner la limite ; ne pas présenter P6 distribué sur les deux OS comme terminé. Aucun relief mondial, réseau/cloud, refonte physique ou quadtree ajouté au hasard. Si l’utilisateur demande encore une correction du ciel, la traiter concrètement avec captures, sans perdre les propriétaires/contrats ci-dessus.

## Vérifier et reproduire

Pour tout changement significatif : `cargo fmt --check`, `cargo check --locked`, `cargo check --all-targets --locked`, `cargo test --locked`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo build --release --locked`. Compiler puis lancer les GPU successivement. Dossiers finaux nouveaux, parent existant ; F12/SOLAR_CAPTURE_DIR exige un dossier créé. Préserver toutes les preuves ; actualiser README, documents techniques, ROADMAP, VALIDATION et cette passation, puis fournir son lien.

```bash
python3 scripts/validate_sky.py --focus-x11 --mesa-immediate --output docs/mon-nouveau-ciel
python3 scripts/validate_sky.py --construction --focus-x11 --mesa-immediate --output docs/mon-nouveau-p5
python3 scripts/profile_planetary.py --scene global --lod adaptive --repeats 2 --gpu-probe --camera-probe --focus-x11 --mesa-immediate --output docs/mon-nouveau-profil
```

Le profil historique reste le défaut, global explicite. Les variantes actuelles sont baseline/no_sky/no_game_camera/half_resolution/msaa_off/surface_unlit ; les anciennes variantes de caméra restent seulement dans les preuves historiques. `--camera-probe` teste désormais no_sky, pas une seconde caméra. Les hashes du profiler incluent les shaders. X11/Mesa concernent seulement ce poste ; omettre sur Windows/autres pilotes. Le helper cible le client, pas la décoration Mutter, et ne simule aucun clavier/souris OS ; aucune API X11 de gameplay.
