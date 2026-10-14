> **Précision matérielle — 14 octobre 2026 :** RTX 4050 **6 Go au mieux**, CPU « i5 12700K » à confirmer. P7 reçoit des budgets initiaux de textures modestes, 128 Mio de nouvelles textures résidentes et un objectif 1080p/60 FPS à vérifier sur la cible ; [contrat](ART_DIRECTION.md), [protocole](PERFORMANCE.md). Mise à jour documentaire uniquement, aucun asset/résultat de performance ajouté ; budgets non encore instrumentés.

> **Planification post-MVP — 14 octobre 2026 :** le MVP fonctionnel P1–P6 est réalisé sur Ubuntu. P7 visuel/textures est désormais l’objectif suivant ; [cadrage et sources](ART_DIRECTION.md), [roadmap prolongée](ROADMAP.md). Cette passe ne modifie que la documentation : aucun nouveau rendu, asset, test Cargo ou lancement graphique. Les résultats ci-dessous restent ceux du jeu `6898bdb`. La validation Windows native demeure ouverte et transversale.

# Validation Solar Survival 0.2 — P6, aventures multiples et interface

14 octobre 2026. Départ réel **22efa72**, P4/P5/LOD/caméra unique déjà réalisés. La reprise P4 reçue était périmée ; l’utilisateur a corrigé le cadrage, puis exigé plusieurs nouvelles aventures nommées et une interface de jeu conventionnelle. La passe à emplacement unique dans [p6-validation](p6-validation/NOTES.md) et le contrôle P5 dans [p5-recheck](p5-recheck-2026-10-13/) restent des preuves intermédiaires. Les résultats courants sont exclusivement ceux ci-dessous. [Contrat des profils](PERSISTENCE.md), [essais et limites](p6-profiles-validation/NOTES.md).

Rust stable 1.99.0, édition 2024, Bevy **exactement =0.19.1**, Cargo.toml/Cargo.lock inchangés, aucune dépendance ajoutée. Ubuntu 24.04.5 LTS/X11, i7-6700K/GTX 1060 3 Go/NVK/Mesa 25.2.8 ; contournement local Mesa immediate. [Contexte relevé](p6-profiles-validation/host.json), adaptateur/pilote dans les journaux graphiques. Windows 11 et RTX 4050/Core i5 **non validés**.

## Résultats actuels

| Vérification | Résultat |
|---|---|
| fmt, check, check-all-targets, tests, Clippy -D warnings, release --locked | Six commandes réussies ; [codes/durées](p6-profiles-validation/build-responsive/results.json), release **152,440 s** |
| Tests | **130 réussis**, 49 internes ; [journal complet](p6-profiles-validation/build-responsive/tests.txt) |
| Profils/menu/HUD 1440×900 | Deux processus, **48,373 + 54,377 s**, codes 0, 32 captures, aucun ERROR/WARN/panic ; [résultats](p6-profiles-validation/graphics-responsive/results.json) |
| Archive extraite, 1280×720 | Deux processus hors checkout, **48,358 + 54,431 s**, codes 0, 32 captures, aucun ERROR/WARN/panic ; [résultats](p6-profiles-validation/packaged-720/results.json) |
| Inspection visuelle | **64 captures** sur seize planches et originaux de menus/catalogue/erreur ; [grande fenêtre](p6-profiles-validation/graphics-responsive/inspection/verified.json), [720p](p6-profiles-validation/packaged-720/inspection/verified.json) |
| Concordance | Sources Rust/tests/shaders/Cargo et binaire inchangés dans les deux campagnes ; [1440×900](p6-profiles-validation/graphics-responsive/concordance.json), [720p](p6-profiles-validation/packaged-720/concordance.json) |
| Packaging Ubuntu | Tar.gz **23 393 264 octets**, **731 fichiers** vérifiés après extraction, binaire identique au release ; [empreintes](p6-profiles-validation/package.json) |
| Windows natif et matériel cible | Non exécutés ; distribution sur les deux OS encore à terminer |

Les lancements GPU se succèdent après compilation. build-ui-initial précède le dernier réglage de hauteur/pagination ; seul build-responsive est la référence finale. Aucun changement du jeu après ce build, uniquement documentation et scripts de preuves/packaging.

## Ce qui est effectivement vérifié

Nouvelle aventure demande un nom via KeyboardInput, écrit `saves/Nom/session.solar` et remet la progression à zéro. Alpha récolte 12 unités et construit le même abri P4 via B → catalogue → Prévisualiser → confirmation : deux abris, stock nul. Bravo démarre neuf, récolte une seule caisse : un abri, six unités. Le retour sélectionné à Alpha retrouve ses objets et sa réserve ; un second processus retrouve séparément Alpha et Bravo. Création et changement sauvegardent la partie quittée ; recharger le profil actif n’écrase pas sa sauvegarde avant lecture. Le catalogue bloque le pilotage, libère le curseur et conserve la consommation réelle ; fermer/annuler ne dépense rien.

Les tests couvrent noms Unicode/vides/réservés/chemins/doublons, fichiers absents/corrompus/incompatibles/trop grands, données invalides, refus sans mutation, écriture et secours. Roundtrip complet : f64 corporels, abris/identités/colliders, caisses/stock, oxygène/échec, espace/surface, carte après rebasage/en l’air, options et calendrier. Recharge de l’abri restauré, prochaine identité/construction et règles R sont vérifiées. Les caches/contacts sont reconstruits avant publication ; même GameCamera conservée. Les régressions terrain/contacts/horloges/caméra restent réussies. Le remplacement des dépôts par une session vide supprime aussi les anciens proxies.

Les captures vérifient accueil, nom, liste de profils, sélection explicite, reprise, catalogue avec et sans stock suffisant, placement, HUD et erreur de profil Broken avec Alpha conservé. Menu/HUD partagent palette et tailles cohérentes ; navigation N et télémétrie H sont à la demande en surface. Aucun texte coupé ou chevauchement constaté dans ces poses aux deux résolutions. Il s’agit d’une vérification de lisibilité, pas d’une approbation esthétique de l’utilisateur.

Le diagnostic prépare explicitement poses proches des caisses/abris, franchissement de rebasage, réserves 60/45/0,001 s, JD et options ; il n’injecte pas d’alliage ni d’abri construit. Les clics/commandes sont ECS, pas périphériques OS. La recharge/reprise est réelle, sans attente continue de trois minutes ni prétention de benchmark. Les contacts/marches/portes sont également couverts par les tests conservés ; les campagnes mondiales historiques ne sont pas rejouées sans changement de leur physique.

## Limites conservées

Un profil actif en mémoire ; plusieurs répertoires indépendants sur disque. Pas de sauvegarde automatique à Quitter, cloud, gestion de suppression/renommage, saisie riche/IME ou prise en charge de plusieurs processus écrivant le même profil. Format local version 1, checksum de corruption accidentelle, secours `.bak` ; pas de garantie universelle de panne électrique. Les paramètres existants sont persistés sans ajout arbitraire d’options.

Archive native Linux testée depuis /tmp avec assets voisins du binaire. Le script sait assembler un zip à partir d’un build Windows natif ; aucun exe Windows ni test Windows n’a été produit ici. Pas de nouveau benchmark/RAM/VRAM exhaustive, campagne P2 complète, DPI/périphériques/suspension. Les résultats historiques LOD/ciel ci-dessous restent datés et ne mesurent pas universellement P6.

---

# Historique : caméra unique et ciel local

13 octobre 2026. Départ réel HEAD **43f163b** ; seuls NEXT_CHAT et ROADMAP modifiés au démarrage, copies exactes dans [sky-validation](sky-validation/NEXT_CHAT-before.md) et [ROADMAP-before](sky-validation/ROADMAP-before.md). Les preuves LOD/P4/P5 historiques sont conservées. Bevy **exactement =0.19.1**, édition 2024, Rust stable 1.99.0 ; aucune dépendance ni modification de Cargo.toml/Cargo.lock. Ubuntu 24.04.5 LTS/X11, i7-6700K/GTX 1060 3 Go/NVK/Mesa 25.2.8, présentation Mesa immediate. [Contexte](sky-validation/host.json), [architecture et approximations](SURFACE_SKY.md), [essais intermédiaires](sky-validation/NOTES.md).

## Résultats finaux

| Vérification | Résultat |
|---|---|
| fmt/check/check-all-targets/test/Clippy/release --locked | Six commandes réussies ; [codes et durées](sky-validation/final-matte/build-results.json) |
| Tests | **120 réussis**, 40 internes ; les 115 régressions antérieures passent |
| Ciel release | Code 0, **72,416 s**, 24 captures, aucun ERROR/WARN/panic, mêmes sources/shaders/binaire pendant l'essai |
| P5 release | Code 0, **59,373 s**, 16 captures, aucun ERROR/WARN/panic, concordance vraie |
| Mesures release | Sept profils avant/après réussis, ~51→29 ms au repos ; [comparaison](sky-validation/comparison.json) |
| Inspection | **40 captures finales inspectées** sur dix planches ; nuit/crépuscule aussi originales |
| Caméras | Une seule Camera3d/identité active dans chaque frame de diagnostic ; au plus une caméra extraite, une observée côté renderer |
| Windows natif / RTX 4050 et Core i5 | Non exécutés |

Journaux des six contrôles dans [final-matte](sky-validation/final-matte/), [tests](sky-validation/final-matte/tests.txt). Parcours ciel : [résultat](sky-validation/sky-matte/results.json), [journal](sky-validation/sky-matte/game.txt), [inspection](sky-validation/sky-matte/inspection/verified.json). P5 : [résultat](sky-validation/p5-matte/results.json), [journal](sky-validation/p5-matte/game.txt), [inspection](sky-validation/p5-matte/inspection/verified.json). Tous les lancements GPU sont successifs, après compilation. Les campagnes antérieures nommées sky-final/sky-verified précèdent le dernier réglage mat ; leur nom ne les rend pas finales pour ce bilan.

## Numérique et architecture vérifiés

Cinq tests nouveaux : rayon angulaire asin(R/d) et direction calculés en SI/f64 avant conversion ; projections identiques à celles de la caméra globale à J2000/J2000+0,5/JD 2461325,5 ; directions/angles et repère inertiel conservés au rebasage ; même entité/mesh/panorama/matériau pendant vingt alternances espace/surface et rotations ; une caméra réutilisée par les vraies sorties M/G/O/U avec pose locale/globale, near/far et couches adaptées avant rendu. Les tests de FOV/MSAA/menu ciblent désormais cette seule caméra. Les anciennes assertions de pose du fond spatial en surface ont été adaptées à la pose locale, sans modifier l'origine f64 publiée.

Le fond est un triangle opaque à profondeur infinie, sans écriture de profondeur ; la scène locale le masque dans la même vue. Aucun second render target, caméra de ciel ou capture par frame en production. Le panorama artistique reste statique (4 096×2 048 RGBA8, 32 Mio) ; neuf entrées célestes dynamiques dans un uniforme de 608 octets. HDR/tonemapping/MSAA communs ; bloom conservé en espace, halo shader en surface. Les budgets de terrain et la simulation ne changent pas. Une caméra ne signifie pas une seule passe GPU ; ces plafonds d'assets ne mesurent pas toute la RAM/VRAM du moteur.

## Inspection finale et correction visuelle

Les 24 poses couvrent Soleil de jour, visée/horizon, nuit, toit et paroi occultant le fond, crépuscule, vraie course à 8 m/s et un rebasage, P/T, menu/reprise, M/G/O/U et leurs retours, échec/R, autre JD et autre site préparé. La caméra garde l'identité **266v0** dans ce parcours. Les préparations de site/JD/réserve sont explicites et distinctes des commandes de déplacement/navigation. Les étoiles sont fines, irrégulières et d'éclats variés. La palette reste artistique et poussiéreuse ; le contraste sol/ciel et l'horizon simplifié restent visibles, sans revendication d'horizon complet ou de relief lointain.

La première proposition a été rejetée par l'utilisateur : panorama à points trop grands et quadrillage régulier, ciel brun sombre uniforme. Les essais successifs et leurs captures ne sont pas supprimés. La bande claire au sol sous Soleil rasant persistait après rapprochement du brouillard/suppression du bloom de surface ; elle disparaît dans la capture finale avec **reflectance du sol à 0**, rugosité/couleur/lumières conservées. Cette valeur mate est un choix artistique, pas une mesure martienne. Brume **40–60 m**, contre 70–125 m auparavant, pour terminer avant le disque local certifié de 66 m ; visibilité lointaine explicitement réduite. Pas de support physique dans le fond de sol sous l'horizon.

P5 conserve récupération réelle (6 puis 12 unités), refus/preview/orientation, annulation sans coût, stock 0 après confirmation, second abri, porte/recharge, mur, course/rebasage, espace et menu. H conserve les HUD vitaux/construction. Les tests préservent les contacts, les propriétaires corporels, le LOD, les horloges et les règles d'oxygène. Aucun replay P2 complet ou campagne mondiale de 120 s supplémentaire sans modification de ces mécanismes.

## Limites

Rayons et directions issus du catalogue, sans nouvelle donnée astronomique. Les corps subpixel sont des points agrandis pour visibilité ; phases/éclats/couleurs/halo atmosphérique restent des approximations de présentation. Le panorama de surface diffère du champ triangulaire historique en espace. Horizon radial idéal sans dépression d'altitude, réfraction ou relief lointain. Les marqueurs UI filtrent horizon/boîtes à 120 m ; ils ne font pas une occlusion des collines historiques ou du texte par pixel. Les disques/étoiles utilisent, eux, la profondeur locale réelle.

Les captures suivent la stabilisation de chaque pose ; pas de vidéo/pixel-audit de toutes les frames de transition ni nouvelle campagne des périphériques OS/DPI/suspension. Aucun nouveau modèle atmosphérique scientifique, carte IAU, retard lumineux, sauvegarde, relief, quadtree ou physique inertielle ajouté. Windows et matériel cible restent à valider nativement. Le bilan de comparaison release est dans [PERFORMANCE](PERFORMANCE.md).

---

# Historique : validation moteur planétaire adaptatif

13 octobre 2026. Départ Git réel **ebddd90**, P5 commité, seul NEXT_CHAT modifié par l’utilisateur ; copie exacte dans [NEXT_CHAT-before](lod-validation/NEXT_CHAT-before.md). Historiques et essais intermédiaires conservés. Rust stable 1.99.0, édition 2024, Bevy **exactement =0.19.1**, Cargo.toml/Cargo.lock inchangés, aucune dépendance ajoutée. Ubuntu 24.04.5 LTS/X11, i7-6700K/GTX 1060 3 Go/NVK/Mesa 25.2.8. [Contexte](lod-validation/host.json), [sources et binaires finaux](lod-validation/final/environment.json), [journal et limites](lod-validation/NOTES.md).

Le terrain mondial adapte réellement sa résolution intérieure **32/16/8 selon distance**, avec hystérésis, bords communs fins et index de contact local. Il conserve les contacts proches et les objets corporels P4/P5. Le niveau spatial des clés reste 15 : LOD de résolution, sans quadtree ni relief. [Stratégie préalable](lod-validation/STRATEGY.md), [contrat actuel](GLOBAL_TERRAIN.md).

## Contrôles des sources finales

| Vérification | Résultat |
|---|---|
| cargo fmt --check, cargo check --locked | Réussis |
| cargo check --all-targets --locked | Réussi, exemples inclus |
| cargo test --locked | **115 tests réussis**, dont 35 internes et 4 tests adaptatifs dédiés ; les 110 tests P5 conservés |
| cargo clippy --all-targets --locked -- -D warnings | Réussi sans alerte |
| cargo build --release --locked | Réussi, **124,244 s** |
| cargo build --release --locked --example lod_seams | Réussi, **98,567 s** |
| Raccords mixtes release | Code 0, 36,323 s, six captures |
| P5 sur LOD adaptatif | Code 0, 59,650 s, seize captures |
| Monde sur LOD adaptatif | Code 0, 120,641 s, 36 captures, douze franchissements en l’air/douze rebasages, apex 2,131380 m |
| Profils release | **16 essais réussis**, dont neuf sur le binaire final, répétitions monde/P5, recorder GPU et comparaison de caméra ; aucun ERROR/WARN/panic |
| Inspection | **58 captures finales inspectées** ; deux planches de raccords, quatre P5 et neuf mondiales, original du coin également inspecté |
| Concordance | Sources Rust/tests/exemples/Cargo et binaires inchangés depuis les builds et pendant les parcours réussis |
| Windows 11 / RTX 4050 et Core i5 | **Non exécutés** |

[Journaux des six contrôles](lod-validation/final/), [temps des builds](lod-validation/final/build-results.json), [diagnostic de raccords](lod-validation/graphics/results.json), [parcours P5/mondial](lod-validation/graphics-focused/results.json), [concordance](lod-validation/graphics-focused/concordance.json), [inspection des raccords](lod-validation/graphics/inspection/verified.json), [inspection P5/monde](lod-validation/graphics-focused/inspection/verified.json). Le fichier graphics/results contient aussi un essai P5 échoué ; seule son entrée seams est une réussite finale. Les contrôles all-targets/tests/Clippy consignés dans final sont ceux déjà réussis sur les mêmes sources, conservés sans répétition inutile.

## Numérique, contacts et ressources

Cinq tests supplémentaires :

- Bandes d’hystérésis 80/110 et 160/200 m, mille répétitions sans ping-pong dans chacune, raffinement direct 8→32.
- Toutes les faces, raccords internes et quatre coins de face, quatre directions d’arête et neuf combinaisons 32/16/8 : **topologie en disque**, incidences d’arêtes 1/2, triangles extérieurs, 128 segments de frontière. Chacune des 32 paires de sommets du raccord est exactement commune en f64, même inversée entre faces.
- 150 directions sur six faces/arêtes/coins et repères tournés : extrema sous les empreintes 0/0,3/1/4,5/5 m identiques au fournisseur fin à 10⁻⁹ m près dans le disque opérationnel. Maxima observés de l’index : **361 cases, 10 613 références et 5 375 triangles**, au lieu d’indexer tout le voisinage.
- Plans de triangles des trois résolutions, centres et coins des six faces : flèche conservative <0,1 mm sur la sphère sans relief. Pas de changement des triangles proches ni de support abstrait distinct du mesh.
- Vingt changements d’ancre : budgets résidents/transitoires et Arc communs vérifiés, **39 changements effectifs de résolution**. Le test ECS de rendu antérieur vérifie désormais aussi longueurs/indices du mesh après changement de détail, mêmes handles, stabilité puis libération en espace.

Les régressions antérieures couvrent toujours marche/course/saut, 288 traversées dirigées aller-retour à 30/60/144 FPS et 24 trajets de coins, transport des pieds/yeux/vitesse/visée f64, repos sans allocation, P/T, M/G/O/U, oxygène/menu/R, collecte/admission/paiement et service du même abri construit. Données de bâtiment et de dépôt indépendantes des patches et du LOD.

## Inspection et essais intermédiaires

Les six vues du diagnostic isolé montrent les trois densités et les éventails au bord, raccords orientés/inversés et le coin de trois faces. Les scènes 2 et 6 remplacent les résolutions avec les mêmes quatre handles (trois surfaces et grille). Ce diagnostic impose ses niveaux pour examiner la topologie ; les tests et les compteurs du jeu prouvent séparément leur choix selon distance.

P5 confirme la preview rouge/verte et les motifs lisibles, deux récoltes réelles, stock 12 puis 0 à la confirmation, abri utilisable/recharge, mur, rebasage, Mars spatiale et menu. La capture initiale affiche **6/5/2 patches** et **16 328 triangles**, contre 26 624 en mode fixe. Dans le diagnostic mondial, 8/4/1 devient 6/6/1 avec deux changements de détail au remplacement. La course et la réception restent continues ; aucun trou/fissure visible observé. Les vues nocturnes limitent l’inspection fine lointaine, complétée par la grille éclairée isolée et les tests exacts. Bordure lointaine du voisinage et marqueurs astronomiques restent des limites de présentation préexistantes.

Deux premiers parcours P5 échouent avant la collecte, pieds immobiles : [graphics](lod-validation/graphics/results.json) et [graphics-final](lod-validation/graphics-final/results.json), code 101, conservés. La tentative de focus direct ciblait la décoration Mutter portant le même titre. La relance réussie active la fenêtre via le gestionnaire X11 après deux secondes ; le client du jeu reçoit le focus. **Aucun Rust/LOD/contact/collecte modifié entre ces essais.** Le helper de profilage cible désormais explicitement le client ; il ne simule aucune touche/souris OS. Ces opérations de diagnostic restent hors de la logique Rust et sont facultatives via --focus-x11. [Détail](lod-validation/NOTES.md).

## Mesures et limites

Les baselines monde/P5 ont précédé les changements du terrain. Deux comparaisons release finales fixe/adaptatif, mêmes 1440×900/MSAA ×4/deux caméras, incluent un vrai remplacement au milieu de la course. Sur ces deux remplacements mesurés : transaction CPU fixe **5,676/6,533 ms**, adaptative **1,140/0,979 ms** ; navigation maximale **5,791/6,669 ms** contre **1,282/1,098 ms**. L’index est le principal coût supprimé. Les frames de course restent **51,417–51,493 ms** en fixe et **51,386–51,561 ms** en adaptatif : pas de gain FPS notable établi. [PERFORMANCE](PERFORMANCE.md) détaille répétitions, GPU, comparaison des caméras et P5 ; [synthèse](lod-validation/summary.json) et concordances [fixe](lod-validation/final-fixed/concordance.json), [adaptatif](lod-validation/final-adaptive/concordance.json), [P5](lod-validation/final-p5/concordance.json).

Les budgets demeurent ≤13 meshes, ≤14 157 sommets/26 624 triangles résidents conservateurs ; ≤26 occurrences CPU pendant remplacement, Arc partagés, pas de second voisinage de handles GPU. Les résolutions moyennes/grossières ont réellement 413/205 sommets et 696/280 triangles. Ni planète entière ni mesh par frame. Les compteurs de structures ne mesurent pas la RAM/VRAM exhaustive ; les durées de remise aux assets excluent les uploads effectifs GPU. Les spans GPU asynchrones et partagés entre vues ne donnent pas une attribution exhaustive par caméra/upload et ne s’additionnent pas en un total.

Pas de nouvelle suspension réelle, autre DPI, essai OS des périphériques, cycle P2 complet ou validation Windows/cible. Le LOD est synchrone et borné, spécifique à la sphère sans relief ; pas de quadtree ou sauvegarde disque. Les préparations de pose/réserve des diagnostics restent explicites, sans injection d’alliage ni installation directe P5. Les paramètres astronomiques, unités et horloges sont conservés. Aucun 60 FPS cible revendiqué.

---

# Historique : validation Solar Survival 0.2 — P5 : récupération et construction

13 octobre 2026. Départ réel HEAD **94036f8**, P4 commité, seul NEXT_CHAT modifié par l’utilisateur ; copie exacte dans [NEXT_CHAT-before](p5-validation/NEXT_CHAT-before.md). Changements P5 et preuves **non commités**. Historiques conservés. Rust stable 1.99.0, édition 2024, Bevy **exactement =0.19.1**, Cargo.toml/Cargo.lock et dépendances inchangés. Ubuntu 24.04.5 LTS/X11 DISPLAY=:1, i7-6700K/GTX 1060 3 Go, Vulkan/NVK/Mesa 25.2.8. [Environnement et empreintes finaux](p5-validation/final/environment.json), [contexte graphique](p5-validation/graphics-final/environment.json), [notes des essais](p5-validation/NOTES.md).

La boucle normale est récupération→inventaire→prévisualisation→confirmation→abri utilisable. Huit caisses finies, E à 2,5 m, six alliage par caisse, recette à 12. B/flèches/J-L/Entrée, vert/rouge avec motif, X/B/Échap pour annuler sans coût. Le même OxygenShelter P4 est installé avec ses contacts et son service. Stock/dépôts/bâtiments conservés en session ; aucune sauvegarde disque. [Règles et architecture](CONSTRUCTION.md).

## Vérifications finales P5

| Vérification | Résultat |
|---|---|
| cargo fmt --check | Réussi |
| cargo check --locked, check --all-targets --locked | Réussis |
| cargo test --locked | **110 tests** : 34 internes, 9 construction, 21 fondations, 15 surface, 9 cube-sphere, 4 contacts, 3 migrations, 4 hors centre, 4 mondiaux, 7 survie |
| cargo clippy --all-targets --locked -- -D warnings | Réussi |
| cargo build --release --locked | Réussi, **154,685 s** murales sur les sources finales ; premier build 158,094 s conservé |
| Diagnostic final P5 release | **Code 0**, **59,466 s** murales, **16 captures**, aucun ERROR/WARN/panic |
| Inspection visuelle | Toutes les seize captures via quatre planches ; originaux 06, 14 et 15 également inspectés |
| Concordance | Empreintes sources/binaire inchangées pendant et après la campagne finale ; sources conformes au build final |
| Windows natif / RTX 4050 et Core i5 | Non exécutés |

[Six commandes et résultats finaux](p5-validation/final/build-results.json), [journaux tests](p5-validation/final/tests.txt), [campagne finale](p5-validation/graphics-final/results.json), [journal](p5-validation/graphics-final/game.txt), [captures/télémétrie](p5-validation/graphics-final/captures/), [manifest d’inspection](p5-validation/graphics-final/inspection/manifest.json). Aucun changement de source Rust après ce build/cette campagne ; seules documentation et preuves d’inspection complétées ensuite.

## Couverture numérique et ECS

Les 99 tests antérieurs passent. Onze tests supplémentaires :

- Portée E, caisse épuisée une seule fois, stock réel, conservation des huit dépôts/épuisement et inventaire après M/G/O/U ; segment de visibilité bloqué par une pièce solide.
- Preview interdite faute d’alliage, confirmation sans installation/paiement, annulation X/Échap à stock intact, confirmation admise payée une seule fois, publication de six nouveaux colliders. Oxygen consomme pendant la manipulation.
- Refus joueur/abri/obstacle/caisse ; couverture locale insuffisante ; terrain historique trop irrégulier et extrema des triangles comparés aux hauteurs sous le carré entier. Les tests mondiaux antérieurs vérifient toujours aire/empreintes partielles, faces/arêtes/coins et contacts.
- Flèches et rotation à 30/60/144 FPS, position et orientation corporelles de la preview ; tolérance 10⁻⁶ sur les durées manuelles arrondies aux nanosecondes.
- Marche par la porte de l’abri **construit** puis recharge pleine, mur bloquant. Rebasage et recharge du premier abri et de l’abri construit dans la **carte déjà rebasée**, sans enter_home, poses fixes et nombre de colliders conservés.
- Deux abris construits par les mêmes touches/recette, identités et orientations distinctes, recharge de chacun, conservation à chaque sortie M/G/O/U. Pas de logique particulière pour la seconde installation.
- Revalidation d’un stock devenu insuffisant malgré l’ancienne preview admise, annulation à échec oxygène et conservation d’inventaire lors de R.
- Menu ECS/Échap : annulation avant ouverture, stock/instances conservés et absence de construction par Entrée à la reprise du menu.
- Proxies de caisses/preview : un seul cube partagé, huit caisses et six pièces, mêmes entités au rebasage, reconstruction des centres corporels à moins de 5 µm, matériau rouge après refus, suppression après récolte et en espace, recréation des sept dépôts restants au retour.

Ces tests injectent les touches ECS et préparent certaines poses. Ils ne simulent pas les périphériques OS. Les régressions physiques et numériques P2/P3/P4 sont conservées ; pas de replay graphique systématique de toutes les campagnes historiques.

## Parcours graphique et correction visuelle

Le premier parcours [graphics](p5-validation/graphics/results.json) termine avec code 0, 59,478 s, quinze captures, aucune erreur/alerte/panic et empreintes inchangées. Ses assertions de récolte, placement, confirmation, recharge, mur, rebasage, espace et menu passent. L’original 06 inspecté révèle que l’aide générale recouvre le HUD P5. Les preuves restent conservées ; cette capture ne valide pas la lisibilité finale.

Le HUD P5 est remonté à 100 px du bas. Le diagnostic final déplace aussi le fantôme plus loin, masque la télémétrie avec H après les premières vues, neutralise les deltas souris OS parasites et ajoute une capture du menu. Il collecte réellement deux caisses après marche ECS, teste manque d’alliage/confirmation refusée, stock 6 puis 12, volume occupé rouge, déplacement/orientation verte, annulation à stock intact, nouvelle preview/confirmation, stock 0 et deuxième abri visible. Il prépare le départ devant sa porte, marche dedans et recharge, pousse contre le mur, prépare le départ de course puis parcourt ~64 m en course avec **un rebasage**, effectue M aller/retour, annule la preview dans le menu avec réserve/stock constants, puis prépare l’approche et marche/recharge à nouveau dans l’abri construit. Aucun alliage ni bâtiment injecté : E, B, flèches, L et Entrée utilisent la production. Les réserves de 60 s et départs devant porte/course sont des préparations diagnostiques explicites ; il ne s’agit pas d’un parcours manuel continu ou d’une simulation des périphériques OS.

L’inspection des quatre planches et originaux 06/14/15 confirme : commandes/motifs lisibles au-dessus de l’aide générale, couleurs et ouverture de l’abri dans la preview, fantôme déplacé/tourné entièrement dans le cadre, stock inchangé à annulation puis 0 à confirmation, portes/murs/toit, HUD de recharge sur abri 2, site après rebasage, Mars spatiale, menu sans HUD/preview et retour fonctionnel. Avec H, oxygène et inventaire restent visibles. Assemblage simple de cubes artistiques sans sas/décoration ; caisses sans collider de déplacement ; marqueurs astronomiques et limite lointaine du terrain restent ceux du prototype.

## Budgets et limites

Cube partagé par abri/caisses/preview, trois matériaux P5 supplémentaires ; ≤8 proxies de caisse, six de preview, six par abri local à 120 m. Admission ≤32 abris, plafond conservateur 206 proxies hors terrain/obstacles ; ressources normales finies permettent seulement cinq abris totaux et ≤44 proxies conservateurs. Données physiques corporelles f64 et collections linéaires bornées ; aucune génération de mesh par frame. Le terrain conserve ≤13 patches, niveau 15/32 segments, **sans LOD adaptatif**. Ces plafonds structurels ne mesurent pas toute la RAM/VRAM ou les buffers du renderer.

Le rebasage final isolé journalise **6,567 ms CPU**, captures actives, hors synchronisation des meshes et GPU. Aucun benchmark P5, attribution de régression/gain ou garantie 60 FPS. Les premières captures avec télémétrie affichent environ 14–15 FPS, relevés ponctuels non comparables aux baselines historiques. La prochaine phase dédiée doit mesurer le rendu, les deux caméras, le streaming et les transitions LOD sur des fenêtres comparables ; [PERFORMANCE](PERFORMANCE.md), [ROADMAP](ROADMAP.md).

Pas de nouvelle suspension réelle, autre DPI, cycle P2 complet, test OS des périphériques, validation native Windows/RTX 4050/Core i5 ou mesure mémoire exhaustive. Ressource initiale finie, pas de réseau mondial/renouvellement ; aucune sauvegarde disque, relief mondial scientifique ou LOD livré par P5. **P5 réalisé sur Ubuntu** dans ces conventions ; la phase complète d’optimisation/LOD/streaming vient maintenant avant sauvegarde/distribution P6.

---

# Historique : validation Solar Survival 0.2 — P4 : survie et abri réutilisable

13 octobre 2026. Départ réel Git **07e7af8**, terrain mondial déjà commité, avec seulement NEXT_CHAT modifié par l’utilisateur ; le prompt précédent citait un HEAD plus ancien. La copie exacte de cette passation est conservée dans [NEXT_CHAT-before](p4-validation/NEXT_CHAT-before.md). Les nouveaux changements P4 sont **non commités**. Aucune preuve historique supprimée/écrasée. Rust stable 1.99.0, édition 2024, Bevy **exactement 0.19.1**, Cargo.toml/Cargo.lock et dépendances inchangés. Ubuntu 24.04.5 LTS/X11 DISPLAY=:1, i7-6700K/GTX 1060 3 Go, Vulkan/NVK/Mesa 25.2.8. [Environnement final](p4-validation/final/environment.json), [contexte graphique et empreintes](p4-validation/graphics/environment.json), [notes](p4-validation/NOTES.md).

La partie normale offre réserve de 180 s, consommation 1 s/s, recharge 30 s/s dans un abri accessible, HUD/alerte à 30 s et échec verrouillé avec reprise **R**. Le menu suspend la survie ; orbite/gravité continuent. Les visites spatiales conservent réserve/échec sans recharge ; M retrouve le site fixé à la première arrivée. P/T n’affectent pas la consommation. L’abri est instanciable avec identité stable et pose corporelle f64, réutilisable pour construction P5. [Règles et architecture](SURVIVAL.md).

## Vérifications finales P4

| Vérification | Résultat |
|---|---|
| cargo fmt --check | Réussi |
| cargo check --locked et --all-targets --locked | Réussis |
| cargo test --locked | **99 tests** : 32 internes, 21 fondations, 15 surface, 9 cube-sphere, 4 contacts, 3 migrations, 4 hors centre, 4 mondiaux, 7 survie |
| cargo clippy --all-targets --locked -- -D warnings | Réussi |
| cargo build --release --locked | Réussi ; premier build P4 **140,313 s**, vérification finale incrémentale également réussie |
| Parcours graphique P4 release | **Code 0**, 56,469 s murales, 13 captures, aucun ERROR/WARN/panic |
| Inspection visuelle | Treize captures via quatre planches ; originaux 00, 01 et 06 également inspectés |
| Windows 11 / RTX 4050 et Core i5 | Non exécutés |

[Contrôles finaux](p4-validation/final/build-results.json), [premier build](p4-validation/build-results.json), [campagne](p4-validation/graphics/results.json), [journal](p4-validation/graphics/game.txt), [captures/télémétrie](p4-validation/graphics/captures/), [manifest d’inspection](p4-validation/graphics/inspection/manifest.json). Les sept tests de survie ont été renforcés après la campagne ; les sources du **jeu** et le binaire graphique n’ont pas changé. L’environnement final distingue ces empreintes et vérifie cette correspondance. Pas de nouvelle campagne graphique après cette seule modification de test.

## Couverture et preuve du bâtiment réutilisable

Neuf tests ajoutés aux 90 tests P3, tous conservés et réussis :

- Réserve en temps réel à 30/60/144 FPS, pause et accélération orbitale, delta de 12 s consommé entièrement ; bornes de réserve, recharge saturée, valeurs temporelles invalides ignorées et échec verrouillé.
- Menu ECS : consommation suspendue pendant que JD et gravité avancent ; R ignoré dans le menu, reprise fonctionnelle. Recharge également suspendue par le test de politique.
- Marche réelle par la porte ouverte, recharge pleine, mur bloquant, sortie par la porte, épuisement et immobilisation ; M vers l’espace ne réanime pas, R retourne au premier abri.
- M/G/O/U et accélération de Mars : réserve gelée en espace, première pose/identité corporelle et nombre d’abris conservés, retour au site de session.
- Deuxième installation via la même API, puis course traversant une clé de patch et rebasage ; six boîtes corporelles par instance, poses exactes immuables, conversion des contacts sous 2 nm et axes sous 10⁻¹². **Recharge de la deuxième et du premier abri dans la carte déjà rebasée**, sans appel au retour de site.
- Huit coins du volume joueur exigés dans le volume utile, rejet derrière le mur et au-dessus du toit, pose de bâtiment inclinée.
- Proxies : même unité de cube/entités au rebasage, restitution des centres dans le corps sous 5 µm, suppression en espace sans perte de données ; un mesh partagé.

Le parcours graphique consomme dehors, entre par les vrais contrôles ECS, recharge, sort, ouvre/reprend le menu, court environ 88 m puis revient ; **un rebasage** pendant cette course aller-retour, pose d’abri conservée. Il provoque l’échec, presse R, effectue M aller/retour, installe la seconde instance et marche/recharge dedans. Il initialise une réserve de 80 s puis 1 s pour raccourcir l’échec, et 60 s devant le second abri. Le second départ est repositionné par le diagnostic ; aucune construction joueur, aucun changement de taux de production ou voyage intersite n’est revendiqué.

L’inspection montre porte/murs/toit, progression du HUD, masque du menu, message d’échec/R lisible, Mars spatiale puis site conservé et deuxième abri identifié. Les vues intérieures regardent la paroi de face à hauteur des yeux ; l’abri reste un assemblage simple de boîtes, sans décoration/sas. L’éclairage devient plus sombre sous ×1000 ; le masque solaire historique reste actif. Les marqueurs peuvent se superposer, limite ancienne. Les captures espacées ne prouvent pas chaque frame ; les poses/contacts sont vérifiés numériquement/ECS. Aucun nouveau test des périphériques OS ou suspension réelle.

## Portée, coûts et suites

Six boîtes/proxies par abri, un mesh de cube et deux matériaux ; un abri normal, deux dans le diagnostic, proxies chargés dans un rayon de 120 m et supprimés en espace. Poses/boîtes autoritaires restent en CPU ; requêtes linéaires avec exclusion conservative. Les budgets du terrain restent ≤13 patches niveau 15/32 segments. **Ce chargement dynamique à résolution fixe n’est pas un LOD adaptatif**. Plusieurs niveaux de détail selon la distance du joueur sont désormais exigés et planifiés dans [ROADMAP](ROADMAP.md), avec raccords de niveaux mixtes et budgets à valider.

Pas de benchmark complet P4, de mesures exhaustives RAM/VRAM, de cible 60 FPS, de validation Windows/RTX 4050 ni de nouveaux tests d’autres DPI. Le rebasage graphique isolé journalise 8,218 ms CPU, captures actives : hors GPU/meshes, sans comparaison contrôlée permettant d’attribuer une variation au nouvel abri. Pas de replay graphique massif P3/P2 ; les 90 régressions numériques/ECS et leurs anciennes preuves sont conservées. Les profils/scénarios historiques gardent bâtiments et survie désactivés.

**P4 terminé sur Ubuntu**, dans les conventions de gameplay documentées. P5 doit rendre constructible **le même OxygenShelter** ; inventaire, recettes et placement joueur restent à faire, sauvegarde disque à P6. Pas de dépendance ajoutée ni d’architecture vide. Le LOD adaptatif demeure une extension distincte à implémenter, jamais présenté comme réalisé par les treize patches.

---

# Historique : validation Solar Survival 0.2 — Mars entière traversable

13 octobre 2026. Départ Git `2031710` (P3.4 commité), avec uniquement la correction utilisateur dans NEXT_CHAT. L’utilisateur a ensuite demandé explicitement les six faces traversables et autorisé la révision de la roadmap : le cadrage d’une chaîne fixe est remplacé par un terrain mondial complet, chargé localement. Les changements de ce chat sont **non commités** ; aucune preuve antérieure supprimée/écrasée. Ubuntu 24.04.5 LTS/X11, DISPLAY=:1, Rust stable 1.99.0 édition 2024, Bevy exactement 0.19.1, i7-6700K/GTX 1060 3 Go/NVK/Mesa 25.2.8. Cargo.toml/Cargo.lock et dépendances inchangés. [Environnement et empreintes](global-validation/environment.json), [décisions/essais intermédiaires](global-validation/NOTES.md).

Le lancement normal utilise maintenant le terrain mondial après M : six faces, arêtes et coins, chargement dynamique ≤13 patches, pas de borne de site. Réutilisation des géométries/meshes communs, repère transporté indépendamment des axes des faces, contacts sous toute l’empreinte et obstacles orientés exacts. Anciens modes historique/P3.2–P3.4 disponibles pour régression ; le profiler conserve le parcours historique sauf sélection explicite `SOLAR_PROFILE_TERRAIN=global`. [Architecture, budgets et commandes](GLOBAL_TERRAIN.md).

## Vérifications finales

| Vérification | Résultat |
|---|---|
| fmt, check --locked, check --all-targets --locked | Réussis, exemple P3.1 inchangé compris |
| cargo test --locked | **90 tests** : 30 internes, 21 fondations, 15 surface historique, 9 géométrie, 4 contacts centraux, 3 migrations centrales, 4 hors centre, 4 mondiaux |
| Clippy --all-targets --locked -- -D warnings | Réussi sans alerte |
| build release --locked | Réussi, **133,907 s** murales |
| Monde, 120 s | Code 0, **12 franchissements d’arête en l’air, 12 rebasages**, apex **2,131379 m**, 36 captures |
| Profilage historique–mondial–historique | Trois codes 0, 42 s chacun, quatre phases/CSV ; pas de régression notable de rythme |
| Profilage mondial avec remplacement mesuré | Code 0, 54 s ; cinq nouveaux patches, navigation maximum 5,616 ms |
| Régressions P3.4 / centrale P3.3 / P3.2 / historique | Quatre codes 0 : huit/huit migrations, sept franchissements P3.2, dix captures historiques |
| Windows 11 / RTX 4050 et Core i5 | Non exécutés |

[Commandes/codes/temps de build](global-validation/build-results.json), [campagnes](global-validation/graphics-results.json), [détail des régressions](global-validation/legacy/graphics-results.json), [synthèse](global-validation/summary.json). **Neuf lancements du jeu successifs**, après compilation : monde, quatre profils puis quatre régressions. Aucun ERROR/WARN/panic dans leurs journaux finaux. Empreintes Rust/tests/exemples/Cargo et binaire vérifiées après les campagnes ; aucun Rust modifié après le build final. Le premier lanceur a échoué avant de lancer les régressions (argument manquant), puis a repris sans écraser les essais réussis : détails dans NOTES.

## Numérique, ECS et couverture

Les 82 tests historiques restent réussis. Huit tests supplémentaires vérifient couverture, traversées et ressources mondiales :

- 150 directions sur les six faces (u/v = −1/−0,5/0/0,5/1), arêtes et huit coins, repère tourné horizontalement ; disque opérationnel, empreintes 0,6×0,6 m et leurs quatre coins, indices extérieurs. Les aires des polygones découpés certifient chaque empreinte avec tolérance arithmétique 10⁻⁹ m². Une empreinte partiellement couverte avec centre valide est refusée par un test dédié.
- **288 traversées dirigées d’arête**, aller-retour sur les 24 arêtes dirigées, marche/course et 30/60/144 FPS, saut au raccord, réception et repos ; **288 rebasages**. Les huit coins sont également traversés dans trois directions chacun (24 trajets). Pieds au-dessus du support, déplacements par frame bornés, pas de borne terminale mondiale.
- Conservation f64 lors du transport : pieds/yeux <4 nm, axes de visée <10⁻¹³ et vitesse <10⁻¹³ m/s sur orientations de face/pôle/coin ; état temporel/saut conservé. Petits réajustements de support après reboîtage sous 50 µm, puis repos stable sous 0,1 µm. Aucun volume orienté invariant revendiqué.
- Réutilisation Arc des géométries communes, absence de génération au repos ; deux anneaux locaux, ≤13 patches et 26 624 triangles. Relevé de couverture : au plus 9 155 cases/91 320 références ; ce relevé est dans les diagnostics progressifs, dont la construction d’index est inchangée dans les sources finales.
- Contrôles ECS mondiaux : marche réelle, rebasages, pause/P/T et M/G/O/U ; pose publiée même sans sous-pas. Rendu : handles/entités des patches communs conservés, slots entrants recyclés, 13 meshes maximum sur le parcours testé, libération à la sortie ; vertices locaux reconstruits dans le corps sous 30 µm.
- SAT continu contre les boîtes orientées exactes : concordance avec les boîtes alignées, collision balayée inclinée et refus du faux contact qu’aurait donné une enveloppe AABB. Les supports d’obstacles utilisent leurs faces triangulaires exactes.

Le disque n’est pas une nouvelle limite de voyage : son origine est remplacée après 64 m, avant la fin de couverture à 66 m. La direction radiale choisit librement la face/clé suivante ; voisins ±U/±V et arêtes orientées alimentent le cache. Les tests ne parcourent pas chaque clé d’une planète entière, ni une circumnavigation à vitesse réelle ; ils couvrent le mécanisme local répété, les 24 raccords dirigés et les huit coins.

## Inspection et performances

**112 captures inspectées** via [13 planches](global-validation/inspection/manifest.json) : 36 mondiales, 22 hors centre, 22 centrales, 22 P3.2, dix historiques. Cinq originaux supplémentaires : monde 06/26, hors centre 06, P3.2 05, historique 06. Monde : sol continu et HUD cohérent, sauts/réceptions et rebasages, faces éclairées ou nocturnes ; aucun trou local visible. Les captures espacées ne prouvent pas la continuité instantanée, vérifiée en f64/ECS. La sphère n’a pas de relief ; le bord lointain du voisinage peut rester visible à l’horizon, et les marqueurs superposés restent une limite antérieure. Régressions : raccords/colors, obstacles, bornes propres aux anciens modes, Mars/Terre et terrain artistique conservés.

Douze rebasages release du parcours mondial, captures actives : **4,644–7,203 ms**, médiane **5,094 ms**, géométrie/index/transfert/libération ; hors meshes/GPU. Comparaison 42 s au repos : mondial **52,361 ms / 19,10 FPS**, historique avant/après **52,257/52,188 ms / 19,14/19,16 FPS**. Course : **51,778 ms** mondial contre **51,838–51,913 ms** historique. La phase mondiale de 8 s n’inclut pas de remplacement retenu ; l’essai de 54 s en inclut un : navigation max **5,616 ms**, rendu-sync max **0,165 ms**, frame moyenne/p95/max **52,005/54,932/56,741 ms**. Ces essais ne montrent pas de régression notable sur ce poste, sans garantie universelle ou mesure cible. [PERFORMANCE](PERFORMANCE.md) détaille portée et reproduction.

## État et limites

**L’objectif de Mars traversable sur six faces est réalisé et validé sur Ubuntu de développement** dans ce parcours sphérique à résolution fixe. Chargement CPU/GPU borné, SI/échelle 1:1, données physiques f64, conversions locales explicites f32 et Bevy inchangés. Pas de planète entière résidente, relief scientifique, LOD mixte, corps dynamiques, dépendance supplémentaire, survie active ou sauvegarde. Les anciens plafonds de deux cartes ne limitent plus le parcours normal. Les budgets transitoires sont structurels, sans mesure exhaustive RAM/VRAM/buffers internes de Bevy ; aucun 60 FPS cible revendiqué. Pas de nouvel essai des périphériques OS, menu interactif, suspension réelle ou cycle P2 complet ; preuves antérieures conservées. Windows et RTX 4050 restent à vérifier nativement.

---

# Historique : validation Solar Survival 0.2 — P3.4 hors centre validé sur Ubuntu

13 octobre 2026. Départ Git **propre à `ea3a32b`**, P3.3 déjà commité contrairement à NEXT_CHAT ancien. Ses preuves sont préservées. Changements P3.4 laissés **non commités**. Ubuntu 24.04.5 LTS, X11 `DISPLAY=:1`, Rust stable 1.99.0 édition 2024, Bevy **exactement 0.19.1**, i7-6700K/GTX 1060 3 Go/NVK et Mesa 25.2.8. Cargo.toml/Cargo.lock et dépendances inchangés. [Empreintes sources/binaire et environnement](p34-validation/environment.json), [décisions et portée](p34-validation/NOTES.md).

P3.4 ajoute `SOLAR_CUBE_OFFCENTER=1`, deux cartes fixes franchement hors centre (u≈0,5/v≈0,5), même face et mêmes niveau 15/64 segments. Le nouveau module `planets/cube_migration.rs` prépare leur recouvrement certifié, impose l'hystérésis dans le repère commun d'entrée et rejette avant toute construction. Centrale P3.3 et P3.2 restent leurs options indépendantes ; historique demeure le défaut. [CUBE_SPHERE](CUBE_SPHERE.md) définit le périmètre, les formules, les réserves et le plafond transitoire ; [ASTRONOMY](ASTRONOMY.md) précise les conventions physiques conservées.

## Vérifications finales

| Vérification | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check --locked`, `cargo check --all-targets --locked` | Réussis, exemple P3.1 inchangé compris |
| `cargo test --locked` | **82 tests réussis** : 26 internes, 21 fondations, 15 surface historique, 9 géométrie, 4 contacts centraux, 3 migrations centrales, 4 hors centre |
| `cargo clippy --all-targets --locked -- -D warnings` | Réussi sans alerte |
| `cargo build --release --locked` | Réussi, **2 min 06 s** (127,04 s murales du lanceur) |
| Hors centre, diagnostic P3.4 de 88 s | **Code 0**, huit migrations dont une en l'air, apex **2,130986 m**, 22 captures inspectées |
| Centrale, régression P3.3 de 88 s | **Code 0**, huit migrations dont une en l'air, apex **2,130985 m**, 22 captures inspectées |
| Paire seule, régression P3.2 de 88 s | **Code 0**, six franchissements, apex **2,131309 m**, 22 captures inspectées |
| Historique, rotation 50 s | **Code 0**, dix captures inspectées |
| Windows natif / RTX 4050 et Core i5 | **Non exécutés** |

[build-results.json](p34-validation/build-results.json) et [graphics-results.json](p34-validation/graphics-results.json) conservent commandes, variables, codes, durées et comptes ; [synthèse](p34-validation/summary.json). Journaux [fmt](p34-validation/fmt.txt), [check](p34-validation/check.txt), [all-targets](p34-validation/all-targets.txt), [tests](p34-validation/tests.txt), [Clippy](p34-validation/clippy.txt), [release](p34-validation/release.txt), [hors centre](p34-validation/offcenter.txt), [centrale](p34-validation/central-regression.txt), [P3.2](p34-validation/p32-regression.txt), [historique](p34-validation/historical.txt). **Aucun ERROR/WARN/panic dans les quatre journaux graphiques.** Aucun changement Rust après le build final ; les empreintes des sources et du binaire sont vérifiées après les campagnes. Compilations puis lancements GPU strictement successifs, présentation Mesa immediate. Le lanceur exact est conservé en snapshot ; seule la date de métadonnées de l'utilitaire réutilisable a ensuite été rendue dynamique, sans changement des commandes du jeu.

## Preuves numériques et ECS

Le volume d'admission est exprimé dans la carte d'entrée : x≈77,096929±8 m, |y|≤100 m, |z|≤49,444528 m. Chacun de ses coins, puis une grille de requêtes aux altitudes −100/0/+100 m, est vérifié dans les rectangles de **deux** cartes sur les six faces, avec toute l'empreinte et une réserve de reboîtage. Les seuils aller/retour sont c±1 m ; rien n'est extrapolé hors rectangles et les quatre bornes sont testées en l'air puis à réception. [Relevé des plans et index](p34-validation/overlap.txt).

Les transactions aller et retour passent sur les six faces au sol/2/20/99 m. Pieds/yeux/centres d'obstacles conservés dans le corps à <4 nm, axes de visée à <10⁻¹³ sans unité, vitesse à <10⁻¹³ m/s, gravité à <10⁻¹³ m/s² ; rayon du support à <2 µm et hauteur verticale au support à <20 µm. Yaw/pitch, accumulator, pending jump et grounded ne sont pas réinitialisés. Les comparaisons en l'air distinguent le rayon du support et les projections verticales propres aux deux cartes.

Le changement d'alignement de la boîte joueur est quantifié avec les huit coins et la rotation complète : **83,015 µm**, sous le plafond d'admission **0,25 mm** et la peau 2 mm. Angle ~4,54916×10⁻⁵ rad, PLUS PETIT que P3.3 : ce test hors centre inclut le roulis, sans prétendre valider une plus grande inclinaison. Les trois poses d'obstacles restent fixes dans le corps et leurs huit coins sont contenus dans l'enveloppe AABB de contact ; expansion analytique maximale **0,227457 mm** ([calcul](p34-validation/contact-budget.json)), limite admise 0,5 mm par dimension. La migration exclut une proximité de contact par réserve totale 2,75 mm avant construction. Aucun volume orienté rigoureusement invariant ni contact pour grand changement de repère n'est revendiqué.

Les marches/courses aller-retour, saut pendant migration, réception et repos passent à 30/60/144 FPS sur les six faces. Refus répétés hors couloir/altitude/domaine et dans l'hystérésis : milliers de tentatives sans construction ni révision ; refus près d'un objet également testé. Les vrais systèmes ECS vérifient pause, P/T, publication de pose et M/G/O/U hors centre et au centre. Le rendu ECS vérifie géométrie issue des f64, indices, couleurs du patch commun, **mêmes deux handles et entités** après migration et réentrées. La paire et les trois objets sont toujours réutilisés ; aucun troisième proxy n'est ajouté.

## Inspection graphique et portée

**76 captures 1440×900 inspectées**, par 21 planches répertoriées dans [manifest.json](p34-validation/inspection/manifest.json) : six hors centre, six centrales, six P3.2, trois historiques. Cinq inspections supplémentaires à taille originale : hors centre [entrée/raccord](p34-validation/offcenter/crossing-00.png), [saut après migration](p34-validation/offcenter/crossing-06.png), [limite positive](p34-validation/offcenter/crossing-11.png), P3.2 [repos au raccord](p34-validation/p32-regression/crossing-05.png), historique [course/franchissement](p34-validation/historical/docs/rotation-surface-06.png).

Le diagnostic hors centre montre repos, marche/course aller-retour et saut/réception, pause puis ×1000, deux limites X à vitesse nulle, quatre sorties/retours M/G/O/U puis Terre. Les poses de préparation à x=c−3 m et z=hz/2 ne comptent pas comme marches ; les deux migrations explicites aux limites sont également exclues. Huit migrations sont comptées pendant les commandes ECS, dont une avec grounded=false. Les planches montrent un sol continu sur les vues inspectées, visée/HUD cohérents, couleur du patch partagé stable et bords extérieurs inclinés attendus pour cette couverture limitée. Ces captures espacées ne sont pas une vidéo prouvant la continuité instantanée : conservation par tests f64. Les marqueurs astronomiques superposés sont une limite antérieure.

Les régressions centrales et P3.2 conservent leurs raccords et commandes ; l'historique montre relief artistique, obstacles, marche/saut, course à travers les tuiles, repos puis Mars/Terre. Ni cycle P2 complet, ni menu/périphériques OS, ni suspension réelle du processus/OS rejoués. Les anciennes preuves restent valides dans leur périmètre, sans être présentées comme de nouveaux essais.

## Ressources, observations CPU et limites

Après entrée ou migration : deux patches dans une paire CPU, **8 385 sommets physiques/16 384 triangles**, deux meshes/handles de terrain. Préparation et remplacement : **deux paires CPU maximum**, 16 770 sommets/32 768 triangles plus positions locales/index/temporaires. À une réentrée, l'ancienne paire est libérée AVANT les deux constructions de préparation. La seconde géométrie préparée est ensuite libérée ; seules les deux métadonnées de cartes restent. Compteur deux constructions à l'entrée, une par migration admise, aucune pour un refus. Budget structurel contrôlé par le code et les tests de taille/handles ; pas de mesure exhaustive des allocations/RAM/VRAM ou des buffers GPU internes.

Rectangles hors centre : ~280,987×124,217 m et ~280,973×124,212 m. Index observés **2 978 cases**, **44 610/44 627 références**, maximum **18 triangles/case**. Pas d'extrapolation des plafonds à une autre résolution, rayon, relief ou planète.

Dix mesures murales de construction/transaction hors centre, huit migrations et deux préparations, captures actives : **6,918–9,750 ms**, médiane **7,306 ms**. Régression centrale : 7,758–11,458 ms, médiane 8,0705 ms. Elles excluent la synchronisation ultérieure du mesh, le GPU et la préparation initiale du plan ; aucune attribution de gain de FPS ni absence de pic de frame. Relevés HUD hors centre ~19–25 FPS ponctuels, distincts d'un benchmark. [PERFORMANCE](PERFORMANCE.md) conserve cette portée et le protocole cible.

**P3.4 terminé dans ce périmètre Ubuntu** : deux cartes franchement hors centre sur une même face, couverture et contacts bornés, marche/course/saut, refus sans reconstruction, assets réutilisés et régressions inspectées. Aucun chargement de chaîne, migration ±V, passage jouable entre faces, planète entière, LOD mixte, relief scientifique, survie, Avian ou refonte du contrôleur. La suite P3.5 doit cadrer une courte chaîne bornée sur la même face, notamment l'inclinaison cumulée des objets du site d'entrée et le budget de préparation, avant les faces. Windows natif et matériel cible restent non validés.

---

# Historique : validation Solar Survival 0.2 — P3.3 migration bornée validée sur Ubuntu

12 octobre 2026. Départ Git **propre au commit `b317d28`** : P3.2 avait été commité après la passation ancienne. Ubuntu 24.04.5 LTS/X11 (`DISPLAY=:1`), Rust stable 1.99.0 édition 2024, Bevy exactement 0.19.1, i7-6700K/GTX 1060 3 Go/NVK, Mesa 25.2.8 ; Cargo.toml/Cargo.lock et dépendances inchangés. Les preuves précédentes sont préservées. Les changements P3.3 restent non commités.

`SOLAR_CUBE_MIGRATION=1` active une transaction réversible entre centrale et voisine +U ; `SOLAR_CUBE_SURFACE=1` seul garde P3.2, le terrain historique reste le défaut. Fournisseur non central avec rectangle certifié par toutes les arêtes extérieures ; visée transportée, pieds/yeux/vitesse/gravité en f64, obstacles possédés par le site d'entrée. [CUBE_SPHERE](CUBE_SPHERE.md) décrit couverture, seuils, compromis de contact, budgets et reproduction ; [ASTRONOMY](ASTRONOMY.md) précise les conventions physiques.

**76 tests réussis** : 24 internes, 21 fondations, 15 surface historique, 9 géométrie cube-sphere, 4 contacts P3.2, 3 migrations. Les nouveaux tests et les tests étendus couvrent les rectangles de cinq positions sur chacune des six faces (centre, décentrées, coins), absence d'extrapolation, limites d'allocation, index/brute force et empreinte entière. La transaction est testée sur six faces et trois altitudes (sol, 2 m, 20 m) : pieds/yeux/centres d'obstacles conservés à <3 nm dans le corps, axes de visée à <10⁻¹³ sans unité, vitesse à <10⁻¹³ m/s et gravité à <10⁻¹³ m/s², support radial à <1 µm et écart vertical à <10 µm. La boîte de contact demeure alignée sur la carte : on ne prétend pas conserver un volume orienté exact entre deux cartes.

Marche/course aller-retour et saut pendant migration passent à 30/60/144 FPS avec repos et publication sans sous-pas. Un test des vrais contrôles ECS vérifie aller-retour en pause, P/T et les quatre sorties/retours M/G/O/U. Le test de rendu vérifie les mêmes deux handles et entités, les positions/indices f32 issus des nouvelles positions f64, et la couleur conservée du patch partagé. L'enveloppe de contact des trois obstacles est testée à moins de 0,31 mm d'expansion sur chaque dimension ; leur centre, taille et rotation visuelle restent fixes dans le corps.

Les essais intermédiaires et leurs corrections sont détaillés dans [NOTES](p33-validation/NOTES.md), y compris les assertions mal adaptées des nouveaux tests, la comparaison de borne et le premier compteur de diagnostic. Ils ne remplacent pas les validations finales. Les phases de préparation sont explicites et ne comptent pas comme des migrations à pied.

## Vérifications et lancement final

| Vérification | Résultat final |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check --locked`, `cargo check --all-targets --locked` | Réussis, exemple P3.1 inchangé compris |
| `cargo test --locked` | **76 tests réussis** |
| `cargo clippy --all-targets --locked -- -D warnings` | Réussi sans alerte |
| `cargo build --release --locked` | Réussi, **2 min 08 s** |
| P3.3, `SOLAR_CUBE_MIGRATION=1` + diagnostic | **Code 0**, 88 s, huit migrations dont une en l'air, apex **2,131306 m**, 22 captures inspectées |
| Régression P3.2 seule | **Code 0**, 88 s, six franchissements/apex **2,130718 m**, 22 captures inspectées |
| Régression historique seule, rotation 50 s | **Code 0**, dix captures inspectées |
| Windows natif / RTX 4050 et Core i5 | **Non exécutés** |

Commandes/codes/temps : [build-results.json](p33-validation/build-results.json), [graphics-results.json](p33-validation/graphics-results.json). Empreintes sources/binaire et environnement : [environment.json](p33-validation/environment.json), synthèse : [summary.json](p33-validation/summary.json). Journaux [fmt](p33-validation/fmt.txt), [check](p33-validation/check.txt), [all-targets](p33-validation/all-targets.txt), [tests](p33-validation/tests.txt), [Clippy](p33-validation/clippy.txt), [release](p33-validation/release.txt), [P3.3](p33-validation/migration.txt), [P3.2](p33-validation/p32-regression.txt), [historique](p33-validation/historical.txt). Aucun ERROR/WARN/panic dans ces **trois journaux graphiques finaux**. Sources et binaire correspondent aux empreintes finales ; aucun changement Cargo. Lancements successifs Mesa immediate, après compilation et sans autre essai GPU concurrent.

Le lanceur regroupant les essais finaux a terminé avec code **143** pendant la fin de son premier essai historique : dix captures, mais aucun code de sortie du jeu enregistré ; cause indéterminée. Les deux premiers essais avaient déjà leurs codes 0. Journal/captures de l'essai interrompu : [historical-launcher-143.txt](p33-validation/historical-launcher-143.txt), [dossier](p33-validation/historical-launcher-143/), [résumé](p33-validation/launcher-interruption.json). Ils restent distincts de la **relance historique seule**, qui a terminé code 0 avec le même binaire. Aucun succès final n'est déduit du code 143.

## Inspection et portée des preuves

Les **54 captures finales à 1440×900** ont toutes été inspectées via **15 planches** (6 P3.3, 6 P3.2, 3 historiques), répertoriées dans [inspection/manifest.json](p33-validation/inspection/manifest.json). P3.3 : [22 captures](p33-validation/migration/), avec [saut après migration](p33-validation/migration/crossing-06.png), [repos](p33-validation/migration/crossing-08.png) et [limite positive](p33-validation/migration/crossing-11.png) également inspectés à taille originale. Régression P3.2 : [22 captures](p33-validation/p32-regression/), [repos au raccord](p33-validation/p32-regression/crossing-05.png) également original. Historique : [dix captures](p33-validation/historical/docs/), [course](p33-validation/historical/docs/rotation-surface-06.png) et [Mars spatiale](p33-validation/historical/docs/rotation-surface-08.png) également originales.

P3.3 montre marche/course aller-retour, saut/réception, repos, P puis ×1000, deux limites X à vitesse nulle, quatre sorties/retours M/G/O/U et Terre. La migration en l'air a été comptée par changement de révision, avec `grounded=false`, pendant les vraies commandes ECS ; la réception reste au support. Les préparations x=96/z=30 et les deux migrations explicites de préparation des limites sont exclues du compteur des huit migrations. Le compteur P3.2 de signe X est désactivé pour P3.3. Les captures montrent un sol continu et une visée cohérente ; la conservation instantanée de pose/visée repose sur les tests f64, pas sur une preuve vidéo. Les bords extérieurs visibles aux limites sont attendus : seules deux surfaces sont chargées, pas une planète entière.

Le parcours P3.2 conserve son raccord fermé sans migration. La régression historique montre terrain/obstacles, saut, marche en pause, course/franchissement de tuile jusqu'à z=−150,815159 m, repos, Mars puis Terre. Aucun menu ou cycle P2 complet rejoué ; preuves anciennes et tests ECS préservés. Les touches sont injectées dans l'ECS : aucun nouvel essai des périphériques OS, de suspension complète de l'OS ou de Windows n'est revendiqué.

## Ressources, coût observé et limites

Après chaque remplacement : 8 385 sommets physiques/16 384 triangles, deux patches et deux handles de terrain réutilisés. Le test ECS de rendu contrôle leurs entités/handles et leur mise à jour, sans compter les meshes internes de Bevy. Coexistence synchrone CPU de l'ancienne paire et de la candidate uniquement : deux CubeTerrain, 16 770 sommets physiques/32 768 triangles, plus leurs tableaux locaux/index et temporaires bornés. Aucun troisième proxy GPU ni génération ordinaire par frame ; cette borne découle des deux constructions de taille testée et de la transaction, sans instrumentation complète des allocations/du renderer.

Index observés dans le parcours : **5 408 cases**, **53 240 références** pour la centrale −X et **53 255** pour la voisine, maximum **18 triangles/case**. Le relevé numérique des **30 paires non centrales** ([journal](p33-validation/noncentral-index.txt)) observe 1 112–5 408 cases, 34 283–53 255 références, maximum **44 triangles/case**, sous les plafonds testés 5 700/85 000/64. Près des coins, le rectangle conservateur a des demi-largeurs ~80,52×27,46 m contre ~205,88×102,44 m au centre : les seuils centraux ne sont pas transportables à ces paires.

Dix mesures murales de construction/transaction CPU (huit migrations de parcours et deux préparations), en release et avec captures actives : **8,004–11,657 ms**, médiane **8,567 ms**. Elles excluent la mise à jour des meshes/rendu GPU et ne sont ni un benchmark complet ni une comparaison P1. Les captures cube affichent ponctuellement **23–25 FPS** sur ce poste, sans prédiction de cible. Aucune mesure nouvelle de VRAM totale, buffers GPU transitoires du moteur ou cible RTX 4050/Core i5. [PERFORMANCE](PERFORMANCE.md) précise cette portée.

P3.3 est terminé **dans ce périmètre** : centrale↔voisine +U, même face, sphère sans relief, couloir/altitude d'admission bornés, boîte joueur alignée sur la carte et enveloppe AABB conservatrice des obstacles. Hors couloir et aux limites terminales, les bornes restent actives. Le joueur au sol/en l'air reste entraîné cinématiquement par Mars ; les forces d'inertie demeurent omises. Pas de passage jouable entre faces, LOD mixte, relief scientifique, planète entière, survie ou nouvelle bibliothèque physique. La suite doit d'abord définir les seuils/recouvrement et réévaluer les contacts pour une paire franchement décentrée sur la **même face**.


---

# Historique : validation Solar Survival 0.2 — P3.2 contacts et franchissement borné

12 octobre 2026. Départ Git propre au commit `da2b25c` : P3.1 était commité. Environnement : Ubuntu 24.04.5 LTS, X11 `DISPLAY=:1`, Rust stable 1.99.0, édition 2024, Bevy **exactement 0.19.1**, i7-6700K, GTX 1060 3 Go/NVK et Mesa 25.2.8. Aucune dépendance ni modification de Cargo.toml/Cargo.lock. Historiques P1/P2/P3.1 conservés ; changements de ce chat laissés non commités.

## Sources et vérifications

`planets/cube_terrain.rs` est le fournisseur opt-in de support pour une paire centrale d'une même face, sphérique sans relief. Positions/indices de `PatchPair` partagés avec le rendu, un seul repère tournant, empreinte entière découpée sur les triangles, index spatial fixe, rectangle intérieur conservateur. Le terrain historique demeure le défaut. [CUBE_SPHERE](CUBE_SPHERE.md) décrit résolution, unités, bornes, ressources et reproduction ; [ASTRONOMY](ASTRONOMY.md) décrit les conventions physiques.

**71 tests réussis** : 22 internes, 21 fondations, 15 surface historique, 9 cube-sphere géométrie et 4 nouveaux tests de contacts. Les huit tests ajoutés à P3.1 vérifient : couverture des six paires centrales et budgets métriques ; index contre une recherche sur tous les triangles, empreinte entière et continuité du raccord ; marche/course répétées dans les deux sens, repos et saut/réception ; quatre limites chargées, support en l'air et vitesse bloquée ; 30/60/144 FPS et publication sans sous-pas ; entrée éclairée à plusieurs époques ; systèmes de contrôle/horloge/publication et M/G/O/U ; positions/indices du mesh, précision du proxy et réutilisation de deux handles aux déplacements et retours. Ce sont des vérifications numériques et ECS, pas des essais de périphériques OS.

Fmt/check/check-all-targets/tests/Clippy et `cargo build --release --locked` réussis sur les sources finales (build 2 min 07 s). L’exemple P3.1, inchangé, est vérifié par `--all-targets`. Journaux dans [p32-validation](p32-validation/), commandes/codes dans [build-results.json](p32-validation/build-results.json).

## Parcours final de la paire

[Journal final](p32-validation/crossing.txt), [22 captures](p32-validation/crossing/), [empreintes des sources/binaire](p32-validation/environment.json), [commandes/codes](p32-validation/graphics-results.json). Lancement release **88 s**, code **0**, aucun ERROR/WARN/panic, sans compilation ni autre lancement GPU concurrent, présentation Mesa immediate. L'injection utilise les vrais contrôles ECS ; aucun périphérique OS n'est simulé.

Le compteur observe **six franchissements** pendant les phases de déplacement ; l'apex réel atteint **2,130988 m**. Les phases 1/2 montrent marche aller/retour à 4,5 m/s, 3/4 course aller/retour à 8 m/s, 5/8 repos sur X=0 avec vitesse nulle, 6 saut franchissant le raccord et 7 réception/retour. Le JD reste J2000 pendant la pause ; après P, les phases 9/10 courent à ×1000. Aux deux bornes X (phases 11/12), les pieds restent à ±205,578661 m, au sol et à vitesse nulle. Les sorties M/G/O/U, leurs quatre retours et la Terre sont visibles aux phases 13–21. Les limites Z et le repos long sont vérifiés numériquement ; le scénario graphique ne marche pas sur les quatre limites.

Toutes les **22 captures finales sont inspectées** via [six planches](p32-validation/inspection/), avec inspection supplémentaire à taille originale du [repos au raccord](p32-validation/crossing/crossing-05.png), du [saut](p32-validation/crossing/crossing-06.png) et de la [borne X](p32-validation/crossing/crossing-11.png). Les deux couleurs rejoignent le même bord sans trou visible ; obstacles/HUD, poses au sol/en l'air et transitions spatiales sont présents. Les extrémités du terrain limité sont visibles contre le fond astronomique, volontairement sans extension ou mur visuel. Les marqueurs astronomiques peuvent toujours se superposer au terrain, limite historique.

Ressources : 8 385 sommets physiques, 16 384 triangles, deux meshes propres à la paire. Les deux handles et leurs mises à jour aux réentrées sont vérifiés dans le test ECS de streaming ; on ne confond pas ces deux assets avec le total interne de Bevy. L'index observé utilise **5 408 cases**, **53 240–53 260 références** selon la face, maximum **18 triangles par case**. Les plafonds testés portent sur les six paires centrales, pas sur toute la planète. Les images affichent ponctuellement 22–25 FPS en surface ; ce sont des relevés du HUD, sans benchmark, attribution de gain CPU/GPU ni prédiction sur RTX 4050.

## Régression du terrain par défaut et limites

Après le parcours, `SOLAR_ROTATION_SMOKE_SECONDS=50` est lancé **sans** `SOLAR_CUBE_SURFACE`, depuis [game/](p32-validation/game/) pour préserver les captures historiques. [Journal](p32-validation/game.txt) : code **0**, dix captures et aucun ERROR/WARN/panic. Toutes les captures sont inspectées via trois planches, puis [repos](p32-validation/game/docs/rotation-surface-00.png), [course après changement de tuile](p32-validation/game/docs/rotation-surface-06.png) et [Mars spatiale](p32-validation/game/docs/rotation-surface-08.png) à taille originale. Terrain artistique, obstacles, HUD et vues Mars/Terre restent présents. Le journal conserve marche en pause au même JD, saut, course à 8 m/s, franchissement jusqu'à z=−151,006826 m et vitesse nulle au repos. Les tests de streaming vérifient encore neuf meshes, le voisinage 3×3 et la libération des tuiles.

**P3.2 terminé sur le poste Ubuntu de développement.** Cette campagne valide le parcours fonctionnel borné et les contacts, sans benchmark, essai OS des nouvelles touches ou replay complet du menu/cycle P2. Windows 11, RTX 4050/Core i5 et suspension complète de l'OS restent non validés. Pas de relief cube-sphere, migration du repère, passage jouable entre faces, chargement au-delà de la paire, LOD mixte ou planète entière. Les forces d'inertie, l'orientation IAU complète et les occultations détaillées restent omises. Suite limitée : [P3.3](ROADMAP.md), première migration bornée sur la même face ; prompt autonome dans [NEXT_CHAT](NEXT_CHAT.md).

## Essais intermédiaires conservés

Le [premier test](p32-validation/tests-initial.txt) échoue sur une requête ponctuelle : l'empreinte nulle rend le découpage polygonal dégénéré et perd un point par arrondi. La requête ponctuelle emploie maintenant une interpolation barycentrique ; les véritables empreintes continuent de découper les triangles. Un import inutilisé est retiré. Le [second test](p32-validation/tests-second.txt) passe. Le [premier Clippy](p32-validation/clippy-before-clippy-fix.txt) signale une condition imbriquée et une boucle de test indexée ; corrigés sans nouvelle dépendance. Les validations et le build précédant la précision d'entrée éclairée sont conservés sous `*-before-lit-entry` : ils ne représentent pas les sources finales.

Le premier parcours release complet, [journal](p32-validation/crossing-before-cursor-boundary.txt) et [22 captures](p32-validation/crossing-before-cursor-boundary/), termine avec code **0**, sept franchissements et apex **2,131371 m**. Il contient un ERROR et un WARN X11 au démarrage : le diagnostic demandait la capture du curseur alors que la fenêtre n'était pas encore visible. La phase initiale laisse maintenant le curseur libre ; la capture commence à la marche. Il montrait aussi des pieds correctement bornés aux deux limites X, avec une vitesse encore ±8 m/s : le bornage annule désormais la composante de vitesse bloquée, pour la paire et le site historique. Ces observations justifient ces petites corrections avant le lancement final. Sources/binaire de cet essai : [environment-before-cursor-boundary.json](p32-validation/environment-before-cursor-boundary.json) ; code et commande dans [graphics-results-before-cursor-boundary.json](p32-validation/graphics-results-before-cursor-boundary.json).

Captures intermédiaires effectivement inspectées : 00, 05 et 06 (entrée, repos sur raccord et saut). Les autres captures intermédiaires restent conservées ; aucune inspection exhaustive de cet essai n'est revendiquée.

---

# Historique : validation Solar Survival 0.2 — P3.1 cube-sphere

12 octobre 2026. Environnement : Ubuntu 24.04.5 LTS, X11 `DISPLAY=:1`, Rust stable 1.99.0, édition 2024, Bevy **exactement 0.19.1**, i7-6700K et GTX 1060 3 Go/NVK, Vulkan, Mesa 25.2.8. Git était propre au départ, commit `cfc745e` (P2 déjà commité), contrairement au prompt de reprise ancien. Aucune dépendance ni modification de Cargo.toml/Cargo.lock. Historiques P1/P2 préservés. Empreintes des sources/binaries : [environment.json](p3-validation/environment.json).

| Vérification finale | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check --locked` et `--all-targets` | Réussis, y compris l'exemple |
| `cargo test --locked` | **63 tests réussis** : 18 internes, 21 fondations, 15 surface, 9 cube-sphere |
| `cargo clippy --all-targets --locked -- -D warnings` | Réussi |
| `cargo build --release --locked` (jeu) | Réussi |
| `cargo build --release --locked --example cube_seam` | Réussi, reconstruction finale 1 min 43 s |
| Démonstration release `cube_seam`, 32 s | Code 0, quatre captures inspectées, aucun ERROR/WARN/panic |
| Régression FPS `SOLAR_ROTATION_SMOKE_SECONDS=50`, release | Code 0, dix captures, aucun ERROR/WARN/panic |
| Windows 11 / RTX 4050 et Core i5 | Non exécuté |

Journaux : [fmt](p3-validation/fmt.txt), [check](p3-validation/check.txt), [all-targets](p3-validation/check-all-targets.txt), [tests](p3-validation/tests.txt), [Clippy](p3-validation/clippy.txt), [build du jeu](p3-validation/release-game.txt), [build de la démo](p3-validation/release.txt), [démo](p3-validation/seams.txt). Les lancements utilisent `MESA_VK_WSI_PRESENT_MODE=immediate`, successivement et sans compilation concurrente. Commandes, répertoires et codes : [graphics-results.json](p3-validation/graphics-results.json).

## Contrat et vérification numérique

Le [contrat documenté](CUBE_SPHERE.md) est implémenté dans `planets/cube_sphere.rs`, réexporté par `planets` ; le terrain jouable et le contrôleur restent inchangés. Conversions face/u/v ↔ direction en f64, fractions locales de patch, voisins de même niveau et orientations des arêtes, puis paire de patches avec sommets/indices communs. Sphère au rayon SI du catalogue sans relief. Identité cubique entière et ordre déterministe des grilles : aucune soudure par tolérance. Deux patches seulement ; génération plafonnée à 64 segments, 8 385 sommets uniques et 16 384 triangles.

Neuf nouveaux tests d'intégration vérifient : conversions sur les six faces et leurs bords ; priorité des égalités de l'inverse et directions non normalisées extrêmes ; entrées invalides refusées avant allocation ; tous les voisins et arêtes orientées aux niveaux 0–3 ; niveau maximal 24 ; les huit coins avec trois positions exactement identiques chacun ; indices communs, rayon, triangles extérieurs et fermeture topologique du raccord ; maximum d'allocation et reproductibilité de la géométrie ; graines historiques avec trois valeurs de référence et distinction des clés valides. Les bords sont comparés par égalité exacte en f64, y compris avec 7 segments (non puissance de deux), et les incidences topologiques contrôlent les deux triangles de chaque segment partagé.

Les 54 tests antérieurs demeurent réussis, notamment contacts/streaming de neuf tuiles, horloges, repère tournant, M/G/O/U et menu. Ces vérifications distinguent géométrie numérique et systèmes ECS ; aucun nouveau test de périphériques OS n'est revendiqué.

## Inspection des quatre raccords

Toutes les captures finales ont été inspectées à 1440×900 : [même face](p3-validation/seams/seam-00.png), [+X/−Z](p3-validation/seams/seam-01.png), [+X/−Y avec inversion](p3-validation/seams/seam-02.png), [près du coin −Y/+X avec inversion](p3-validation/seams/seam-03.png). Les surfaces orange/bleue et leurs grilles rejoignent le même bord sans fissure visible ; textes et cadrage sont lisibles. La dernière vue contient deux patches touchant un coin, pas les trois faces entières ; l'identité des trois représentations du coin est vérifiée numériquement.

Chaque phase emploie 32 segments au niveau 15, soit 33 sommets partagés, 2 145 sommets uniques et 4 096 triangles. Longueurs polygonales des raccords consignées par le programme : **206,879 m**, **146,285 m**, **146,285 m**, **97,526 m**. Elles montrent la variation métrique de la projection, sans décider la résolution du terrain jouable.

Le rendu soustrait l'origine de la paire en f64 et applique un DQuat avant conversion locale bornée f32, en mètres 1:1. Deux meshes de surface et un mesh de lignes ont des handles réutilisés aux trois transitions. Le moteur possède aussi ses propres meshes : le journal observe sept assets au total après les transitions, dont les trois appartenant à la démo. Aucune géométrie n'est régénérée par frame. Grille relevée de 5 cm et trait du raccord de 10 cm, couleurs unlit : présentation de diagnostic, sans éclairage scientifique. La sortie est normale avec code 0. Ce scénario ne mesure ni performances ni contacts du joueur.

## Diagnostic initial conservé

Le premier essai graphique a enregistré [une capture](p3-validation/seams-before-fix/seam-00.png), puis terminé avec code **101** au passage à la deuxième phase. Une assertion comptait tous les meshes du moteur et exigeait trois, alors que Bevy en possédait sept : [journal](p3-validation/seams-before-fix.txt). La vérification corrigée contrôle les trois handles propres à la démo. Ce panic concernait le diagnostic, pas la géométrie ou le FPS. Le cadrage initial coupait le bas d'un patch et le trait du raccord se superposait à la grille ; distance de caméra et décalage du trait ont aussi été corrigés avant le lancement final.

Le [premier Clippy](p3-validation/clippy-before-fix.txt) a signalé deux usages de `chunks_exact(3)`, remplacés par `as_chunks::<3>()`. Le [premier build](p3-validation/release-before-demo-fix.txt) est réussi mais précède les corrections graphiques. Une reconstruction a été volontairement arrêtée par SIGTERM après lecture de la capture, pour intégrer le nouveau cadrage : [journal incomplet](p3-validation/release-interrupted-for-framing.txt), code 143. Cette interruption décidée ici ne renseigne pas la cause inconnue du code 143 historique P2.

## Régression du FPS tournant

Après la démo et sans compilation concurrente, le binaire normal exécute `SOLAR_ROTATION_SMOKE_SECONDS=50` depuis un nouveau répertoire, pour préserver les noms de captures historiques. [Journal](p3-validation/game.txt), [dix captures](p3-validation/game/docs/). Code 0, aucune occurrence ERROR/WARN/panic.

Captures effectivement inspectées : [repos à ×1000](p3-validation/game/docs/rotation-surface-00.png), [saut à ×1000](p3-validation/game/docs/rotation-surface-02.png), [course après changement de tuile](p3-validation/game/docs/rotation-surface-06.png), [repos pendant P](p3-validation/game/docs/rotation-surface-07.png), [retour spatial Mars](p3-validation/game/docs/rotation-surface-08.png), [Terre](p3-validation/game/docs/rotation-surface-09.png). Terrain, obstacles, HUD et sphères spatiales restent présents. Le journal montre la marche en pause au même JD, course à 8 m/s, pieds jusqu'à z = −151,007 m (au-delà de −128 m), puis vitesse nulle au repos. L'injection de commandes est ECS, sans essai des périphériques OS.

Ces vues affichent ponctuellement 21–24 FPS en surface, 40 FPS près de Mars et 51 FPS près de la Terre. Ce sont des relevés du HUD, pas un benchmark ou une mesure de la cible. La superposition des marqueurs et leur occultation approximative restent des limites historiques. Le cycle P2 complet et le menu n'ont pas été rejoués dans cette campagne ; leurs preuves précédentes et les tests ECS restent conservés.

## Limites et suite

P3.1 établit le contrat géométrique de même niveau/résolution. Aucun relief cube-sphere, contact jouable, migration du repère, passage jouable entre faces, LOD mixte ou planète entière n'est ajouté. Le FPS actuel garde ses neuf tuiles, la borne ±4 096 m, le repère tournant et les deux horloges. La démo est un exemple séparé, sans système installé en partie normale. Windows 11, la cible RTX 4050/Core i5 et la suspension système complète restent non validés. P3.2 préparera un premier franchissement optionnel de deux patches d'une même face et les contacts avec leurs triangles f64 dans un repère local unique.

---

# Historique : validation Solar Survival 0.2 — P2 et menu principal

12 octobre 2026. Environnement exécuté : Ubuntu 24.04.5 LTS, X11 `DISPLAY=:1`, Rust stable 1.99.0, Bevy **exactement 0.19.1**, i7-6700K, GTX 1060 3 Go/NVK, Vulkan et Mesa 25.2.8. Lancements successifs avec `MESA_VK_WSI_PRESENT_MODE=immediate`, sans compilation ni autre lancement du jeu concurrent. Aucun changement de dépendance, de Cargo.toml ou Cargo.lock. Empreintes des sources et du binaire : [p2-environment.json](p2-environment.json). Windows 11 et la cible RTX 4050/Core i5 ne sont pas validés sur ce poste.

| Vérification finale | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check --locked` | Réussi |
| `cargo test --locked` | **54 tests réussis** : 18 internes, 21 fondations, 15 surface |
| `cargo clippy --all-targets --locked -- -D warnings` | Réussi |
| `cargo build --release --locked` | Réussi, 1 min 59 s |
| `SOLAR_P2_SECONDS=160`, release | Code 0, 35 captures, aucun ERROR/WARN/panic |
| Menu avec entrées X11, options, crédits, reprise et deux suspensions de 12 s | 15 captures ; aucun ERROR/WARN/panic ; code final 143, cause non établie |
| Lancement séparé, clic X11 sur Quitter | Code 0, aucun ERROR/WARN/panic |
| Profilage de régression 30 s, variante msaa_off sans recorder GPU | Code 0, quatre phases, CSV valide, aucun ERROR/WARN/panic |
| Windows 11 / RTX 4050 et Core i5 | Non exécuté |

Journaux finaux : [fmt](validation-p2-fmt.txt), [check](validation-p2-check.txt), [tests](validation-p2-tests.txt), [Clippy](validation-p2-clippy.txt), [release](validation-p2-release.txt), [cycle P2](validation-p2-cycle.txt), [menu](validation-p2-menu.txt), [Quitter](validation-p2-menu-quit.txt). Les historiques sont conservés ci-dessous et leurs fichiers n’ont pas été écrasés.

## Modifications et couverture

P2 ajoute un scénario opt-in borné qui injecte les touches avant `camera::controls` : 96 s à ×1000, P, marche et saut en pause, P/T vers ×1/10/100/1000, puis deux séries de sorties M/G/O/U à différentes phases de rotation et retour final en surface. Le premier essai a révélé un défaut de lumière directe sur les parois nocturnes ; le Soleil sous l’horizon radial est désormais masqué dans le rendu local. Les 11 000 lux, l’ambiance et le brouillard restent des choix artistiques. Le masque n’est pas une occlusion détaillée du relief ni une photométrie scientifique : limitations dans [ASTRONOMY](ASTRONOMY.md). La simulation physique, les repères f64, les deux temps et les neuf tuiles restent inchangés.

Le premier essai et ses 35 images sont conservés dans [p2-before-horizon](p2-before-horizon/). Il montrait aussi une erreur de séquence du scénario : la première commande M partait déjà de la surface et inversait certaines entrées/sorties. Le scénario final conserve ce premier site à l’étape 18 puis exécute ses huit sorties réellement depuis la surface. Cet essai initial constitue un diagnostic, pas la validation finale des transitions.

Le menu est dans `ui/menu.rs` : Jouer/Reprendre, Options, Crédits, Quitter, sous-écrans réutilisés, cible UI sur la caméra active. Les options agissent sur les deux projections/MSAA, la sensibilité des deux modes de visée et le mode de fenêtre. Les marqueurs utilisent le FOV sélectionné. Défauts conservés : sensibilité ×1, FOV vertical 65°, MSAA ×4, fenêtré. Les réglages sont en mémoire pour la session ; aucune sauvegarde ni audio ajouté. Le menu bloque les commandes de navigation et libère le curseur ; calendrier et gravité continuent. Reprendre ne recrée ni site ni état physique. Les scénarios automatisés contournent le menu initial ; le profilage conserve la priorité sur les options graphiques. `SOLAR_CAPTURE_DIR` installe seulement sur demande la capture F12.

Les 48 tests antérieurs restent réussis. Six nouveaux vérifient : blocage des contrôles/capture sans arrêt de l’horloge dans le menu ; parcours Échap/crédits/reprise ; application et reset des options aux deux caméras et à la fenêtre ; interruption manuelle ECS de 12 s avec avance de 12 000 s du calendrier et 0,25 s de physique locale ; frontières du cycle P2 ; exécution réelle des huit sorties via le scénario et les contrôles. Le test de rendu existant vérifie désormais aussi extinction et retour de la lumière directe, avec les mêmes meshes et poses locales pendant la rotation. Les tests ECS ne constituent pas des tests des périphériques OS.

## Cycle complet et transitions inspectés

Les douze captures [00](p2-stage-00.png) à [11](p2-stage-11.png) ont toutes été inspectées. Elles couvrent la journée, l’entrée dans la nuit, la nuit, l’aube et le retour au jour. Le JD atteint environ J2000 + 1,08841 jour à la capture 11, au-delà de la période sidérale de 1,02595676 jour. Le site conserve sa révision 1, ses pieds et obstacles pendant tout le cycle. La direction solaire locale passe d’une composante verticale positive à négative puis positive. Comparaison de la paroi éclairée sous horizon [avant correction](p2-before-horizon/p2-stage-04.png) et [après correction](p2-stage-04.png) : la face lumineuse nocturne a disparu. La nuit reste volontairement visible sous l’ambiance artistique. Il s’agit d’un échantillonnage sur un cycle entier, pas d’une vidéo de chaque frame.

[Marche pendant P](p2-stage-12.png) et [saut pendant P](p2-stage-13.png) effectivement inspectés : même JD 2451546,11059492, marche 4,50 m/s, saut à environ 1,96 m au-dessus du support. Les étapes 14–17 du journal reprennent à ×1 puis ×10/100/1000. Les quatre sorties de la première série [M](p2-stage-19.png), [G](p2-stage-21.png), [O](p2-stage-23.png), [U](p2-stage-25.png), celles de la seconde série [M](p2-stage-27.png), [G](p2-stage-29.png), [O](p2-stage-31.png), [U](p2-stage-33.png), puis [le retour final au site](p2-stage-34.png), ont été inspectés. Les modes et repères attendus sont aussi consignés dans le journal ; la sphère réapparaît en espace et les obstacles en surface.

Les captures du cycle donnent ponctuellement environ 22–25 FPS au repos de surface, 19 FPS pendant la marche en pause et 42–44 FPS dans les sorties spatiales inspectées. Ce sont des valeurs du HUD, **pas un nouveau benchmark**, ni une prévision de la cible RTX 4050. Aucun réglage de qualité par défaut n’a été réduit. Les marqueurs peuvent encore se superposer en vue globale ou être dessinés devant un obstacle : leur occultation reste approximative et préexistante.

## Menu et interruption réelle

L’essai interactif utilise XTest/libX11 depuis un script temporaire de validation, sans dépendance ajoutée au jeu ; les entrées passent par les périphériques X11 puis Winit/Bevy. Actions horodatées : [validation-p2-menu-actions.txt](validation-p2-menu-actions.txt). La sélection initiale de la décoration de fenêtre a été corrigée vers le client ; certains premiers clics ont été répétés pendant la mise au point du pilote. Les captures et journaux sont tous conservés. Ce pilote temporaire Ubuntu ne constitue pas une API de gameplay ni une validation Windows.

Écrans effectivement inspectés : [menu initial](p2-menu/capture-00.png), [options](p2-menu/capture-04.png), [options modifiées et plein écran 1920×1080](p2-menu/capture-06.png), [reset et retour fenêtre 1440×900](p2-menu/capture-07.png), [crédits](p2-menu/capture-08.png), [menu Reprendre en surface](p2-menu/capture-13.png), [surface après reprise](p2-menu/capture-14.png). Textes lisibles et commandes fonctionnelles aux deux tailles inspectées. Les petites fenêtres, autres DPI et la navigation au clavier par flèches n’ont pas été validés ; Entrée et Échap fonctionnent, les options se sélectionnent à la souris.

Une première injection du keysym `m` sur le clavier AZERTY n’a pas déclenché le `KeyM` physique et l’interruption a donc eu lieu en espace : [trace conservée](p2-interruption-space.json), capture 10. L’entrée de surface a ensuite été faite avec le code physique approprié. Ce décalage concerne le pilote de test, pas une modification du mapping WASD/ZQSD du jeu.

La deuxième suspension SIGSTOP/SIGCONT dure **12,0002 s**, effectuée après un saut en surface à ×1000 : [horodatage](p2-interruption.json). [Avant](p2-menu/capture-11.png), le joueur est au sol sur Mars. [Immédiatement après la reprise](p2-menu/capture-12.png), effectivement inspecté, il est toujours en l’air à environ 2,12 m du support, vitesse relative au site 0,32 m/s ; le calendrier a avancé et le Soleil local a changé. Le mouvement reste cohérent avec la physique locale plafonnée plutôt qu’une chute rattrapée sur douze secondes. Les captures avant/après incluent aussi le temps d’interaction ; elles ne mesurent pas isolément 12 000 s d’avance orbitale. Cette valeur et la pose synchronisée après interruption sont vérifiées par le test ECS. Une mise en veille complète de l’OS n’a pas été exécutée.

L’essai interactif long a renvoyé **143** après les dernières captures et le clic final, sans diagnostic d’erreur moteur ; sa cause n’est pas établie, et il n’est pas présenté comme une fermeture normale. Un nouvel essai court sur le même binaire, avec clic X11 sur **Quitter**, a terminé avec **code 0**, sans erreur. Les journaux des deux essais sont conservés.

## Régression du profilage

Un lancement release séparé de 30 s, `SOLAR_PROFILE_GPU=0`, variante `msaa_off`, a contourné le menu et effectué repos, contact, streaming et sortie spatiale. Code 0, aucun ERROR/WARN/panic, quatre phases et valeurs finies/non négatives dans le [CSV](p2-profile-smoke.csv) ; [journal](validation-p2-profile-smoke.txt). Le joueur a franchi x = 128 m. Cet essai vérifie le scénario/export après ajout du menu et la priorité de la variante graphique ; il n’est pas une nouvelle campagne P1 et n’établit pas un gain de performance.

## Décision et limites

P2 est terminé sur ce poste, avec cycle complet inspecté, défaut observé corrigé, P/T, sorties répétées, interruption réelle et tests ciblés. Le menu demandé est opérationnel. La prochaine étape limitée est P3.1 : contrat géométrique et raccords entre patches cube-sphere, avant tout franchissement jouable. Voir [ROADMAP](ROADMAP.md) et [NEXT_CHAT](NEXT_CHAT.md).

Le site reste ±4 096 m et neuf tuiles, les forces d’inertie sont omises, l’orientation et les textures sont conventionnelles/artistiques. L’horizon nocturne idéal, l’absence d’ombres de relief, l’occultation astronomique compressée et les marqueurs gardent leurs limites. Les réglages ne sont pas persistés. Windows 11, RTX 4050/Core i5, autres DPI et suspension système complète restent à valider dans leurs environnements propres.

---

# Historique : validation Solar Survival 0.2 profilage et roadmap

12 octobre 2026. Environnement exécuté : Ubuntu 24.04.5 LTS, X11 `DISPLAY=:1`, Rust stable 1.99.0, Bevy **exactement 0.19.1**, Intel i7-6700K à 4 GHz, NVIDIA GTX 1060 3 Go/NVK, Vulkan et Mesa 25.2.8. C'est le **poste de développement** ; la cible déclarée par l'utilisateur est RTX 4050 avec Intel Core i5. Ses performances et Windows 11 restent à valider nativement sur cette machine.

| Vérification sur les sources finales | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check --locked` | Réussi |
| `cargo test --locked` | **48 tests réussis** : 12 internes, 21 fondations, 15 surface |
| `cargo clippy --all-targets --locked -- -D warnings` | Réussi |
| `cargo build --release --locked` | Réussi, 1 min 59 s |
| `python3 scripts/profile_suite.py --help` | Réussi |
| Huit essais release de profilage, successifs, 42 s chacun | Codes 0, quatre phases par essai, aucun ERROR/WARN/panic |
| Scénario graphique normal sans profiler, rotation surface 50 s | Code 0, dix captures |
| Windows 11 / RTX 4050 et Core i5 | Non exécuté, aucun hôte cible disponible |

Journaux : [fmt](validation-profile-fmt.txt), [check](validation-profile-check.txt), [tests](validation-profile-tests.txt), [Clippy](validation-profile-clippy.txt), [release](validation-profile-release.txt).

## Changements et couverture

Le profilage est optionnel dans `config/profiling.rs` ; il n'installe aucun système ni recorder GPU sans `SOLAR_PROFILE_SECONDS`. Le scénario compare repos, contact, streaming et visite spatiale dans une scène fixe, avec délais d'échauffement et de stabilisation exclus. Les variantes conservent la simulation et modifient seulement les caméras, résolution, MSAA ou matériaux pendant la mesure. Le fonctionnement normal et les contraintes f64/GameSet/temps réel restent conservés. Aucune dépendance ajoutée, Cargo.toml et Cargo.lock inchangés.

Les 46 tests précédents restent réussis. Deux tests couvrent les frontières temporelles du protocole et l'absence de ressources/systèmes de profilage dans une partie normale. La campagne réelle vérifie les variantes graphiques et l'export CSV ; les mesures et leurs limites figurent dans [PERFORMANCE](PERFORMANCE.md).

La [ROADMAP](ROADMAP.md) est la référence de développement, avec critères de fin par étape et distinction poste/cible. AGENTS demande maintenant de la lire en début de chat et de préparer [NEXT_CHAT](NEXT_CHAT.md) en fin de chat. P1 est terminé comme diagnostic sur le poste de développement ; P2, consolidation du prototype tournant, est la prochaine étape. Les étapes de terrain, survie, construction et distribution restent prévues, pas implémentées.

## Campagne et inspection graphique

La campagne [`scripts/profile_suite.py --seconds 42 --mesa-immediate --gpu-probe`](../scripts/profile_suite.py) a exécuté huit essais, sans compilation ni autre essai GPU concurrent. Les [CSV bruts, journaux et métadonnées](profiles/dev-nvk-2026-10-12/) et la [synthèse](profiles/dev-nvk-2026-10-12/summary.csv) sont conservés. Les données orientent vers le budget de rendu et la composition MSAA de deux vues ; les limites de couverture CPU et des chemins GPU partagés sont détaillées dans PERFORMANCE. Ces différences ne constituent pas une mesure de la RTX 4050. Chaque variante a une seule répétition, sauf baseline sans recorder GPU qui est répété en fin de campagne.

Après la campagne, un lancement release normal avec `SOLAR_ROTATION_SMOKE_SECONDS=50`, présentation Mesa immediate et **sans profiler**, a produit dix captures et terminé avec le code 0. Le [journal](validation-profile-normal.txt) ne contient aucun ERROR, WARN ou panic. Captures effectivement inspectées : [surface au repos](profile-normal-surface-00.png), [repos en pause après la course](profile-normal-surface-07.png), [retour spatial Mars](profile-normal-surface-08.png). Le terrain, les obstacles, l'UI et la réapparition de la sphère martienne restent visibles. Les captures historiques rotation-surface ont été restaurées, les nouvelles copies portent le préfixe profile-normal-surface. Cette inspection confirme le rendu normal après ajout du profiler ; les essais de comparaison mesurent le fonctionnement graphique mais ne valident pas la qualité visuelle des variantes qui retirent une partie de l'image.

---

# Historique : validation Solar Survival 0.2 repère tournant — 12 octobre 2026

Environnement exécuté : **Ubuntu 24.04.5 LTS**, session X11 (`DISPLAY=:1`), Rust stable **1.99.0**, édition 2024, Bevy **exactement 0.19.1**. Le lancement release sélectionne **NVIDIA GeForce GTX 1060 3GB (NVK GP106)**, Vulkan, **Mesa 25.2.8-0ubuntu0.24.04.2**. Aucune dépendance ni version ajoutée/modifiée ; `Cargo.toml` et `Cargo.lock` sont inchangés.

| Vérification exécutée sur les sources finales | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check --locked` | Réussi |
| `cargo test --locked` | **46 tests réussis** : 10 internes, 21 fondations, 15 surface |
| `cargo clippy --all-targets --locked -- -D warnings` | Réussi, aucune alerte |
| `cargo build --release --locked` | Réussi, 1 min 54 s |
| `MESA_VK_WSI_PRESENT_MODE=immediate SOLAR_ROTATION_SMOKE_SECONDS=50 RUST_LOG=info cargo run --release --locked` | Sortie 0, dix captures |
| `MESA_VK_WSI_PRESENT_MODE=immediate SOLAR_SMOKE_SECONDS=66 RUST_LOG=info cargo run --release --locked` | Sortie 0, onze captures spatiales |
| Windows 11 natif | **Non exécuté** : aucun hôte Windows disponible |

Journaux : [fmt](validation-rotation-fmt.txt) (vide car réussi), [check](validation-rotation-check.txt), [tests](validation-rotation-tests.txt), [Clippy](validation-rotation-clippy.txt), [release](validation-rotation-release.txt), [surface tournante](validation-rotation-surface.txt), [espace](validation-rotation-space.txt).

## Architecture et couverture

La rotation conventionnelle f64 est désormais dans `CelestialBody::rotation`, partagée par le rendu des planètes et le suivi de surface. Le site fixe appartient aux axes tournants de Mars ; ses coordonnées, obstacles et meshes ne sont pas régénérés pour la rotation. `SurfaceState` conserve l’état local autoritaire. La navigation publie la pose globale après la physique, même sans sous-pas, puis l’origine flottante. La caméra locale conserve les différences métriques f64 dans le site ; la lumière est convertie depuis la direction globale du Soleil. L’ordre des `GameSet`, les visites spatiales translatoires, l’horloge orbitale et le voisinage de neuf tuiles restent conservés. Formules et limites : [ASTRONOMY](ASTRONOMY.md), utilisation : [README](../README.md).

Les 40 tests précédents restent réussis. Les six nouveaux couvrent :

- Convention de rotation sur les dix corps : phase J2000, inclinaison, période signée, rotations rétrogrades, temps négatif et retour après une période.
- Entrée côté éclairé à plusieurs époques, dont une date éloignée de J2000.
- Pose au repos et en l’air entraînée avec les obstacles, distances et orientations relatives conservées, sans mutation de pieds/vitesse/site/révision, y compris après 1 000 rotations.
- Mise à jour de la pose, de l’origine flottante et de la caméra spatiale sur une frame sans sous-pas physique.
- Trajectoires locales identiques pendant marche et saut entre P et ×1000, reprise par P et changement de vitesse par T.
- Systèmes réels de rendu local : caméra, tuiles et obstacle inchangés en coordonnées de rendu pendant une rotation ; direction lumineuse attendue, neuf meshes et mêmes handles conservés.

Le test existant M/G/O/U vérifie désormais les sorties après suivi tournant sous ×1000, puis une visite terrestre. Les tests préexistants couvrent encore gravité, contacts, stabilité au repos, saut, limites du site, streaming et déplacements à différentes fréquences. Les essais ECS n’injectent pas les périphériques au niveau de l’OS.

## Inspection graphique de surface

Lancement release réalisé **sans compilation concurrente**, avec le contournement Mesa immediate déjà documenté. Les dix captures sont produites, sortie normale 0, aucun ERROR, WARN ou panic dans le journal. Captures effectivement inspectées : [repos sous ×1000](rotation-surface-00.png), [saut sous ×1000](rotation-surface-02.png), [marche pendant P](rotation-surface-03.png), [course après changement de tuile](rotation-surface-06.png), [repos pendant P](rotation-surface-07.png), [retour spatial Mars](rotation-surface-08.png), [Terre après sortie](rotation-surface-09.png).

Le scénario alterne ×1000 aux étapes 0–2, pause à 3, ×1000 à 4, ×1 à 5, ×1000 à 6, pause à 7, puis ×1 aux sorties 8–9. Le journal montre le JD conservé à 2451545,17352902 pendant la marche en pause ; la capture 03 affiche 4,50 m/s. La course reste à 8 m/s sous ×1000, puis les pieds atteignent z = −151,090 m, au-delà de la limite de tuile −128 m. La capture 07 affiche une vitesse relative au site nulle après libération des commandes. Le JD atteint environ J2000 + 0,290 jour ; la direction solaire évolue et le sol apparaît moins éclairé en fin de parcours. L’ambiance et le brouillard restent artistiques : ces vues ne valident pas une photométrie scientifique ni un cycle jour/nuit complet.

Les vues de surface inspectées affichent **22–24 FPS** ; les sorties spatiales Mars et Terre affichent respectivement 34 et 51 FPS. Ces relevés ponctuels du HUD sont proches de la validation précédente, **sans établir un benchmark ni une amélioration de performance**. Aucun timeout de présentation n’apparaît dans ce lancement. L’ancien diagnostic NVK/VSync et ses journaux sont conservés dans l’historique ci-dessous ; aucun ralentissement n’est attribué ici au code du repère.

## Régression graphique spatiale

Le second lancement release visite les dix corps puis la vue globale, sans compilation ni lancement GPU concurrent. Il termine avec le code 0, onze captures et aucun ERROR, WARN ou panic. Les nouvelles captures ont été conservées sous `rotation-space-00.png` à `rotation-space-10.png` après le lancement, qui écrit normalement les noms `v02-space-*` ; les captures historiques `v02-space-*` ont été restaurées sans modification.

Vues effectivement inspectées : [Terre](rotation-space-00.png), [Saturne et anneaux](rotation-space-03.png), [Lune](rotation-space-06.png), [Soleil](rotation-space-09.png), [vue globale](rotation-space-10.png). Les sphères, leur éclairage, les anneaux, le halo et les marqueurs fonctionnent ; la superposition de marqueurs en vue globale reste une limite préexistante. Ces vues affichent 36–52 FPS, sans mesure de performance dédiée.

## Comportement et limites

P fige orbite et rotation, mais pas marche, gravité, saut ni visée. T accélère orbite et rotation à partir du même calendrier ; la physique locale reste en secondes réelles. Le joueur reste dans le site tournant même en l’air. Coriolis, centrifuge, Euler, accélération translatoire orbitale et marées sont omises, ainsi que les impulsions inertiales lors des changements P/T. Les vitesses sont relatives au site ; les téléportations M/G/O/U ne transfèrent aucune vitesse inertielle. Après une longue interruption, la pose suit le JD courant, tandis que le contrôleur n’accepte que 0,25 s de physique locale par frame.

L’orientation réutilise la phase et le pôle conventionnels du prototype ; elle n’est pas IAU exacte. Le site reste borné à ±4 096 m, sans réseau mondial cube-sphere, grottes ou corps dynamiques. La profondeur astronomique compressée et ses occultations restent approximatives. Les tests graphiques n’équivalent pas à des essais manuels des touches/clics. **Windows 11 n’a été ni compilé ni lancé depuis cet environnement Linux.**

---

# Historique : validation Solar Survival 0.2 translatoire — 12 octobre 2026

Environnement exécuté : **Ubuntu 24.04.5 LTS**, session X11 (`DISPLAY=:1`), Rust stable **1.99.0**, édition 2024, Bevy **exactement 0.19.1**. Le lancement release sélectionne **NVIDIA GeForce GTX 1060 3GB (NVK GP106)**, Vulkan, **Mesa 25.2.8-0ubuntu0.24.04.2**. Aucune dépendance ajoutée ; `Cargo.lock` ne change que la version de l'application (0.2.0).

| Vérification exécutée | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check --locked` | Réussi |
| `cargo test --locked` | **40 tests réussis** : 7 tests internes, 21 fondations, 12 tests de surface |
| `cargo clippy --all-targets --locked -- -D warnings` | Réussi, aucune alerte |
| `cargo tree -i bevy --depth 1` | Bevy 0.19.1 confirmé, une seule version |
| `cargo build --release --locked` | Réussi sur les sources finales, 1 min 55 s |
| `SOLAR_SURFACE_SMOKE_SECONDS=50 RUST_LOG=info cargo run --release --locked` | Réussi avec présentation Mesa immediate : 10 captures, sortie normale |
| `SOLAR_SMOKE_SECONDS=66 RUST_LOG=info cargo run --release --locked` | Réussi avec présentation Mesa immediate : 11 captures, sortie normale |
| Windows 11 natif | **Non exécuté** : aucun hôte Windows disponible |

Les deux scénarios release finaux ajoutent **`MESA_VK_WSI_PRESENT_MODE=immediate`** à leur environnement. Ce réglage de diagnostic est propre au pilote Mesa ; aucune branche Linux n'est introduite dans la logique du jeu.

Journaux : [check](validation-v02-check.txt), [tests](validation-v02-tests.txt), [Clippy](validation-v02-clippy.txt), [release](validation-v02-release.txt), [surface](validation-v02-surface.txt), [espace](validation-v02-space.txt).

## Couverture nouvelle

Les 23 tests précédents restent réussis. Les 17 nouveaux tests couvrent : chute de 20 m puis repos sans dérive, gravité et hauteur du saut martien, contacts du volume avec murs/plafonds/boîtes et glissement, réception stable sur une boîte et saut depuis son dessus, vitesse résolue nulle devant un obstacle, déplacement à 30/60/144 FPS, saut mis en attente entre deux pas physiques sans répétition, arrêt des commandes sans arrêt de la gravité, limitation des longues interruptions, limite du site, contacts continus aux coordonnées négatives et limites de cellules, verticale et hauteur de caméra dans un site arbitrairement orienté, concordance du mesh avec ses sommets physiques et raccords entre tuiles.

Les tests ECS vérifient aussi les quatre sorties de surface (M/G/O/U), la reprise d'une visite terrestre, l'indépendance de la physique vis-à-vis de P/T et l'absence de multiplication des deltas souris par le delta time. Le test de chargement vérifie neuf meshes résidents, leur réutilisation sans déplacement, six meshes conservés après franchissement d'une tuile, puis libération totale en quittant la surface.

Le contrôleur et l'architecture sont décrits dans [README](../README.md) et [ASTRONOMY](ASTRONOMY.md). Les attentes Clippy nouvelles concernent uniquement les paramètres et filtres d'emprunt des systèmes ECS.

## Inspection graphique finale

Les deux lancements release finaux ont produit leurs 21 captures attendues et terminé avec le code de sortie 0. Leurs journaux ne contiennent aucun ERROR, WARN ou panic. Le scénario surface montre l'arrêt devant la boîte, le saut, la course jusqu'à z = −150,969 m (franchissement de la limite de tuile z = −128 m), le repos après libération des commandes, puis les visites spatiales Mars et Terre. Le scénario spatial visite les dix corps et la vue globale.

Captures représentatives effectivement inspectées : [site au repos](v02-surface-00.png), [approche/contact](v02-surface-01.png), [saut](v02-surface-02.png), [course après changement de tuile](v02-surface-06.png), [retour spatial Mars](v02-surface-08.png), [Terre après sortie](v02-surface-09.png), [Saturne et anneaux](v02-space-03.png), [Lune](v02-space-06.png), [Soleil](v02-space-09.png) et [vue globale](v02-space-10.png). Le terrain apparaît en 3D, les obstacles restent à l'échelle humaine, l'aide et le HUD changent avec le mode et les sphères spatiales réapparaissent après sortie. Des marqueurs se superposent encore dans la vue globale, limitation préexistante.

Ces captures affichent environ **22–23 FPS en surface** et **42–55 FPS sur les vues spatiales inspectées**, avec la présentation Mesa immediate et sans compilation concurrente. Ce sont des valeurs ponctuelles du HUD, pas un benchmark ; **60 FPS n'est pas atteint ni revendiqué**. Le coût des deux caméras et le profilage GPU/CPU restent à examiner.

## Diagnostic de présentation sur cette session

Le premier essai release en présentation AutoVsync a montré environ **1 FPS**, avec un timeout d'acquisition de texture dans le lancement spatial et une capture asynchrone non reçue avant sa fermeture. À ce rythme, le plafonnement de 0,25 s par frame ralentissait volontairement la physique : cet essai ne validait pas le parcours prévu. Les journaux sont conservés : [surface VSync](validation-v02-surface-vsync.txt), [espace VSync](validation-v02-space-vsync.txt).

Le nouvel essai avec `MESA_VK_WSI_PRESENT_MODE=immediate`, [variable officielle Mesa](https://docs.mesa3d.org/envvars.html), a rétabli environ **23 FPS** sur la première capture de surface. Le journal montre ensuite l'arrêt devant l'obstacle (z = −10,698 m, vitesse nulle), puis le saut. Cela indique un problème de présentation dans les conditions de cette session ; ce n'est pas une preuve générale de la cause ni un benchmark. La configuration normale du jeu conserve AutoVsync. Le contournement est décrit dans le README ; Windows n'a pas été testé.

## Limites du prototype et de la validation

Le terrain est une grille locale de hauteur courbée sur la sphère moyenne, **pas encore un réseau mondial cube-sphere**. Le site translatoire est borné à ±4 096 m ; pas de rotation du terrain, de corps dynamiques, de grottes ni de pentes arbitraires. Le joueur utilise une boîte alignée sur le site ; sa caméra et la gravité restent radiales. La marche impose l'adhérence sur le relief doux. Les collisions avec les trois obstacles sont balayées ; les contacts terrain reposent sur les triangles f64 sous l'empreinte du joueur. Les sous-pas de 1/120 s acceptent au plus 0,25 s par frame, sans rattrapage des longues interruptions.

Les scénarios graphiques injectent des commandes dans l'ECS ; ils ne simulent pas les touches, souris ou clics au niveau de l'OS. Les captures servent à vérifier le rendu 3D et les transitions, pas à prouver tous les contacts physiques. Pas de validation native Windows 11 ni de benchmark dédié à 60 FPS. Le rendu astronomique compressé garde ses limites d'occultation et d'éclairage. La survie reste inactive.

---

# Historique : validation Solar Survival 0.1 — 12 octobre 2026

Environnement réel : Ubuntu Linux, session X11, Rust stable 1.99.0, Cargo 1.99.0, édition Rust 2024, Bevy **0.19.1**. GPU sélectionné par wgpu : **NVIDIA GeForce GTX 1060 3GB (NVK GP106)**, Vulkan, pilote **Mesa 25.2.8**, nouveau/NVK. VS Code : rust-analyzer et Even Better TOML installés. Aucun paquet système avec sudo n'a été nécessaire pour compiler et lancer sur ce poste.

| Vérification exécutée | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check` | Réussi |
| `cargo check --locked` | Réussi |
| `cargo test` | **23 tests réussis** : 2 tests du contrôleur ECS + 21 tests des fondations |
| `cargo clippy --all-targets -- -D warnings` | Réussi, aucune alerte non traitée |
| `cargo tree -i bevy --depth 1` | Bevy 0.19.1 confirmé |
| `cargo build --release` final | Réussi sur les sources finales (2 min 05 s) |
| `SOLAR_SMOKE_SECONDS=66 cargo run` | Réussi : lancement graphique, onze vues capturées, sortie normale |
| `SOLAR_SMOKE_SECONDS=66 cargo run --release` final | Réussi : onze étapes, captures PNG, sortie normale |
| Windows 11 natif | **Non exécuté** : aucun hôte Windows dans cet environnement |

Les signatures à huit paramètres de systèmes Bevy et le ParamSet de textes possèdent des attentes Clippy documentées : ce sont des paramètres ECS injectés, pas des appels métier à huit arguments.

## Tests et vérifications

[Sortie des tests](validation-tests.txt) et [sortie Clippy](validation-clippy.txt), [compilation release](validation-release.txt) et [journal de lancement release](validation-graphics.txt).

Les tests couvrent unités SI/UA, distances à plusieurs milliers de milliards de mètres, résidu de Kepler, inclinaisons, périastre/apoastre, périodes, catalogue et dimensions, barycentre Terre–Lune, orbite lunaire, stabilité sur la plage JPL, dates limites grégoriennes, repères rigides et conversions, préservation angulaire du proxy, pause/accélération temporelle, collision balayée supraluminique, gravité radiale, visites et zones cube-sphere reproductibles.

Les deux tests du contrôleur exécutent les vrais systèmes de caméra Bevy dans une App avec temps manuel : déplacement à 250 m/s pendant une seconde à 60 et 144 FPS, sélection de Mars, visite, capture puis libération du curseur. Ils ne simulent pas de touches au niveau de l'OS et n'équivalent pas à un test manuel de périphérique. Les clics sur les boutons n'ont pas fait l'objet d'un test manuel automatisé au niveau du système d'exploitation.

## Inspection graphique réalisée

Le lancement de validation visite les dix corps, puis la vue globale. Onze captures ont été produites et inspectées : Terre, Mars, Jupiter, Saturne, Uranus, Neptune, Lune, Mercure, Vénus, Soleil, système. Pas de panic, crash ou erreur de shader signalés dans le lancement de 66 secondes. Le renderer utilise bien le GPU discret Vulkan, et non un affichage console ou un schéma 2D.

| Capture | Vue |
|---|---|
| [screenshot.png](screenshot.png) | Terre |
| [smoke-01.png](smoke-01.png) | Mars |
| [smoke-02.png](smoke-02.png) | Jupiter |
| [smoke-03.png](smoke-03.png) | Saturne et anneaux |
| [smoke-04.png](smoke-04.png) | Uranus |
| [smoke-05.png](smoke-05.png) | Neptune |
| [smoke-06.png](smoke-06.png) | Lune et Terre distante |
| [smoke-07.png](smoke-07.png) | Mercure |
| [smoke-08.png](smoke-08.png) | Vénus |
| [smoke-09.png](smoke-09.png) | Soleil émissif avec halo |
| [smoke-10.png](smoke-10.png) | Vue globale et marqueurs |

Le test graphique a permis de corriger les accents manquants de la police initiale, d'aligner les sphères UV Z-up de Bevy avec les rotations planétaires Y-up et de réduire le halo sur les planètes non émissives. Les tests ECS ont permis de corriger la hausse de vitesse à molette immobile (`signum(0.0)` valait 1). La borne du calendrier a été recalculée et vérifiée indépendamment.

Les captures de développement indiquaient environ 23–50 FPS suivant la vue, pendant une compilation release concurrente. Il ne s'agit pas d'un benchmark dédié ; l'objectif 60 FPS n'est pas déclaré atteint sur ce poste. Le lancement release sans compilation concurrente a également fonctionné ; les captures Terre/Mars montrent environ 43–46 FPS sur ce pilote. Le seuil de 60 FPS reste à atteindre par profilage GPU/CPU et mesures dédiées.

## Limites restant à valider

Pas de FPS de surface, de terrain collisionnable ou de survie active : ces fonctionnalités appartiennent à la prochaine étape Mars. Pas d'éphéméride lunaire réelle, de cartographie terrestre réelle, de photométrie physique ni d'occultations exactes après compression du rendu. Des marqueurs peuvent se superposer dans la vue globale. Le prototype utilise des sphères et textures artistiques, avec une simulation physique 1:1 distincte.

La compatibilité Windows est prévue par les dépendances et le code portable ; sa compilation et son lancement natifs doivent encore être vérifiés sur Windows 11. La session graphique Linux a été effectivement testée.
