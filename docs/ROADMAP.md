# Roadmap Solar Survival

Le projet avance par étapes vérifiables vers une boucle de survie sur Mars, en préservant l'exploration spatiale et l'échelle physique 1:1. Cette roadmap est la référence pour choisir le travail du prochain chat. Les dates de livraison dépendront des résultats ; les étapes futures ne sont pas des fonctionnalités déjà réalisées.

Dernière mise à jour : 13 octobre 2026. Objectif **Mars traversable sur ses six faces avec chargement dynamique réalisé sur Ubuntu**. **P4, première boucle de survie jouable, terminé sur Ubuntu**. Prochaine fonctionnalité prévue : P5, ressources et construction du même abri. À la demande explicite de l’utilisateur, le cadrage d’une chaîne fixe P3.5 est remplacé par cette fonctionnalité complète. P3.1 à P3.4 restent les fondations et régressions. La passation détaillée est dans [NEXT_CHAT](NEXT_CHAT.md), les résultats dans [VALIDATION](VALIDATION.md).

## Fondations réalisées

Version 0.2 : exploration spatiale et catalogue embarqué, repères/origine flottante en f64, FPS martien avec marche/course/saut, gravité radiale, collisions, neuf tuiles locales et repère tournant conventionnel. Les tests actuels et les lancements Ubuntu sont consignés dans VALIDATION. Le menu principal, les options de session et les crédits sont disponibles ; P2 a consolidé le cycle complet, P/T et la reprise après interruption. P3.1 ajoute le contrat géométrique cube-sphere et sa démonstration isolée. P3.2 implémente une paire centrale opt-in avec contacts communs au rendu ; sa validation est consignée ci-dessous. P3.3 ajoute une transaction réversible centrale↔voisine +U, avec transport f64 et obstacles fixes dans le corps. P3.4 valide deux cartes franchement hors centre, avec admission calculée depuis leur recouvrement et refus avant allocation. Le parcours normal est désormais mondial : 90 tests, traversées des six faces/arêtes/coins, cache ≤13 patches et neuf lancements finaux code 0, comparaison de performance sans régression notable sur ce poste. P4 ajoute désormais l’oxygène et l’abri de session : 99 tests et parcours graphique ciblé de 56 s réussi, treize captures inspectées. Windows 11 reste à valider nativement.

## Étapes prévues

| Étape | État | Travail | Critères de fin |
|---|---|---|---|
| P1 Performances | Terminé sur le poste de développement | Protocole release fixe, temps de frame et distribution, mesures CPU/GPU, comparaison des caméras, résolution et matériaux | Mesures reproductibles conservées, limites du pilote identifiées, coût principal établi ou incertitude explicitée, prochaine action fondée sur les résultats |
| P2 Consolidation Mars et menu | Terminé sur Ubuntu de développement | Cycle jour/nuit complet inspecté, masque solaire sous horizon radial, P/T, suspension réelle de 12 s, deux séries M/G/O/U ; menu/options/crédits demandés par l’utilisateur | 54 tests réussis, 35 captures P2, essais X11 du menu et reprise ; limitations explicites dans VALIDATION |
| P3 Terrain au-delà du site | Terminé sur Ubuntu dans le parcours sphérique mondial | Six faces, arêtes/coins, chargement dynamique borné et repère transporté ; anciens modes en régression | Traverser des zones sans rupture de contacts ou de précision, voisinage chargé borné, budgets CPU/GPU vérifiés ; aucune génération de planète entière |
| P4 Première survie | Terminé sur Ubuntu | Oxygène réel, abri instanciable fixe dans Mars, HUD, échec et reprise | Consommer/recharger/échouer/R, menu/espace/P/T, pose et service conservés après rebasage/changement de patch, seconde instance sans logique spécifique |
| P5 Ressources et construction | Prochaine fonctionnalité après P4 | Une ressource récoltable, inventaire minimal, construction du **même OxygenShelter P4** | Récolter/stocker/construire/utiliser, instances fixes dans le corps, collisions et chargement local ; persistance de session, sauvegarde disque à P6 |
| P6 Version distribuable | Prévue | Sauvegarde/chargement, persistance et réglages graphiques complémentaires, validation Windows 11 et packaging Ubuntu/Windows | Sauvegarde testée, instructions de lancement, builds et essais natifs sur les deux plateformes |

Les étapes P3, P5 et P6 seront découpées en sous-étapes lorsque leur architecture concrète sera examinée. Une régression bloquante ou un résultat de profilage peut justifier une étape corrective avant la suivante ; la raison et les nouveaux critères seront inscrits ici. Le suivi de performance et les réglages restent transversaux après P1 ; refaire les mesures sur la cible avant de fixer la qualité finale. La validation Windows peut commencer plus tôt dès qu'un hôte est disponible. L'objectif de 60 FPS doit être mesuré sur le matériel cible.

## Matériel de développement et cible finale

Le poste actuel Ubuntu avec GTX 1060 3 Go et pilote NVK est un **poste de développement**, pas la référence finale de performance. La cible annoncée par l'utilisateur le 12 octobre 2026 est une **RTX 4050 et un Intel Core i5**. Le modèle exact du processeur, la RAM, la configuration de la carte, la résolution et les réglages seront consignés au moment des essais sur cette machine.

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
