# Prochain chat Solar Survival — terminer P6

**Étape réelle : P6.** Sauvegarde/chargement de session et packaging natif implémentés ; validation Ubuntu, validation Windows 11 encore nécessaire avant de déclarer la distribution sur les deux OS terminée. Mise à jour : 13 octobre 2026.

## Prompt autonome de reprise

Nous poursuivons Solar Survival dans `/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL`. Lis AGENTS.md, README.md, docs/ASTRONOMY.md, docs/CUBE_SPHERE.md, docs/GLOBAL_TERRAIN.md, docs/SURVIVAL.md, docs/CONSTRUCTION.md, docs/PERSISTENCE.md, docs/SURFACE_SKY.md, docs/VALIDATION.md, docs/ROADMAP.md, docs/PERFORMANCE.md et cette passation. Examine Git et le code avant de choisir l'action suivante. **Ne pas repartir de P4/P5 : ils sont réalisés et commités**, de même que le LOD adaptatif et la caméra unique.

Départ réel de la passe P6 : HEAD `22efa72`, avec seulement NEXT_CHAT remplacé par une ancienne reprise P4. P4 `94036f8`, P5 `ebddd90`, LOD `43f163b`, caméra/ciel `22efa72` étaient déjà publiés. L'utilisateur a corrigé explicitement le mauvais cadrage de reprise : étape **P6**. Les passations reçues et les essais intermédiaires sont conservés dans `docs/p6-validation/NEXT_CHAT-before.md` et `docs/p5-recheck-2026-10-13/`. Préserver toutes les preuves. L'utilisateur demande de commiter et pousser le travail terminé sur GitHub sans redemander l'autorisation ; vérifier l'état réel de HEAD/origin au début de la reprise.

## Fonctionnalité P6 actuelle

Menu principal : **Sauvegarder / F5**, **Charger la session / F9**, puis Reprendre. Chargement disponible au lancement, sauvegarde après Jouer. Statut et chemin visibles ; refus de chargement sans mutation de la partie courante. Les raccourcis sont limités au menu principal. Charger conserve le menu ouvert et libère le curseur. Quitter ne sauvegarde pas automatiquement.

Un emplacement local : `saves/session.solar` relatif au dossier de travail, ou fichier choisi via `SOLAR_SAVE_PATH`. Format UTF-8 **SOLAR_SURVIVAL 1**, nombres f64 à aller-retour exact, contrôle FNV-1a (corruption accidentelle, pas sécurité cryptographique), maximum 64 Kio/32 abris/8 dépôts. Temporaire exclusif synchronisé/fermé, ancien fichier `.bak`, publication par renommage et rollback tenté en cas d'erreur. Charger utilise `.bak` si le principal manque, avec message explicite ; principal présent mais corrompu/incompatible refusé. Temporaire préexistant conservé. Pas de garantie de toute coupure matérielle ni de plusieurs processus écrivant le même emplacement simultanément.

Données conservées : identités/poses corporelles des mêmes abris, compteur d'identités, site de retour, caisses/épuisement/stock, oxygène/échec, mode/astre/position/visée/vitesse du joueur, pieds/vitesse/grounded/accumulateur du contrôleur, orientation de la carte active et obstacles du premier site, fournisseur mondial/artistique et graine, JD/pause/multiplicateur, sensibilité/FOV/MSAA/fenêtre. Pas de rattrapage du temps hors application.

Toutes les validations et la construction d'une MarsTerrain candidate précèdent la publication des ressources. Le voisinage/LOD/index sont reconstruits autour de la carte sauvegardée ; pose, collisions et services restent f64. Abri courant recalculé, six BodyBox par instance republies sans nouvelle installation ni paiement. Révision de rendu renouvelée depuis la session remplacée, même entité GameCamera conservée. Preview/entrées/capture/boost/saut pending annulés au chargement. Aucun mesh/index/handle/Transform/entité GPU sauvegardé. Les diagnostics historiques qui désactivent les assets de survie refusent sauvegarde et chargement.

Propriétaires : `src/persistence.rs`, `src/ui/menu.rs`, `src/buildings/mod.rs`, `src/construction.rs`, `src/player/{mod,surface}.rs`, `src/planets/terrain.rs`, `src/rendering/{terrain,buildings,construction}.rs`. Diagnostic opt-in `src/config/persistence_validation.rs`, lanceur `scripts/validate_persistence.py`. Pas de dépendance supplémentaire ; Cargo.toml/Cargo.lock inchangés, Rust stable 2024/Bevy **exactement =0.19.1**.

## Distribution et suite de P6

`config::asset_directory` recherche assets à côté du binaire, puis le checkout en développement. `scripts/package_release.py` assemble un **build natif déjà compilé**, assets/shaders/police, licence du jeu et textes des dépendances Cargo verrouillées, manifeste et lanceur. Linux : tar.gz + lancer.sh ; Windows natif : exe/zip/lancer.cmd. Les lanceurs stabilisent le dossier de travail. Les fichiers personnels `saves/`, archives `dist/` et caches Python sont ignorés par Git.

**Prochaine action : terminer les critères de distribution P6**, principalement build/essai/package natifs Windows 11 et cible RTX 4050/Core i5. Relever CPU exact, RAM, pilote, résolution, alimentation et réglages. Vérifier extraction/lancement hors checkout, écriture/remplacement/chargement, reprise après fermeture, fichiers absent/corrompu/incompatible, modes espace/surface/échec, réglages et contacts/recharge. Les instructions PowerShell ne sont pas une validation Windows. Sans hôte Windows, garder cette limite explicite ; ne pas inventer un build ou déclarer P6 livré sur les deux OS.

Sur Ubuntu, réutiliser l'archive validée dans `dist/solar-survival-ubuntu-p6-final.tar.gz` ou en créer une neuve. Une extension de qualité graphique n'est pas automatiquement requise : les options existantes sont déjà persistées. Choisir toute évolution supplémentaire avec un besoin utilisateur ou des mesures, sans modules vides, nouvelle dépendance ou chantier terrain ajouté au hasard.

## Contrats P3/P4/P5 à préserver

M ouvre Mars sphérique traversable sur les six faces/arêtes/coins. Niveau spatial 15, voisinage local ≤13 patches, **résolutions 32/16/8 réellement choisies selon distance**, bords 32 communs et éventails mixtes sans fissure. Hystérésis 80/110 et 160/200 m, sélection à l'entrée/rebasage de 64 m ; contacts proches fins, disque 66 m, index XZ 8 m limité à ±72 m. Streaming synchrone borné, pas de planète entière, quadtree ou relief mondial. Treize patches seuls ne prouvent pas le LOD. [GLOBAL_TERRAIN](GLOBAL_TERRAIN.md).

Oxygène 180 s, consommation 1 s/s réel dehors, recharge +30 s/s dans l'abri, alerte 30 s. Menu suspend réserve/recharge ; gravité et calendrier continuent. Espace conserve réserve/échec sans consommation. R après échec repart au premier abri plein, stock/dépôts/bâtiments conservés. H laisse les HUD vitaux/construction visibles.

Huit caisses à six alliage : E à 2,5 m, pas à travers un solide. Douze alliage par abri ; B/flèches/J-L/Entrée, preview verte/rouge avec motif, X/B/Échap sans coût. Revalidation à confirmation, admission exacte du carré complet et enveloppes SAT, débit unique. Même OxygenShelter P4, six pièces/contacts, porte 2,4 m, poses DVec3/DQuat fixes dans Mars et BuildingId monotone. Proxies locaux à 120 m avec assets partagés ; ni patch ni proxy ne possède les données physiques.

Une seule GameCamera espace/surface ; ciel local artistique, direction/rayon angulaire des astres issus du catalogue en f64, occultation par profondeur. Panorama statique 32 Mio, uniforme borné ; brouillard 40–60 m et sol mat. Rendu révisé testé, sans prétendre une approbation esthétique donnée par l'utilisateur. [SURFACE_SKY](SURFACE_SKY.md).

## Vérifications et environnement

Sept nouveaux tests P6 portent le total à **127** : reprise complète après construction/rebasage/en l'air/options, recharge et prochaine identité, espace/échec/R/historique/avant première arrivée, fichiers invalides/refus sans mutation, écriture/secours, menu/boutons/F5-F9, destination de diagnostic sans assets. Le test de proxies P5 couvre désormais aussi le remplacement des dépôts par une session vide. Les régressions terrain/horloges/contacts/caméra restent conservées. Résultats Cargo et graphiques finaux, empreintes, inspection et archive dans [VALIDATION](VALIDATION.md) et `docs/p6-validation/` ; les dossiers initial/intermédiaire ne remplacent pas les références finales.

Ubuntu 24.04.5 LTS/X11 DISPLAY=:1, Rust stable 1.99.0, i7-6700K/GTX 1060 3 Go/NVK/Mesa 25.2.8. **Windows natif et RTX 4050/Core i5 non validés.** Pas de nouvelle campagne P2, périphériques OS, suspension réelle/DPI, benchmark ou RAM/VRAM exhaustive. Les FPS de capture ne sont pas des mesures comparables aux profils historiques. Les mesures caméra/LOD précédentes restent dans PERFORMANCE avec leurs limites.

Pour toute modification significative : `cargo fmt --check`, `cargo check --locked`, `cargo check --all-targets --locked`, `cargo test --locked`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo build --release --locked`. Compiler puis lancer les GPU successivement ; dossiers de preuves finaux nouveaux dans un parent existant. Omettre X11/Mesa sur Windows/autres pilotes.

```bash
python3 scripts/validate_persistence.py --focus-x11 --mesa-immediate --output docs/mon-nouveau-p6
python3 scripts/package_release.py --output dist/ma-nouvelle-archive
```

SI/1:1, positions/transforms globaux f64 ; soustraction d'origine f64 avant proxies f32. GameSet Input→Simulation→Navigation→Rendering→Interface ; orbite Time<Real>, souris déjà intégrée sans delta time. Physique locale pas 1/120 s/max 0,25 s, oxygène temps réel complet hors menu/espace. Visites translatoires, repère de surface tournant, forces d'inertie omises. Aucun modèle atmosphérique/physiologique scientifique implicite. Actualiser README/ROADMAP/VALIDATION/documents techniques et cette passation à la fin, fournir son lien.
