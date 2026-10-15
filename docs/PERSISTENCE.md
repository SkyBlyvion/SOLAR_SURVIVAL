> Depuis P12 : écriture **SOLAR_SURVIVAL 6**, lecture **4/5/6** ; commandes par aventure, valeurs par défaut pour4/5. Formats1/2/3 refusés sans suppression. [Contrat courant](P12_CONTROLS.md). Les formats cités dans les sections historiques ci-dessous décrivent leurs livraisons propres.

> P11 : écriture **SOLAR_SURVIVAL 5**, volume et mode muet par aventure. Lecture du
> format P10 **4 conservée** avec volume 50 %/son activé, sans réécriture au simple
> chargement ; prochaine écriture en 5 (F5 ou sauvegarde avant changement d’aventure). Formats 1/2/3 toujours refusés.
> [Contrat courant audio et compatibilité](P11_AUDIO.md).

> Livraison historique P10, 15 octobre 2026 : **SOLAR_SURVIVAL 4**, fichiers ≤256 Kio.
> Porté organisé, coffres/contenus, équipements, énergie J, réserve O₂ et renouvellement
> des sources, ainsi que les objectifs/relevés P10, sont conservés par profil.
> Formats 1/2/3 refusés sans migration ni suppression des fichiers. [Contrat P9](P9_BASE.md), [expéditions P10](P10_EXPLORATION.md).

# P6 — aventures et sauvegardes par profil

## Créer et choisir une aventure

Le menu principal propose **Nouvelle aventure**, **Charger une aventure**, Options, Crédits et Quitter. Avec une aventure active, **Reprendre** et **Sauvegarder l’aventure** sont également disponibles.

Nouvelle aventure ouvre un champ de nom obligatoire. Saisir le nom, puis cliquer **Créer l’aventure** ou Entrée. Le jeu crée un répertoire indépendant, écrit la session initiale et démarre une partie neuve : joueur près de la Terre, oxygène plein, aucun stock, aucun abri, aucun dépôt initialisé ni site de retour repris d’une autre partie. Les réglages d’affichage et de sensibilité servent de préférences initiales ; la progression est réinitialisée. Arriver sur Mars installe ensuite le premier abri et les huit caisses propres à cette aventure.

**Charger une aventure / F9** ouvre une liste. Cliquer un nom ou utiliser ↑/↓, puis **Charger cette aventure / Entrée**. La sélection parcourt toutes les aventures ; la liste présente cinq noms par page en grande fenêtre, trois en petite fenêtre. Le chargement laisse le menu ouvert ; Reprendre retrouve la partie choisie. Le nom de l’aventure active reste affiché dans le menu.

**Sauvegarder l’aventure / F5** met à jour uniquement le profil actif. Créer une autre aventure ou charger un autre profil sauvegarde d’abord le profil quitté ; si cette écriture échoue, le changement est refusé. Charger le profil déjà actif relit sa sauvegarde sans la réécrire auparavant. Annuler la création n’installe aucun nouveau répertoire. Quitter ne déclenche pas de sauvegarde automatique.

## Répertoires indépendants

Racine par défaut : `saves/` dans le dossier de travail. `SOLAR_SAVE_ROOT` permet de choisir une autre racine. Exemple :

```text
saves/
  Alpha/
    session.solar
    session.solar.bak
  Bravo/
    session.solar
    session.solar.bak
```

Chaque répertoire possède sa session et sa copie précédente. Le nom accepte 1 à 40 lettres/chiffres, espaces, tirets et `_` ; les espaces extérieurs sont retirés. Chemins, noms réservés, noms vides et doublons sont refusés, sans écrasement. Les créations ignorent la casse pour éviter Alpha/alpha sur des OS différents. Les répertoires de profil liés symboliquement ne sont pas admis. Le chargement lit uniquement le profil choisi ; une aventure corrompue ne remplace pas l’aventure active.

Le format d’instantané courant est la version 5 ; la lecture de la version 4 est conservée. Les anciens profils peuvent apparaître dans la liste, mais leur chargement est refusé explicitement. Les preuves de cette implémentation intermédiaire restent dans `docs/p6-validation/` ; elles ne prouvent pas la gestion de plusieurs profils. Les nouvelles preuves sont dans `docs/p6-profiles-validation/`.

## État de chaque aventure

- Identités et poses DVec3/DQuat des abris, compteur d’identités et site de retour.
- Porté de 8 piles et coffres de 16 piles (20 objets/pile), ordre et contenu typés alliage/circuits ; équipements, énergie collective en J, O₂ collectif en secondes, sources renouvelables et délais.
- Caisses : positions/orientations corporelles et épuisement, sans récolte ni paiement lors du chargement.
- P10 : trois découvertes, rapports validés et données en cours. Poses des balises reconstruites depuis le premier abri et la graine ; données en cours admises seulement en surface sans échec. Aucun paiement au chargement.
- Oxygène et échec ; abri de recharge recalculé depuis la pose restaurée. R conserve les constructions et ressources de ce profil.
- Joueur en espace/surface : astre suivi, sélection, position/rotation f64, vitesse configurée ; pieds/vitesse/visée/grounded/accumulateur du contrôleur.
- Fournisseur mondial/artistique, graine et orientation de la carte active, trois obstacles du premier site.
- JD, pause/multiplicateur, sensibilité/FOV/MSAA/fenêtre, volume sonore et mode muet.
- Les voix audio et les temporisations d’alerte P11 sont transitoires et supprimées à la restauration.

Aucun patch résident, mesh, index de contact, handle GPU, entité ou Transform n’est sauvegardé. Charger reconstruit les caches depuis les poses f64, republie les mêmes six BodyBox par abri et conserve la même GameCamera. Les prévisualisations, catalogues de construction, inventaire ouvert/sélection et entrées/capture sont annulés. Les anciens proxies sont retirés, y compris au démarrage d’une nouvelle aventure vide. Le temps passé application fermée n’est pas rattrapé.

## Format, refus et secours

`src/persistence.rs` gère l’instantané ; `src/persistence/profiles.rs` gère les répertoires, nouvelles parties et changements de profil. UTF-8 **SOLAR_SURVIVAL 5** (lecture 4 conservée), nombres décimaux à aller-retour f64, collections bornées à 32 abris/64 équipements-coffres/8 caisses/4 sources et fichier limité à **256 Kio**. FNV-1a 64 bits détecte une corruption accidentelle ; ce n’est pas une signature de sécurité.

Versions inconnues, fichiers tronqués/supplémentaires, valeurs non finies, quaternions/indices/JD/réserve invalides, identités, types/piles/capacités, énergie/O₂ ou délais de renouvellement incohérents, graine incompatible et pose non couverte/sous le terrain sont refusés avant remplacement du World. Le terrain candidat est construit avant publication des ressources ; sa révision renouvelle les caches. Les modes historiques sans assets de survie refusent création et chargement de parties normales.

Écriture temporaire exclusive, synchronisée et fermée ; conservation de l’ancien fichier dans `.bak`, puis publication par renommage. En cas d’échec, remise en place de la copie tentée. Un temporaire préexistant n’est ni écrasé ni supprimé. Si le principal manque, le secours du profil est lu et signalé ; un principal présent mais corrompu est refusé, sans récupération silencieuse d’une autre version. Pas de garantie de toute panne matérielle/coupure électrique ni d’écritures simultanées de plusieurs processus dans le même profil.

## Interface de jeu

**B** ouvre le catalogue de construction, libère le curseur et bloque le pilotage. La fiche de l’abri indique son usage, son coût et le stock disponible. **Prévisualiser / Entrée** passe au placement du même abri P4 ; la prévisualisation reste gratuite même si le stock est insuffisant. Flèches/J-L pour déplacer/orienter, Entrée pour confirmer après admission du terrain/volumes et paiement unique, X/B/Échap pour annuler. Fermer le catalogue n’entraîne aucun coût. L’oxygène et la gravité continuent pendant ce menu de jeu ; le menu principal suspend toujours la survie.

Palette, typographie et fonds des menus/HUD partagés dans `ui::theme`. En surface, oxygène en haut à gauche, stock/instruction en bas à gauche, espace central dégagé. **N** affiche la navigation ; **H** affiche la télémétrie, masquée par défaut. Les instruments vitaux restent indépendants. Le menu de profil ajuste la taille de son en-tête et la pagination à la hauteur disponible. L’apparence est vérifiée sur captures ; une validation de lisibilité ne vaut pas une approbation esthétique de l’utilisateur.

## Distribution et vérification

`config::asset_directory` recherche assets à côté du binaire avant le repli de développement. Le script de packaging assemble le build natif déjà compilé, assets/shaders/police, licences, manifeste et lanceur. Linux : tar.gz + lancer.sh ; Windows natif : exe/zip/lancer.cmd. Les lanceurs stabilisent le dossier de travail et donc la racine des profils. `saves/`, `dist/` et caches Python sont ignorés par Git.

```bash
cargo build --release --locked
python3 scripts/validate_persistence.py --focus-x11 --mesa-immediate --output docs/mon-nouveau-p6
python3 scripts/package_release.py --output dist/ma-nouvelle-archive
```

Dossiers neufs, compilation avant les lancements GPU successifs ; omettre X11/Mesa sur les autres postes. Le diagnostic crée Alpha et Bravo via le menu et la saisie de noms : Alpha construit/rebase/recharge ; Bravo commence vide puis garde un stock différent. Un second processus retrouve ces deux progressions, teste le catalogue, l’espace, échec/R et le refus d’un profil corrompu. Départs près des caisses, rebasage, réserves et JD/options sont préparés explicitement ; commandes ECS, pas périphériques OS. [Résultats, captures et limites](VALIDATION.md). Windows 11 natif reste à valider sur un hôte Windows.
