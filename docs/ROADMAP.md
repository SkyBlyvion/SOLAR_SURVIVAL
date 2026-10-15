# Roadmap Solar Survival

Le projet avance par étapes vérifiables vers une boucle de survie sur Mars, en préservant l'exploration spatiale et l'échelle physique 1:1. Cette roadmap est la référence pour choisir le travail du prochain chat. Les dates de livraison dépendront des résultats ; les étapes futures ne sont pas des fonctionnalités déjà réalisées.

Dernière mise à jour : 15 octobre 2026. Objectif **Mars traversable sur ses six faces avec chargement dynamique réalisé sur Ubuntu**. **P4, première boucle de survie jouable, terminé sur Ubuntu**. **P5, ressources et construction du même abri, réalisé sur Ubuntu**. **Phase moteur planétaire terminée sur Ubuntu** : LOD de résolution 32/16/8, index local, régressions/inspection et comparaisons CPU/GPU réalisés. **Caméra de jeu unique et ciel astronomique léger réalisés sur Ubuntu**, avec reprise visuelle après retour utilisateur. **Le MVP fonctionnel P1–P6 est réalisé sur Ubuntu** : plusieurs aventures nommées, répertoires indépendants, menu principal, catalogue de construction et HUD communs. La validation native Windows 11 reste ouverte comme travail transversal de distribution. **P7, qualité visuelle et textures, est clôturé sur Ubuntu le 15 octobre 2026**, avec huit planètes, matériaux locaux, profils/archive et mesures contrôlés. P8 est réalisé et vérifié sur Ubuntu : relief mondial artistique, paysage lointain et atmosphère dynamique demandée par l’utilisateur. La prochaine étape est P9, base et progression de survie, avec gestion d’inventaire et stockage ajoutés au périmètre utilisateur le 15 octobre. À la demande explicite de l’utilisateur, le cadrage d’une chaîne fixe P3.5 est remplacé par cette fonctionnalité complète. P3.1 à P3.4 restent les fondations et régressions. La passation détaillée est dans [NEXT_CHAT](NEXT_CHAT.md), les résultats dans [VALIDATION](VALIDATION.md).

## Fondations réalisées

Version 0.2 : exploration spatiale et catalogue embarqué, repères/origine flottante en f64, FPS martien avec marche/course/saut, gravité radiale, collisions, neuf tuiles locales et repère tournant conventionnel. Les tests actuels et les lancements Ubuntu sont consignés dans VALIDATION. Le menu principal, les options de session et les crédits sont disponibles ; P2 a consolidé le cycle complet, P/T et la reprise après interruption. P3.1 ajoute le contrat géométrique cube-sphere et sa démonstration isolée. P3.2 implémente une paire centrale opt-in avec contacts communs au rendu ; sa validation est consignée ci-dessous. P3.3 ajoute une transaction réversible centrale↔voisine +U, avec transport f64 et obstacles fixes dans le corps. P3.4 valide deux cartes franchement hors centre, avec admission calculée depuis leur recouvrement et refus avant allocation. Le parcours normal est désormais mondial : 90 tests, traversées des six faces/arêtes/coins, cache ≤13 patches et neuf lancements finaux code 0, comparaison de performance sans régression notable sur ce poste. P4 ajoute désormais l’oxygène et l’abri de session : 99 tests et parcours graphique ciblé de 56 s réussi, treize captures inspectées. P5 ajoute la récupération, l’inventaire et la construction confirmée : 110 tests. Windows 11 reste à valider nativement.

## Étapes du MVP P1–P6

| Étape | État | Travail | Critères de fin |
|---|---|---|---|
| P1 Performances | Terminé sur le poste de développement | Protocole release fixe, temps de frame et distribution, mesures CPU/GPU, comparaison des caméras, résolution et matériaux | Mesures reproductibles conservées, limites du pilote identifiées, coût principal établi ou incertitude explicitée, prochaine action fondée sur les résultats |
| P2 Consolidation Mars et menu | Terminé sur Ubuntu de développement | Cycle jour/nuit complet inspecté, masque solaire sous horizon radial, P/T, suspension réelle de 12 s, deux séries M/G/O/U ; menu/options/crédits demandés par l’utilisateur | 54 tests réussis, 35 captures P2, essais X11 du menu et reprise ; limitations explicites dans VALIDATION |
| P3 Terrain au-delà du site | Terminé sur Ubuntu dans le parcours sphérique mondial | Six faces, arêtes/coins, chargement dynamique borné et repère transporté ; anciens modes en régression | Traverser des zones sans rupture de contacts ou de précision, voisinage chargé borné, budgets CPU/GPU vérifiés ; aucune génération de planète entière |
| P4 Première survie | Terminé sur Ubuntu | Oxygène réel, abri instanciable fixe dans Mars, HUD, échec et reprise | Consommer/recharger/échouer/R, menu/espace/P/T, pose et service conservés après rebasage/changement de patch, seconde instance sans logique spécifique |
| P5 Ressources et construction | Terminé sur Ubuntu | Caisses épuisables, inventaire, recette à 12 alliage, preview orientable et construction du **même OxygenShelter P4** | Récolte→stock→confirmation→porte/recharge, admission sol/volumes, annulation sans coût, poses/contact/proxies et retours de session vérifiés |
| Optimisation moteur planétaire | Terminé sur Ubuntu, sphère sans relief | Profilage du rendu et streaming, **LOD adaptatif selon distance**, raccords mixtes et contacts fins | Mesures comparables, niveaux réellement différents, absence de fissure/oscillation aux faces/arêtes/coins, chargement et CPU/GPU bornés, régressions terrain/survie/construction |
| Caméra unique et ciel de surface | Réalisé et testé sur Ubuntu | Une Camera3d réutilisée espace/surface, ciel intégré à la vue locale (étoiles, Soleil et astres), composition commune | Une seule caméra/vue principale, ciel et occultations cohérents, transitions/UI/options/P4/P5 conservés, ressources bornées ; captures et comparaison release CPU/GPU à qualité comparable |
| P6 Sauvegardes et distribution du MVP | Fonctionnel sur Ubuntu ; validation native Windows encore ouverte | Sauvegarde/chargement complets, options persistées, archives avec assets/licences/lanceurs, essais natifs Ubuntu/Windows | Reprise après redémarrage et refus sans mutation validés, archive Ubuntu testée hors checkout ; build/essai/archive Windows natifs encore requis |

Les étapes P3, P5 et P6 seront découpées en sous-étapes lorsque leur architecture concrète sera examinée. Une régression bloquante ou un résultat de profilage peut justifier une étape corrective avant la suivante ; la raison et les nouveaux critères seront inscrits ici. Le suivi de performance et les réglages restent transversaux après P1 ; refaire les mesures sur la cible avant de fixer la qualité finale. La validation Windows peut commencer plus tôt dès qu'un hôte est disponible. L'objectif de 60 FPS doit être mesuré sur le matériel cible.

## Suite après le MVP — décision du 14 octobre 2026

La fin de P1–P6 termine la première boucle jouable, pas le développement du jeu. L’utilisateur demande de poursuivre avec une qualité de production conventionnelle et de remplacer les rendus provisoires par des textures sur mesure ou des ressources NASA/JPL. Cet objectif place **P7 en premier** ; la validation Windows restante ne bloque pas le travail visuel sur Ubuntu et reste explicitement à réaliser.

Les étapes suivantes décrivent l’ordre de travail retenu pour la reprise ; leurs détails de gameplay restent à préciser au moment de les développer, sans architecture vide ni promesse de date. [Sources et contrat visuel P7](ART_DIRECTION.md).

| Étape après MVP | État | Résultat attendu | Critères de fin |
|---|---|---|---|
| **P7 Qualité visuelle et textures** | **Réalisé et vérifié sur Ubuntu, 15 octobre 2026** | Première scène martienne aboutie : sol poussière/roche, abri et caisses avec matériaux cohérents ; cartes sourcées des huit planètes pour les vues spatiales | Détails à échelle métrique, textures stables aux rebasages/raccords, portes/ressources lisibles, jour/nuit/placement inspectés, crédits et conversion reproductible, budget textures et mesures avant/après, gameplay/profils conservés |
| **P8 Relief, horizons et atmosphère** | **Réalisé et vérifié sur Ubuntu, 15 octobre 2026** | Relief artistique jouable, paysage à plusieurs kilomètres ; ciel ocre/halo bleu/nuit étoilée, voiles de glace et poussière/tempêtes dynamiques | Terrain/contacts communs, raccords/LOD/rebasages, construction/survie et nouveaux profils vérifiés ; cache borné, mesures CPU/GPU et inspection. Sources et conventions distinguées. Format 2, sans migration des anciennes aventures, à la demande utilisateur du 15 octobre |
| **P9 Base, inventaire et progression de survie** | Cadré, non implémenté ; inventaire/stockage demandés le 15 octobre | Panneaux solaires, batteries, production/réserve d’oxygène, récolte durable, inventaire porté et coffres de base, interface et sauvegarde ; [cadrage](P9_BASE.md) | Boucle récolter→transporter→stocker/transférer→construire→produire→recharger ; piles/capacités bornées, transferts sans perte/duplication, accès local, recettes et coûts explicites, effet jour/nuit/tempête, réserves et contenus persistés par profil ; pas de migration ancienne requise pour le moment |
| **P10 Exploration et objectifs** | Prévu, périmètre à préciser | Raisons concrètes de partir de la base et d’y revenir : balises, sites à découvrir et objectifs progressifs | Un parcours complet avec risque/récompense et navigation lisible, découvertes/progression persistées, sans dépendre d’une téléportation spatiale pour l’expédition locale |

**Transversal à chaque étape :** ergonomie et retours utilisateur, sons utiles/retours d’action puis ambiance, remappage/accessibilité selon les besoins, stabilité et temps de chargement, compatibilité des sauvegardes et distribution Ubuntu/Windows. La première passe audio sera une fonctionnalité concrète avec ressources et réglages, pas l’activation d’une dépendance sans contenu. Relever et tester la cible RTX 4050/Core i5 avant d’annoncer une qualité ou 60 FPS.

Le premier livrable de P7 est une amélioration visible dans le vrai jeu, avec captures comparables et sources traçables. Une simple collection de liens, des textures plaquées sans échelle ou une refonte du moteur sans résultat à l’écran ne remplissent pas ce critère. Les détails NASA/JPL sont distingués des matériaux artistiques de proximité ; une mosaïque planétaire ne remplace pas une surface détaillée sous les pieds.

## Matériel de développement et cible finale

Le poste Ubuntu avec GTX 1060 3 Go/NVK reste le poste de développement. Précision utilisateur du **14 octobre 2026** : la **RTX 4050 6 Go est le haut de la cible**, le jeu ne doit pas exiger une machine plus puissante. Le CPU annoncé « i5 12700K » reste à confirmer, la référence 12700K étant un Core i7 ; RAM, puissance/configuration GPU, pilote et alimentation à relever. **Objectif de travail : 1080p/60 FPS sur la cible, à mesurer**. P7 commence avec textures locales 1 024²/petits objets 512² et cartes Terre/Mars 2 048×1 024, budget initial **128 Mio de nouvelles textures résidentes** avec niveaux de texture ; suivre séparément total/pics RAM/VRAM et coût des shaders. [Budgets et critères](ART_DIRECTION.md). Les 13 textures RGBA8/mipmaps représentent 40,168 Mio, vérifiés sur les descripteurs GPU ; total VRAM/staging et performance cible restent non validés.

Le profilage P1 établit les coûts et un protocole transférable ; les FPS actuels ne prédisent pas ceux de la cible. Les optimisations de confort peuvent être évaluées ici, mais les décisions de qualité graphique et l'objectif de 60 FPS seront validés sur la machine cible. Des FPS faibles sur le poste actuel ne bloquent pas à eux seuls P2 ou les fonctionnalités suivantes.

## Contraintes permanentes

Rust stable édition 2024, Bevy exactement 0.19.1, SI et pose physique f64, séparation simulation/rendu, ordre des GameSet et horloge orbitale indépendante des contrôles. Pas de nouvelle dépendance sans besoin concret et compatibilité vérifiée. Garder les assets réutilisés et le chargement local borné. La convention de rotation reste explicitement distincte d'une orientation IAU exacte.

Chaque étape significative passe par `cargo fmt --check`, `cargo check --locked`, `cargo test --locked` et `cargo build --release --locked`, puis un lancement graphique si possible. Les performances sont mesurées sans compilation ni autre scénario GPU concurrent. Windows n'est jamais déclaré validé depuis Linux.

## Passation entre chats

Au début : lire AGENTS, README, ASTRONOMY, VALIDATION, cette roadmap et NEXT_CHAT ; examiner l'état du workspace et établir le plan limité de l'étape. À la fin : mettre à jour le statut, les résultats et les limites ; préparer le prompt autonome du prochain chat dans NEXT_CHAT et fournir son lien. Une étape n'est terminée que lorsque ses critères sont satisfaits ou que ses limites et la suite nécessaire sont explicitement consignées.

## Décisions

- 12 octobre 2026 : conserver une roadmap dans le dépôt et une passation actualisée à chaque chat, à la demande de l'utilisateur.
- 12 octobre 2026 : profiler avant d'étendre le terrain ou la survie, car les relevés Ubuntu/NVK de surface sont de 22–24 FPS. Examiner les journaux de présentation avant d'attribuer un coût au gameplay.
- 12 octobre 2026 : distinguer le poste de développement GTX 1060/NVK et la cible RTX 4050/Core i5, conformément à la précision de l'utilisateur. Établir les mesures sans imposer 60 FPS comme préalable sur le poste de développement.

- 12 octobre 2026 : P1 terminé, huit essais release et 48 tests réussis. Voir [PERFORMANCE](PERFORMANCE.md). Les coûts observés orientent vers résolution, MSAA et deux caméras ; aucun réglage de production changé. Passer à P2 et conserver les optimisations comme décision à valider sur la cible.

## Premier périmètre P3.1

Au début de P3.1, `PatchKey` ne fournissait que le centre et une graine. L'étape a ajouté une géométrie f64 utilisable : direction d'un point sur une face, conversion inverse direction → face/coordonnées, voisins incluant les orientations aux arêtes, sommets communs à deux patches adjacents de même niveau. La génération de démonstration reste limitée à deux patches à la fois ; elle ne remplace pas le site jouable.

Critères P3.1 : aller-retour numérique stable, bords et coins partagés sans fissure, graines reproductibles, tailles et conventions SI documentées, géométrie/rendu séparés et ressources bornées. Le franchissement jouable, les changements de repère et les contacts appartiendront au sous-périmètre suivant, après ce contrat. Garder pendant P3.1 le FPS existant, les neuf tuiles et ±4 096 m ; aucune suppression de la borne avant validation des raccords physiques.

Le contrat est implémenté dans `planets/cube_sphere.rs`, séparé du terrain local, avec réexports conservant l'API historique. Conventions et budgets : [CUBE_SPHERE](CUBE_SPHERE.md). **P3.1 terminé sur ce poste** : neuf nouveaux tests, 63 au total ; fmt/check/test/release et Clippy réussis ; quatre captures de raccords inspectées ; régression du FPS tournant de 50 s, dix captures et code 0. Les deux lancements finaux ne contiennent aucun ERROR/WARN/panic.

| Critère P3.1 | État vérifié |
|---|---|
| Aller-retour numérique | Six faces, bords, directions extrêmes et niveau 24 réussis ; priorité X/Y/Z documentée |
| Arêtes et coins communs | Égalité exacte des positions, trois représentations de chaque coin, indices communs et incidence fermée du raccord |
| Graines et conventions SI | Mélange historique conservé, trois vecteurs de référence ; rayon en mètres et projection/taille non uniforme documentés |
| Géométrie/rendu séparés | Module f64 indépendant de l'ECS/GPU et exemple graphique isolé ; soustraction d'origine avant conversion f32 |
| Ressources bornées | Deux patches, 64 segments maximum ; démo à trois handles propres réutilisés aux transitions |
| Inspection et régression | Quatre raccords inspectés ; repos/saut/course, pause et sorties spatiales du FPS inspectés |

Limites : sphère sans relief, même niveau/résolution uniquement, pas de contacts cube-sphere, migration du repère ou passage jouable entre faces. Les trois faces au coin sont testées numériquement ; la démo n'en charge que deux patches. Les essais initiaux et leur correction sont conservés dans VALIDATION. Ni benchmark nouveau, ni validation Windows/RTX 4050.

## Périmètre suivant P3.2 : premier franchissement local

Après validation P3.1, introduire un parcours jouable **optionnel et borné** de deux patches adjacents de même niveau, d'abord sur une même face. Décider du niveau/résolution pour les contacts à partir des tailles métriques et du coût observé, plutôt que de reprendre automatiquement le niveau 15/32 segments de diagnostic. Partager la géométrie f64 autoritaire entre triangles rendus et contacts ; tout relief artistique doit être continu sur le raccord.

Conserver initialement un repère local tournant unique pour la paire : le franchissement doit être validé avant d'ajouter la migration d'origine ou le passage entre faces. Adapter seulement le fournisseur de support terrain nécessaire au contrôleur, sans refondre simultanément la navigation ou les deux horloges. Le volume du joueur doit interroger les triangles sous toute son empreinte, y compris lorsqu'elle chevauche les patches. La gravité, marche/course/saut restent en secondes réelles, avec la limite de 0,25 s.

Critères prévus : franchissements répétés dans les deux sens, repos sur le raccord, marche/course et saut/réception sans trou ni dérive de contact, concordance triangles/support, pose publiée après physique, ressources plafonnées et réutilisées, sortie et retour M/G/O/U, validations numériques/ECS et graphiques. Conserver le FPS actuel par défaut et sa borne ±4 096 m jusqu'à démonstration des nouveaux contacts. Pas de planète entière, LOD mixte, corps dynamiques, survie ni nouvelle dépendance. Les raccords jouables entre faces et la migration du repère feront l'objet d'un périmètre ultérieur.

- 12 octobre 2026 : P3.1 terminé : contrat numérique et quatre raccords graphiques validés, ressources bornées, terrain jouable préservé. Préparer P3.2 sur une même face et un repère unique pour valider les contacts avant les migrations et arêtes entre faces.

- 12 octobre 2026 : ajout du menu et de réglages de session pendant P2 à la demande explicite de l’utilisateur ; la persistance et les options avancées restent à P6.
- 12 octobre 2026 : P2 terminé sur ce poste : cycle complet et sorties répétées inspectés, défaut réel d’éclairage nocturne corrigé, interruption réelle et tests ciblés réussis. Passer au contrat borné P3.1. L’essai interactif long a renvoyé 143 après ses captures sans erreur moteur ; Quitter confirmé séparément avec code 0. Windows et la cible RTX 4050 restent non validés.


## Réalisation P3.2 : paire centrale et support commun

La paire opt-in se trouve dans `planets/cube_terrain.rs`, sélectionnée par `SOLAR_CUBE_SURFACE=1`. Choix : niveau 15, 64 segments, environ 207 m par patch et 3,23 m par cellule, sphère sans relief. Un repère tournant unique, maximum sous toute l'empreinte avec découpage des triangles, index spatial fixe, bornes intérieures et deux handles de mesh réutilisés. Le contrôleur conserve ses sous-pas et ses deux horloges. La justification métrique et les plafonds sont dans [CUBE_SPHERE](CUBE_SPHERE.md).

Les tests numériques/ECS couvrent les six faces centrales, comparaison de l'index avec tous les triangles, continuité du support sur le raccord, empreinte entière, marche/course dans les deux sens, repos et saut/réception, bornes même en l'air, 30/60/144 FPS, entrée éclairée à plusieurs époques, publication sans sous-pas, M/G/O/U et réutilisation de deux meshes. Le terrain historique demeure le défaut. **P3.2 terminé sur Ubuntu** : 71 tests, fmt/check/all-targets/Clippy/release réussis ; parcours final 88 s, code 0, 22 captures inspectées, six passages comptés et saut/réception, aucun ERROR/WARN/panic. Les corrections observées et les limites sont conservées dans VALIDATION.


| Critère P3.2 | État réel |
|---|---|
| Triangles communs support/rendu | Sommets f64 et indices du raccord partagés ; proxies locaux contrôlés sous 20 µm |
| Empreinte entière sur le raccord | Maximum par découpage, index contre recherche exhaustive ; différence avec hauteur centrale vérifiée |
| Marche/course, repos, saut/réception | Tests répétés dans les deux sens et 30/60/144 FPS ; parcours graphique final inspecté |
| Bornes et ressources | Quatre limites testées, deux limites X inspectées ; deux handles réutilisés, 8 385 sommets/16 384 triangles |
| Horloges, pose, transitions | Marche/saut en pause, course à ×1000, publication sans sous-pas et M/G/O/U testés ; transitions inspectées |
| Régression et coût | Terrain historique par défaut ; régression 50 s code 0, dix captures inspectées ; structures bornées, aucun benchmark nouveau |

Limites P3.2 : paire centrale, même face/niveau/résolution, sphère sans relief, repère unique. Pas de migration ou chargement au-delà, passage jouable entre faces, LOD mixte, corps dynamiques, nouvelle dépendance ou validation Windows. La correction de vitesse lors du bornage s'applique aussi à la limite historique ; le site par défaut conserve ses neuf tuiles et ±4 096 m. Les essais intermédiaires avec diagnostic de curseur et vitesse aux limites sont archivés, sans leur attribuer une validation finale sans erreur.

## Périmètre P3.3 réalisé : une migration sur la même face

Généraliser d'abord le fournisseur à une paire non centrale d'une même face, avec couverture conservatrice de l'empreinte malgré le quadrilatère projeté. Puis réaliser un **unique changement de repère tournant local**, opt-in et borné, vers une paire adjacente de même niveau/résolution. Préserver la pose relative au corps, la visée et la vitesse par transformations f64 ; les coordonnées des pieds, des yeux, de la gravité et du support doivent rester cohérentes. Vérifier au sol et en l'air, sans saut de caméra ou rupture de support. À la transition, deux patches résidents après remplacement ; si un chevauchement temporaire est nécessaire, en justifier et tester le plafond avant ajout.

Critères : conservation numérique de pose/visée/vitesse, aller/retour par la vraie marche/course, saut pendant une migration, continuité du support, assets réutilisés, sorties M/G/O/U, publication après physique et deux horloges conservées. Définir la propriété des obstacles pour éviter une duplication ou un déplacement lors du remplacement. Le passage entre faces reste une étape ultérieure ; pas de LOD mixte, planète entière, survie, Avian ou refonte du contrôleur. Les choix de niveau/résolution actuels restent à réévaluer avec un futur relief et sur la cible, sans supposer leur validité universelle.

- 12 octobre 2026 : P3.2 validé sur Ubuntu : première paire jouable centrale, contacts communs au rendu, erreurs observées de diagnostic/bornage corrigées, preuves conservées. Préparer P3.3 sur une même face avant les arêtes entre faces.


## Bilan P3.3 et périmètre suivant P3.4

P3.3 est **terminé sur Ubuntu de développement**, dans le parcours centrale↔voisine +U seulement. 76 tests, fmt/check/all-targets/Clippy/build release réussis, huit migrations de marche/course dont une en l'air, P/×1000 et M/G/O/U, 54 captures finales inspectées dans les trois campagnes code 0. Le lanceur initial interrompu code 143 pendant la fin de la régression historique est conservé séparément ; relance historique seule code 0. Résultats/limites dans [VALIDATION](VALIDATION.md), conventions dans [CUBE_SPHERE](CUBE_SPHERE.md).

| Critère P3.3 | Résultat et limite |
|---|---|
| Fournisseur non central et couverture entière | Rectangle certifié par tous les segments de frontière ; trente positions de paire testées sur six faces, sphère sans relief |
| Conservation de pose, visée, vitesse et gravité | Transfert f64 testé au sol/2 m/20 m ; base de visée transportée, publication après physique |
| Marche/course aller-retour et saut pendant migration | 30/60/144 FPS ; huit migrations graphiques dont une en l'air, réception et repos inspectés |
| Obstacles | Trois objets possédés par le site d'entrée, poses fixes dans le corps ; enveloppe AABB conservatrice <0,31 mm, limitée à cette petite inclinaison |
| Patches/assets et horloges | Deux patches après remplacement, deux handles réutilisés ; candidate et ancienne paire uniquement en CPU pendant transaction ; P/T et sorties conservés |
| Régressions et coût | P3.2 22 captures, historique 10 captures, codes 0 ; comptes bornés et dix mesures CPU ponctuelles, sans validation GPU/VRAM complète ou cible |

**P3.4 : cadrer puis étendre sur une même face.** Le fournisseur accepte déjà les paires décentrées, mais les seuils de migration et le couloir sont spécifiques au centre. Les tests près des coins observent un rectangle intérieur ~161×55 m contre ~412×205 m au centre : les seuils 100/−110 m ne peuvent pas y être copiés. Ce résultat justifie une étape bornée sur une même face avant les arêtes entre faces.

Choisir une paire franchement décentrée et une voisine de même niveau/résolution ; définir l'admission et l'hystérésis à partir de la couverture/du recouvrement certifiés. Préserver pose/visée/vitesse au sol et en l'air ; réévaluer le changement d'alignement de la boîte joueur et l'enveloppe des obstacles avant une inclinaison accrue. Garder le choix de candidate borné, sans construction répétée en cas de refus. Deux patches après remplacement et handles réutilisés ; justifier/tester tout plafond transitoire supplémentaire. Critères de fin : aller-retour et saut par contrôles ECS, conservation numérique, absence de rupture de support/caméra, couvertures/limites vérifiées et captures inspectées ; centrales/P3.2/historique en régression. Si ces critères ne passent pas, préparer la continuation de P3.4.

Aucun passage jouable entre faces automatique, refonte simultanée du contrôleur, planète entière, LOD mixte, relief scientifique, survie, corps dynamiques, Avian ou changement Bevy. Les budgets CPU/GPU plus complets et la validation cible restent transversaux ; P3 global n'est pas déclaré terminé. Windows reste à valider nativement.

- 12 octobre 2026 : P3.3 validé dans son parcours borné. Suite P3.4 sur même face, motivée par les couvertures non centrales observées et les seuils encore spécifiques au centre ; passages jouables entre faces différés.


## Bilan P3.4 et cadrage suivant P3.5

**P3.4 terminé sur Ubuntu de développement le 13 octobre 2026**, dans un parcours opt-in de deux cartes hors centre seulement. Git initial propre à `ea3a32b` (P3.3 déjà commité), changements P3.4 non commités. 82 tests, fmt/check/all-targets/Clippy/build release réussis. Quatre scénarios successifs code 0 et 76 captures inspectées, aucun ERROR/WARN/panic : hors centre et centrale huit migrations chacun dont une en l'air, P3.2 six franchissements, historique 50 s. [VALIDATION](VALIDATION.md), [contrat](CUBE_SPHERE.md), [preuves](p34-validation/).

| Critère P3.4 | Résultat réel |
|---|---|
| Paire franchement décentrée et couverture | u≈0,5/v≈0,5 ; rectangles certifiés ~281×124 m, plan vérifié sur six faces et empreintes aux altitudes extrêmes |
| Seuils et hystérésis | Recouvrement mesuré, volume d'admission commun, c±1 m, refus hors domaine avant allocation |
| Pose/visée/vitesse au sol et en l'air | Aller/retour six faces, sol/2/20/99 m, conservation f64 et support ; marche/course/saut 30/60/144 FPS |
| Boîte et obstacles réévalués | Coins de boîte déplacés ≤83,015 µm, plafond 0,25 mm ; enveloppe objets ≤0,227457 mm, plafond 0,5 mm ; proximité de contact exclue |
| Ressources et refus bornés | Deux paires CPU transitoires, une après préparation/remplacement, deux handles/entités ; milliers de refus sans reconstruction |
| Inspection et régressions | 76 captures inspectées, limites terminales et M/G/O/U ; centrales/P3.2/historique conservés |

Le changement complet d'orientation hors centre est plus petit que celui de P3.3, avec roulis pris en compte. L'étape ne démontre pas une grande inclinaison, un volume joueur orienté invariant ou une chaîne de cartes. La coexistence CPU est structurellement bornée, sans mesure exhaustive RAM/VRAM ou benchmark GPU. Windows et RTX 4050/Core i5 restent à valider.

**P3.5 : cadrer une courte chaîne sur la même face avant les faces.** Deux choix fixes ne suffisent pas à valider le choix de candidate à plusieurs étapes. Examiner d'abord les domaines certifiés et les budgets cumulés observés, puis choisir un nombre explicite de cartes (par exemple trois, à confirmer par les résultats) le long de U autour du site hors centre. Préparer leurs métadonnées sans garder toutes leurs géométries ; prouver le plafond CPU, y compris à l'entrée. Au plus une candidate construite après admission, hystérésis locale cohérente et refus sans reconstruction. Les obstacles appartiennent toujours à l'entrée : mesurer leur expansion cumulée, pas seulement celle de deux voisines. Réévaluer le reboîtage du joueur et exclure/refuser les contacts non couverts.

Critères à fixer avant implémentation : marche/course dans les deux sens à travers toute la chaîne, saut pendant chaque remplacement, réception/repos, absence de trou et conservation de pose/visée/vitesse, quatre bornes terminales actives, choix sans ping-pong/allocation répétée, deux patches/handles après remplacement et plafond transitoire testé, P/T et M/G/O/U, nouvelles preuves inspectées et régressions P3.4/centrale/P3.2/historique. Si le recouvrement ou le budget d'orientation échoue, garder les bornes et poursuivre P3.5 ; ne pas passer directement aux faces.

Ce découpage découle de la limite réelle « deux choix » de P3.4 et de l'absence de budget d'objets validé pour une orientation cumulée. Pas de migration ±V ou choix mondial implicite, faces automatiques, planète entière, LOD mixte, relief mesuré, Avian, survie ou refonte simultanée du contrôleur. P3 global reste en cours ; les performances cible restent transversales.

- 13 octobre 2026 : P3.4 borné hors centre validé ; poursuivre par le cadrage d'une chaîne courte sur la même face, avant les arêtes entre faces. Conserver les seuils historiques comme régression et mesurer les changements cumulés avant de les autoriser.


## Nouvel objectif P3 : Mars entière traversable

13 octobre 2026 : l’utilisateur autorise explicitement la révision des restrictions précédentes et demande une fonctionnalité complète, jouable, avec tests progressifs, f64 et stabilité préservés. Le cadrage d’une troisième carte fixe est abandonné avant intégration. Les paragraphes P3.5 précédents documentent l’ancienne décision ; ils ne limitent plus le nouveau parcours.

Le fournisseur `globe_terrain` charge un voisinage local de treize patches maximum sur n’importe quelle face, sans liste de cartes fixes. Le repère est transporté par rotation minimale à chaque déplacement local de 64 m ; la conservation f64 est indépendante des axes des faces. Les obstacles mondiaux utilisent un balayage de boîtes orientées exactes, remplaçant l’expansion AABB cumulée. Le rendu réutilise géométries et handles des patches communs. [Architecture et commandes](GLOBAL_TERRAIN.md).

Critères de fin : six faces et 24 arêtes dirigées traversables par marche/course/saut aller-retour, huit coins couverts et traversés ; empreintes complètes certifiées, pose/visée/vitesse conservées, stabilité au repos, P/T et M/G/O/U ; cache CPU/GPU borné et réutilisé sans génération par frame ; lancement graphique inspecté et régressions ; comparaison release des performances historique/mondial avec limites explicites. Aucune précision sacrifiée pour supprimer les anciennes bornes. Pas de planète entière résidente, LOD mixte, relief scientifique, nouvelle dépendance, changement Bevy ou validation Windows depuis Linux.

L’étape globale remplace la séquence de micro-parcours ; les améliorations suivantes seront choisies selon les usages et résultats observés, sans revenir à un nombre fixe de cartes comme objectif.


### Bilan du terrain mondial

Les critères fonctionnels sont satisfaits sur le poste Ubuntu : **90 tests**, dont 288 traversées aller-retour dirigées et 24 trajets de coin ; conservations f64, empreintes entières certifiées par aire, P/T et M/G/O/U, ressources et handles réutilisés. Fmt/check/all-targets/Clippy/release réussis. Neuf lancements du jeu code 0 sans ERROR/WARN/panic, **112 captures inspectées**, anciens parcours préservés. Comparaison historique–mondial–historique : au repos ~19,1 FPS dans les trois essais ; un vrai remplacement mesuré coûte au maximum 5,616 ms de navigation, sans pic de frame notable dans cette seule fenêtre. [VALIDATION](VALIDATION.md), [PERFORMANCE](PERFORMANCE.md).

P3 est terminé pour cette fonctionnalité **sphérique à résolution fixe** ; relief, LOD mixte et géométries complexes seront de futures extensions, pas des prérequis réintroduisant une chaîne bornée. Windows, cible RTX 4050/Core i5 et profilage exhaustif restent transversaux. La prochaine fonctionnalité complète prévue est P4 : consommation d’oxygène en secondes réelles, réserve/recharge dans un abri, HUD et issue jouable, en conservant le terrain mondial et l’exploration. Adapter les vérifications au changement réel, avec tests progressifs ; ne pas rejouer systématiquement toutes les campagnes antérieures sans raison.


## P4 → P5 : survie et bâtiment réutilisable

P4 implémente une réserve de 180 s, consommation en temps réel, recharge 30 s/s, alerte/HUD et échec verrouillé avec reprise R au premier abri. Menu : survie suspendue, calendrier/gravité maintenus. Espace : réserve/échec conservés sans recharge. M revient au site choisi une seule fois en session. Paramètres et architecture dans [SURVIVAL](SURVIVAL.md).

L’abri est un objet instanciable OxygenShelter, BuildingId stable en session, pose corporelle f64 indépendante des patches et de l’origine flottante. Son installation est séparée de la recharge. La même définition de pièces alimente collisions SAT et rendu ; une deuxième instance utilise exactement les mêmes services. Les tests couvrent le changement de patch/rebasage, la conservation de pose/contact/service, les proxies réutilisés/libérés et les sorties spatiales. P5 doit réutiliser ce type et install_shelter/publish_colliders pour le rendre constructible ; aucun remplacement par une zone spécifique au contrôleur. Inventaire, recettes, UI de placement et sauvegarde disque exclus de P4 ; persistance disque à P6.

## Terrain : LOD adaptatif futur selon la distance

13 octobre 2026, précision utilisateur : **treize patches chargés dynamiquement ne constituent pas un LOD adaptatif**. P3 est réalisé pour une sphère traversable à résolution fixe (niveau 15/32 segments). Le nombre de patches chargés et leur distance de graphe ne changent pas leur détail.

Une extension dédiée devra sélectionner plusieurs niveaux de subdivision/résolution selon la distance du joueur. Critères : détail fin et contacts f64 près du joueur, géométrie plus grossière au loin, raccords de niveaux mixtes sans fissures sur faces/arêtes/coins, transitions stables avec hystérésis, chargement/assets bornés et réutilisés, coût CPU/GPU mesuré pendant les changements, inspection graphique et régressions du terrain/survie. La stratégie de raccord devra être définie et validée avant de déclarer cette fonctionnalité terminée. Pas de génération globale/par frame ni de niveau dynamique seulement annoncé. Elle n’est pas réalisée en P4 ; son ordre de livraison sera choisi avec les besoins P5 et le profilage. Relief artistique ou scientifique reste un autre périmètre.


### Bilan P4 du 13 octobre 2026

**99 tests réussis**, fmt/check/all-targets/Clippy/release --locked réussis. Premier build du jeu 140,313 s ; contrôles finaux répétés après renforcement d’un test, binaire et sources du jeu identiques à la campagne graphique. Un parcours release de 56 s code 0, sans ERROR/WARN/panic, treize captures inspectées : porte/recharge, menu, course aller/retour et un rebasage, épuisement/R, espace et seconde instance. Tests à 30/60/144 FPS, delta de 12 s, M/G/O/U, murs, poses corporelles, proxies et recharge des deux abris dans le repère rebasé. [VALIDATION](VALIDATION.md), [preuves](p4-validation/).

P4 → P5 est prêt au niveau du type de bâtiment et de son service ; l’inventaire/recettes/placement/persistance disque ne sont pas réalisés. Prochaine fonctionnalité P5, en conservant l’oxygène et le même abri. Le LOD adaptatif selon la distance demandé par l’utilisateur est explicitement planifié dans l’extension terrain ci-dessus et **reste à implémenter**. Windows 11, RTX 4050/Core i5, autres DPI et mesures complètes de performance restent à valider ; aucun 60 FPS cible revendiqué.


## Bilan P5 et prochaine phase dédiée

13 octobre 2026 : P5 fournit une boucle complète récupération→inventaire→placement→construction→recharge. Huit caisses artistiques à six unités, abri à 12 alliage ; B/flèches/J-L/Entrée et annulation X/B/Échap. Même OxygenShelter, installation et service P4, poses corporelles DVec3/DQuat, stock/dépôts/bâtiments conservés en session. [CONSTRUCTION](CONSTRUCTION.md) expose dimensions, admission exacte des triangles, enveloppes SAT, budgets et limites.

| Critère P5 | État |
|---|---|
| Récolte et stock réels | E à portée, refus hors portée/à travers un solide, caisse épuisée sans doublon ni réapparition ; deux caisses financent un abri |
| Prévisualisation et manipulation | Six pièces partagées, vert/rouge et motif ; flèches, orientation J/L et visée, contrôles à 30/60/144 FPS |
| Admission et coûts | Extrema exacts sous le carré entier, certificat d’aire mondial ; refus relief/coverage/volumes/stock, revalidation à Entrée, annulation sans paiement et débit unique |
| Même abri utilisable | Marche par la porte, mur bloquant, recharge ; deux instances construites avec identités/poses distinctes, service commun |
| Session et chargement | Poses/contacts et recharge en carte rebasée, M/G/O/U et R, menu ; proxies partagés/réutilisés/libérés, stock et épuisement conservés |
| Validation adaptée | 110 tests, six contrôles Cargo réussis, diagnostic final 59,466 s code 0 et seize captures inspectées ; anciennes régressions numériques conservées, résultats détaillés dans VALIDATION |

Ressource limitée à 48 unités (quatre constructions supplémentaires), aucune sauvegarde disque, relief mondial ni LOD adaptatif livré par P5. Les caisses sont des collectibles sans collision de déplacement ; une admission conservative peut refuser un site dont le carré dépasse le terrain utilisable. Windows et la cible RTX 4050/Core i5 restent non validés.

**Décision d’ordre : consacrer maintenant une phase entière à l’optimisation du moteur planétaire**, avant sauvegarde/distribution P6. Elle répond à la demande utilisateur de traiter rapidement LOD et streaming après la première construction jouable ; les FPS de capture P5 ne constituent pas à eux seuls un benchmark ni une preuve de régression. Reprendre les baselines de PERFORMANCE avec fenêtres comparables et véritable streaming, isoler les coûts de présentation/deux caméras et du terrain. Le LOD doit ensuite utiliser réellement plusieurs niveaux suivant la distance, avec contacts fins près du joueur, raccords mixtes sans fissures, hystérésis/transitions stables, budgets et mesures CPU/GPU. Préserver constructions et ressources fixes dans Mars lors des changements. Les treize patches actuels restent à résolution fixe jusqu’à validation de cette extension.

## Bilan moteur planétaire — 13 octobre 2026

Résolutions intérieures **32/16/8 réellement choisies selon distance**, niveau spatial 15 conservé, sélection à l’entrée/rebasage avec hystérésis. Bords 32 communs et cellules en éventail : aucun T-junction, jupe ni reconstruction du voisin pour recoudre le bord. La zone proche garde exactement les triangles et contacts fins ; les constructions et ressources corporelles restent indépendantes du streaming. Index limité au carré ±72 m. Les handles communs restent conservés même quand le détail change. [Contrat actuel](GLOBAL_TERRAIN.md).

| Critère de la phase | Preuve |
|---|---|
| Plusieurs détails réellement choisis | Tests sur 150 directions et compteurs du jeu ; 32/16/8 résidents simultanément, changements de détails mesurés au rebasage |
| Raccords mixtes et transitions | Neuf combinaisons, faces/arêtes inversées/coins, topologie en disque et polylignes f64 exactes ; six vues inspectées montrant les grilles et transitions |
| Contacts fins, marche/course/saut | Hauteurs/extrema identiques au fournisseur fin dans le disque utilisable ; 288 traversées dirigées et 24 trajets de coins à plusieurs fréquences ; douze franchissements graphiques en l’air |
| Constructions et session conservées | Tests P4/P5 et parcours 59,650 s réussi : collecte, placement, même abri utilisable, rebasage, espace, menu ; R et M/G/O/U en régressions ECS |
| Stabilité et budgets | Bandes d’hystérésis testées ; pas de génération au repos, Arc partagés, ≤13 meshes et handles réutilisés/libérés ; index local observé ≤361 cases/10 613 références/5 375 triangles |
| Vérifications et visuel | 115 tests et six contrôles Cargo réussis, jeu et exemple release ; 58 captures finales inspectées, aucun ERROR/WARN/panic dans les trois parcours réussis |
| Performance et limites | Baselines monde/P5 avant changement, comparaisons finales fixe/adaptatif sur un même binaire et spans CPU/GPU ; résultats dans PERFORMANCE/VALIDATION. Pas de gain FPS extrapolé des captures, Windows/cible non validés |

Ce LOD adapte la résolution de patches de taille fixe ; il ne livre pas de quadtree, relief mondial, horizon complet, sauvegarde ou streaming asynchrone. Sur la sphère sans relief, les changements lointains restent sous 0,1 mm ; ce choix devra être réévalué avec du relief. Le chargement synchrone est borné au voisinage local et ses durées mesurées. Une extension n’est pas requise pour réintroduire un nombre fixe de cartes comme objectif.

Mesures finales clôturées : seize profils réussis (baselines puis comparaisons finales), gain CPU de remplacement ~6,1→1,1 ms sur deux cas contrôlés, pas de gain FPS notable. La demande utilisateur suivante place maintenant **la caméra unique et le remplacement du fond spatial en surface avant P6**. P6 reste **sauvegarde/chargement de la session jouable**, puis réglages et distribution avec validation native Windows. Aucun résultat Ubuntu ne vaut validation Windows ni garantie 60 FPS RTX 4050/Core i5.

## Historique : priorité caméra unique — demande du 13 octobre 2026

L’utilisateur demande une seule caméra et un remplacement du ciel astronomique en surface. Les deux vues actuelles coûtent ~51,5–51,7 ms dans le parcours mesuré ; retirer la vue spatiale donne ~32,2 ms mais enlève du contenu. Cette observation motive une fonctionnalité de rendu complète avant P6 : caméra unique réutilisée espace/surface, fond de ciel intégré à la même vue et représentations célestes légères pilotées par la simulation f64. Les choix précis restent à implémenter et mesurer avec Bevy exactement 0.19.1.

Critères : une seule caméra de jeu/vue principale, pas de caméra de ciel cachée hors écran par frame ; étoiles/Soleil/astres orientés et dimensionnés correctement, horizon et occultation par le terrain/abris ; visites spatiales, M/G/O/U/R, menus/options/UI et P4/P5 conservés ; assets réutilisés et budgets bornés ; tests/captures inspectées et répétitions release CPU/GPU à résolution/MSAA/qualité comparables. Détails autonomes dans [NEXT_CHAT](NEXT_CHAT.md). La skybox et les couleurs atmosphériques sont des moyens de présentation à documenter, sans nouvelle prétention scientifique ni garantie de 31 FPS.

Mise à jour documentaire uniquement : le code conserve ses deux caméras, les 115 tests et les anciennes validations décrivent l’implémentation LOD existante. Aucun nouveau test Cargo/GPU effectué pour cette décision d’ordre. Le LOD reste réalisé ; P6 est différé jusqu’au bilan de cette fonctionnalité.


## Bilan caméra unique et ciel — 13 octobre 2026

Une seule GameCamera/Camera3d, réutilisée entre visites spatiales et surface ; pose/projection/couches/options/UI adaptées avant extraction. Un triangle de fond dans la même passe opaque, panorama statique et neuf astres pilotés par directions/rayons angulaires f64 ; aucun second render target ou capture du ciel par frame. [Architecture et paramètres artistiques](SURFACE_SKY.md). La première apparence a été rejetée par l'utilisateur puis reprise : étoiles fines irrégulières, palette plus claire, brume/halo discrets et sol mat supprimant la bande claire rasant l'horizon.

| Critère | État et preuve |
|---|---|
| Caméra/vue unique | Même identité active à toutes les poses ; une caméra extraite côté renderer, pas de vue de ciel séparée |
| Directions, angles, visée et calendrier | Calcul SI/f64, tests à trois JD et après rebasage, P/T et jour/nuit inspectés |
| Masquage et horizon | Profondeur infinie derrière les objets, toit/paroi inspectés ; horizon radial/fond sans support, brume 40–60 m, simplifications documentées |
| Intégration | M/G/O/U et retours, R, menu/reprise et H inspectés ; options et gameplay P4/P5 testés |
| Ressources | Un triangle, un panorama 32 Mio et un matériau/uniforme 608 octets constants, réutilisation testée |
| Validation | 120 tests, six commandes Cargo réussies ; ciel/P5 code 0, 40 captures finales inspectées |
| Mesures | Sept profils avant/après réussis ; deux répétitions 1440×900/MSAA ×4, vrai streaming, CPU/GPU et no_sky ; ~51→29 ms au repos sur le poste |

[VALIDATION](VALIDATION.md) conserve aussi les essais intermédiaires ; [PERFORMANCE](PERFORMANCE.md) distingue gain de refonte, coût du fond et changements visuels. La portée de brume est plus courte et le panorama de surface diffère du champ spatial ; pas d'atmosphère scientifique ou d'horizon global géométrique revendiqué. Les captures ne vérifient pas chaque frame par vidéo. Le retour utilisateur initial a été traité ; le rendu révisé n'est pas présenté comme une approbation esthétique donnée par l'utilisateur.

**Suite P6 : sauvegarde/chargement de session complète**, sans toucher à Bevy, aux horloges ou au LOD. Ajuster encore l'apparence si un nouveau retour utilisateur le demande. Windows natif/cible restent non validés ; aucun 60 FPS universel revendiqué.


## Première implémentation P6 à emplacement unique — 13 octobre 2026 (corrigée ci-dessous)

L’utilisateur corrige explicitement une reprise mal cadrée : **l’étape réelle est P6**. HEAD de départ `22efa72` possède déjà P4/P5, LOD et caméra unique ; la passation P4 reçue est conservée dans [p6-validation/NEXT_CHAT-before](p6-validation/NEXT_CHAT-before.md). La vérification P5 intermédiaire est conservée séparément, sans être présentée comme un nouveau développement P4/P5.

P6 ajoute un emplacement local versionné, contrôles complets avant mutation, écriture temporaire avec secours `.bak`, boutons/menu F5/F9 et reprise en mémoire des données f64. Sauvegarde explicite de l’oxygène/échec, stock/caisses, abris/identités, pose espace/surface, carte active/obstacles/site de retour, calendrier et options. Les caches de terrain/contacts sont reconstruits ; caméra unique conservée. [Contrat](PERSISTENCE.md), [preuves et limites](VALIDATION.md).

| Critère de persistance | État |
|---|---|
| Données autoritaires conservées | Poses corporelles et identités stables, bilan d’alliage/dépôts, oxygène/échec, visée/vitesse et carte après rebasage, options/horloge |
| Réutilisation de P4/P5 | Même abri, service et colliders ; aucun paiement ou récolte au chargement, prochaine identité monotone |
| Reprise de partie | Deux processus successifs avec fermeture normale entre F5 et F9 ; recharge, espace, échec/R et refus corrompu vérifiés |
| Refus sûrs | Fichier absent/corrompu/incompatible/trop volumineux, indices/quaternions/poses invalides ; aucune mutation partielle ; écriture refusée sans perte de copie |
| Menu et rendu | Boutons/raccourcis depuis le menu principal, statut/chemin visibles, preview annulée, paramètres de la même caméra réappliqués |
| Packaging | Assets voisins du binaire, manifeste/licences/lanceur, tar.gz Linux et zip Windows natif ; test Ubuntu extrait hors checkout |
| Suite P6 | Validation native Windows 11 et RTX 4050/Core i5, archive Windows et vérification d’installation sur la cible ; restent non réalisées depuis Linux |

Le format est local et versionné, sans cloud/réseau, nouvelle dépendance, autosave ou rattrapage du temps hors application. Le secours protège un arrêt du processus entre renommages ; pas de garantie de toute coupure matérielle ni d’écriture concurrente de plusieurs instances. Les paramètres de qualité existants sont persistés ; aucun réglage nouveau ajouté sans besoin observé. **Ne pas déclarer P6 distribué sur les deux OS terminé avant les essais Windows.**


## P6 : aventures multiples et reprise de l’interface — 14 octobre 2026

L’utilisateur exige plusieurs nouvelles aventures avec un nom de répertoire lors de la création. La première implémentation à fichier unique ne remplissait pas ce besoin ; elle reste une preuve intermédiaire. Le produit utilise désormais `saves/Nom/session.solar` et un secours par aventure. Nouvelle aventure demande un nom, écrit la session initiale et réinitialise joueur/stock/dépôts/abris/site/temps. La liste de profils permet de choisir la partie à restaurer. Les profils ne sont pas des alias d’une seule session ; Alpha et Bravo sont vérifiés séparément après redémarrage.

La même demande ajoute une refonte concrète du menu principal et du HUD, et un menu de construction en jeu. Interface de jeu classique : nouvelle aventure/reprise/chargement/options/quitter, champ de nom obligatoire, sélection explicite, erreurs sans écrasement, texte et boutons alignés. Palette/fonds/typographie communs ; surface dégagée, navigation N et télémétrie H séparées des instruments vitaux. B ouvre le catalogue avec fiche, coût et stock, puis le placement existant après Prévisualiser/Entrée.

| Critère | Réalisation |
|---|---|
| Plusieurs aventures | Création nommée, répertoires indépendants, liste et sélection, réinitialisation complète de progression |
| Isolation et reprise | Stocks/épuisement/identités/poses/oxygène propres à chaque aventure ; changement sauvegardant le profil quitté, redémarrage réel |
| Noms et erreurs | Nom obligatoire, limites et noms réservés, doublons refusés, aucun écrasement d’un profil existant ; partie active conservée si le chargement échoue |
| Menu principal | Nouvelle partie, saisie de nom, liste paginée et chargement explicite, reprise/options ; captures aux deux tailles de fenêtre |
| Construction en jeu | Catalogue B manipulable à la souris/Entrée, disponibilité/coût réels, fermeture sans dépense ; oxygène actif, pilotage bloqué |
| HUD | Palette et hiérarchie communes, instruments séparés, navigation et télémétrie à la demande ; lisibilité vérifiée sur captures |
| Suite de distribution P6 | Native Windows 11/RTX 4050/Core i5 et archive Windows restent à valider ; aucun résultat Ubuntu ne les remplace |

[Contrat actuel](PERSISTENCE.md), [preuves réelles et limites](VALIDATION.md). Le code reste Rust stable 2024/Bevy exactement 0.19.1, sans dépendance ajoutée. Les validations d’apparence ne sont pas présentées comme une approbation esthétique déjà donnée par l’utilisateur.

Validation finale du périmètre profils/interface : **130 tests**, six contrôles Cargo réussis, quatre processus graphiques code 0 et **64 captures inspectées** à 1440×900 et 1280×720, cette dernière depuis l’archive extraite hors checkout. Aucune erreur/alerte/panic ; profils Alpha/Bravo indépendants après fermeture, erreur Broken refusée avec Alpha conservé. Archives/journaux/empreintes dans [VALIDATION](VALIDATION.md). Le jeu n’a pas changé après le build final ; les anciens résultats à fichier unique restent intermédiaires. La validation/distribution Windows native demeure ouverte en transversal ; l’objectif suivant demandé par l’utilisateur est P7, qualité visuelle et textures.

## P7 intégré — clôture de cette passe, 14 octobre 2026

Sol artistique à période de 4 m fixé au corps, abri/panneaux/caisses partagés, plaque O₂, cartes des huit planètes et crédits **Chef de projet et orchestrateur : SkyBlyvion**. Sources/conditions/projections/empreintes et exports offline documentés dans [CREDITS](../assets/textures/CREDITS.md). Bevy/Cargo/physique/profils inchangés. Six commandes Cargo réussies, **134 tests**. Archive Ubuntu assemblée et empreintes vérifiées hors checkout.

**P7 ne remplit pas encore tous ses critères de fin.** 34 captures du sol/abri/jour/nuit/rebasage/Terre/Mars inspectées avant l’extension aux six autres planètes. Les derniers scénarios de performance échouent lors de marches injectées perturbées par le focus : l’utilisateur travaille sur le même bureau et confirme l’interférence. À sa demande, arrêter les campagnes prenant souris/focus et finir rapidement la livraison existante ; aucune adaptation de fonctionnement retenue. Six autres planètes/crédits à inspecter dans le jeu, mesure finale comparable et lancement de l’archive/profils à terminer dans un créneau adapté. Les essais d’écran virtuel rejetés sont conservés comme diagnostics, pas comme validation. [État exact](p7-validation/NOTES.md). P8–P10 restent prévus ; traiter ces derniers contrôles P7 avant P8.

## Clôture P7 — 15 octobre 2026

Reprise des contrôles ouverts à eeaff80, avec créneau de focus autorisé. Galerie des six autres planètes/crédits inspectée ; couture Jupiter réellement détectée puis corrigée dans l’export offline (maître inchangé). Nouvelle archive finale extraite et lancée : 25 vues sans erreur, 747 fichiers/empreintes vérifiés. Profils P7 à720p dans deux processus, construction/placement/contact/recharge et rebasage contrôlés ; 98 captures inspectées sur26planches pendant cette reprise, en plus des34vues conservées du14. Les134tests/sixcontrôles antérieurs portent toujours sur les mêmes Rust/shaders/Cargo ; deux tests textures reréussis après retouche.

Référence P6 actuelle + deux P7 sans captures + un GPU séparé : surcoût moyen observé environ2,4ms sur le poste GTX1060. Budget textures utile40,168Mio/128Mio. L’échec initial à captures reste conservé ; relance avec focus observé réussie. Aucun écran virtuel ni contrôle spécial ajouté. [Bilan exact](p7-validation/2026-10-15/NOTES.md).

**État de passation avant P8, remplacé par la livraison ci-dessous :** relief et horizons étaient à réaliser. La décision utilisateur du 15 octobre dispense de préserver la compatibilité des anciennes aventures. Windows natif, RTX4050/CPU exact, objectif1080p60 et VRAM totale/staging restent ouverts en transversal. Ne pas présenter l’approbation esthétique utilisateur comme acquise.


## P8 livré sur Ubuntu — 15 octobre 2026

Le périmètre inclut la demande atmosphérique de SkyBlyvion : jour ocre/poussiéreux,
halo bleu au lever et coucher, nuit étoilée, voiles de glace d'eau, poussière
variable et tempêtes occasionnelles. [Paramètres/sources/architecture](MARS_ENVIRONMENT.md).
Relief artistique f64 commun au rendu et aux triangles de contact, paysage raccordé
par couronne jusqu'au-delà de 11 km, visibilité calme 5–8,5 km et tempête 150 m. Pas de
terrain mondial entièrement généré ni nouvelle dépendance. Préservation des
anciennes sauvegardes explicitement retirée du critère : **format 2 sans migration**,
anciens fichiers refusés et laissés intacts.

Critères vérifiés : coutures six faces/LOD mixtes et contour de couronne sans
fissures topologiques, couverture/support et admission, marche/course/saut/rebasages,
services des bâtiments et profils neufs après redémarrage.139 tests,fmt/check/Clippy/
release ;54 captures finales ciel/profils inspectées. Une première version de
nuages trop géométriques corrigée avant les mesures. Le smoke historique demande
un dossier docs pour ses captures ; essai initial en erreur IO conservé, relance
après préparation réussie sans modifier le jeu. [Preuves et limites](VALIDATION.md).

Cache ≤13 patches +1 couronne de6 400 sommets /11 520 triangles ; ni image ni passe
nouvelle. Comparaison P7→P8 release sans captures : surcoût moyen 1,59–1,90 ms/frame
selon phase, trois runs P8 codes 0 et archive Ubuntu vérifiée. [PERFORMANCE](PERFORMANCE.md).
Windows 11 natif, cible RTX 4050/CPU exact,1080p60 et VRAM totale/staging restent
ouverts ; validation artistique utilisateur distincte. Le relief reste doux,
la couronne sans morphing temporel et la météo sans climat/ombres volumétriques.
**Objectif suivant : P9, énergie→oxygène→autonomie avec inventaire et stockage**, à
implémenter et valider dans la prochaine étape, sans présenter P9/P10 comme acquis.


Clôture Git P8 du 15 octobre 2026 : code, documentation et preuves de validation
regroupés en commit à la demande utilisateur. P9 reste l’objectif suivant,
sans modification fonctionnelle supplémentaire pendant cette clôture.


## Cadrage P9 actualisé — 15 octobre 2026

À la demande utilisateur, la **gestion d'inventaire et le stockage de base**
complètent les panneaux solaires, batteries, générateur/réserve d'oxygène, récolte
durable, interface et sauvegarde. Le compteur d'alliage actuel ne remplit pas ce
nouveau critère : inventaire porté multitype, piles organisables, capacité finie,
coffres constructibles à contenus propres et transferts de proximité sont à livrer.

[P9_BASE](P9_BASE.md) définit la boucle complète et les contrôles de conservation
des quantités, accès/capacités, recettes, énergie/oxygène et persistance de chaque
conteneur/profil. Capacités, coûts, unités et règles temporelles seront fixés et
vérifiés pendant l'implémentation. P9 reste **non implémenté** ; cette préparation
modifie uniquement la documentation. Les résultats P8 et limites Windows/matériel
restent inchangés ; pas de migration des anciennes sauvegardes requise pour le moment.
