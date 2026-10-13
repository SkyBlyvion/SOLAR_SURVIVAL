**# Prochain chat Solar Survival**



**\*\*P4, première boucle de survie jouable, est réalisée sur Ubuntu.\*\*** Prochaine fonctionnalité prévue : **\*\*P5, ressources et construction du même abri\*\***. Mise à jour : 13 octobre 2026.



**## Prompt autonome de reprise**



Nous poursuivons Solar Survival dans \`/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL\`. Lis AGENTS.md, README.md, docs/ASTRONOMY.md, docs/CUBE_SPHERE.md, docs/GLOBAL_TERRAIN.md, docs/SURVIVAL.md, docs/VALIDATION.md, docs/ROADMAP.md, docs/PERFORMANCE.md et docs/NEXT_CHAT.md. Examine Git et le code puis avance vers une fonctionnalité complète, jouable. Départ réel du chat P4 : HEAD \`07e7af8\` (terrain mondial déjà commité), seul NEXT_CHAT modifié par l’utilisateur. Les nouveaux changements P4 et preuves sont **\*\*commités\*\*** :  Passation utilisateur précédente conservée dans \`docs/p4-validation/NEXT_CHAT-before.md\`. Aucune preuve historique supprimée/écrasée.



**## Instructions utilisateur prioritaires**



Accélérer le développement : fonctionnalités complètes et progrès fonctionnels mesurables, sous-étapes techniques libres, tests progressifs pour détecter vite les erreurs. Les anciennes restrictions de roadmap/chaîne fixe P3.5 ne limitent plus le parcours mondial réalisé. Maintenir f64, stabilité moteur et vérifications adaptées au changement réel ; ne pas rejouer systématiquement toutes les campagnes graphiques anciennes.



Mettre à jour la documentation, README, ROADMAP, VALIDATION et cette passation à la fin de chaque passe.



**\*\*Treize patches chargés dynamiquement ne constituent pas un LOD adaptatif.\*\*** Le terrain actuel est à résolution fixe. Il faudra implémenter plusieurs niveaux de détail selon la distance du joueur, raccords de niveaux mixtes sans fissures, contacts fins près du joueur, transitions stables et budgets CPU/GPU. Cette extension est inscrite dans ROADMAP ; elle n’est pas réalisée par P3/P4. Choisir son ordre avec les besoins réels et le profilage, sans prétendre qu’elle existe déjà.



**## État réel**



Application 0.2, Rust stable édition 2024, Bevy **\*\*exactement 0.19.1\*\***, aucune nouvelle dépendance ni changement Cargo.toml/Cargo.lock. **\*\*99 tests\*\*** : 32 internes, 21 fondations, 15 surface historique, 9 géométrie cube-sphere, 4 contacts, 3 migrations, 4 hors centre, 4 mondiaux, 7 survie. Exploration spatiale, menu/options/crédits, FPS mondial et oxygène actifs.



**### P4 : survie jouable**



Réserve initiale/maximale **\*\*180 s\*\***, consommation **\*\*1 s/s réel\*\*** dehors, recharge **\*\*30 s/s réel\*\*** dans un abri, alerte à **\*\*30 s\*\***, aucune consommation simultanée à la recharge. Valeurs de gameplay exprimées en secondes de réserve, sans modèle physiologique/pression. Time\<Real> après Navigation, indépendant de P/T et du plafond physique 0,25 s. Menu : consommation/recharge suspendues, orbite et gravité continuent. Espace M/G/O/U : réserve et échec conservés, sans consommation/recharge. À zéro, expédition terminée et pilotage bloqué/curseur libre ; **\*\*R\*\*** revient au premier abri avec une réserve pleine. R inactif avant échec et dans le menu. H masque la télémétrie, conserve le HUD vital et l’issue visibles.



Premier abri turquoise **\*\*six mètres devant l’arrivée\*\***, porte ouverte de 2,4 m côté joueur (+Z du bâtiment). Marcher dedans pour recharger. Le retour normal M retrouve le site fixé à la première arrivée de session, éventuellement nocturne ; il ne replace pas le bâtiment ni ne recharge la combinaison. Les scénarios historiques/profilage désactivent survie et assets/HUD de bâtiments pour conserver leurs scènes. \`SOLAR_MARS_GLOBAL=0\` en partie normale conserve un abri fonctionnel sur le terrain artistique.



**### Bâtiment P4 → P5 : réutilisation obligatoire**



\`buildings::Buildings\` possède des objets \`OxygenShelter\`, identité \`BuildingId\` monotone stable en session, position DVec3 relative au centre de Mars et orientation DQuat dans les axes du corps tournant. **\*\*Pose indépendante des patches, origine flottante, rebasages et JD.\*\*** Installation \`install_shelter(position_m, orientation)\` séparée de \`survival::Oxygen\` et du service \`shelter_at\`. \`install_first\` est seulement le choix initial près de l’arrivée ; le même type fonctionne à plusieurs positions sans logique spéciale.



Six pièces communes au rendu/contact : côtés, fond, deux jambages, toit. Pas de dalle : le terrain fait office de sol. Huit coins du volume joueur exigés dans le volume utile, aucune recharge à travers un mur/depuis le toit. Six BodyBox f64 fixes par abri publiées via \`publish_colliders\` à l’installation seulement. Contacts SAT continus sur 15 axes et support triangulaire exact, convertis depuis le corps vers la carte active. Rebasage sans mutation/duplication de pose.



\`rendering/buildings.rs\` charge les six proxies de chaque abri à moins de 120 m, avec un mesh de cube partagé et deux matériaux. Entités conservées pendant les rebasages locaux, supprimées hors portée/en espace, recréées avec la même identité d’objet au retour. Différence bâtiment−yeux soustraite dans le corps en f64, puis rotation et conversion explicite f32. CPU : données de tous les abris en session, requêtes linéaires avec exclusion conservative ; index spatial à évaluer selon le vrai nombre d’instances P5, sans abstraction spéculative.



En P5, **\*\*rendre constructible ce même OxygenShelter et réutiliser installation/contact/service\*\***, puis ultérieurement persistant. Ne pas réécrire l’abri comme zone liée au patch/contrôleur. Inventaire, recettes, placement joueur et sauvegarde disque **\*\*non réalisés en P4\*\*** ; anciens Vitals/Inventory préparatoires inutilisés remplacés par la fonctionnalité effective.



**### Terrain mondial et physique préservés**



M normal ouvre une Mars sphérique traversable sur les six faces/arêtes/coins sans borne de site ou couloir. SI/1:1 : les trajets planétaires restent longs. \`SOLAR_MARS_GLOBAL=1\` force le monde ; \`=0\` ouvre le terrain artistique historique de neuf tuiles/±4 096 m. P3.2/P3.3/P3.4 et smoke/P2 restent régressions, ne pas mélanger les scénarios.



Niveau **\*\*15/32 segments\*\***, ≤13 patches à distance de graphe ≤2, sans relief ni **\*\*LOD adaptatif\*\***. 1 089 sommets/2 048 triangles par patch, sommets de bord/coin f64 exactement communs via PatchKey::vertex. Géométries Arc partagées, index XZ 8 m ; empreinte entière certifiée par aire des triangles découpés (tolérance 10⁻⁹ m², demi-largeur ≤1 m). Joueur 0,3 m de demi-largeur. Disque de couverture 66 m, rebasage après 64 m ; garde-fou local, jamais borne mondiale.



Transport minimal des normales, indépendant des axes de face ; pieds/yeux/vitesse/visée f64 et état conservés. Boîte joueur alignée sur la carte active, pas de volume orienté invariant revendiqué. Trois objets d’entrée restent fixes dans le corps, OBB exactes en monde. Un mesh par patch dans son repère propre, soustraction d’origine f64 ; meshes/handles communs conservés, slots sortants recyclés, assets terrain libérés à la sortie. Pas de génération de planète entière ou par frame. ≤13 meshes/14 157 sommets/26 624 triangles résidents ; pendant transaction ≤26 occurrences/53 248 triangles locaux, géométries communes partagées. Budgets structurels, sans RAM/VRAM exhaustive.



**### Validations et preuves**



Six contrôles Cargo finaux --locked réussis : fmt/check/all-targets/test/Clippy/release. Premier build du jeu **\*\*140,313 s\*\***, validation finale incrémentale réussie avec binaire inchangé. **\*\*99 tests\*\***. P4 ajoute neuf tests : temps/menu/espace/failure/R, marche/porte/mur/recharge, seconde instance, changement de patch/rebasage, recharge des **\*\*deux abris dans la carte déjà rebasée sans enter_home\*\***, poses/contact/proxies/réutilisation/libération. 30/60/144 FPS et delta de 12 s vérifiés.



Un lancement graphique release **\*\*56,469 s\*\***, **\*\*code 0\*\***, aucun ERROR/WARN/panic, **\*\*13 captures inspectées\*\*** via quatre planches et originaux 00/01/06. Consommation/recharge/menu, course aller-retour avec **\*\*un rebasage\*\***, échec/R, M spatial puis retour fixe, seconde instance et recharge. Le diagnostic prépare réserves 80/1/60 s et départ devant second abri, sans changer les taux de production ; pas de construction joueur ni d’attente continue de trois minutes. Rebasage CPU isolé 8,218 ms, captures actives, hors meshes/GPU, **\*\*pas un benchmark\*\***.



Preuves dans \`docs/p4-validation/\` : build/graphics/final, journaux et empreintes, captures/télémétrie/inspection. Après campagne, seul tests/survival.rs renforcé ; sources du jeu et binaire inchangés, contrôles finaux répétés. Le premier test renforcé demandait carte inchangée exactement au-delà du seuil de 64 m : rebasage automatique normal, point déplacé d’un mètre dans l’abri, test réussi. Tous les essais conservés dans NOTES. Pas de deuxième campagne graphique du même binaire pour cette seule correction de test.



Historiques mondiaux préservés dans \`docs/global-validation/\` : 288 traversées dirigées, 24 trajets de coins, neuf lancements code 0, 112 captures inspectées, profils et budgets. Au repos historique/mondial \~19,1 FPS ; vrai remplacement mesuré sur 54 s, navigation max 5,616 ms. Ces résultats à résolution fixe ne valident ni LOD adaptatif ni performances P4 universelles.



Poste Ubuntu/X11 i7-6700K/GTX 1060 3 Go/NVK/Mesa 25.2.8. Windows natif et RTX 4050/Core i5 **\*\*non validés\*\***, CPU exact/RAM/pilote/réglages cible à relever. Pas de nouveau test des périphériques OS, suspension réelle, autres DPI, cycle P2 complet, benchmark P4 ou RAM/VRAM exhaustive.



**## Objectif suivant : P5, ressources et construction**



Développer une première boucle complète récolter → stocker → construire → utiliser. Une ressource récoltable et un inventaire minimal réels, une recette/installation utile du **\*\*même abri P4\*\***, interface de placement compréhensible, admission sur le terrain et collisions/coûts locaux vérifiés. Le joueur doit pouvoir prévisualiser l’abri avant sa construction, le déplacer et l’orienter. Afficher clairement si le placement est autorisé ou interdit. La construction doit consommer les ressources uniquement après validation du joueur. Prévoir une annulation sans consommation. Conserver les identités/poses corporelles f64 de plusieurs instances et la consommation/recharge indépendante du placement. Vérifier marche/porte/contact après construction, service de recharge de chaque instance, déplacement/rebasage/chargement local et retours spatiaux. Persistance des objets/inventaire en session ; **\*\*sauvegarde disque à P6\*\***, sans prétendre la fournir en P5. Pas de modules vides ou d’architecture générique avant besoin concret.



Nous pouvons lancer P5, mais il faudra assez rapidement consacrer une phase entière à l’optimisation du moteur planétaire, notamment au LOD adaptatif et au streaming. Le LOD selon la distance reste un chantier explicite de la roadmap ; si son implémentation devient prioritaire par demande utilisateur ou performance observée, réorganiser avec justification et critères complets. Aucun « LOD déjà fait » basé sur les treize patches.



**## Contraintes permanentes et vérifications**



Rust stable 2024, Bevy =0.19.1 ; pas de dépendance inutile. SI/1:1, DVec3/DQuat f64 ; soustraction d’origine en f64 puis conversion vers proxies f32. GameSet Input→Simulation→Navigation→Rendering→Interface. Orbite Time\<Real>, deltas souris déjà intégrés sans delta time. Gravité martienne radiale, marche 4,5/course 8/saut 4 m/s, peau 2 mm, pas 1/120 s, physique max 0,25 s/frame ; oxygène en temps réel complet hors menu/espace. P/T commandent calendrier/rotation, pas physique locale ni survie. En l’air, joueur entraîné par Mars ; Coriolis/centrifuge/Euler/accélération translatoire omises. Visites translatoires, M/G/O/U sans transfert inertiel. Pas de cartographie/IAU exacte ni relief scientifique implicite.



Menu/options sans sauvegarde ; Échap libère d’abord le curseur puis ouvre le menu. Profilage prioritaire, historique par défaut ; \`SOLAR_PROFILE_TERRAIN=global\` explicite et essai ≥54 s pour remplacement mesuré. Compiler puis lancer les scénarios GPU successivement, jamais en parallèle. Mesa immediate seulement contournement local NVK. Dossiers de diagnostic finaux nouveaux dans un parent existant ; F12/SOLAR_CAPTURE_DIR exige un dossier créé.



Reproduction ciblée P4 : \`python3 scripts/validate_survival.py --output docs/mon-nouveau-p4 --mesa-immediate\` après compilation, treize captures à inspecter. Vérifications appropriées en fin : cargo fmt --check, cargo check --locked, cargo check --all-targets --locked, cargo test --locked, cargo clippy --all-targets --locked -- -D warnings, cargo build --release --locked ; lancement/captures du nouveau parcours. Documenter résultats/environnement/limites, actualiser README/ROADMAP/VALIDATION/documents techniques/NEXT_CHAT et fournir son lien. Ne jamais prétendre Windows validé depuis Linux.