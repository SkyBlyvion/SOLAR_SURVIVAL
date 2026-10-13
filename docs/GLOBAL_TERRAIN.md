# Mars traversable : terrain mondial local

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
