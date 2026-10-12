# Roadmap Solar Survival

Le projet avance par étapes vérifiables vers une boucle de survie sur Mars, en préservant l'exploration spatiale et l'échelle physique 1:1. Cette roadmap est la référence pour choisir le travail du prochain chat. Les dates de livraison dépendront des résultats ; les étapes futures ne sont pas des fonctionnalités déjà réalisées.

Dernière mise à jour : 12 octobre 2026. Étape courante : **P3.2, premier franchissement local borné et contacts cube-sphere**. P3.1 est terminé sur Ubuntu de développement. La passation détaillée est dans [NEXT_CHAT](NEXT_CHAT.md), les résultats dans [VALIDATION](VALIDATION.md).

## Fondations réalisées

Version 0.2 : exploration spatiale et catalogue embarqué, repères/origine flottante en f64, FPS martien avec marche/course/saut, gravité radiale, collisions, neuf tuiles locales et repère tournant conventionnel. Les 63 tests actuels et les lancements Ubuntu sont consignés dans VALIDATION. Le menu principal, les options de session et les crédits sont disponibles ; P2 a consolidé le cycle complet, P/T et la reprise après interruption. P3.1 ajoute le contrat géométrique cube-sphere et sa démonstration isolée. La survie est inactive. Windows 11 reste à valider nativement.

## Étapes prévues

| Étape | État | Travail | Critères de fin |
|---|---|---|---|
| P1 Performances | Terminé sur le poste de développement | Protocole release fixe, temps de frame et distribution, mesures CPU/GPU, comparaison des caméras, résolution et matériaux | Mesures reproductibles conservées, limites du pilote identifiées, coût principal établi ou incertitude explicitée, prochaine action fondée sur les résultats |
| P2 Consolidation Mars et menu | Terminé sur Ubuntu de développement | Cycle jour/nuit complet inspecté, masque solaire sous horizon radial, P/T, suspension réelle de 12 s, deux séries M/G/O/U ; menu/options/crédits demandés par l’utilisateur | 54 tests réussis, 35 captures P2, essais X11 du menu et reprise ; limitations explicites dans VALIDATION |
| P3 Terrain au-delà du site | En cours : P3.1 terminé, P3.2 suivant | Contrat cube-sphere validé ; franchissement local et contacts à réaliser avant extension des repères/zones | Traverser des zones sans rupture de contacts ou de précision, voisinage chargé borné, budgets CPU/GPU vérifiés ; aucune génération de planète entière |
| P4 Première survie | Prévue | Oxygène, réserve, consommation en temps réel, recharge dans un abri et HUD | Boucle jouable consommer/recharger/échouer, indépendante de T, paramètres de gameplay documentés |
| P5 Ressources et construction | Prévue | Une ressource récoltable, inventaire minimal, placement d'un équipement utile | Récolte, stockage et placement persistants ; collisions et coût local validés |
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
