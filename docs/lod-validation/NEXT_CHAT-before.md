# Prochain chat Solar Survival

**P5, récupération et construction du même abri, est réalisée sur Ubuntu.** Prochaine phase dédiée : **optimisation du moteur planétaire, LOD adaptatif et streaming**, avant sauvegarde/distribution P6. Mise à jour : 13 octobre 2026.

## Prompt autonome de reprise

Nous poursuivons Solar Survival dans `/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL`. Lis AGENTS.md, README.md, docs/ASTRONOMY.md, docs/CUBE_SPHERE.md, docs/GLOBAL_TERRAIN.md, docs/SURVIVAL.md, docs/CONSTRUCTION.md, docs/VALIDATION.md, docs/ROADMAP.md, docs/PERFORMANCE.md et docs/NEXT_CHAT.md. Examine Git et le code puis avance vers une fonctionnalité complète, jouable. Départ réel P5 : HEAD `94036f8`, P4 commité, seul NEXT_CHAT modifié par l’utilisateur. Les changements P5 et preuves sont **commités** à la fin de cette passe. La passation précédente est conservée exactement dans `docs/p5-validation/NEXT_CHAT-before.md`. Aucune preuve historique supprimée/écrasée.

## Instructions utilisateur prioritaires

Accélérer par fonctionnalités complètes et progrès mesurables, sous-étapes techniques libres, tests progressifs. Les anciennes restrictions de chaîne fixe P3.5 ne limitent plus le terrain mondial. Vérifications adaptées au changement réel, sans rejouer systématiquement les campagnes graphiques anciennes. À chaque fin de passe, actualiser README, documents techniques, ROADMAP, VALIDATION et cette passation.

**Treize patches à résolution fixe ne constituent pas un LOD adaptatif.** Les niveaux selon distance, raccords mixtes sans fissures, contacts fins proches, transitions stables et budgets CPU/GPU restent à implémenter. L’utilisateur demande une phase entière d’optimisation rapidement après P5 : l’ordre choisi est maintenant cette phase avant P6, justification/critères dans ROADMAP. Aucun gain de performance déduit des FPS ponctuels des captures P5.

## État réel

Rust stable, édition 2024, application 0.2, Bevy **exactement =0.19.1**. Aucun changement Cargo.toml/Cargo.lock ni dépendance ajoutée. **110 tests** : 34 internes, 9 construction, 21 fondations, 15 surface historique, 9 cube-sphere, 4 contacts, 3 migrations, 4 hors centre, 4 mondiaux, 7 survie. Exploration spatiale, menu/options/crédits, FPS mondial, oxygène et construction actifs.

### P5 : boucle complète de session

Huit caisses dorées artistiques, initialisées une fois au premier site (x=±8 m ; z=0/−6/−12/−18 m), positions DVec3 et orientations DQuat dans les axes du corps tournant de Mars. **E** à portée de 2,5 m récupère six unités d’alliage et épuise la caisse, sans réapparition ni double récolte. Un solide coupant le segment yeux→caisse interdit la collecte. Collectibles sans collision de déplacement, pas de minerai scientifique, réseau mondial de ressources ou renouvellement.

Inventaire initial vide, **12 alliage par abri**, 48 unités disponibles : quatre constructions supplémentaires possibles. **B** ouvre le fantôme du même OxygenShelter P4 ; souris/visée et marche le suivent, **flèches** déplacent (distance 4–24 m, latéral ±16 m, 4 m/s), **J/L** orientent (1 rad/s). Fantôme et HUD vert/autorisée ou rouge/interdite avec motif, visibles même avec H. **Entrée** confirme après revalidation de l’état actuel ; **X/B/Échap** annulent sans dépense. Échap annule/libère avant menu. Menu, espace ou échec annulent aussi ; oxygène indépendant du placement.

Admission : extrema exacts des triangles sous un carré conservateur de demi-largeur 4,5 m, aire entièrement couverte certifiée à 10⁻⁹ m² en monde, variation ≤0,4 m. Carré entièrement dans le disque local 66 m ; avancer jusqu’au rebasage permet de placer plus loin. Centre au milieu des extrema, terrain entre −0,2 et +0,2 m. Sol historique utilisable selon mêmes triangles, ±4 096 m ; anciens scénarios/profilage gardent survie/construction désactivées. Une enveloppe OBB 6,3×3,95×6,3 m réserve tout l’abri : SAT 15 axes contre joueur, autres abris, trois obstacles, caisses actives, marge 0,2 m sur les axes. Politique conservative, pas un calcul de distance euclidienne ou admission de pente arbitraire. Limite de session 32 abris, pas d’index spatial spéculatif.

`construction::Construction` possède stock/dépôts/commandes ; `confirm` réutilise **Buildings::install_shelter et publish_colliders**, déduit le coût une seule fois après admission/installation. Même type, pièces/contact/service P4. Stock, caisses épuisées et bâtiments persistent en session, y compris M/G/O/U, menu et R. **Aucune sauvegarde disque**, prévue P6.

Six proxies de preview, ≤8 de caisse, cube de l’abri partagé, trois matériaux supplémentaires. Proxies locaux ≤120 m, conservés au rebasage, libérés en espace/hors portée/à épuisement. Aucun mesh par frame. Différences corps−yeux f64, rotation vers carte puis f32 explicite. Admission ≤32 abris : ≤206 proxies P4/P5 hors terrain/obstacles ; ressources normales permettent seulement cinq abris totaux et ≤44 proxies conservateurs. CPU linéaire sur collections bornées. [CONSTRUCTION](CONSTRUCTION.md).

### P4 et bâtiment à préserver

Oxygène 180 s, consommation 1 s/s réel dehors, recharge +30 s/s dans l’abri, alerte ≤30 s. Time<Real> après Navigation, indépendant de P/T et du plafond physique 0,25 s. Menu : réserve/recharge suspendues, gravité/orbite continuent. Espace : réserve/échec conservés sans recharge. À zéro, échec verrouillé, pilotage bloqué, R revient au premier abri avec réserve pleine. R conserve le stock et les constructions ; inactif avant échec/dans menu. H conserve les HUD vital et P5.

Premier abri six mètres devant l’arrivée, porte 2,4 m côté +Z du bâtiment. Retour normal M retrouve le site de première arrivée, éventuellement nocturne, sans déplacement d’objet ni recharge implicite. Buildings possède OxygenShelter, BuildingId monotone et pose corporelle f64 fixe, indépendante des patches/origine flottante/JD. Six pièces communes au rendu/contact ; pas de dalle, terrain comme sol. Huit coins du volume joueur exigés dans le volume utile ; pas de recharge à travers un mur/depuis toit. Six BodyBox f64 par instance publiées seulement à installation, contacts SAT continus/triangles de support. Proxies à moins de 120 m, entités conservées au rebasage, identité retrouvée au retour. [SURVIVAL](SURVIVAL.md).

### Terrain mondial et contraintes physiques

M normal ouvre Mars sphérique traversable sur six faces/arêtes/coins, sans borne de site mondiale. SI/1:1, trajets planétaires longs. `SOLAR_MARS_GLOBAL=0` conserve le terrain artistique historique. P3.2–P3.4 et smoke/P2 restent régressions distinctes.

Niveau **15/32 segments**, ≤13 patches à distance de graphe ≤2, sans relief ni **LOD adaptatif**. 1 089 sommets/2 048 triangles par patch, bords/coins f64 communs via PatchKey::vertex. Géométries Arc partagées, index XZ 8 m ; contact joueur avec aire certifiée, demi-largeur ≤1 m (joueur 0,3 m), requête de construction séparée ≤5 m. Disque 66 m, rebasage 64 m : garde-fou local, jamais borne mondiale.

Transport minimal des normales indépendant des faces, conservation pieds/yeux/vitesse/visée f64. Boîte joueur alignée sur la carte active, pas de volume orienté invariant revendiqué. Trois obstacles fixes dans le corps, OBB exactes en monde. Mesh propre par patch, origine soustraite f64, handles communs conservés/slots recyclés, assets terrain libérés en espace. ≤13 meshes/14 157 sommets/26 624 triangles résidents ; transaction ≤26 occurrences/53 248 triangles locaux avec géométries communes partagées, sans RAM/VRAM exhaustive.

GameSet Input→Simulation→Navigation→Rendering→Interface ; Oxygen après navigate, Construction après Oxygen. Gravité martienne radiale, marche 4,5/course 8/saut 4 m/s, peau 2 mm, pas 1/120 s, max physique 0,25 s/frame ; survie en delta réel complet. Temps orbital Time<Real>, souris déjà intégrée sans delta time. En l’air, joueur entraîné par Mars ; Coriolis/centrifuge/Euler/accélération translatoire omises. Visites translatoires et M/G/O/U sans transfert inertiel. Pas de relief scientifique, cartographie/IAU exacte implicite.

## Validations et limites

Contrôles Cargo fmt/check/check-all-targets/test/Clippy/release --locked réussis sur les sources finales. Les 110 tests couvrent collecte/déplétion/portée/solide, refus terrain et volumes, coûts/annulation/revalidation, manipulation à 30/60/144 FPS, porte/mur/service après construction et carte rebasée, deux instances construites avec orientations distinctes, M/G/O/U et R, menu/Échap, partage/proxies/libération. Historiques numériques conservés.

Premier parcours P5 `docs/p5-validation/graphics/` : code 0, 59,478 s, quinze captures, aucune erreur/alerte/panic ; inspection 06 révèle l’aide générale recouvrant le HUD P5. Correction HUD à 100 px du bas et seconde campagne finale **code 0, 59,466 s**, seize captures inspectées via quatre planches et originaux 06/14/15, menu inclus, aucun ERROR/WARN/panic. Build final 154,685 s ; sources et binaire inchangés pendant et après le parcours. Résultats dans VALIDATION et `graphics-final/results.json`. Diagnostic neutralise la souris OS parasite et masque la télémétrie avec H après les premières vues. Récolte/placement/rotation/confirmation utilisent les commandes ECS de production, **aucune injection d’alliage ou installation directe**. Départs devant portes/course et réserves de 60 s préparés explicitement ; pas de périphériques OS simulés ni voyage continu entièrement manuel. [Notes des essais](p5-validation/NOTES.md).

Environnement : Ubuntu 24.04.5 LTS/X11, DISPLAY=:1, Rust 1.99.0, i7-6700K/GTX 1060 3 Go/NVK/Mesa 25.2.8. **Windows natif et RTX 4050/Core i5 non validés** : modèle CPU/RAM/pilote/réglages cible à relever. Pas de nouveau cycle P2, suspension réelle, autres DPI, benchmark P5 ou mesure RAM/VRAM exhaustive. Historiques P4/P3 et profils conservés ; leurs FPS ne valident pas le LOD futur. Captures finales et empreintes à consulter avant de modifier.

## Objectif suivant : phase complète moteur planétaire

À la demande utilisateur, traiter maintenant l’optimisation, le LOD adaptatif et le streaming avant P6. Commencer par une baseline release reproductible du parcours mondial actuel et P5, même résolution/MSAA/matériel, repos/contact/placement/construction et **vrai remplacement** (profil mondial ≥54 s), répétitions et mesure CPU/GPU. Distinguer coûts de rendu/deux caméras/pilote, génération/index et uploads ; ne pas attribuer les FPS des captures à un composant sans comparaison.

Développer ensuite de vrais niveaux selon distance : contacts fins f64 près du joueur, géométrie plus grossière au loin, sélection avec hystérésis, **raccords mixtes sans fissures** sur faces/arêtes/coins et transitions stables. Définir la stratégie de raccord et budgets CPU/GPU avant implémentation, l’intégrer en fonctionnalité jouable complète avec compteurs et mesures de changements. Réutiliser/libérer assets, charger seulement les zones nécessaires ; pas de planète entière ni construction de meshes à chaque frame. Les abris, dépôts, services et inventaire doivent survivre aux nouveaux changements de terrain ; ne pas réécrire ces objets comme zones liées au LOD.

Critères : plusieurs détails observables et réellement choisis, continuité numérique/visuelle des niveaux mixtes, marche/course/saut et contacts proches sans trou, pose f64 et constructions conservées, absence d’oscillation au repos, budgets résidents/transitoires testés, mesures release pendant streaming/transition, captures inspectées, régressions appropriées de survie/construction/terrain et retours spatiaux. Aucune prétention de LOD déjà livré avant ces critères. Pas de nouvelle dépendance inutile, relief mondial ou sauvegarde disque ajouté au hasard ; P6 demeure sauvegarde/chargement, réglages complémentaires, validation native Windows et distribution.

Vérifications finales : `cargo fmt --check`, `cargo check --locked`, `cargo check --all-targets --locked`, `cargo test --locked`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo build --release --locked` ; compiler puis lancer les scénarios GPU successivement. Profil historique par défaut ; `SOLAR_PROFILE_TERRAIN=global` explicite. Mesa immediate seulement sur ce poste NVK. Dossiers de diagnostic finaux nouveaux dans un parent existant ; F12/SOLAR_CAPTURE_DIR exige un dossier créé. Pour P5 ciblé : `python3 scripts/validate_construction.py --output docs/mon-nouveau-p5 --mesa-immediate`, seize captures à inspecter. Mettre à jour docs/README/ROADMAP/VALIDATION/NEXT_CHAT, donner le lien de la passation ; ne jamais déclarer Windows validé depuis Linux.
