# Solar Survival — version 0.1

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

Le jeu commence près de la Terre, à J2000 (1 janvier 2000, 12 h à l'échelle temporelle du modèle). Cliquez droit pour piloter ; Échap libère le curseur. Cliquez sur Mars dans le panneau gauche puis sur **Visiter**, ou utilisez **4 puis G**. La visite est une téléportation de gameplay à une distance de sécurité, avec suivi orbital de la cible.

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
| WASD / ZQSD | Déplacement dans le repère orienté de la caméra |
| Espace / Ctrl | Monter / descendre dans ce repère |
| Shift | Vitesse temporaire ×10 |
| Molette (pilotage), Pg↑ / Pg↓, + / − | Changer de niveau de vitesse |
| Tab | Sélection suivante |
| 0–9 | Soleil, Mercure, Vénus, Terre, Mars, Jupiter, Saturne, Uranus, Neptune, Lune |
| F | Orienter la caméra vers la cible |
| G | Visiter la cible, à 3,5 rayons de son centre (5,5 pour Saturne) |
| O | Vue globale depuis 35 UA au-dessus du plan écliptique |
| U | Passer au repère héliocentrique en conservant la position |
| T | Temps orbital ×1 / ×10 / ×100 / ×1000 |
| P | Pause des orbites et des rotations |
| H | Masquer / afficher l'interface |

Les vitesses disponibles sont 5 m/s, 250 m/s, 20 km/s, 20 000 km/s, 2 millions km/s et 100 millions km/s. Ce sont des choix de gameplay. L'interface affiche la **vitesse de déplacement configurée** ; à l'arrêt elle demeure affichée. L'accélération temporelle n'affecte pas les contrôles ni l'interface.

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
- Contrat de zones cube-sphere et graines reproductibles ; structures de survie et gravité radiale préparées.

## Architecture

```text
src/main.rs           assemblage fenêtre/plugins et test graphique optionnel
src/lib.rs            ordre des systèmes Input → Simulation → Navigation → Rendu → UI
src/universe/         catalogue, unités SI, calculs orbitaux, horloge, repères f64
src/rendering/        meshes, LOD, matériaux GPU et étoiles
src/camera/           entrées et orchestration de navigation
src/player/           état d'exploration, repère suivi et modes
src/physics/          garde sphérique et calcul de gravité radiale
src/planets/          clés de zones cube-sphere et graines
src/survival/         données vitales et inventaire futur
src/ui/               panneau de navigation, télémétrie et marqueurs
src/config/           paramètres de démarrage
assets/shaders/       shader WGSL de présentation des planètes
tests/               tests des fondations physiques et numériques
```

La logique de jeu et de simulation est en Rust. WGSL est utilisé uniquement pour le calcul graphique sur le GPU. Aucun éditeur de jeu, modèle externe, Blender, cloud ou service en ligne n'est requis. Avian n'est pas intégré : la garde de navigation actuelle ne nécessite pas de moteur physique. Le chargement dynamique sera introduit avec les vraies zones de Mars ; il n'existe pas de module de chargement vide.

La police DejaVu Sans est embarquée avec sa licence dans `assets/fonts/` pour assurer le rendu français sur les deux plateformes.

Les systèmes astronomiques créent des entités `CelestialBody` + `AstronomicalPosition` ; les entités de rendu séparées ne sont que des proxies. `BodyId` indexe le catalogue interne et fournit un identifiant stable. Les futurs satellites pourront compléter le catalogue et les évaluateurs d'orbites ; une hiérarchie de parents est présente, mais l'évaluateur actuel gère explicitement la Lune.

## Dates et diagnostic

Une date configurable est acceptée par `SOLAR_JD`, comprise entre le 1 janvier 1800 et le 1 janvier 2050. Une valeur invalide utilise J2000 ; l'horloge se met en pause à la borne supérieure pour ne pas extrapoler hors de la plage JPL.

Ubuntu :

```bash
SOLAR_JD=2461325.5 cargo run
RUST_LOG=info cargo run --release
SOLAR_SMOKE_SECONDS=30 cargo run --release
```

PowerShell :

```powershell
$env:SOLAR_JD="2461325.5"
cargo run
Remove-Item Env:SOLAR_JD
```

`SOLAR_SMOKE_SECONDS` active uniquement un lancement borné de validation, avec visite automatique des dix corps, vue globale et captures PNG dans `docs/`. Ne pas définir cette variable pour jouer normalement.

## Limites actuelles et suite

Les données et conventions sont détaillées dans [docs/ASTRONOMY.md](docs/ASTRONOMY.md). Ce modèle képlérien n'est pas une éphéméride scientifique : pas de N-corps, perturbations lunaires, précession exacte, retards lumineux ou éclipses. La phase et l'orientation lunaires sont conventionnelles, explicitement non observées. La période lunaire et ses dimensions sont réelles.

Le rendu compresse chaque profondeur selon `d_render = 10 000 × d / (d + 10⁸)` et applique le même facteur au rayon du corps. L'angle apparent d'une sphère est conservé ; la profondeur GPU est un proxy. Les occultations entre sphères volumineuses et les anneaux ne sont pas exactes après compression. L'éclairage est directionnel par corps et artistiquement exposé, sans photométrie en inverse carré ni ombres entre corps. Les continents, étoiles, cratères visuels et nuages ne sont pas une cartographie réelle. Les axes ont une inclinaison indicative avec longitude de pôle conventionnelle ; les objets utilisent des rayons moyens sphériques sans aplatissement.

La garde de navigation maintient au moins 1 km d'altitude et borne les voyages à 10¹⁶ mètres du Soleil. Elle ne simule pas de contacts physiques, et peut expulser la caméra si une planète mobile la rattrape. Une planète lointaine peut être subpixel : les marqueurs et visites permettent de la retrouver. Le GPU doit supporter les fonctionnalités 3D de Bevy/wgpu. 60 FPS est un objectif à mesurer sur le matériel cible, pas une garantie.

**Prochaine étape : prototype FPS sur Mars.** Construire quelques zones cube-sphere autour du joueur avec repère local en mètres, chargement/déchargement, collision terrain et gravité radiale. Introduire Avian seulement après vérification de sa compatibilité avec Bevy 0.19.1. Puis ajouter oxygène, ressources, inventaire et construction. Le mode Surface n'est pas activé en 0.1 ; les structures de survie ne consomment rien.
