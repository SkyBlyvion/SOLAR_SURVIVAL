# Prochain chat Solar Survival — P9, base, inventaire et stockage

Mise à jour : **15 octobre 2026**. **P1–P8 réalisés sur Ubuntu de développement.**
P9 et P10 ne sont pas implémentés. Windows natif et matériel cible restent ouverts.
Chef de projet et orchestrateur : **SkyBlyvion**.

## Prompt autonome

Nous poursuivons Solar Survival dans `/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL`.
Lire AGENTS.md, README.md, docs/ROADMAP.md, cette passation, ASTRONOMY.md,
P9_BASE.md, MARS_ENVIRONMENT.md, GLOBAL_TERRAIN.md, SURVIVAL.md, CONSTRUCTION.md,
PERSISTENCE.md, ART_DIRECTION.md, VALIDATION.md, PERFORMANCE.md et
`docs/p8-validation/2026-10-15/NOTES.md`. Examiner Git avant de modifier et préserver
les changements utilisateur et toutes les preuves historiques. P8 est commité sur
`main` dans **`ee87770` — `feat: add P8 Mars relief, horizons and dynamic atmosphere`**.
La présente préparation documentaire ajoute le périmètre inventaire/stockage demandé
par l'utilisateur ; elle ne commence pas l'implémentation P9. Vérifier l'état réel
au démarrage et préserver ce cadrage, sans supposer que HEAD/origin sont identiques.

**Décision utilisateur actuelle : aucune compatibilité avec les anciennes
sauvegardes n'est nécessaire pour le moment.** Format courant `SOLAR_SURVIVAL 2`,
format 1 refusé explicitement, sans migration ni suppression des fichiers.
Ne pas réintroduire une obligation de migration depuis les anciennes passations.

## État réel livré

- M ouvre Mars mondiale, six faces, contacts/poses SI f64 et origine mobile.
  P4/P5 : oxygène 180 s, recharge dans le même abri, huit caisses, stock et
  construction à 12 alliage. Menus/options/catalogue/HUD et profils indépendants.
  L'inventaire actuel est un compteur d'alliage dans Construction : inventaire
  multitype à piles/capacité finie et coffres de base ne sont pas encore livrés.
- P7 : treize textures partagées, 40,168 Mio utiles GPU / budget 128 Mio ; cartes
  des huit planètes sourcées. Soleil/Lune et détails locaux artistiques. Le raccord
  de Jupiter avait été corrigé offline avant P8 ; maîtres/crédits/preuves conservés.
- P8 : champ de hauteur artistique mondial en axes corporels f64, bruit de valeur
  3D quintique à quatre échelles 3600/900/240/60 m, amplitudes 70/14/1,2/0,1 m,
  |h| ≤85,3 m. Aucun relief MOLA ou site géographique réel. Les mêmes sommets/indices
  servent au rendu et aux contacts ; normales visuelles métriques du même champ.
- Niveau spatial 15, intérieurs 32/16/8, bords communs 32, hystérésis et contacts
  proches fins conservés. Cache ≤13 patches, index 8 m sur ±72 m ; rebasage à 64 m.
  `MarsTerrain::chart` soustrait l'origine corporelle avant rotation inverse :
  test de collider conservé à sa tolérance initiale de 2 nm.
- Paysage : une couronne depuis le contour exact des patches, sans jupe ni fissure
  topologique, neuf anneaux supplémentaires jusqu'à +11 km du contour ; ≤6400
  sommets /11520 triangles. Même champ de hauteur, approximation visuelle sans
  contacts lointains. Recréée au changement de patch central/site seulement,
  Arc/mesh réutilisés sinon. Pas de quadtree, asynchrone ni morphing temporel.
- Atmosphère : jour ocre, halo bleu localisé au lever/coucher, nuit étoilée, voiles
  de glace d'eau et poussière/tempêtes dynamiques. `rendering/atmosphere.rs` dérive
  l'état depuis JD/position/graine : pas d'intégrateur ni de météo sérialisée.
  P/T agit sur le calendrier et la météo ; menus laissent ce calendrier évoluer.
  Pression d'ordre 600 Pa /0,6 % Terre affichée, pas de pression locale simulée.
  Sources NASA et conventions artistiques distinctes dans MARS_ENVIRONMENT.md.
- Une caméra, même triangle de ciel ; nuages analytiques sur coquille artistique
  20 km avec deux échelles de bruit 3D, sans volume/texture/passe nouvelle. Bloc
  uniforme 704 octets. Lumière/ambiante/étoiles/disques/brouillard atténués par
  poussière et voiles. Visibilité calme 5–8,5 km, tempête 150 m ; far surface 15 km.
  Les anciens modes de régression gardent la brume 40–60 m et leur terrain propre.

## Vérifications acquises, ne pas répéter sans motif

- `final-checks.json` : fmt, check all-targets, **139 tests**, Clippy -D warnings,
  probe CPU et release réussis ; release 161,732 s. Correction ultérieure **WGSL
  seule** des nuages trop géométriques, build confirmé 0,20 s et shader validé GPU.
  Même binaire, dernier shader présent dans l'archive et les campagnes finales.
- Tests : coutures six faces/LOD mixtes bit-identiques, couronne jointe et bornée,
  contacts/admission fins, marches/courses/sauts/rebasages, bâtiments et profils ;
  météo reproductible/continue, événements rares et états jour/nuit, invariance
  des paramètres d'observateur et météo au rebasage. Anciennes fixtures refusées.
- Ciel final : 66,411 s, code 0, 22 captures inspectées sur quatre planches ;
  lever/coucher, nuit, nuages, tempête, rebasage, abri/menu/retours et autres sites.
  Dates/poses préparées naturellement, pas d'observation continue d'une tempête
  complète. Première galerie aux nuages géométriques conservée séparément.
- Référence P7 actuelle avec son binaire ET ses assets propres : 59,494 s, code 0.
  Deux P8 sans recorder 59,461 /59,495 s, puis GPU séparé 59,506 s, trois codes 0,
  aucun diagnostic, hashes inchangés. 1440×900 MSAA4, même parcours P5 sans captures.
  Surcoût moyen P8 : +1,59 ms preview, +1,84 ms streaming, +1,90 ms abri repos.
  Tableau p95, spans CPU/GPU et RSS/HWM dans PERFORMANCE/comparison.json.
  Rebasage observé ≤5,725 ms ; sync meshes ≤9,361 ms, retours inclus. Peu
  d'événements, pas une garantie de pire cas. Aucun gain 60 FPS annoncé.
- Probe CPU dev optimisé, hors renderer : sphère 0,753 ms moyen /0,974 p95 ;
  relief+couronne 2,280 /5,145 ms. 200 rebasages sur 12,8 km ; ce n'est pas du
  release/GPU et cela exclut les normales/uploads.
- Archive extraite : deux processus profils 48,414 +54,388 s, codes 0, 32 captures
  inspectées sur six planches ; Alpha deux abris/stock0, Bravo un abri/stock6,
  fermeture/chargement/rebasage/recharge, catalogue/espace/échec/R et refus Broken.
  Profils **neufs format 2**. Binaire/assets hors checkout, CWD du helper checkout.
  Smoke supplémentaire avec CWD extérieur : 8,418 s, code 0, audit textures et
  fichiers distribués inchangés. Premier smoke en erreur IO car son vieux
  diagnostic suppose un dossier docs pour capturer ; création explicite puis
  relance sans modification de jeu. Échec/preuves conservés, pas masqués.
- **54 captures finales ciel/profils inspectées**, dix planches ; original nuageux
  en plus. Les captures intermédiaires/smoke ne sont pas comptées dans ces 54.
  L'inspection n'est pas une approbation esthétique donnée par l'utilisateur.

Archive actuelle : `dist/solar-survival-ubuntu-p8.tar.gz`, **36 907 688 octets**,
747 fichiers vérifiés. SHA256 `96c28d2c71351278b5fa36207627c668b0730a6c271d098e7264a8c3d102f003`.
Binaire `b27bdbd4207a0685132897ddac2b11c416d1a6df91aaca7ba5f8569efd00d5c3`.
Extraction temporaire : `/tmp/solar-survival-p8-2026-10-15/solar-survival-ubuntu-p8`.

## Prochain objectif : P9 — périmètre confirmé

Implémenter le cadrage [P9_BASE](P9_BASE.md), comprenant **panneaux solaires,
batteries, générateur et réserve d'oxygène, récolte durable, gestion d'inventaire,
stockage de base, interface et sauvegarde**. L'inventaire et les coffres font
partie de P9 à la demande explicite de l'utilisateur, pas d'une étape ultérieure.

Boucle complète : **explorer → récolter → transporter → déposer/reprendre →
fabriquer/construire → produire/stocker énergie et oxygène → repartir**.

Commencer par examiner les propriétaires réels dans Construction, Buildings,
Oxygen et Session, puis choisir une première implémentation opérationnelle :

- Inventaire porté avec types utiles, quantités, piles organisables et capacité
  bornée ; dépasser le seul compteur d'alliage actuel.
- Coffres constructibles, identités et poses corporelles f64 fixes, contenus et
  capacités propres ; accès de proximité et interface inventaire/conteneur.
- Transferts de quantité/pile/maximum explicites et atomiques : capacité, portée
  et disponibilité vérifiées, refus sans perte, ni duplication ni double débit.
  Une récolte refusée pour inventaire plein ne doit pas épuiser le dépôt.
- Recettes reliées aux ressources réellement accessibles ; provenance des
  matériaux explicite. Proposition initiale : payer depuis l'inventaire porté,
  après retrait du coffre ; aucune ponction distante implicite. Revalidation à
  confirmation et annulation sans dépense comme dans P5.
- Équipements énergie/oxygène et ressources durables au-delà des huit caisses,
  avec règles d'amorçage et de reprise après panne pour éviter une aventure bloquée.
- Interface claire : capacité portée/stockée, quantités et refus de transfert,
  production/consommation, charge de batterie, stock et service d'oxygène.
- Sauvegarde de tous les inventaires/conteneurs, états des sources et équipements,
  réserves/identités/poses ; deux profils indépendants après fermeture, aucun
  handle/cache/UI transitoire sérialisé. Choisir la version du nouveau format lors
  de l'implémentation, sans migration ancienne requise pour le moment.

Définir capacités, limites de piles, recettes, puissances, débits et unités pendant
le cadrage concret ; ne pas présenter ces valeurs comme déjà décidées. Distinguer
stockage d'objets, énergie (W et J/Wh explicités) et réserve d'oxygène (lien à
l'autonomie P4 documenté). Les couleurs/lux/poussières de P8 ne sont pas une
puissance électrique : définir un modèle de gameplay utilisant les conditions
solaires et la météo. Définir les horloges de production, menus/inventaire/espace,
longues interruptions et absence ou présence de rattrapage hors application.
Préserver les règles P/T et la physique en temps réel, sans changement implicite.

Contrôles attendus : boucle neuve complète, capacités/piles pleines, quantités
insuffisantes, transferts partiels explicites et répétitions, construction/refus,
jour/nuit/tempête, arrêt/reprise énergie/O2, rebasage et conservation après M/R.
Vérifier restauration des contenus/charges/ressources après redémarrage et refus
sans mutation d'un fichier incohérent. Exécuter fmt/check/tests/Clippy/release,
inspection graphique de la boucle et des écrans dans un créneau autorisé, mesures
comparables des coûts et documentation des résultats/limites. Les vérifications
P8 déjà acquises restent distinctes de ces futurs contrôles P9.

Préserver contacts, marche, précision f64, chargement borné et séparation des
propriétaires. Pas de modules vides ou de réseau logistique spéculatif ; créer les
modules uniquement avec leur fonction réelle. P10 reste ultérieur. Les retours
visuels utilisateur sur P8 peuvent réorienter le travail s'il en donne.

## Contraintes et limites permanentes

Rust stable édition 2024, Bevy **exactement =0.19.1**, Cargo.lock versionné ; aucune
nouvelle dépendance ni Avian sans fonctionnalité réelle et compatibilité vérifiée.
SI f64 (DVec3/DQuat), origine soustraite avant conversion explicite f32. Ordre
Input→Simulation→Navigation→Rendering→Interface. Horloge orbitale Time<Real>,
deltas souris sans delta time ; marche4,5/course8/saut4m/s, gravité radiale,
pas1/120s et0,25s max/frame. Oxygène180s, dehors1s/s, recharge30s/s, menu suspend
la survie ; P/T ne modifie ni physique ni survie. Visites spatiales translatoires ;
surface tournante, forces d'inertie du repère toujours omises.

P8 garde des collines douces, une couronne visuelle approximative sans morphing,
une sphère orbitale P7 sans ce relief, pas d'ombres de terrain/nuages, de vent
physique, de pression locale ni de climat scientifique. Ce sont des limites
explicites, pas des fonctionnalités P9/P10 déjà réalisées.

**Ne pas reprendre le focus sans un créneau explicitement convenu dans le nouveau
chat.** L'accord reçu couvrait cette campagne, maintenant terminée. Pas d'écran
virtuel ni de contournement des règles de capture d'entrée. Scénarios GPU
successifs après compilation, sources/assets gelés, sorties neuves et aucune
capture pendant les mesures. Vérifier graphiquement si une session est disponible.

Poste confirmé : Ubuntu/X11, i7-6700K,32Go,GTX1060 3Go/NVK Mesa25.2.8. **Windows11
natif, RTX4050 6Go au mieux, CPU cible exact (« i5 12700K » à confirmer),1080p60 et
VRAM totale/pilote/staging non validés**. Ne pas extrapoler les résultats Ubuntu.

À la fin du prochain chat : actualiser critères/état ROADMAP, README, VALIDATION,
PERFORMANCE et cette passation selon les résultats réels ; fournir son lien.
