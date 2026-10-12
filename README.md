# Solar Survival — version 0.2 · prototype FPS Mars

Prototype d'exploration 3D de notre système solaire, fondation d'un jeu de survie spatial en première personne. **Les positions, rayons et distances physiques restent en mètres à l'échelle 1:1, en `f64`.** Bevy ne reçoit que des représentations relatives à la caméra.

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

Appuyez sur **M**, ou sur **Surface Mars / retour** dans le panneau gauche, pour rejoindre le prototype FPS martien. Clic droit capture la souris ; WASD / ZQSD permet de marcher, Shift de courir et Espace de sauter. **M** revient à la visite spatiale de Mars ; G, O et U permettent aussi de quitter la surface. Le jeu commence près de la Terre, à J2000 (1 janvier 2000, 12 h à l'échelle temporelle du modèle). Cliquez droit pour piloter ; Échap libère le curseur. Cliquez sur Mars dans le panneau gauche puis sur **Visiter**, ou utilisez **4 puis G**. La visite est une téléportation de gameplay à une distance de sécurité, avec suivi orbital de la cible.

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

## Contrôles

Les touches de déplacement désignent des positions physiques : WASD sur QWERTY, ZQSD sur AZERTY.

| Commande | Fonction |
|---|---|
| Clic droit | Capturer la souris et activer le pilotage |
| Échap | Libérer la souris et arrêter le déplacement au clavier |
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

Les vitesses disponibles sont 5 m/s, 250 m/s, 20 km/s, 20 000 km/s, 2 millions km/s et 100 millions km/s. Ce sont des choix de gameplay. En espace, l'interface affiche la **vitesse de déplacement configurée** ; à l'arrêt elle demeure affichée. En surface, elle indique aussi la vitesse résolue après contacts, l'état au sol/en l'air et la hauteur des pieds au-dessus du support. La marche utilise 4,5 m/s, la course 8 m/s et le saut une impulsion de vitesse de 4 m/s : conventions de gameplay. L'accélération temporelle n'affecte pas les contrôles ni l'interface.

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
- FPS martien : gravité radiale, marche/course/saut, contacts avec terrain et trois obstacles statiques.
- Terrain artistique local courbé sur le rayon moyen de Mars ; neuf tuiles de 128 m, grille de 4 m, chargement et déchargement à chaque changement de tuile.
- Contrat cube-sphere et graines reproductibles ; structures de survie encore inactives.

## Architecture

```text
src/main.rs           assemblage fenêtre/plugins
src/config/validation.rs scénarios graphiques optionnels espace et surface
src/lib.rs            ordre des systèmes Input → Simulation → Navigation → Rendu → UI
src/universe/         catalogue, unités SI, calculs orbitaux, horloge, repères f64
src/rendering/        proxies astronomiques et caméra/meshes de terrain en mètres
src/camera/           entrées et orchestration de navigation
src/player/           état d'exploration, repère suivi et contrôleur FPS
src/physics/          garde spatiale, gravité radiale et contacts cinématiques f64
src/planets/          clés cube-sphere, graines et géométrie du terrain local
src/survival/         données vitales et inventaire futur
src/ui/               panneau de navigation, télémétrie et marqueurs
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
```

PowerShell :

```powershell
$env:SOLAR_JD="2461325.5"
cargo run
Remove-Item Env:SOLAR_JD
```

`SOLAR_SMOKE_SECONDS` active uniquement un lancement borné de validation, avec visite automatique des dix corps, vue globale et captures PNG `docs/v02-space-00.png` à `v02-space-10.png`. `SOLAR_SURFACE_SMOKE_SECONDS` (minimum 40 s, 50 s conseillé) active un autre scénario : repos, marche contre obstacle, saut, déplacement latéral, course à travers les tuiles, libération des commandes, retour spatial et Terre. Il produit `docs/v02-surface-00.png` à `v02-surface-09.png`. Si les deux variables sont définies, le scénario surface est prioritaire. Les entrées sont injectées dans l'ECS, sans simuler les périphériques de l'OS. Ne pas définir ces variables pour jouer normalement.

Sur cette session Ubuntu/NVK, certains lancements en présentation VSync sont tombés à environ 1 FPS, avec un timeout de présentation. Le lancement suivant a rétabli environ 23 FPS en surface ; c'est un contournement local du pilote, pas une mesure de performance garantie :

```bash
MESA_VK_WSI_PRESENT_MODE=immediate cargo run --release
```

Cette variable est documentée par [Mesa](https://docs.mesa3d.org/envvars.html). Elle peut provoquer du tearing ; elle n'est pas nécessairement utile sur un autre pilote et ne concerne pas le lancement Windows.

## Limites actuelles et suite

Les données et conventions sont détaillées dans [docs/ASTRONOMY.md](docs/ASTRONOMY.md). Ce modèle képlérien n'est pas une éphéméride scientifique : pas de N-corps, perturbations lunaires, précession exacte, retards lumineux ou éclipses. La phase et l'orientation lunaires sont conventionnelles, explicitement non observées. La période lunaire et ses dimensions sont réelles.

Le rendu compresse chaque profondeur selon `d_render = 10 000 × d / (d + 10⁸)` et applique le même facteur au rayon du corps. L'angle apparent d'une sphère est conservé ; la profondeur GPU est un proxy. Les occultations entre sphères volumineuses et les anneaux ne sont pas exactes après compression. L'éclairage est directionnel par corps et artistiquement exposé, sans photométrie en inverse carré ni ombres entre corps. Les continents, étoiles, cratères visuels et nuages ne sont pas une cartographie réelle. Les axes ont une inclinaison indicative avec longitude de pôle conventionnelle ; les objets utilisent des rayons moyens sphériques sans aplatissement.

En mode espace, la garde de navigation maintient au moins 1 km d'altitude et borne les voyages à 10¹⁶ mètres du Soleil. Elle ne simule pas de contacts physiques, et peut expulser la caméra si une planète mobile la rattrape. Une planète lointaine peut être subpixel : les marqueurs et visites permettent de la retrouver. Le GPU doit supporter les fonctionnalités 3D de Bevy/wgpu. 60 FPS est un objectif à mesurer sur le matériel cible, pas une garantie.

Le prototype FPS utilise un site local **translatoire**, sans rotation de surface, borné à ±4 096 m dans ses deux axes horizontaux. Le terrain est une grille de hauteur sur la sphère moyenne, et non encore un réseau mondial cube-sphere. Ses triangles CPU f64 sont la source commune du rendu et des contacts. Le volume joueur est une boîte de 0,6 × 1,8 × 0,6 m alignée sur le repère du site ; la caméra et la gravité suivent la verticale radiale. Ce contrôleur assume les pentes douces générées, sans grottes, corps dynamiques ni simulation de frottement générale. L'adhérence et la correction de hauteur assurent marche et repos ; les obstacles utilisent des collisions balayées et du glissement.

Le pas physique est de 1/120 s, alimenté par `Time<Real>` dans `GameSet::Navigation`, après l'orbite. Chaque frame accepte au plus 0,25 s de physique : les longues interruptions sont abandonnées plutôt que rattrapées brutalement. P et T affectent seulement le temps orbital. Échap retire les commandes de mouvement, mais la gravité et les contacts continuent. Le rendu local utilise une seconde caméra et une profondeur distincte ; Mars est masquée comme sphère globale pendant la visite au sol. L'éclairage, le brouillard et le ciel restent artistiques.

**Suite :** repère martien tournant, zones cube-sphere au-delà du site borné, profilage du rendu et contrôleur pour géométries plus complexes. Puis oxygène, ressources, inventaire et construction ; les structures de survie ne consomment encore rien.
