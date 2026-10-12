# Solar Survival — version 0.2 · FPS Mars et repère tournant

Prototype d'exploration 3D de notre système solaire, fondation d'un jeu de survie spatial en première personne. **Les positions, rayons et distances physiques restent en mètres à l'échelle 1:1, en `f64`.** Bevy ne reçoit que des représentations relatives à la caméra.

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

Le menu libère la souris et bloque les commandes de navigation, P/T et H. Le calendrier orbital continue avec `Time<Real>` ; pour le figer, utiliser P avant d'ouvrir le menu. En surface, la gravité et les contacts continuent, sans entrée de déplacement. Cette convention conserve les deux horloges du prototype. Les scénarios de validation/profilage démarrent directement en jeu.

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
| H | Masquer / afficher l'interface |

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
- Terrain artistique local courbé sur le rayon moyen de Mars ; neuf tuiles de 128 m, grille de 4 m, chargement et déchargement à chaque changement de tuile.
- Contrat cube-sphere f64 : conversions, voisins orientés, graines reproductibles et raccord de deux patches de même niveau ; structures de survie encore inactives.

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
src/config/cube_validation.rs parcours graphique borné de franchissement
examples/cube_seam.rs diagnostic graphique borné, indépendant du terrain jouable
src/survival/         données vitales et inventaire futur
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

## Limites actuelles et suite

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

**Suite :** zones cube-sphere au-delà du site borné, profilage du rendu et contrôleur pour géométries plus complexes. Puis oxygène, ressources, inventaire et construction ; les structures de survie ne consomment encore rien.


## Premier parcours cube-sphere P3.2

`SOLAR_CUBE_SURFACE=1 cargo run --release --locked` active deux patches centraux d'une même face lors de M, dans un seul repère tournant. Environ 207 m par patch, cellules de 3,23 m, contacts avec toute l'empreinte du joueur ; marche/course/saut et retours spatiaux conservés. Les deux couleurs repèrent le raccord. Cette paire sphérique sans relief est bornée ; elle ne charge aucune zone suivante. Sans l'option, le FPS actuel reste le défaut, avec neuf tuiles et ±4 096 m.

Après compilation, le scénario de 88 s avec 22 captures se lance avec `SOLAR_CUBE_SURFACE=1 SOLAR_CUBE_VALIDATE_DIR=docs/mon-nouveau-parcours target/release/solar_survival` ; le parent doit exister et le dossier final doit être nouveau. Ajouter `MESA_VK_WSI_PRESENT_MODE=immediate` sur ce poste NVK. Conventions, budgets et vérifications dans [CUBE_SPHERE](docs/CUBE_SPHERE.md), résultats dans [VALIDATION](docs/VALIDATION.md). Pas de migration du repère, passage entre faces, LOD mixte ou relief martien mesuré.
