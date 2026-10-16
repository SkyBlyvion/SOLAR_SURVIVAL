# Prochain chat Solar Survival — P13, outils et base productive

Mise à jour : **16 octobre 2026**. Chef de projet : **SkyBlyvion**.

## Prompt autonome

Nous poursuivons Solar Survival dans `/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL`.
Lire AGENTS.md, README.md, ROADMAP.md, cette passation, P12_CONTROLS.md,
P13_GAMEPLAY.md et les premières sections de VALIDATION.md. Lire ASTRONOMY.md
avant toute modification physique/astronomique et consulter les contrats du
domaine concerné.

Examiner Git avant d’éditer. La reprise partait du dépôt propre à **808ed4d**
(titre « p11 », contenant aussi P12 et sa campagne). Ce chat laisse des
modifications **non commitées**, sans commit demandé : deux aides de construction,
configuration Cargo/ignore, README, documents et preuves du 16 octobre, puis
réorientation de la roadmap et nouveau contrat P13_GAMEPLAY.md.
Préserver tous les changements et les anciennes preuves.

**Objectif suivant : implémenter le lot A de P13, puis B et C.** La liste de
travail est désormais **P13_GAMEPLAY.md** ; ne pas recommencer une phase de cadrage
ni créer une nouvelle liste. Ce chat a actualisé les documents, **sans coder P13**.

Dernières précisions utilisateur : s’inspirer du principe de Planet Crafter,
récolter plusieurs matériaux, construire des bâtiments et machines productives,
ajouter une barre rapide pour les outils, enrichir B, poser les machines dans
les bâtiments comme dehors. Il demande une grande liste d’ajouts/modifications
et d’avancer beaucoup plus vite. Ces demandes remplacent la proposition antérieure
centrée sur trois lieux et une liaison de secours à réparer.

Boucle : récolter → fabriquer → construire → alimenter → produire → améliorer
→ explorer plus loin. Le but du premier palier est une base productive qui sert
son propre développement ; aucune fin arbitraire après trois balises.

Suivre les trois lots de **P13_GAMEPLAY.md**, avec interface et sauvegarde ensemble :

1. **A — outils/inventaire :** barre rapide8 cases liée aux vrais objets,
   sélection remappable, outil de récolte/construction, inventaire adapté et cinq
   ressources proposées (fer/titane/silice/cobalt/glace), dépôts persistants.
2. **B — construction/aménagement :** catégories, fondations, atelier,
   établi/four/extracteur, support terrain/plancher, accès, déplacement/démontage.
3. **C — production :** recettes, entrées/sorties/cycles alimentés, budgets
   énergétiques, composants utilisables, premier palier d’autonomie/déblocages.

Dépendance observée dans `Construction::candidate` : la pose s’appuie seulement
sur le terrain et exclut le volume complet des abris6,3×3,95×6,3m. Il faut utiliser
les vrais solides/supports pour aménager l’intérieur, pas désactiver les collisions.
Les machines sur plancher et terrain gardent leur pose f64 au rebasage/rechargement.

Les nombres/recettes du contrat sont un premier périmètre de conception à équilibrer,
pas des fonctionnalités existantes. Aujourd’hui, Item a deux variantes et BuildKind
six recettes. Réutiliser inventaire/base/énergie/contacts existants. Prévoir
les données de sauvegarde dès chaque lot ; définir le traitement des profils4/5/6
avant tout nouveau format, sans conversion silencieuse, écrasement ou suppression.
Les touches1–8 de barre rapide doivent cohabiter avec la visite numérique des
planètes via la séparation aventure/visite libre, et respecter le remappage P12.

La suite P14–P18 : habitat/fabrication avancée, transformation planétaire à cadrer,
exploration/rover, logistique, alpha martienne ; vol et second monde ensuite.
La terraformation est une piste cohérente avec la référence, pas un modèle détaillé
explicitement spécifié par l’utilisateur ni une simulation scientifique livrée.

**Méthode :** travailler par lots jouables, réutiliser les fondations, regrouper
les vérifications nécessaires en fin d’ensemble cohérent et cibler les corrections.
Ne pas étendre la documentation à chaque petit ajout, multiplier les captures,
recréer les archives ou relancer les benchmarks sans besoin concret. Les règles
AGENTS sur les changements significatifs restent applicables. Ne pas échanger
l’intérêt du jeu contre le nombre de tests. P12/démarrage Cargo sont réparés.

## Consigne de validation

L’utilisateur a autorisé la campagne du 15 octobre, puis interdit de répéter la
passe d’expédition et demandé l’arrêt des revalidations répétées. Le 16 octobre,
« reprends » a conduit à corriger les deux aides restantes, puis « lance le jeu »
a autorisé un contrôle rapide Mars/HUD/menu. Ensuite, le signalement d’échec de
`cargo run` a nécessité une reproduction et une seule vérification après réparation.
Ces demandes ne rouvrent pas les suites, benchmarks ou expéditions. Les cadrages
P13 n’ont lancé ni compilation, ni jeu, ni tests ; aucune manipulation de la partie
potentiellement ouverte. **Ne rien relancer automatiquement pour compléter les
cases ouvertes.** Pour une implémentation significative future, regrouper les
contrôles Cargo requis par AGENTS sur un ensemble cohérent ; éviter de répéter les
mêmes parcours et respecter le créneau graphique/focus demandé par l’utilisateur.

Le contrôle rapide a compilé le profil dev en 28,19 s ; captures K/Y inspectées,
sans diagnostic. Le jeu d’essai PID16108 était fermé au début du dépannage Cargo.
Le lancement final est **`cargo run --locked`**, compilation dev en 20,15 s puis
fenêtre X11 et Mars visibles, sans entrée injectée, scénario SOLAR ni réglage Mesa.
**Jeu laissé actif au contrôle final, PID18485**, session outil82592. Examiner
l’état réel avant toute interaction, sans supposer que ce PID reste vivant.
Ce lancement ordinaire utilise les profils normaux ; aucune écriture de sauvegarde
ni chargement automatisé effectué par l’agent pendant ce dépannage.
[Résultats](VALIDATION.md), preuves dans `docs/p12-validation/2026-10-16/`.

## État réel et configuration du poste

- P12 : 26 commandes physiques remappables, Options → Commandes (cinq pages),
  conflits/réservations refusés, annulation/reset, modificateurs gauche/droite liés.
  GameplayKeys partagé entre les domaines ; touches maintenues dans le menu
  bloquées jusqu’au relâchement. Aides résolues après Interface.
- Deux chaînes de `src/ui/construction.rs` emploient maintenant `<key:KeyE>` et
  `<key:KeyB>` : approche d’une caisse et rappel du catalogue. K/Y vus au contrôle.
- Clic droit, souris/molette, Entrée/Échap, flèches, 0–9, +/−, F5/F9/F12 fixes.
  Pas de combos, manette ou remappage souris. Sauvegardes écrites en6, lues en4/5/6,
  affectations par aventure ; formats1/2/3 refusés sans suppression.
- P9 : porté8/coffre16/pile20, six recettes, énergie/O₂, services à40m.
  P10 : trois missions120/240/360m, récompenses uniques et données en cours.
  P11 : onze WAV originaux, huit voix, alertes O₂, volume/muet persistants.
- Ubuntu/X11, i7-6700K/32Go/GTX1060 3Go NVK. L’erreur Cargo était l’absence du
  paquet système `libasound2-dev`/`alsa.pc`, malgré le runtime ALSA présent.
  L’ancien sysroot `/tmp/solar-p12-build-deps` **n’est plus nécessaire**.
- Sans sudo disponible, le paquet Ubuntu `libasound2-dev 1.2.11-1ubuntu0.3`
  correspondant au runtime est extrait dans **`.local-deps/` ignoré par Git**.
  Son `alsa.pc` utilise un préfixe relatif et sa bibliothèque de développement
  pointe sur le runtime installé. `.cargo/config.toml` configure
  `PKG_CONFIG_PATH_x86_64_unknown_linux_gnu` avec ce chemin relatif ; chemins
  système conservés. **`cargo run` fonctionne sans `source` supplémentaire**.
  Sur un autre poste, installer normalement les prérequis du README : ce dossier
  local n’est pas distribué. Bevy/Cargo.toml/Cargo.lock et logique audio inchangés.

## Preuves historiques et limites

Le 15 octobre : **165 tests CPU/ECS**, fmt/check all-targets, Clippy et release
réussis. Une seule passe graphique des trois expéditions, 268,660s, jamais répétée.
Menus720p/900p, backend audio réel/absent, sauvegarde/reprise, archive extraite et
entrées natives contrôlés. Parcours automatisés avec poses parfois préparées ;
pas de partie humaine ni d’approbation esthétique/sonore. Capture numérique audio
sans écrêtage ; confort subjectif non établi. Voir `docs/p12-validation/2026-10-15/`
et `docs/validation-groupee/2026-10-15/` ; leurs deux anciennes aides fixes sont
maintenant corrigées. Le 16 octobre n’a pas rejoué les suites.

Références P8 en échec avant export : **aucune comparaison de performance valide**.
Charge64 équipements/audio actif-muet, VRAM/staging, Windows11 natif, RTX4050,
CPU cible exact et1080p60 restent ouverts, sans relance automatique.

Archive `dist/solar-survival-ubuntu-p12.tar.gz` non reconstruite depuis la retouche,
SHA256 `1506af817db4cc20f41e8b34b040c0dfab9e41ca98fa4e6adcfc5262ef5d65c5`.
Elle contient encore les rappels E/B du 15 octobre ; ne pas lui attribuer le code
corrigé le16. Aucun nouveau build release demandé pour ce dépannage.

## Contraintes permanentes

Rust stable2024, Bevy **exactement0.19.1**, Cargo.lock versionné, Ubuntu/Windows11.
SI/f64/DVec3/DQuat, origine soustraite avant f32, proxies en lecture.
Input→Simulation→Navigation→Rendering→Interface ; Time<Real> orbital indépendant,
souris sans delta time. Visites spatiales translatoires et surface tournante
conservées. Aucun changement physique dans cette reprise. En fin de prochain chat,
actualiser roadmap, validations/limites et cette passation, puis fournir son lien.
