# P6 : sauvegarde locale et chargement de session

La session se sauvegarde depuis le menu principal avec **Sauvegarder / F5** et se restaure avec **Charger la session / F9**, y compris au lancement avant Jouer. Charger laisse le menu ouvert ; **Reprendre** revient au jeu, curseur libre. Les raccourcis agissent seulement dans ce menu. Quitter ne sauvegarde pas automatiquement. Une sauvegarde remplace l'emplacement courant et garde sa copie précédente en `.bak`.

Fichier par défaut : `saves/session.solar`, relatif au dossier de travail du programme. `SOLAR_SAVE_PATH` choisit un autre fichier, sur Ubuntu et Windows. Le menu affiche le résultat et le chemin. Le dossier est créé lors de la première sauvegarde ; un emplacement inaccessible produit un message et conserve la partie. Les lanceurs d'archive se placent dans le dossier du jeu pour stabiliser le chemin par défaut. Les sauvegardes personnelles et archives locales sont ignorées par Git.

## État conservé

- Abri initial et abris construits : BuildingId, compteur d'identités, poses corporelles DVec3/DQuat et site de retour.
- Inventaire et huit caisses : positions/orientations corporelles et épuisement, sans nouvelle récolte ni paiement au chargement.
- Oxygène et échec : réserve exacte, issue verrouillée. L'abri de recharge courant est recalculé depuis la pose du joueur ; R conserve stock et constructions.
- Joueur : mode espace/surface, astre suivi et sélection, position/rotation f64, vitesse spatiale configurée ; pieds/vitesse/visée/grounded/accumulateur du contrôleur en surface.
- Terrain : fournisseur mondial ou artistique, graine, orientation de la carte active et poses des trois obstacles d'entrée. Le site d'arrivée reste distinct de la carte rebasée.
- Calendrier : JD, pause et multiplicateur ; réglages de sensibilité, FOV, MSAA et fenêtre.

Aucun mesh, index de contact, patch résident, handle GPU, Transform, entité ou horloge du système d'exploitation n'est écrit. Le temps passé hors de l'application n'est pas rattrapé. Capture de souris, boost, demandes de navigation, saut en attente et prévisualisation de construction sont annulés au chargement. Le calendrier et la gravité continuent dans le menu selon les règles existantes ; l'oxygène y reste suspendu.

## Format et restauration

`src/persistence.rs` possède un instantané explicite des propriétaires actuels et un format UTF-8 **SOLAR_SURVIVAL version 1**. Champs ordonnés et nommés, nombres décimaux Rust préservant l'aller-retour f64, collections bornées à 32 abris/8 caisses, fichier limité à **64 Kio** avant parsing. Le contrôle FNV-1a 64 bits détecte la corruption accidentelle ; ce n'est pas une signature cryptographique ni une protection contre la modification volontaire.

Le chargement refuse les versions inconnues, données tronquées/supplémentaires, valeurs non finies, quaternions non unitaires, indices/JD/réserve hors domaine, identités incohérentes, bilan d'alliage invalide, graine incompatible ou pose non couverte/sous le sol. Le rayon et la graine du catalogue actuel restent les références ; aucun paramètre astronomique ajouté. Une évolution du catalogue/générateur ou du schéma nécessitera une nouvelle version ou migration explicite.

Les modes de diagnostic historiques qui désactivent survie/assets refusent sauvegarde et chargement, afin de préserver leurs scènes. Toutes les validations et la construction d'une candidate MarsTerrain précèdent la mutation du World. Le voisinage local et son index sont reconstruits avec l'orientation sauvegardée, en conservant les contacts proches fins. Les obstacles restent au premier site ; les bâtiments republient leurs six BodyBox sans nouvelle installation ni nouvel identifiant. La révision de rendu augmente depuis la session remplacée afin de renouveler les caches. La même entité GameCamera est conservée ; navigation/rendu réappliquent pose, projection, couches et réglages dans l'ordre habituel Input→Simulation→Navigation→Rendering→Interface. Les anciennes caisses de rendu sont libérées même si la session chargée précède la première arrivée et ne possède aucun dépôt.

## Écriture et secours

La sauvegarde écrit un temporaire avec création exclusive et `sync_all`, ferme le fichier, conserve l'ancien emplacement dans `.bak`, puis publie le nouveau par renommage. Les renommages vers un nom libre sont compatibles avec le comportement de std sous Linux/Windows, sans dépendance d'OS dans le gameplay. Un échec de publication tente de remettre l'ancienne copie en place ; un temporaire préexistant n'est ni écrasé ni supprimé.

Si le fichier principal manque après une interruption entre les renommages, Charger lit `.bak` et le signale. Si le fichier principal existe mais est corrompu/incompatible, il est refusé avec maintien de la partie ; le secours n'est pas chargé silencieusement. Pour revenir explicitement à la copie précédente, choisir son chemin avec SOLAR_SAVE_PATH. Le protocole conserve une copie lors d'un arrêt du processus ; il ne garantit pas la résistance à toute panne matérielle/coupure électrique, les métadonnées du dossier n'étant pas synchronisées sur tous les OS. Deux processus écrivant le même emplacement simultanément ne sont pas pris en charge.

## Distribution locale

`config::asset_directory` recherche d'abord `assets` à côté du binaire, puis utilise le checkout comme repli de développement. `scripts/package_release.py` copie le build **natif** déjà compilé, shaders/police, licence du jeu et textes de licence disponibles dans les dépendances Cargo verrouillées. Il produit un manifeste de fichiers et une archive tar.gz sous Linux, zip sous Windows. Le lanceur `.sh`/`.cmd` stabilise le dossier de travail. Ce script ne compile pas pour un autre OS.

```bash
cargo build --release --locked
python3 scripts/package_release.py --output dist/solar-survival-ubuntu
# Extraire dans un dossier neuf, puis lancer lancer.sh.
```

Sur Windows 11 natif : compiler avec Rust stable/MSVC, puis exécuter le même script avec un nouveau dossier Windows ; il choisit `.exe`/`.zip`/`.cmd`. Pilotes graphiques et bibliothèques de l'OS restent nécessaires ; Ubuntu/X11/Vulkan selon README. Un protocole Windows n'est pas une validation exécutée depuis Linux.

## Vérification ciblée

```bash
cargo test --locked
cargo build --release --locked
python3 scripts/validate_persistence.py --focus-x11 --mesa-immediate --output docs/mon-nouveau-p6
```

Dossier nouveau dans un parent existant, compilation avant les lancements, GPU successifs. Omettre X11/Mesa sur un autre poste. Le premier processus récolte deux caisses avec E, construit par B/Entrée, rebase, recharge et sauvegarde par F5 dans le menu, puis ferme normalement. Le second démarre avec des ressources neuves, charge par F9, vérifie les poses/stocks/options, reprend la recharge, sauvegarde/charge en espace et après échec, teste R et un fichier corrompu. Départs près des caisses, abri/rebasage et réserve de 60 s sont préparés explicitement ; les commandes sont ECS, pas des périphériques OS. Vingt captures à inspecter. Les résultats et les limites réellement observés sont dans [VALIDATION](VALIDATION.md).
