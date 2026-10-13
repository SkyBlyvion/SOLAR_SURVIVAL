# Mars traversable : LOD adaptatif et streaming local

13 octobre 2026. Le parcours normal et le profiler mondial utilisent maintenant trois **résolutions réellement choisies selon la distance** : 32, 16 et 8 cellules par côté. Les clés spatiales restent au niveau 15, avec un voisinage de distance de graphe ≤2, au plus treize patches. Cette optimisation conserve Mars sphérique sans relief, les mètres 1:1 et les poses/contacts f64. Elle ne génère ni une planète entière ni un mesh par frame.

## Sélection et contacts fins

À l’entrée et au rebasage de 64 m, la direction des pieds détermine l’ancre du streaming. La distance employée est une **borne inférieure** de distance corporelle au patch entier : distance à son centre sphérique moins sa demi-diagonale conservative. La normalisation du cube est Lipschitz sur le rectangle de face ; on divise sa demi-diagonale cubique `sqrt(2) × R / count` par la norme minimale du rectangle. Les seuils sont des choix numériques, sans nouvelle donnée astronomique.

Fin à distance ≤80 m ; une résolution fine déjà résidente reste fine jusqu’à 110 m. Moyen à ≤160 m ; une résolution moyenne déjà résidente reste moyenne jusqu’à 200 m. Au-delà : 8 cellules. Une approche peut passer directement de 8 à 32. Les distances se rapportent à l’ancre choisie à la dernière sélection, donc au plus 64 m horizontalement du joueur entre deux rebasages. Aucun choix ni reconstruction au repos, pendant la rotation, la visée ou le menu. L’hystérésis conserve les niveaux dans leurs bandes.

Les requêtes restent dans le disque de 66 m, demi-largeur joueur ≤1 m et construction ≤5 m. Leur distance corporelle reste sous 80 m : tous les triangles interrogés gardent **exactement la triangulation 32** précédente. Le support, les extrema et le certificat d’aire sous l’empreinte entière restent les mêmes, à tolérance arithmétique 10⁻⁹ m². Pas de sol lisse distinct du mesh ni contact grossier sous le joueur.

L’index XZ 8 m ne conserve que les triangles rencontrant le carré ±72 m, avec cases coupées à ce carré (66+5 m et réserve 1 m). Il n’indexe plus les patches lointains inutiles aux requêtes. Les géométries corporelles demeurent `Arc` ; seuls les triangles locaux utiles et leurs références de cases sont préparés au rebasage.

## Raccords de résolutions mixtes

Chaque patch conserve les quatre bords à **32 segments** produits par `PatchKey::vertex`. Les intérieurs grossiers utilisent une sous-grille de cette même identité cubique. Dans les cellules adjacentes au bord, un éventail orienté vers l’extérieur relie tous les sommets fins du contour au centre sphérique de la cellule. Les cellules intérieures ont deux triangles. Tous les sommets sont réellement utilisés ; le mesh grossier ne conserve pas un tableau complet de sommets fins.

Les deux représentations d’une arête, y compris inversée entre faces, utilisent la même polyligne f64. Les trois faces d’un coin partagent le même sommet. Aucune jupe, soudure approximative ni T-junction. La couture n’exige pas de reconstruire un voisin lorsque sa résolution change. Le niveau spatial est fixe ; cette implémentation fournit un **LOD de résolution**, pas un quadtree à tailles de patches variables.

Sur cette sphère sans relief, la flèche du plan de tout triangle grossier testée reste <0,1 mm, bien sous la peau de contact de 2 mm ; les changements lointains sont donc directs. La zone proche ne change pas de triangulation. Ce choix ne vaut pas pour un futur relief : ses transitions et ses erreurs devront être évaluées avant intégration.

## Assets, budgets et instrumentation

| Résolution intérieure | Sommets par patch | Triangles par patch | Bord |
|---|---:|---:|---|
| 32 | 1 089 | 2 048 | 32 segments |
| 16 | 413 | 696 | 32 segments |
| 8 | 205 | 280 | 32 segments |

Maximum résident conservateur : treize meshes, 14 157 sommets et 26 624 triangles (tous fins). Ancien et nouveau fournisseur : ≤26 occurrences, ≤28 314 sommets/53 248 triangles ; les géométries communes à clé et résolution identiques sont partagées. Réservations de vecteurs corporels dimensionnées à leurs comptes exacts. L’index local a au plus 361 cases ; les maxima de triangles/références observés sur 150 directions et les mesures release sont dans VALIDATION/PERFORMANCE. Ces comptes ne mesurent pas la RAM/VRAM exhaustive ni les buffers temporaires internes de Bevy.

Le cache GPU conserve l’entité et le handle d’une clé commune, même quand sa résolution change. Il remplace alors seulement le contenu du mesh. Les slots sortants servent aux entrants ; les assets sont libérés en espace. Les meshes sont exprimés dans leur repère propre : soustraction d’origine corporelle f64, rotation DQuat puis conversion f32 bornée. Aucun proxy ne modifie les données physiques.

Télémétrie : nombres de patches 32/16/8, triangles résidents et changements de résolution. `RUST_LOG=info,solar_survival::player::surface=debug,solar_survival::rendering::terrain=debug` conserve durées CPU géométrie/index/transaction/synchronisation des meshes et nombre de meshes soumis au remplacement. Ces dernières durées excluent extraction, uploads effectifs GPU et présentation. Les spans GPU asynchrones du profiler restent distincts, non additionnables et sans attribution exhaustive par caméra ou upload.

Abris, dépôts et inventaire restent dans leurs propriétaires corporels P4/P5 ; aucun objet n’est réinstallé par le LOD. Transport minimal du repère, OBB exactes, horloges et visites translatoires restent les contrats précédents. `SOLAR_TERRAIN_LOD=fixed` permet une comparaison avec les treize patches 32 et l’ancien index intégral, sans changer le gameplay. `SOLAR_LOD_DEBUG=1` ajoute seulement des couleurs de diagnostic.

## Vérifier

Les scripts et résultats sont décrits dans [PERFORMANCE](PERFORMANCE.md) et [VALIDATION](VALIDATION.md). Le diagnostic `examples/lod_seams.rs` montre trois patches 32/16/8, raccords internes, arêtes orientées/inversées, trois faces de coin et changements de résolution, avec six captures et quatre handles réutilisés. Il impose ses niveaux pour isoler la topologie ; la sélection selon distance est vérifiée dans le jeu, ses compteurs et les tests. La marche/course/saut sur faces/arêtes/coins et la construction utilisent les scénarios de production. Windows/RTX 4050/Core i5 restent à valider nativement.

---

# Historique P3–P5 : terrain mondial à résolution fixe

13 octobre 2026. À la demande de l'utilisateur, l'objectif de P3 est désormais une Mars jouable sur **ses six faces**, avec chargement dynamique, plutôt qu'une nouvelle chaîne fixe. Les limitations à deux ou trois cartes sont levées pour ce parcours ; les anciens modes P3.2–P3.4 restent disponibles comme régressions.

## Jouer et reproduire

Le lancement normal utilise maintenant le terrain mondial lors de M. Marche/course/saut et souris restent inchangés ; toutes les faces, arêtes et coins sont accessibles par déplacement. La planète fait toujours des millions de mètres : aucune vitesse artificielle de déplacement ni compression du sol n'est introduite. `SOLAR_MARS_GLOBAL=1` force ce parcours, `SOLAR_MARS_GLOBAL=0` retrouve le terrain historique. Les variables historiques cube/P2/smoke de surface sélectionnent leur ancien parcours quand l'option mondiale n'est pas forcée. Le profilage reste prioritaire ; sa scène historique reste le défaut. `SOLAR_PROFILE_TERRAIN=global` sélectionne explicitement le fournisseur mondial pour une comparaison dans le même protocole.

Après compilation :

```bash
SOLAR_MARS_GLOBAL=1 cargo run --release --locked
# Sur ce poste NVK seulement, ajouter MESA_VK_WSI_PRESENT_MODE=immediate.
SOLAR_MARS_GLOBAL=1 SOLAR_GLOBAL_VALIDATE_DIR=docs/mon-nouveau-parcours-global target/release/solar_survival
```

Le diagnostic exige un parent existant et un dossier final nouveau. Il prépare douze départs, six mètres avant une arête (+U et +V de chacune des six faces), puis marche/court/saute via les vraies touches ECS pendant dix secondes chacun. Les préparations sont des repositionnements de diagnostic, pas un voyage entre sites. Il conserve 36 captures et vérifie franchissements, rebasages et apex. La validation numérique couvre aussi −U/−V, retour, marche/course à 30/60/144 FPS et les huit coins. Ne pas combiner les scénarios ; compiler puis lancer successivement.

## Géométrie et chargement

`planets/globe_terrain.rs` utilise les clés et voisins orientés P3.1 sur toutes les faces, au niveau 15 et **32 segments** par patch. Sphère au rayon moyen martien du catalogue, sans relief. Résolution fixe locale : **pas un LOD adaptatif selon la distance du joueur** ; les cellules centrales font ~6,47 m et leur flèche sphérique quelques micromètres. Leur taille ne réduit pas l'empreinte interrogée pour les contacts.

Chaque sommet est produit par `PatchKey::vertex`, donc par la même identité cubique entière et normalisation sur les deux représentations d'une arête ou d'un coin. Les patches construits indépendamment ont des positions de bord f64 identiques. Les indices sont orientés vers l'extérieur. Chaque patch a 1 089 sommets/2 048 triangles.

Un voisinage de distance de graphe ≤2 autour de la clé courante contient **au plus treize patches** ; il inclut les diagonales et les trois faces près d'un coin. Une géométrie dans les axes du corps est conservée par `Arc` et réutilisée tant que sa clé reste résidente. La nouvelle clé vient directement de la direction radiale des pieds, sans liste de cartes préautorisées. Les patches sortants sont libérés, seuls les entrants sont générés.

La couverture utilisée par la physique est un disque local de **66 m**, avec rebasage après **64 m** horizontaux. Les deux mètres supplémentaires dépassent le déplacement de course par sous-pas (0,0667 m) et l'empreinte. Le rayon martien et le niveau 15 permettent un voisinage assez large même aux coins ; une grille du disque et les quatre coins de chaque empreinte sont vérifiés sur 150 directions (six faces × cinq u × cinq v), avec rotation horizontale du repère. Hors du disque, le fournisseur refuse le support : ce garde-fou local n'est pas une borne de gameplay, car le repère est remplacé avant de l'atteindre. Aucune borne mondiale ±4 096 m ni restriction de couloir n'est appliquée au parcours global.

L'index local a des cases XZ de 8 m. Les triangles f64 sont ceux du rendu ; les requêtes découpent tous les triangles intersectés par l'empreinte 0,6×0,6 m et prennent leur maximum. La couverture ne repose pas sur une seule requête au centre : les aires XZ des polygones découpés sont additionnées une fois par triangle, puis comparées à l’aire de l’empreinte (tolérance arithmétique 10⁻⁹ m²). La projection locale est injective, donc les triangles ne se recouvrent pas en aire. Une empreinte partielle renvoie None, même avec un centre valide ; les requêtes sont bornées à une demi-largeur de 1 m, supérieure à celle du joueur. Les tests du disque sont une vérification échantillonnée, complétée par les traversées des raccords ; ils ne sont pas une inspection exhaustive de toutes les clés martiennes.

## Repère transporté et obstacles

Le repère est indépendant des axes U/V des faces. Lors d'un rebasage, les pieds dans le corps donnent la nouvelle normale ; une rotation minimale transporte l'ancienne normale vers celle-ci et conserve la base tangentielle. Traverser une arête du cube ne provoque donc aucun changement d'orientation de 90°. L'origine devient le point correspondant sur la sphère moyenne. Les pieds sont transférés en f64 par soustraction des origines dans le corps, puis rotation ; vitesse et base de visée suivent la transaction P3.3. Yaw/pitch, accumulator, saut en attente et grounded sont conservés. Publication après physique et deux horloges inchangées.

La boîte joueur reste alignée sur le repère actif. Les rebasages ordinaires n'introduisent qu'un angle ~64/R, soit 1,89×10⁻⁵ rad ; leurs petits réajustements du support au sous-pas suivant restent sous la peau de 2 mm. On ne revendique pas un volume orienté invariant. Le repère peut cumuler une grande orientation pendant un voyage ; la visée est transportée, sans imposer les axes du nouveau cube.

Les trois obstacles restent fixés dans le corps au site d'entrée. Pour le parcours mondial, les contacts utilisent leur **boîte orientée exacte**, avec balayage continu SAT sur 15 axes et glissement. L'expansion AABB cumulée n'est plus utilisée. Le support des faces supérieures est calculé sur leurs triangles exacts sous l'empreinte ; rendu à rotation exacte. Les objets éloignés sont exclus des contacts par une distance conservatrice ; ils ne sont jamais recentrés ou dupliqués lors du chargement. Les anciennes branches de contacts P3.2–P3.4 restent conservées. Pas de corps dynamiques ni de nouvelle bibliothèque physique.

## Proxies et budgets

Chaque mesh est exprimé dans un repère fixe propre au patch, après soustraction f64 de son origine. Chaque frame convertit seulement la différence entre cette origine dans le corps et les yeux, puis la rotation relative. Les poses astronomiques ne remplissent jamais directement un Transform f32. Le rebasage ne réécrit pas les meshes conservés ; les slots/handles sortants peuvent servir aux entrants. Aucun mesh ni patch n'est généré par frame ordinaire ou pendant la rotation de Mars.

Au repos : ≤13 patches, **14 157 sommets et 26 624 triangles**, index local et ≤13 meshes de terrain. Pendant un remplacement : ancien et nouveau fournisseur coexistent, donc ≤26 occurrences de patch/53 248 triangles locaux ; les géométries communes sont partagées et ne sont pas doublées. Borne conservative de géométrie non partagée : 28 314 sommets. À l'entrée, l'ancien fournisseur est libéré avant préparation. Les handles de terrain sont recyclés sans ajouter un second voisinage GPU complet. Ces budgets sont structurels et ne mesurent pas tous les buffers internes/RAM/VRAM de Bevy.

Les tests mesurent des plafonds d'index sur les directions de couverture et vérifient absence de génération au repos, `Arc` communs, handles communs et libération à la sortie. Les coûts muraux de rebasage sont journalisables avec `RUST_LOG=info,solar_survival::player::surface=debug`. Ils excluent la synchronisation des meshes et le GPU. Les résultats graphiques et leurs limites sont consignés dans [VALIDATION](VALIDATION.md) ; Windows/RTX 4050 ne sont pas validés depuis Linux.

Ce parcours est une sphère martienne complète jouable, avec chargement local. Il ne fournit pas encore relief mesuré, LOD mixte, carte géographique ou sauvegarde ; la survie P4 est décrite dans [SURVIVAL](SURVIVAL.md). Il ne fournit pas non plus de dynamique inertielle : les paramètres astronomiques, forces omises et visites spatiales translatoires restent ceux d'[ASTRONOMY](ASTRONOMY.md).


## Validation réalisée

90 tests et vérifications Cargo finales réussis. 288 traversées dirigées d’arêtes aller-retour en marche/course et 30/60/144 FPS, 24 trajets de coins, conservation f64, contrôles ECS et réutilisation des ressources. Campagne mondiale de 120 s : code 0, douze franchissements en l’air, douze rebasages, 36 captures inspectées. Quatre régressions code 0 ; 112 captures inspectées au total. Comparaison release : ~19,1 FPS au repos pour historique/mondial/historique, remplacement réel mesuré dans un essai mondial de 54 s. [Preuves et limites](VALIDATION.md).


## Extensions après P4

Le chargement dynamique de treize patches est un voisinage à niveau/résolution fixes, **pas un système de LOD adaptatif**. L’extension demandée devra choisir plusieurs niveaux de détail selon la distance du joueur, produire des raccords entre niveaux sans fissures, gérer les transitions sans oscillation et conserver des contacts précis autour du joueur. Budgets de préparation/chargement CPU et meshes GPU à mesurer ; pas de génération de planète entière. Cette fonctionnalité reste future et possède ses propres critères dans [ROADMAP](ROADMAP.md).

P4 ajoute un abri de recharge fixe dans le corps. Le retour normal M conserve désormais le site de session ; R après échec revient dans l’abri. Poses, contacts orientés, proxies et règles d’oxygène dans [SURVIVAL](SURVIVAL.md). Les campagnes historiques demeurent sans survie.


## P5 : admission locale des constructions

Le fournisseur expose aussi les extrema exacts sous un carré de construction (demi-largeur ≤5 m), avec le même index et certificat d’aire ; les contacts du joueur restent bornés à ≤1 m. La preview utilise un carré conservateur de 4,5 m entièrement dans le disque local de 66 m, sans charger de candidate ni modifier le terrain. Près de la bordure, avancer jusqu’au rebasage peut rendre le placement admissible. Bâtiments et dépôts sont fixes dans le corps et survivent aux rebasages/retours spatiaux. [CONSTRUCTION](CONSTRUCTION.md) décrit sol, volumes, coûts et budgets. Le chargement reste à résolution fixe, sans LOD adaptatif.
