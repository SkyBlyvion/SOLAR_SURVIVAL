# Prochain chat Solar Survival — P11 audio intégré, campagne groupée reportée

Mise à jour : **15 octobre 2026**. Chef de projet : **SkyBlyvion**.
P1–P9 livrés sur Ubuntu ; P10 intégré, validation graphique ouverte. P11 ajoute
l'audio et les alertes à la demande explicite de l'utilisateur. Les résultats
finaux des contrôles sans bureau sont dans VALIDATION et les notes P11.

## Consigne utilisateur prioritaire

**Ne lancer automatiquement aucun test ouvrant une fenêtre, prenant le focus
ou capturant la souris.** Les campagnes P9/P10 sont reportées à la demande
utilisateur ; l’écoute P11 rejoint cette même campagne, sans sortie sonore
automatique pendant le travail courant. Référence persistante :
[TESTS_A_REALISER.md](TESTS_A_REALISER.md). Aucun écran virtuel, réactivation du
focus ou modification silencieuse de la capture du gameplay. Les contrôles
ordinaires CPU/ECS/compilation restent autorisés sans redemander de permission.
Les tests P11 n'installent aucun backend audio matériel ; décoder un WAV ne le joue
pas. Une entité Window en test MinimalPlugins n'ouvre pas de fenêtre OS.

## Prompt autonome

Nous poursuivons Solar Survival dans `/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL`.
Lire AGENTS.md, README.md, ROADMAP.md, cette passation, ASTRONOMY.md,
TESTS_A_REALISER.md, P11_AUDIO.md, P10_EXPLORATION.md, P9_BASE.md, PERSISTENCE.md,
et les derniers résultats de VALIDATION.md/PERFORMANCE.md et
`docs/p11-validation/2026-10-15/NOTES.md`. Pour terrain/rendu, lire aussi
MARS_ENVIRONMENT.md/GLOBAL_TERRAIN.md ; pour survie/construction, leurs contrats.
Examiner Git avant toute modification. HEAD au départ P11 : `fec803a` (P9 fini),
avec les modifications P10 **déjà présentes et non commitées**. Elles ont été
conservées ; P11 est ajouté au même workspace. Aucun commit demandé ni créé.
Ne pas assimiler le diff contre HEAD à du code entièrement écrit pendant P11.

**Objectif suivant : traiter les retours P11 et tenir à jour la checklist des
tests différés ; exécuter la campagne groupée seulement lorsque l'utilisateur
la demandera.** Corriger et rejouer les cas concernés, conserver preuves et limites.
Si l'utilisateur souhaite poursuivre le code, cadrer la suite avec lui : **P12
n'est pas défini**. Ne pas inventer une étape livrée ni considérer écoute ou
performance validées par des tests ECS. Le sans-focus n'empêche pas les corrections
et contrôles utiles sans bureau. Ne pas imposer une relance P9 en préalable.

## État réel P11

- `audio.rs` : onze signaux originaux, événements après résultats des commandes :
  récolte/transfert, construction, relevé/rapport, refus, menu/options et opérations
  de profil. Aucun succès sonore déduit d'un texte/UI ou d'un proxy de rendu.
- Observation O₂ après Navigation : à ≤30 s toutes les5s actives, ≤10s toutes les
  1,5s actives ; changement de seuil immédiat, début de recharge effective et
  passage à l'épuisement. Un seul rappel par frame longue ; P/T ne multiplie pas
  le rythme. Menu/espace suspendent, I/B laissent la survie active. R ne rejoue
  pas une recharge de base et un échec chargé n'est pas une nouvelle mort.
- Options volume0/25/50/75/100 %, défaut50 %, muet conservant la sélection ;
  Reset50 %/activé. Sons en cours coupés à zéro/muet ; pas de rattrapage à la
  réactivation. Les messages et le HUD O₂ restent indépendants de l'audio.
- Onze WAV PCM mono16bits/22050Hz, **187020octets** de fichiers, générés offline
  par `scripts/prepare_p11_audio.py`, sans échantillons tiers ; licence MIT,
  crédits et manifeste dans `assets/audio/`. Chargés une fois, non spatialisés,
  présentés comme signaux de combinaison/interface. Pas de musique/voix/ambiance/
  pas ou propagation martienne. Aucun asset GPU ou mesh ajouté.
- Huit voix maximum, onze demandes uniques, espacement150ms/motif, priorités
  échec→critique→faible→actions→UI avec préemption. DESPAWN + expiration2s même
  sans sink ; clip encore non chargé ignoré. Transitoires supprimés au chargement.
- Bevy reste **exactement =0.19.1**, fonctionnalités `bevy_audio`/`wav` ajoutées.
  Cargo.lock inclut bevy_audio0.19.1,rodio0.22.2,cpal0.17.3,hound3.5.1 et backends
  natifs ; aucune dépendance audio directe supplémentaire ni changement de physique.
  Les diagnostics/profilers existants désactivent AudioPlugin et le mixage P11.
- **Écriture SOLAR_SURVIVAL5, lecture4 conservée.** Nouvelle ligne audio validée
  (index0–4, booléen), paramètres par aventure. Charger P10/4 utilise50 %/activé,
  sans réécrire le fichier ; prochaine écriture (F5 ou changement d’aventure) écrit5, avec le secours habituel.
  Formats1/2/3 toujours refusés, jamais supprimés. Ancien binaire P10 ne lit pas5.

## Compilation audio sur ce poste

Ubuntu requiert désormais **pkg-config et libasound2-dev pour compiler**, plus
`libasound.so.2` et une sortie audio configurée pour jouer. Installation standard
à faire par un utilisateur habilité : `sudo apt install pkg-config libasound2-dev`.
Pendant P11, sudo demandait un mot de passe : aucune installation système faite.
Les paquets Ubuntu ont été téléchargés/extraits localement sous
`/tmp/solar-p11-build-deps/root` ; `source /tmp/solar-p11-build-deps/env.sh` règle
PKG_CONFIG, LD_LIBRARY_PATH et les chemins du sysroot pour Cargo. Ce dossier
**temporaire peut disparaître** ; les notes P11 donnent le procédé de reprise.
Ne pas introduire ces chemins dans la logique portable ou le lanceur de livraison.
La bibliothèque runtime standard est déjà installée. Aucun contournement du
contrôle de sudo ni mot de passe enregistré. Windows utilise son backend CPAL.
Sans périphérique à l'initialisation, Bevy avertit puis continue silencieusement ;
reconnexion à chaud non implémentée. Ce comportement reste à essayer réellement.

## État P10/P9 conservé

P10 : trois missions120/240/360m, F à3m puis F dans le premier abri, récompenses
uniques8+4/12+6/16+8 alliage/circuits, lot entier ou refus intact. Échec/sorties
M/G/O/U perdent les données non rapportées, découvertes gardées. R ne paie rien.
HUD cap horizontal/distance/base et estimation minimale retour÷4,5m/s+20s, sans
itinéraire garanti. Deux cubes/balise débloquée dans600m, six proxies, trois
matériaux constants ; poses corporellesf64. Trois vrais trajets ECS existants,
visée automatiquement orientée, sans périphérique OS ; pas une partie humaine.

P9 : porté8/coffre16/pile20, alliage/circuits, six recettes, lots6/30s actives,
base collective40m ; panneau160W de gameplay, batterie4,8MJ, générateur40W→4sO₂/s,
réserve720s+3600s/réservoir, combinaison180s/recharge30s/s. R est un secours
illimité sans remplir la base. Aucun câble/démantèlement ou avant-poste autonome.
Terrain≤13patches, LOD32/16/8, rebasage64m, relief et couronne P8 inchangés.
P9+P10≤312 proxies hors terrain/ciel/obstacles. Ces bornes ne mesurent pas VRAM/GPU.

## Vérifications et limites

**159 tests CPU/ECS réussis**, fmt/check all-targets, Clippy -D warnings et release
réussis. Archive Ubuntu P11 préparée, extraite hors checkout :803 fichiers et
onze WAV vérifiés, dépendances runtime résolues, **sans lancement ni écoute**.
Voir les commandes/codes finaux dans `docs/p11-validation/2026-10-15/checks.json`,
le journal release, VALIDATION et les notes P11. Les tests nouveaux vérifient le
vrai décodage Bevy, plafonds/priorités/expiration sans périphérique, muet et
suppression menu/espace/restauration, seuils/temps, options et compatibilité4/5.
Les tests de construction et des trois expéditions capturent aussi les événements
réels et vérifient absence de doubles succès. Pas d'AudioOutput installé en test.

**Aucune écoute, fenêtre de jeu, capture, GPU ou benchmark P11 exécuté.** Confort,
distinction des motifs, vrai périphérique Ubuntu, boutons Options à720p/900p,
archive lancée et coût audio réel restent ouverts dans AUD-01–07/PERF-05. Les
mesures P8→P9/P9→P10 et les autres cas antérieurs restent ouverts. Ne pas utiliser
un profiler silencieux comme preuve du coût audio actif. Aucune approbation
esthétique/sonore donnée par l'utilisateur.

Contraintes permanentes : Rust stable2024, Bevy=0.19.1, SI/f64/DVec3/DQuat,
origine soustraite avant f32, proxies en lecture ; Input→Simulation→Navigation→
Rendering→Interface, Time<Real> et souris sans facteur delta time. Visites
spatiales translatoires, surface tournante et forces d'inertie omises conservées.
Poste Ubuntu/X11,i7-6700K/32Go/GTX1060 3Go NVK. **Windows11 natif, RTX4050 6Go au
mieux, CPU exact («i5 12700K» à confirmer),1080p60 et VRAM/staging non validés.**
Fin du prochain chat : actualiser roadmap, validations/limites et cette passation,
puis fournir son lien. Préserver le sans-focus et tous les changements utilisateur.
