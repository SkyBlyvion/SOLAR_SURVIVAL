# Solar Survival — version 0.2 · Exploration et survie sur Mars

Prototype d'exploration 3D de notre système solaire, fondation d'un jeu de survie spatial en première personne. **Les positions, rayons et distances physiques restent en mètres à l'échelle 1:1, en `f64`.** Bevy ne reçoit que des représentations relatives à la caméra.

**M ouvre un terrain martien mondial : six faces traversables, arêtes et coins continus, LOD adaptatif à trois résolutions et chargement local borné.** Les anciens parcours bornés restent des modes de régression. [Architecture et commandes](docs/GLOBAL_TERRAIN.md).

**Construction P5 :** récupérer les caisses dorées avec **E**, placer le même abri avec **B**, déplacer avec les **flèches**, orienter avec **J/L**, confirmer avec **Entrée** ou annuler avec **X/Échap**. Coût : 12 alliage ; prévisualisation verte/rouge, inventaire et bâtiments conservés en session. [Règles et architecture](docs/CONSTRUCTION.md).

**Survie P4 :** une combinaison de 180 s d’oxygène et un abri de recharge accessible à six mètres devant l’arrivée. Entrez par la porte pour recharger ; à zéro, **R** relance une expédition à l’abri. [Règles et architecture](docs/SURVIVAL.md).

La suite du développement suit [la roadmap](docs/ROADMAP.md). Le [prompt du prochain chat](docs/NEXT_CHAT.md) est actualisé à chaque fin de chat de développement avec l'état réel et l'objectif suivant.

## Démarrer sur ce poste Ubuntu

Rust stable et les extensions VS Code rust-analyzer / Even Better TOML sont installés lors de la création du projet. Le terminal VS Code ouvert avant l'installation peut ne pas encore connaître Cargo. Dans ce terminal :

```bash
source "$HOME/.cargo/env"
cargo run
```

La tâche VS Code **Solar Survival: lancer** fonctionne aussi avec `Terminal → Exécuter la tâche`. La première compilation Bevy peut prendre plusieurs minutes ; les suivantes réutilisent le cache. Pour obtenir de meilleures performances :

```bash
cargo run --release
```

Depuis le menu principal, cliquez sur **Jouer** (ou appuyez sur Entrée). Appuyez ensuite sur **M**, ou sur **Surface Mars / retour** dans le panneau gauche, pour rejoindre le prototype FPS martien. Clic droit capture la souris ; WASD / ZQSD permet de marcher, Shift de courir et Espace de sauter. **M** revient à la visite spatiale de Mars ; G, O et U permettent aussi de quitter la surface. Le jeu commence près de la Terre, à J2000 (1 janvier 2000, 12 h à l'échelle temporelle du modèle). Cliquez droit pour piloter ; Échap libère le curseur. Cliquez sur Mars dans le panneau gauche puis sur **Visiter**, ou utilisez **4 puis G**. La visite est une téléportation de gameplay à une distance de sécurité, avec suivi orbital de la cible.

## Installation sur un autre Ubuntu

Installer Rust avec [rustup](https://rustup.rs/). Un compilateur/linker C, une session graphique et un pilote Vulkan récent sont nécessaires :

```bash
sudo apt update
sudo apt install build-essential curl libx11-6 libxi6 libxrandr2 libxcursor1 libvulkan1 mesa-vulkan-drivers
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cargo run
```

Le backend Linux choisi est X11 ; une session Wayland doit disposer de XWayland. Aucun paquet de développement ALSA/udev n'est requis : audio et manettes sont désactivés dans ce prototype. Les pilotes propriétaires peuvent remplacer Mesa selon la carte graphique.

## Installation sous Windows 11

1. Installer [Rust via rustup](https://rustup.rs/) avec la chaîne stable `x86_64-pc-windows-msvc`.
2. Installer les [Build Tools Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/) avec **Développement Desktop en C++**, outils MSVC et Windows SDK. Ces outils servent uniquement au linker ; le projet et sa logique sont écrits en Rust.
3. Installer un pilote graphique récent. Ouvrir le dossier dans VS Code et redémarrer son terminal après l'installation.
4. Exécuter `cargo run` dans le dossier du projet, puis `cargo run --release` pour jouer avec les optimisations.

La source est multiplateforme ; la validation effective sur cette machine est décrite dans [docs/VALIDATION.md](docs/VALIDATION.md). Un build natif Windows reste à effectuer sur Windows.

## Commandes Cargo

```bash
cargo fmt --check
cargo check
cargo test
cargo build --release
cargo run
```

## Menu principal et options

Le lancement normal affiche **Jouer**, **Options**, **Crédits** et **Quitter**. Après avoir joué, **Reprendre** retrouve la même session et la même pose ; il ne réinitialise pas le site. Le bouton **Menu principal** du HUD et Échap lorsque le curseur est libre ouvrent le menu. Un premier Échap en pilotage libère seulement la souris. Entrée lance/reprend depuis l'écran principal ; Échap revient depuis les sous-écrans.

Les options s'appliquent immédiatement : sensibilité souris ×0,5/1/1,5/2, champ de vision **vertical** 55/65/75/90°, MSAA désactivé/×2/×4, affichage fenêtré/plein écran sans bordure. **Rétablir les valeurs par défaut** restaure ×1, 65°, MSAA ×4 et fenêtré. Les valeurs initiales de rendu restent celles du prototype. Les réglages sont conservés pour la session, sans sauvegarde sur disque. Il n'y a pas d'option sonore car l'audio n'est pas activé.

Le menu libère la souris et bloque les commandes de navigation, P/T et H. Le calendrier orbital continue avec `Time<Real>` ; pour le figer, utiliser P avant d'ouvrir le menu. En surface, la gravité et les contacts continuent, sans entrée de déplacement. L’oxygène et sa recharge sont suspendus pendant le menu ; le calendrier et la gravité continuent. Cette convention conserve les deux horloges du prototype. Les scénarios de validation/profilage démarrent directement en jeu.

## Contrôles

Les touches de déplacement désignent des positions physiques : WASD sur QWERTY, ZQSD sur AZERTY.

| Commande | Fonction |
|---|---|
| Clic droit | Capturer la souris et activer le pilotage |
| Échap | Libérer la souris ; curseur déjà libre, ouvrir le menu principal ; depuis Options/Crédits, revenir |
| Souris | Orientation |
| WASD / ZQSD | Vol orienté en espace ; déplacement tangent en surface |
| Espace / Ctrl | Monter / descendre en espace ; Espace saute en surface |
| Shift | Vitesse temporaire ×10 en espace ; course à 8 m/s en surface |
| Molette (pilotage), Pg↑ / Pg↓, + / − | Changer de niveau de vitesse en espace |
| Tab | Sélection suivante |
| 0–9 | Soleil, Mercure, Vénus, Terre, Mars, Jupiter, Saturne, Uranus, Neptune, Lune |
| F | Orienter la caméra vers la cible en espace |
| G | Visiter la cible, à 3,5 rayons de son centre (5,5 pour Saturne) |
| M | Surface de Mars / retour à la visite spatiale de Mars |
| O | Vue globale depuis 35 UA au-dessus du plan écliptique |
| U | Passer au repère héliocentrique ; depuis la surface, revenir d’abord à distance de sécurité |
| T | Temps orbital ×1 / ×10 / ×100 / ×1000 |
| P | Pause des orbites et des rotations |
| H | Masquer / afficher la télémétrie ; le HUD vital reste visible |
| E | En surface : récupérer une caisse dorée à moins de 2,5 m |
| B | Ouvrir / annuler le placement d’un abri (12 alliage) |
| Flèches, J/L | Pendant le placement : déplacer / tourner la prévisualisation |
| Entrée, X / Échap | Pendant le placement : confirmer / annuler sans dépense |
| R | Après épuisement de l’oxygène : repartir au premier abri, réserve pleine |

Les vitesses disponibles sont 5 m/s, 250 m/s, 20 km/s, 20 000 km/s, 2 millions km/s et 100 millions km/s. Ce sont des choix de gameplay. En espace, l'interface affiche la **vitesse de déplacement configurée** ; à l'arrêt elle demeure affichée. En surface, elle indique aussi la vitesse résolue après contacts, l'état au sol/en l'air et la hauteur des pieds au-dessus du support. La marche utilise 4,5 m/s, la course 8 m/s et le saut une impulsion de vitesse de 4 m/s : conventions de gameplay. P et T commandent l’orbite et la rotation de Mars ; la marche, la course, le saut et la gravité restent en secondes réelles. La vitesse résolue affichée en surface est relative au site, sans vitesse d’entraînement orbitale ou rotatoire.

## Fonctionnalités

- Soleil, huit planètes et Lune, orbites elliptiques et inclinées dépendantes du temps.
- Catalogue embarqué NASA/JPL, indépendant de toute API ou connexion pendant le jeu.
- Terre et Lune positionnées autour du barycentre Terre–Lune fourni par JPL.
- Soleil émissif avec halo, étoiles, rotation, faces diurnes/nocturnes suivant la direction physique du Soleil.
- Matériaux procéduraux artistiques : continents et nuages, relief visuel martien, bandes des géantes, anneaux de Saturne.
- Navigation libre en `f64`, visite instantanée, origine flottante, repères et conversion locale contrôlée.
- Garde sphérique avec intersection balayée contre le passage à travers les corps à haute vitesse.
- Deux meshes de sphère partagés pour le LOD ; étoiles regroupées dans un mesh ; pas de création de mesh par frame.
- Interface sélectionnable à la souris : FPS, temps, cible, altitude, rayon, repère, vitesse et astre le plus proche. Repères visuels pour les corps trop petits à l'écran.
- FPS martien : site fixé au repère tournant de Mars, gravité radiale, marche/course/saut, contacts avec terrain et trois obstacles statiques dans le site.
- Terrain mondial sphérique : six faces, voisinage local de treize patches maximum, marche/course/saut sans borne de site ; coordonnées et contacts f64, meshes réutilisés. Résolutions intérieures 32/16/8 choisies selon la distance, bords communs fins et contacts proches précis. Le terrain artistique historique de neuf tuiles reste disponible avec `SOLAR_MARS_GLOBAL=0`.
- Contrat cube-sphere f64 : conversions, voisins orientés, graines reproductibles et raccord de deux patches de même niveau.
- Première boucle de survie : réserve/consommation/recharge/HUD, expédition terminée et reprise ; abris instanciables avec identités et poses fixes sur Mars pour leur réutilisation en P5.
- Ressources et construction : huit caisses épuisables, inventaire réel, recette du même abri, preview orientable avec admission terrain/volumes et paiement après confirmation ; persistance de session.
- LOD adaptatif et streaming local : trois résolutions, hystérésis, raccords mixtes sur faces/arêtes/coins, index de contact limité à la zone utile. Le compteur de télémétrie affiche les détails résidents et leurs changements ; [stratégie, budgets et limites](docs/GLOBAL_TERRAIN.md).

## Architecture

```text
src/main.rs           assemblage fenêtre/plugins
src/config/validation.rs scénarios graphiques optionnels espace et surface
src/config/profiling.rs mesures opt-in CPU/GPU et variantes de comparaison
src/lib.rs            ordre des systèmes Input → Simulation → Navigation → Rendu → UI
src/universe/         catalogue, unités SI, calculs orbitaux, horloge, repères f64
src/rendering/        proxies astronomiques et caméra/meshes de terrain en mètres
src/camera/           entrées et orchestration de navigation
src/player/           état d'exploration, repère suivi et contrôleur FPS
src/physics/          garde spatiale, gravité radiale et contacts cinématiques f64
src/planets/          clés cube-sphere, graines et géométrie du terrain local
src/planets/cube_sphere.rs conversions, voisins et paire de patches à sommets partagés
src/planets/cube_terrain.rs fournisseur optionnel de support de la paire f64
src/planets/globe_terrain.rs voisinage mondial dynamique et contacts f64
src/config/cube_validation.rs parcours graphique borné de franchissement
examples/cube_seam.rs diagnostic graphique borné, indépendant du terrain jouable
src/buildings/        abris instanciables, identités et poses corporelles f64
src/construction.rs   récupération, inventaire, admission et transaction de placement
src/rendering/construction.rs fantôme et caisses locaux, cube partagé
src/ui/construction.rs stock, commandes et motifs de refus
src/config/construction_validation.rs parcours graphique ciblé P5
src/survival/         réserve d’oxygène et issue de l’expédition
src/rendering/buildings.rs proxies locaux des abris et assets partagés
src/ui/survival.rs    HUD vital et instruction de reprise
src/ui/               panneau de navigation, télémétrie et marqueurs
src/ui/menu.rs        menu principal, options de session et crédits
src/config/           paramètres de démarrage
assets/shaders/       shader WGSL de présentation des planètes
tests/               tests des fondations physiques et numériques
```

La logique de jeu et de simulation est en Rust. WGSL est utilisé uniquement pour le calcul graphique sur le GPU. Aucun éditeur de jeu, modèle externe, Blender, cloud ou service en ligne n'est requis. Avian n'est pas intégré : ce prototype utilise un contrôleur cinématique spécifique au relief doux et aux boîtes statiques. La [table officielle d'Avian](https://github.com/avianphysics/avian/blob/main/README.md) annonce la compatibilité 0.7 avec Bevy 0.19, vérifiée le 12 octobre 2026, mais aucune dépendance supplémentaire n'a été ajoutée. Les corps dynamiques et les terrains arbitraires demanderont une nouvelle évaluation. Le chargement local réside dans `rendering/terrain.rs`, avec libération des meshes sortants.

La police DejaVu Sans est embarquée avec sa licence dans `assets/fonts/` pour assurer le rendu français sur les deux plateformes.

Les systèmes astronomiques créent des entités `CelestialBody` + `AstronomicalPosition` ; les entités de rendu séparées ne sont que des proxies. `BodyId` indexe le catalogue interne et fournit un identifiant stable. Les futurs satellites pourront compléter le catalogue et les évaluateurs d'orbites ; une hiérarchie de parents est présente, mais l'évaluateur actuel gère explicitement la Lune.

## Dates et diagnostic

Une date configurable est acceptée par `SOLAR_JD`, comprise entre le 1 janvier 1800 et le 1 janvier 2050. Une valeur invalide utilise J2000 ; l'horloge se met en pause à la borne supérieure pour ne pas extrapoler hors de la plage JPL.

Ubuntu :

```bash
SOLAR_JD=2461325.5 cargo run
RUST_LOG=info cargo run --release
SOLAR_SMOKE_SECONDS=66 cargo run --release
SOLAR_SURFACE_SMOKE_SECONDS=50 RUST_LOG=info cargo run --release
MESA_VK_WSI_PRESENT_MODE=immediate SOLAR_ROTATION_SMOKE_SECONDS=50 RUST_LOG=info cargo run --release
```

PowerShell :

```powershell
$env:SOLAR_JD="2461325.5"
cargo run
Remove-Item Env:SOLAR_JD
```

`SOLAR_SMOKE_SECONDS` active uniquement un lancement borné de validation, avec visite automatique des dix corps, vue globale et captures PNG `docs/v02-space-00.png` à `v02-space-10.png`. `SOLAR_SURFACE_SMOKE_SECONDS` (minimum 40 s, 50 s conseillé) active un autre scénario : repos, marche contre obstacle, saut, déplacement latéral, course à travers les tuiles, libération des commandes, retour spatial et Terre. Il produit `docs/v02-surface-00.png` à `v02-surface-09.png`. Le scénario `SOLAR_ROTATION_SMOKE_SECONDS` reprend ce parcours en alternant ×1000, pause et ×1 ; il produit `docs/rotation-surface-00.png` à `rotation-surface-09.png`. Priorité si plusieurs variables sont définies : rotation, surface, espace. Les entrées sont injectées dans l'ECS, sans simuler les périphériques de l'OS. Ne pas définir ces variables pour jouer normalement.

Sur cette session Ubuntu/NVK, certains lancements en présentation VSync sont tombés à environ 1 FPS, avec un timeout de présentation. Le lancement suivant a rétabli environ 23 FPS en surface ; c'est un contournement local du pilote, pas une mesure de performance garantie :

```bash
MESA_VK_WSI_PRESENT_MODE=immediate cargo run --release
```

Cette variable est documentée par [Mesa](https://docs.mesa3d.org/envvars.html). Elle peut provoquer du tearing ; elle n'est pas nécessairement utile sur un autre pilote et ne concerne pas le lancement Windows.

## Validation P2 reproductible

Après compilation release, sans compilation ni autre lancement GPU concurrent :

```bash
MESA_VK_WSI_PRESENT_MODE=immediate SOLAR_P2_SECONDS=160 RUST_LOG=info target/release/solar_survival
```

`SOLAR_P2_SECONDS` accepte 160 à 300 secondes. Le scénario est prioritaire sur les anciens smoke, mais le profilage reste prioritaire. Il impose J2000, entre sur Mars et maintient ×1000 pendant 96 secondes (plus d'une rotation). Douze captures réparties sur ce cycle précèdent P, une marche en pause, un saut, la reprise à ×1 et trois commandes T. Deux séries d'entrées/sorties M/G/O/U suivent, puis un dernier retour en surface. Il injecte les touches dans `ButtonInput` avant les vrais contrôles. Les images `docs/p2-stage-00.png` à `34.png` et le journal conservent calendrier, vitesse, état local et direction solaire. Ces noms sont remplacés lors d'une répétition : préserver chaque campagne avant de relancer.

Pour capturer un essai manuel du menu : `SOLAR_CAPTURE_DIR=docs/mon-essai-menu` active **F12** et écrit `capture-00.png`, etc. Créer le dossier avant le lancement. Ce diagnostic n'est installé qu'avec cette variable. Une interruption réelle du processus ou de la session doit être distinguée d'un delta manuel dans un test ECS.

## Profilage reproductible

Le [protocole de performance](docs/PERFORMANCE.md) distingue le poste de développement GTX 1060/NVK et la cible annoncée RTX 4050/Core i5. Un mode optionnel mesure repos, contact, franchissement de tuile et visite spatiale dans une scène canonique. Il conserve des échantillons en millisecondes, puis une synthèse moyenne/médiane/p95/p99. Les diagnostics GPU et les variantes de comparaison ne sont pas actifs en partie normale.

```bash
cargo build --release --locked
python3 scripts/profile_suite.py --seconds 42 --mesa-immediate --gpu-probe --output docs/profiles/mon-essai
```

Le script utilise uniquement la bibliothèque standard Python et lance les essais successivement. `--mesa-immediate` est réservé au contournement Mesa de ce poste ; l'omettre sur la machine cible. Choisir un nouveau dossier de sortie pour chaque campagne. Un essai individuel sans Python est décrit dans PERFORMANCE.

## LOD et mesures du moteur planétaire

Le parcours normal choisit réellement 32, 16 ou 8 cellules par côté suivant la distance. L’emprise spatiale des patches reste au niveau 15 ; leurs quatre bords gardent 32 segments communs. La zone de marche et de construction reste fine. La sélection se fait à l’entrée et lors du rebasage de 64 m, avec hystérésis ; aucun mesh n’est construit au repos. C’est un LOD de résolution, sans quadtree ni relief mondial. `SOLAR_TERRAIN_LOD=fixed` conserve le fournisseur précédent pour comparaison.

Après compilation release, sans autre lancement GPU :

```bash
python3 scripts/profile_planetary.py --scene global --lod adaptive --repeats 2 --gpu-probe --camera-probe --mesa-immediate --output docs/mon-profil-lod
python3 scripts/profile_planetary.py --scene p5 --lod adaptive --repeats 2 --gpu-probe --mesa-immediate --output docs/mon-profil-p5
cargo build --release --locked --example lod_seams
target/release/examples/lod_seams docs/mon-diagnostic-raccords
```

Choisir des dossiers nouveaux ; omettre Mesa sur les autres pilotes. Le profil P5 conserve collecte, placement, construction et recharge, et désactive uniquement les captures. `SOLAR_LOD_DEBUG=1` colore les patches fins en vert, moyens en bleu, grossiers en rouge ; ce diagnostic est facultatif. Résultats et limitations : [PERFORMANCE](docs/PERFORMANCE.md), [VALIDATION](docs/VALIDATION.md).

## Limites et parcours historiques

Le parcours mondial actuel est décrit dans [GLOBAL_TERRAIN](docs/GLOBAL_TERRAIN.md). Les restrictions de site/paire ci-dessous concernent les modes historiques de régression.

Le [contrat P3.1](docs/CUBE_SPHERE.md) et sa démonstration optionnelle vérifient les premiers raccords cube-sphere. Après `cargo build --release --locked --example cube_seam`, lancer `target/release/examples/cube_seam docs/mon-nouveau-raccord` (ajouter `MESA_VK_WSI_PRESENT_MODE=immediate` sur ce poste). Le programme affiche quatre cas en 32 secondes et écrit quatre captures dans un nouveau dossier. Il génère deux patches à la fois ; le franchissement du joueur et les contacts cube-sphere sont désormais disponibles dans un parcours opt-in borné, décrit ci-dessous. Le FPS normal conserve les neuf tuiles et la borne ±4 096 m.

Les données et conventions sont détaillées dans [docs/ASTRONOMY.md](docs/ASTRONOMY.md). Ce modèle képlérien n'est pas une éphéméride scientifique : pas de N-corps, perturbations lunaires, précession exacte, retards lumineux ou éclipses. La phase et l'orientation lunaires sont conventionnelles, explicitement non observées. La période lunaire et ses dimensions sont réelles.

Le rendu compresse chaque profondeur selon `d_render = 10 000 × d / (d + 10⁸)` et applique le même facteur au rayon du corps. L'angle apparent d'une sphère est conservé ; la profondeur GPU est un proxy. Les occultations entre sphères volumineuses et les anneaux ne sont pas exactes après compression. L'éclairage est directionnel par corps et artistiquement exposé, sans photométrie en inverse carré ni ombres entre corps. Les continents, étoiles, cratères visuels et nuages ne sont pas une cartographie réelle. Les axes ont une inclinaison indicative avec longitude de pôle conventionnelle ; les objets utilisent des rayons moyens sphériques sans aplatissement.

En mode espace, la garde de navigation maintient au moins 1 km d'altitude et borne les voyages à 10¹⁶ mètres du Soleil. Elle ne simule pas de contacts physiques, et peut expulser la caméra si une planète mobile la rattrape. Une planète lointaine peut être subpixel : les marqueurs et visites permettent de la retrouver. Le GPU doit supporter les fonctionnalités 3D de Bevy/wgpu. 60 FPS est un objectif à mesurer sur le matériel cible, pas une garantie.

Le prototype FPS utilise un site local **tournant avec Mars**, borné à ±4 096 m dans ses deux axes horizontaux. À chaque entrée, le site est choisi côté éclairé puis fixé dans les axes du corps ; il ne suit ensuite plus le Soleil. La rotation réutilise exactement la convention planétaire du prototype (phase nulle à J2000, inclinaison autour de Z fixe), sans orientation IAU exacte ni coordonnées géographiques revendiquées. Les visites spatiales restent translatoires. Le terrain est une grille de hauteur sur la sphère moyenne, et non encore un réseau mondial cube-sphere. Ses triangles CPU f64 sont la source commune du rendu et des contacts. Le volume joueur est une boîte de 0,6 × 1,8 × 0,6 m alignée sur le repère du site ; la caméra et la gravité suivent la verticale radiale. Ce contrôleur assume les pentes douces générées, sans grottes, corps dynamiques ni simulation de frottement générale. L'adhérence et la correction de hauteur assurent marche et repos ; les obstacles utilisent des collisions balayées et du glissement.

Le pas physique est de 1/120 s, alimenté par `Time<Real>` dans `GameSet::Navigation`, après l'orbite. Chaque frame accepte au plus 0,25 s de physique : les longues interruptions sont abandonnées plutôt que rattrapées brutalement. P et T affectent le calendrier orbital, donc les orbites et la rotation du site, sans accélérer la physique locale. Échap retire les commandes de mouvement, mais la gravité et les contacts continuent. Le rendu local utilise une seconde caméra et une profondeur distincte ; Mars est masquée comme sphère globale pendant la visite au sol. L'éclairage, le brouillard et le ciel restent artistiques.

`SurfaceState` conserve pieds, vitesse, yaw et pitch dans le site tournant. L’orientation fixe du site appartient à `MarsTerrain` ; `CelestialBody::rotation` fournit la rotation f64 commune à la surface et aux sphères. Après la physique, la navigation compose ces rotations et publie la pose globale dans `Explorer`, puis l’origine flottante. La caméra locale et les obstacles utilisent directement les différences dans le site ; la lumière solaire est convertie vers ses axes. Tourner Mars ne modifie pas les clés de tuiles ni leur révision et ne recrée aucun mesh.

| État du calendrier | Surface entraînée | Contrôleur local |
|---|---|---|
| ×1 / ×10 / ×100 / ×1000 | Orbite et rotation à cette vitesse | Marche, gravité et saut en temps réel |
| Pause P ou borne de date | Centre et rotation figés | Marche, gravité, saut et visée continuent |
| Longue interruption | Pose au JD courant, sans interpolation de l’entraînement | Au plus 0,25 s de physique par frame |

Même en l’air, le joueur reste exprimé dans le site tournant. Aucune force de Coriolis, centrifuge, d’Euler ni due à l’accélération translatoire orbitale n’est appliquée ; aucune vitesse inertielle n’est transférée lors des téléportations M/G/O/U. Ce contrôleur est un compromis de gameplay, particulièrement visible sous T, pas une intégration inertielle d’un astronaute. Voir les formules et conventions dans [ASTRONOMY](docs/ASTRONOMY.md).

**Suite :** phase dédiée au profilage/LOD adaptatif et au streaming, puis sauvegarde/distribution P6. P5 fournit maintenant récupération, inventaire et construction du même abri. LOD adaptatif selon la distance, relief et géométries plus complexes restent des extensions distinctes ; le chargement mondial actuel ne fournit pas de LOD mixte. [Roadmap](docs/ROADMAP.md).


Les sections P3.2–P3.4 ci-dessous décrivent les anciens modes de régression ; le lancement normal utilise le terrain mondial et la survie.

## Premier parcours cube-sphere P3.2

`SOLAR_CUBE_SURFACE=1 cargo run --release --locked` active deux patches centraux d'une même face lors de M, dans un seul repère tournant. Environ 207 m par patch, cellules de 3,23 m, contacts avec toute l'empreinte du joueur ; marche/course/saut et retours spatiaux conservés. Les deux couleurs repèrent le raccord. Cette paire sphérique sans relief est bornée ; elle ne charge aucune zone suivante. Sans l'option, le FPS actuel reste le défaut, avec neuf tuiles et ±4 096 m.

Après compilation, le scénario de 88 s avec 22 captures se lance avec `SOLAR_CUBE_SURFACE=1 SOLAR_CUBE_VALIDATE_DIR=docs/mon-nouveau-parcours target/release/solar_survival` ; le parent doit exister et le dossier final doit être nouveau. Ajouter `MESA_VK_WSI_PRESENT_MODE=immediate` sur ce poste NVK. Conventions, budgets et vérifications dans [CUBE_SPHERE](docs/CUBE_SPHERE.md), résultats dans [VALIDATION](docs/VALIDATION.md). Pas de migration du repère, passage entre faces, LOD mixte ou relief martien mesuré.

## Premier changement de repère local P3.3

`SOLAR_CUBE_MIGRATION=1 cargo run --release --locked` permet, après M, de marcher entre la paire centrale et sa voisine +U de la même face. Ce parcours reste borné : niveau 15/64 segments, sphère sans relief, couloir |z|≤50 m, deux patches résidents et retour réversible. Les pieds, yeux, visée et vitesse sont transportés en f64 ; les obstacles restent attachés au site d'entrée. Le terrain historique reste le défaut, et `SOLAR_CUBE_SURFACE=1` seul conserve le parcours P3.2 sans migration. Le profilage a priorité sur les deux options.

Pour le diagnostic après compilation, `SOLAR_CUBE_MIGRATION=1 SOLAR_CUBE_VALIDATE_DIR=<nouveau dossier>` conserve le parcours de 88 s, avec marche/course aller-retour et saut pendant migration. Les contrôles injectés sont ECS ; la [validation](docs/VALIDATION.md) distingue les preuves numériques et visuelles. Les limites de couverture, de contacts et de ressources sont décrites dans [CUBE_SPHERE](docs/CUBE_SPHERE.md). Le passage jouable entre faces reste ultérieur.

## Migration hors centre P3.4

`SOLAR_CUBE_OFFCENTER=1 cargo run --release --locked` ouvre après M un autre site, franchement hors du centre de la face, avec deux cartes adjacentes seulement. Leur couverture certifiée détermine le couloir de migration (~±49,44 m en Z) et deux seuils espacés de 2 m dans le repère d'entrée. Les refus ne construisent aucun terrain ; les deux patches/handles restent bornés et réutilisés. Boîte joueur et enveloppes d'obstacles sont réévaluées ; le terrain historique demeure le défaut, et les options P3.2/P3.3 seules conservent leurs parcours.

Pour le diagnostic de 88 s, ajouter `SOLAR_CUBE_VALIDATE_DIR=<nouveau dossier>` après compilation. La suite `python3 scripts/validate_cube_suite.py --output docs/ma-nouvelle-campagne --mesa-immediate` lance successivement hors centre, centrale, P3.2 et historique, et conserve codes/empreintes/journaux/captures ; `--mesa-immediate` concerne ce poste NVK. Les captures demandent ensuite une inspection humaine. [Validation et limites](docs/VALIDATION.md), [contrat détaillé](docs/CUBE_SPHERE.md). La suite de P3 est cadrée dans [NEXT_CHAT](docs/NEXT_CHAT.md).


## Première boucle de survie P4

Après Jouer et M : marcher vers l’abri turquoise à six mètres, entrer par la porte ouverte et laisser la combinaison se recharger. Réserve maximale 180 s ; dehors, 1 s consommée par seconde réelle ; dedans, +30 s/s. Le HUD indique la réserve, l’alerte à 30 s et la distance/direction de l’abri. À zéro, l’expédition se termine et R repart à l’abri avec la réserve pleine. R n’agit pas avant un échec.

P/T ne modifient pas la consommation. Le menu suspend la survie ; les visites spatiales conservent la réserve sans consommation/recharge. Le retour M retrouve le site fixé à la première arrivée de la session, éventuellement de nuit. Les bâtiments gardent leur identité et leur pose corporelle f64, indépendantes des patches/rebasages et des proxies. Aucun inventaire, recette, placement joueur ou sauvegarde disque en P4.

Diagnostic ciblé, après compilation release : `python3 scripts/validate_survival.py --output docs/mon-nouveau-p4 --mesa-immediate`. Le dossier doit être neuf ; l’option Mesa concerne seulement ce poste NVK. Le parcours de 56 s vérifie consommation/recharge/échec/R, menu, rebasage et seconde instance, avec treize captures. Il prépare des réserves raccourcies sans modifier les taux. Les anciens scénarios et le profilage gardent la survie désactivée. [Détail technique et reproduction](docs/SURVIVAL.md), [preuves et limitations](docs/VALIDATION.md).


## Ressources et construction P5

Huit caisses dorées autour du départ fournissent six alliage chacune. Approcher puis E, récolter deux caisses, B pour placer un abri à 12 unités. La visée et la marche déplacent le fantôme ; flèches pour ajuster, J/L pour tourner. Vert signifie autorisé ; rouge affiche le refus. Entrée confirme après revalidation du sol et des volumes occupés, X/B/Échap annulent sans dépense. Entrer dans l’abri construit recharge avec le même service P4.

Stock, caisses épuisées et bâtiments sont conservés pendant la session, y compris M/G/O/U, menu et R ; aucune sauvegarde disque. Ressource finie (48 unités), quatre abris constructibles en plus du premier. L’oxygène continue pendant le placement. Le terrain mondial reste à résolution fixe, sans LOD adaptatif.

Diagnostic après compilation : `python3 scripts/validate_construction.py --output docs/mon-nouveau-p5 --mesa-immediate`, dossier neuf, seize captures à inspecter. [Conventions, architecture et limites](docs/CONSTRUCTION.md), [validation](docs/VALIDATION.md).
