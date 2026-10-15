# Prochain chat Solar Survival — après P9, poursuivre le jeu sans perturber le bureau

Mise à jour : **15 octobre 2026**. Chef de projet : **SkyBlyvion**.
**P1–P8 réalisés ; P9 livré et validé fonctionnellement sur Ubuntu.** Sa comparaison
P8/P9 de performance est reportée à la demande utilisateur. P10 n'est pas implémenté.

## Consigne utilisateur prioritaire

**Ne plus lancer automatiquement de test qui ouvre une fenêtre, prend le focus,
capture la souris ou gêne son travail dans une autre application.** L'utilisateur
a demandé d'arrêter ces campagnes et de continuer le code/le jeu ; il accepte des
vérifications sans interaction avec son bureau. Les campagnes graphiques restantes
attendront un créneau explicitement demandé. Aucun jeu/benchmark n'est laissé actif.
Pas d'écran virtuel, de réactivation répétée du focus ni de changement silencieux
des règles de capture du gameplay. Les lanceurs P9 ont maintenant une garde de
créneau visible explicite. Ne pas demander à nouveau l'autorisation pour des
vérifications ordinaires sans bureau déjà autorisées ; ne pas les répéter sans motif.

## Prompt autonome

Nous poursuivons Solar Survival dans `/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL`.
Lire AGENTS.md, README.md, ROADMAP.md, cette passation, ASTRONOMY.md, P9_BASE.md,
MARS_ENVIRONMENT.md, GLOBAL_TERRAIN.md, SURVIVAL.md, CONSTRUCTION.md,
PERSISTENCE.md, ART_DIRECTION.md, VALIDATION.md, PERFORMANCE.md et
`docs/p9-validation/2026-10-15/NOTES.md`. Examiner Git avant modification et préserver
les changements utilisateur ainsi que toutes les preuves. HEAD observé au départ
et à la reprise : **09f7027 — preparation pour P9**. P9 est dans le workspace,
**aucun commit demandé ni créé** ; ne pas supposer que HEAD/origin sont identiques.

**Objectif suivant : poursuivre le code/le jeu, normalement P10, exploration et
objectifs**, selon la demande du prochain chat. Ne pas recommencer P9 ni relancer
ses mesures de bureau comme préalable. Cadrer une première boucle opérationnelle
qui donne une raison de partir de la base et d'y revenir : site/balise, objectif,
récompense et navigation locale lisible, progression par profil. Choisir les règles
concrètes avant de coder ; P10 n'a encore ni sites ni objectifs livrés. Préserver
l'autonomie/récolte/stockage P9 ; si une expédition exige une réserve portable ou
une desserte éloignée, le définir explicitement plutôt qu'inventer un réseau
logistique abstrait. Pas de nouvelle dépendance ou modules vides anticipatoires.

## État réel P9

- `inventory.rs` : alliage/circuits, porté de **8 piles**, coffre de **16**, limite
  **20 objets/pile**. Quantités exactes 1–20 (bouton1/5/20 ou ↑↓), pile ou maximum
  admissible, regroupement et échange d'emplacements. Refus exact sans mutation.
- I ouvre le porté et un coffre proche ; E ouvre aussi un coffre à2,5 m. Revalidation
  portée et segment dégagé à chaque opération ; fermeture de l'accès devenu
  impossible. Inventaire/catalogue libèrent la souris et bloquent pilotage/P/T,
  mais survie/production restent actives. Menu principal/espace/échec les suspendent.
- Construction conserve le porté et les huit caisses finies ; quatre sources
  durables ocre/cyan derrière l'arrivée, x=±12/±16m,z=+4m, lot de6, renouvelé après
  **30 s réelles actives**. Une récolte refusée ne vide pas le lot. Pas de minerais
  scientifiques ni génération mondiale de dépôts.
- B propose abri12alliage, coffre6, panneau8+4circuits, batterie8+4,
  générateur12+6, réservoir6. Paiement uniquement depuis le porté après admission
  et confirmation. 32 abris/64 équipements-coffres maximum, poses/id fixes dans Mars.
  Admission sur triangles fins, OBB et refus de construire sur les sources.
- `Buildings` possède abris, équipements/coffres et contenus. `Base` possède
  sources, énergie collective J et réserve O₂ collective en secondes d'autonomie.
  Un bus de gameplay à **40 m du premier abri**, sans câbles : équipements à40 m,
  abris desservis à40 m ; coffres et abris peuvent être plus éloignés mais ces
  derniers n'ont alors pas de recharge. Pas de démantèlement/outpost autonome.
- Panneau160 W crête ×incidence positive ×exp(−0,65 poussière P8) ×(1−0,25 nuages).
  Aucun usage électrique des lux artistiques. Batterie4,8 MJ (1333,33 Wh), installée
  vide ; générateur40 W →4s O₂/s, réservoir+3600 s vide. Réserve initiale720 s,
  combinaison180 s/recharge30 s/s. Si la base se vide pendant une longue frame,
  le temps restant consomme de nouveau la combinaison. Réserves bornées.
- R après échec garde la base/contenus et rend une combinaison pleine, sans
  remplir la base : secours de gameplay volontaire. M et le streaming ne recréent
  rien. Production/renouvellement en secondes réelles ; P/T change le Soleil/météo,
  sans multiplier les débits. Conditions de fin de frame, pas de rattrapage fermé.
- **SOLAR_SURVIVAL 3**, sans migration des formats1/2 ni suppression des fichiers,
  à la demande utilisateur. Porté organisé, coffres, identités/poses, équipements,
  charges/réserves et délais restaurés transactionnellement ; UI/preview annulées.
  Limite256 Kio, checksum/secours et profils indépendants conservés.
- Proxies corporels f64 avant conversion locale f32, cube et13 textures P7 partagés,
  huit matériaux supplémentaires, boîtiers simples colorés sans animation ; pas
  de nouveau mesh/texture GPU par frame. Max306 proxies hors terrain/ciel/obstacles.
  Le terrain et l'atmosphère P8 restent ceux de la passation précédente.

## Vérifications acquises

`docs/p9-validation/2026-10-15/checks.json` : fmt, check all-targets,
**147 tests**, Clippy -D warnings et release réussis. Régressions de précision,
contacts/terrain/marche/sauts/menus conservées. P9 couvre capacités/transferts,
répétitions et conservation, vraie UI, amorçage/récoltes intactes, renouvellement,
jour/nuit/tempête, panne/reprise, réserve pleine, longue frame, rebasage, deux
coffres et ordre/charges/délais par profils ; fichiers incohérents sans mutation.

Archive extraite : **116,404s +26,465s, deux codes 0**, aucun diagnostic,
binaire/assets et **CWD hors checkout**. Collecte/construction/transferts réels,
dates/poses/réserves d'approche préparées ; deux processus, deux profils et refus
Broken, menu/rebasage/nuit/R/M. **40 captures finales inspectées**, quatre planches
(36) plus quatre vues des équipements, campagne séparée118,061s code 0. Ce n'est
ni une partie humaine sans repositionnement, ni une approbation esthétique donnée
par l'utilisateur. Première campagne corrigée conservée séparément.

Archive : `dist/solar-survival-ubuntu-p9.tar.gz`, **36 994 649 octets**,747 fichiers
vérifiés. SHA256 `8fb7c54a2928a07ee378d24f53d8aeeaac95c071a3fc265b037bad61c72611ea`.
Binaire `b8436414b812a9e453c1c8346dfe54b99db24577c65be2d3bd240f102d125499`.
Extraction : `/tmp/solar-survival-p9-2026-10-15/solar-survival-ubuntu-p9`.

**Performance reportée :** références P8 en échec avant CSV, codes 101 à26,385/8,612s,
marche/collecte non atteintes, perte de focus tracée. Ne pas inventer de comparaison
ou de régression P9. Les13 textures restent40,168 Mio ; pire cas64 équipements, VRAM
et staging non mesurés. Suivre la décision d'arrêt des tests de bureau ci-dessus.

## Contraintes permanentes

Rust stable édition 2024, Bevy **exactement =0.19.1**, Cargo.lock versionné, aucune
nouvelle dépendance/Avian sans besoin réel et compatibilité vérifiée. SI/f64,
DVec3/DQuat, origine soustraite avant conversion explicite f32 ; proxies en lecture.
Ordre Input→Simulation→Navigation→Rendering→Interface ; calendrier Time<Real>,
souris déjà intégrée sans delta time. Marche4,5m/s/course8/saut4m/s, gravité radiale,
sous-pas1/120s et0,25s max/frame inchangés. Visites spatiales translatoires ; surface
martienne tournante, forces d'inertie du repère omises. Terrain ≤13 patches, LOD32/16/8,
rebasage64 m et paysage P8 artistique, pas de cartographie mesurée.

Poste : Ubuntu/X11, i7-6700K/32 Go/GTX1060 3 Go NVK Mesa 25.2.8, Rust 1.99.0.
**Windows 11 natif, RTX4050 6 Go au mieux, CPU cible exact (« i5 12700K » à confirmer),
1080p60 et VRAM totale/staging restent non validés.** Ne pas extrapoler Ubuntu.
À la fin du prochain chat : actualiser état/critères ROADMAP, validations/limites et
cette passation autonome, puis fournir son lien. Préserver la consigne sans focus.
