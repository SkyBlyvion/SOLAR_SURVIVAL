# Cube-sphere : contrat P3.1 et contacts bornés P3.2

**Parcours actuel : [Mars entière avec chargement local dynamique](GLOBAL_TERRAIN.md).** Les contrats P3.1–P3.4 ci-dessous restent les fondations et modes historiques de régression ; leurs restrictions de paire ne limitent plus le terrain mondial.

La géométrie opérationnelle est dans `src/planets/cube_sphere.rs`, réexportée par `planets` pour conserver les chemins `CubeFace` et `PatchKey`. Elle ne dépend d'aucun système ECS, mesh GPU ou contrôleur. Le terrain par défaut de `planets/terrain.rs` reste le site tournant de neuf tuiles borné à ±4 096 m. P3.2 ajoute un fournisseur optionnel de contacts pour une paire ; voir ci-dessous.

## Repère et projection

Axes du corps tournant, repère direct Y-up ; aucune latitude/longitude ni orientation IAU revendiquée. Les coordonnées de face `u,v ∈ [-1,1]` sont sans unité. La direction est `normalize(N + u U + v V)`, avec `U × V = N`.

| Face | N | U | V | Vecteur avant normalisation |
|---|---|---|---|---|
| +X | +X | −Z | +Y | (1, v, −u) |
| −X | −X | +Z | +Y | (−1, v, u) |
| +Y | +Y | +X | −Z | (u, 1, −v) |
| −Y | −Y | +X | +Z | (u, −1, v) |
| +Z | +Z | +X | +Y | (u, v, 1) |
| −Z | −Z | −X | +Y | (−u, v, −1) |

Ces conventions conservent celles du centre historique de `PatchKey`. Ce choix est une projection radiale du cube normalisé ; les surfaces et longueurs ne sont pas uniformes.

L'inverse choisit la composante absolue dominante, avec priorité **X, puis Y, puis Z en cas d'égalité exacte** ; son signe choisit la face. Ensuite `u = dot(d,U)/dot(d,N)`, `v = dot(d,V)/dot(d,N)`. Le vecteur est d'abord divisé par sa composante absolue maximale pour accepter des directions non normalisées très petites ou très grandes sans calculer une norme susceptible de déborder. Zéro et valeurs non finies sont refusés. Aux arêtes/coins, l'aller-retour conserve la direction, pas nécessairement la face initiale ; près d'une égalité, un arrondi peut changer le choix de face sans rompre la direction.

## Patches et voisins

`level ∈ [0,24]`, `count = 2^level`, `x,y ∈ [0,count-1]`. Le niveau est un indice de subdivision, pas une longueur SI. Les fractions locales `s,t ∈ [0,1]` donnent `u = 2(x+s)/count − 1`, `v = 2(y+t)/count − 1`. Aucune extrapolation hors d'une face ou d'un patch ; les entrées invalides renvoient `None`.

Pour les arêtes ±u, le paramètre croît suivant v ; pour ±v, il croît suivant u. Un voisin interne conserve le paramètre et rejoint l'arête opposée. Entre faces, `PatchNeighbor` fournit la clé de même niveau, l'arête du voisin et `reversed` : paramètre voisin `1−t` si inversé, `t` sinon. Les axes entiers de la base déterminent ces résultats sans epsilon ni projection d'un point au-delà du bord.

| Face source | −u | +u | −v | +v |
|---|---|---|---|---|
| +X | +Z, +u | −Z, −u | −Y, +u, inversé | +Y, +u |
| −X | −Z, +u | +Z, −u | −Y, −u | +Y, −u, inversé |
| +Y | −X, +v, inversé | +X, +v | +Z, +v | −Z, +v, inversé |
| −Y | −X, −v | +X, −v, inversé | −Z, −v, inversé | +Z, −v |
| +Z | −X, +u | +X, −u | −Y, +v | +Y, −v |
| −Z | +X, +u | −X, −u | −Y, −v, inversé | +Y, +v, inversé |

À un coin, chaque arête mène à son voisin ; trois faces représentent la même position. Il n'existe pas de voisin diagonal unique au coin.

## Géométrie bornée et sommets communs

`PatchPair::new(source, edge, segments, radius_m)` produit exactement deux patches adjacents de même niveau et de même résolution. `segments ∈ [1,64]` ; rayon fini strictement positif, en mètres. Les positions `DVec3` sont relatives au **centre du corps**, à son rayon moyen, sans relief. Les triangles sont orientés vers l'extérieur.

Pour un sommet de grille `i,j`, on construit les entiers :

```text
d = count × segments
a = 2 × (x × segments + i) − d
b = 2 × (y × segments + j) − d
cube = N × d + U × a + V × b
p_m = normalize(cube) × radius_m
```

Le triplet cubique entier est l'identité du sommet dans la paire. Il reste exactement représentable en f64 jusqu'au niveau 24 et 64 segments. Les deux patches utilisent **les mêmes indices** au raccord : aucune moyenne de positions, tolérance de soudure ou jupe de dissimulation. Une construction séparée avec les mêmes paramètres fournit également des positions de bord exactement identiques. Cette garantie décrit la paire historique de même niveau/résolution. Le fournisseur mondial adapte désormais sa résolution intérieure tout en conservant les bords à 32 segments ; voir [GLOBAL_TERRAIN](GLOBAL_TERRAIN.md).

La paire contient `2(segments+1)² − (segments+1)` sommets uniques et `4 segments²` triangles. Maximum : **8 385 sommets, 16 384 triangles**, plus deux groupes d'indices et les indices de l'arête partagée. Le HashMap sert seulement à dédupliquer ; les indices suivent l'ordre déterministe des grilles et ne dépendent pas de son ordre d'itération.

La graine historique SplitMix64 et les identifiants explicites des six faces sont conservés. Des vecteurs de référence fixent ce comportement. La graine distingue les patches valides à graine globale fixée ; elle ne définit pas un relief continu. Un futur relief devra être échantillonné depuis une identité/position commune aux faces, jamais depuis des bruits indépendants de chaque patch.

## Taille et démonstration

Le niveau **15** et **32 segments** sont des choix de diagnostic, pas une taille finale de terrain. À proximité du centre d'une face, la largeur nominale est `2R/32768 ≈ 206,879 m` pour le rayon martien du catalogue (3 389 500 m), et le pas nominal ≈ 6,465 m. Les longueurs réelles diminuent et varient vers les arêtes/coins ; le journal de la démo mesure la longueur polygonale du raccord. Le niveau 24 est une borne numérique admissible, pas une recommandation de LOD jouable.

`examples/cube_seam.rs` est un programme optionnel de 32 secondes : raccord interne, +X/−Z, +X/−Y inversé, puis deux patches près du coin −Y/+X. Deux patches seulement sont présents à chaque phase ; deux meshes de surface et un mesh de lignes sont créés une fois, puis leurs handles réutilisés aux trois transitions. Pas de génération par frame, de planète entière, de troisième patch au coin, de relief ou de contacts.

Le rendu soustrait une origine locale en f64 et applique une rotation `DQuat` avant `local_to_render` : les positions relatives restent en mètres 1:1. Les normales radiales et couleurs sont de présentation ; la grille est relevée de 5 cm et le trait de raccord de 10 cm pour l'inspection. Les captures illustrent le raccord, tandis que les tests vérifient égalité, indices, rayon et fermeture topologique.

Pour construire, puis lancer sans compilation concurrente :

```bash
cargo build --release --locked --example cube_seam
MESA_VK_WSI_PRESENT_MODE=immediate RUST_LOG=info target/release/examples/cube_seam docs/mon-nouveau-raccord
```

L'argument facultatif désigne un **nouveau dossier** de captures dans un parent existant ; un dossier existant est refusé. Sans argument, aucune capture n'est écrite. La variable Mesa concerne uniquement le contournement local Ubuntu/NVK. Ce diagnostic utilise une caméra fixe et termine automatiquement ; il ne remplace pas une validation du joueur franchissant le raccord.


## Paire jouable P3.2 (opt-in)

`SOLAR_CUBE_SURFACE=1` choisit le fournisseur `planets/cube_terrain.rs` lors de l'entrée M. Le menu et les commandes restent disponibles. Sans cette variable, les neuf tuiles, le relief artistique et ±4 096 m restent le défaut. Le profilage désactive cette option afin de garder sa scène canonique ; le diagnostic P3.1 demeure indépendant.

La face est la composante dominante de la direction éclairée dans le corps tournant, avec la priorité historique X/Y/Z. On choisit les patches centraux `(x=16383,y=16384)` et `(x=16384,y=16384)` au **niveau 15, 64 segments**. Le site est au milieu du raccord `u=0,v=1/32768`, son axe X est U projeté tangent, Y est radial, Z=X×Y. Ce site conventionnel peut différer du point éclairé du FPS historique ; ce choix permet un rectangle intérieur simple et commun aux six faces. Il n'est pas une localisation cartographique.

Au rayon martien 3 389 500 m : chaque patch mesure environ **206,879 m**, chaque cellule **3,232 m**. La paire couvre environ 413,757 × 206,879 m. La flèche sphérique maximale d'une diagonale de cellule est d'environ **0,8 µm** (`diagonale²/(8R)`, estimation locale), très inférieure aux 2 mm de marge du contrôleur. Les cellules sont un peu plus petites que les 4 m du terrain historique. L'empreinte de 0,6 × 0,6 m interroge les triangles qu'elle recouvre, même si son centre est dans un seul patch : la résolution ne doit pas être confondue avec un échantillonnage central du joueur. Cette résolution fixe vise ce premier parcours sphérique ; elle ne définit pas un LOD général pour du relief.

Il n'y a **aucun relief cube-sphere**. Les positions SI de `PatchPair` sont l'autorité. À l'entrée seulement, le fournisseur soustrait l'origine dans les axes du corps en f64, applique l'inverse du DQuat du site, et conserve les positions locales. Contacts et deux meshes réutilisent ces positions et les mêmes indices partagés au raccord. Les couleurs orange/bleue servent à identifier les deux patches ; elles peuvent changer au raccord, sans changer la géométrie. Les trois obstacles de gameplay restent fixés au support triangulaire.

Le contrôleur cinématique, la gravité radiale, les deux horloges et la publication de pose ne sont pas refondus. Le fournisseur sélectionne les triangles par leur boîte XZ dans un index de cases de **4 m**, construit une fois avec la paire. Il découpe chaque triangle par les quatre côtés de l'empreinte, puis prend le maximum des sommets du polygone obtenu. Une requête ponctuelle utilise l'interpolation barycentrique pour éviter la dégénérescence du découpage. L'index peut référencer un triangle dans plusieurs cases ; les doublons n'affectent pas le maximum. Aucune recherche de planète entière ni génération par frame.

Un rectangle conservateur est inscrit dans les coins externes de cette paire centrale, puis réduit de **1 m**. Le contrôleur y borne le centre des pieds avec **0,3 m** de réserve supplémentaire pour l'empreinte, y compris en l'air. Le fournisseur refuse une empreinte hors de ce rectangle au lieu d'extrapoler un sol. Ce sont des limites de gameplay du parcours opt-in ; aucun chargement au-delà, raccord entre faces, LOD mixte ou migration d'origine n'est ajouté. Elles ne constituent pas des murs visibles. Lorsqu'un axe est bloqué par le bornage, sa composante de vitesse publiée est annulée, y compris pour la borne du site historique.

Budgets fixes : deux patches, **8 385 sommets physiques uniques**, **16 384 triangles**, 65 sommets communs. Les deux meshes conservent chacun le tableau local de 8 385 positions et leurs indices respectifs (16 770 sommets de proxy au total ; environ 0,83 Mio de données positions/normales/couleurs/indices, sans surcoût du moteur). Deux handles sont créés au premier accès, cachés en espace, puis réutilisés et mis à jour uniquement lors du choix d'un site. L'index est testé sous 5 700 cases, 85 000 références et 32 triangles par case ; une empreinte de 0,6 m visite au plus quatre cases. Ces plafonds de structures ne sont pas une mesure RAM/GPU complète ou un benchmark.

### Reproduction du parcours

Après compilation, créer seulement le parent du dossier ; le diagnostic crée et exige un **nouveau** dossier de captures :

```bash
cargo build --release --locked
MESA_VK_WSI_PRESENT_MODE=immediate SOLAR_CUBE_SURFACE=1 SOLAR_CUBE_VALIDATE_DIR=docs/mon-nouveau-franchissement RUST_LOG=info target/release/solar_survival
```

Sans `SOLAR_CUBE_VALIDATE_DIR`, l'option est jouable normalement depuis le menu, avec M, marche/course/saut et sorties M/G/O/U. Sur Windows, définir les deux variables via PowerShell et lancer le même binaire natif ; omettre la variable Mesa. Ce protocole Windows n'est pas une validation réalisée.

La phase initiale laisse le curseur libre pour attendre la visibilité de la fenêtre ; la capture commence à la marche. Le scénario dure **88 s**, 22 phases de 4 s : repos, marche aller/retour, course aller/retour, repos sur le raccord, saut franchissant le raccord et réception, retour, repos, reprise à ×1000 avec course aller/retour, deux limites extérieures, puis sorties M/G/O/U avec quatre retours, enfin Terre. L'entrée, les repos au raccord et les approches des limites emploient des poses de préparation explicites ; les franchissements et le saut utilisent les touches ECS avant les contrôles réels. Le scénario vérifie le support à chaque frame, compte les changements de signe X pendant les phases de mouvement, exige au moins six franchissements et un apex supérieur à 1,9 m, et enregistre 22 captures. Il ne simule pas les périphériques OS. Il a priorité sur P2 et les anciens smoke ; le profilage reste prioritaire.

Les résultats et limites de la campagne sont dans [VALIDATION](VALIDATION.md).

## Migration P3.3 bornée sur la même face

`SOLAR_CUBE_MIGRATION=1` active la paire et sa migration ; le profilage reste prioritaire. `SOLAR_CUBE_SURFACE=1` seul garde le parcours P3.2 sans migration, et l'absence des deux options garde le terrain historique. Aucun passage jouable entre faces n'est ajouté.

Le fournisseur `CubeTerrain::from_key` accepte des paires +U non centrales, **sur la même face**, au niveau 15 et 64 segments. Il refuse les autres niveaux et les largeurs nominales de patch hors **128–256 m** avant allocation : limite de ce fournisseur local, pas du contrat géométrique P3.1. Au rayon martien, cette largeur est 206,879 m. La nouvelle origine est au milieu du raccord de la paire ; Y radial, X tangent à U, Z=X×Y. Toutes les positions sont soustraites puis tournées en f64, une fois par construction.

La borne n'est plus déduite seulement de quatre coins. Les arêtes incidentes à un seul triangle forment la frontière projetée XZ. L'intersection de **tous** leurs demi-plans orientés vers le centre est le noyau conservateur de ce polygone simple. Un rectangle centré y est inscrit, à au moins **1 m** de chaque droite de frontière. Chaque empreinte complète de 0,6×0,6 m reste ainsi sur les triangles résidents, même lorsque les bords projetés sont inclinés ou légèrement courbes. Le contrôle du rectangle accepte 1 nm d'arrondi arithmétique, dans cette réserve de 1 m ; aucune extrapolation de triangle n'est ajoutée. L'index de cases de 4 m et le découpage exact des triangles restent les fournisseurs de hauteur. Ce rectangle sacrifie de la couverture près des coins : il ne constitue pas un chargement mondial.

La migration jouable est volontairement restreinte à **deux choix** : paire centrale `(16383,16384)/(16384,16384)` et voisine `(16384,16384)/(16385,16384)`. Le patch commun a les mêmes positions SI et triangles. Aller lorsque x>100 m dans la première carte ; retour lorsque x<−110 m dans la seconde, soit environ 3,12 m d'hystérésis en position dans le corps. Admission seulement pour |z|≤50 m et |y|≤100 m, en l'air comme au sol. Ces seuils sont des conventions du premier parcours ; les bornes restent actives hors du couloir et aux extrémités. La nouvelle empreinte est vérifiée avant toute mutation. Aucun nouveau patch n'est généré pour un mouvement ordinaire, une rotation ou une tentative hors du couloir.

La transaction intervient après un sous-pas physique et avant la publication dans Navigation. Avec `A=Q_nouveau⁻¹ Q_ancien`, les pieds sont convertis depuis leur position dans le corps (soustraction des origines en f64), la vitesse devient `A v`. La base `heading` transporte aussi la visée : `L_nouveau⁻¹ A L_ancien heading`, où L aligne Y sur la normale radiale des pieds. Yaw/pitch et leurs deltas souris restent les commandes existantes. Yeux, gravité et support utilisent alors la nouvelle carte ; accumulator, saut pending et grounded ne sont pas réinitialisés. La boîte de contact/empreinte reste alignée sur la carte active : c'est toujours le compromis cinématique du prototype, avec une inclinaison entre cartes d'environ 6,10×10⁻⁵ rad. En l'air, la projection verticale vers le support change légèrement de point : comparer son rayon et l'écart aux pieds, et non exiger l'identité de deux projections horizontales.

Les **trois obstacles appartiennent au site d'entrée**, où leurs tailles et hauteurs sont figées une fois. Leur centre et orientation dans le corps ne changent pas ; aucun obstacle n'est recréé au centre de la nouvelle carte. Les trois entités/meshes déjà présents rendent leur rotation exacte après conversion. Pour garder le contrôleur de boîtes, leurs contacts emploient l'enveloppe AABB des huit coins dans la carte active : expansion conservatrice, testée inférieure à **0,31 mm** sur chaque dimension de ce parcours. Elle ne permet pas de généraliser ces contacts à de grands changements d'orientation. Les obstacles peuvent rester visibles près d'une borne même si leur face extérieure n'est pas accessible ; le retour automatique restitue la carte d'entrée avant de les approcher dans le couloir.

Après remplacement : **deux patches** CPU et les **deux handles de terrain** existants. Pendant la transaction synchrone, ancienne paire + candidate coexistent uniquement en CPU : plafond de **deux `CubeTerrain`**, soit quatre occurrences de patch, **16 770 sommets physiques et 32 768 triangles**. Aucun troisième mesh de terrain ni chevauchement GPU n'est ajouté ; la paire abandonnée est libérée au remplacement. Les index et tableaux temporaires de construction restent bornés par ces paires. Les deux meshes sont mis à jour lors de la révision suivante dans Rendering, avant le rendu de la pose publiée. Les couleurs dépendent de la parité X de la clé de patch : le patch conservé garde ainsi sa couleur au remplacement. Ces plafonds ne mesurent ni la VRAM totale ni les buffers internes du moteur.

### Reproduction P3.3

Après `cargo build --release --locked`, sans compilation ni autre lancement GPU concurrent :

```bash
MESA_VK_WSI_PRESENT_MODE=immediate SOLAR_CUBE_MIGRATION=1 SOLAR_CUBE_VALIDATE_DIR=docs/mon-nouveau-parcours-p33 RUST_LOG=info target/release/solar_survival
```

Le parent doit exister, le dossier final doit être nouveau. Sans variable de capture, M ouvre le parcours jouable normalement. Le diagnostic commun dure 88 s et conserve 22 captures. En P3.3, les poses de préparation x=96 m/z=30 m précèdent marche/course aller-retour et saut ; ces poses ne comptent pas comme migrations. Les vraies commandes ECS provoquent huit migrations attendues, dont une en l'air, puis repos, P/×1000, limites et quatre sorties/retours M/G/O/U, enfin Terre. Les deux préparations de limite migrent explicitement et sont exclues du compteur de marche. Une mesure murale CPU de construction/transaction est journalisée à chaque remplacement ; elle n'est ni un profiler complet ni une mesure GPU. Résultats effectivement obtenus dans [VALIDATION](VALIDATION.md).

## P3.4 : deux cartes franchement hors centre, même face

`SOLAR_CUBE_OFFCENTER=1` implique cube et migration, sauf profilage prioritaire. Il choisit `(24575,24576)/(24576,24576)`, puis `(24576,24576)/(24577,24576)`, sur la face dominante de la direction solaire dans le corps à l'entrée. Origine de la première vers `u=0,5, v=0,5000305`, à ~35,26° du centre de face : nouvelle localisation de gameplay, sans coordonnées géographiques mesurées. La face sélectionnée ne réoriente jamais le site pendant une visite. `SOLAR_CUBE_MIGRATION=1` seul garde le parcours central P3.3 ; P3.2 seule et défaut historique restent disponibles. Pas de trajet automatique depuis la centrale jusqu'à ce site distant.

Le module réel `planets/cube_migration.rs` prépare seulement deux `Chart` (clé, DQuat, rectangle certifié). À l'entrée, il construit les deux paires, conserve la première et libère la géométrie de la seconde. Le recouvrement provient ainsi des mêmes frontières/triangles que les contacts, pas d'une largeur nominale. Au rayon martien : première demi-largeur ~140,493461×62,108457 m, seconde ~140,486413×62,105910 m ; centres séparés de ~154,193858 m selon X de la première. Index observés : 2 978 cases, 44 610/44 627 références, maximum 18 triangles/case. Niveau/résolution et rayon inchangés.

### Admission et hystérésis

Toutes les décisions utilisent les coordonnées **dans la carte d'entrée**, même quand la voisine est active. Avec `c` égal à la moitié de la séparation des origines projetée selon X (~77,096929 m), le plan exige une boîte de pieds `|x−c|≤8 m`, `|y|≤100 m`, `|z|≤hz`. Pour chacune des deux cartes, `p'=A p+t` est calculé avec soustraction des origines en f64. Chaque contrainte X/Z du rectangle certifié inclut le rayon d'empreinte **0,30025 m** et les extrêmes de toute la boîte :

```text
|p'_centre[j]| + |A[j,X]| × 8 + |A[j,Y]| × 100
                  + |A[j,Z]| × hz + 0,30025 ≤ rectangle.max[j], j=X,Z
```

`hz` est le minimum autorisé par les quatre inégalités, réduit encore de 20 % (~49,444528 m). Si les contraintes ne laissent aucun couloir utilisable, aucun plan n'est fourni. Les ±8 m, ±100 m, réserve de 20 % et hystérésis de 2 m sont des conventions bornées de gameplay ; la couverture est calculée, pas une donnée martienne ni un contrat mondial.

Aller si `x>c+1 m` ; retour si `x<c−1 m`, dans ce repère commun. L'hystérésis est donc 2 m **dans la même coordonnée**, indépendante d'un décalage de repère. Le plan sélectionne au plus l'autre carte, vérifie encore les deux empreintes et rejette avant construction hors du volume, hors rectangle, dans la bande d'hystérésis ou pour une clé étrangère. Le joueur qui contourne le couloir peut atteindre la borne de la carte active : aucune borne n'est ouverte. Un mouvement en course ne parcourt que 0,0667 m par sous-pas ; la zone de 8 m laisse une large réserve, sans rattrapage mondial d'une téléportation hors admission.

Les tests vérifient tous les coins du volume sur six faces, des requêtes de support sur une grille et aux altitudes −100/0/+100 m dans les deux cartes, les quatre bornes en l'air/réception, et des milliers de refus sans augmentation du compteur de constructions ni de révision. Le plan ne retient aucun cache de géométrie candidate et ne reconstruit rien pendant les refus.

### Compromis de contacts réévalué

Le transfert conserve les formules P3.3 pour pieds/yeux/vitesse/visée/gravité et la publication après physique. Tests aller et retour sur six faces, au sol/2/20/99 m : positions et centres d'objets <4 nm, axes de visée et vitesse/gravité <10⁻¹³ dans leurs unités, rayon du support <2 µm, écart vertical au support <20 µm. Le volume de contact demeure aligné sur la carte active. Le déplacement de chacun de ses huit coins lors du reboîtage est calculé en incluant la rotation complète : maximum **83,015 µm**, refus si >**0,25 mm**. L'angle complet ~4,54916×10⁻⁵ rad est inférieur au central, mais inclut aussi le roulis hors centre. On ne revendique toujours pas un volume orienté invariant ni un changement de face/grande inclinaison validé.

Les obstacles restent possédés par le site d'entrée, leurs trois poses exactes dans le corps sont conservées. Leurs enveloppes `abs(A) × taille` sont admises seulement si l'expansion de chaque dimension reste ≤**0,5 mm** avant allocation ; calcul observé maximum **0,227457 mm**. Les huit coins sont testés inclus dans ces AABB. La migration est refusée à proximité d'un contact d'objet : le volume joueur dans la carte active est élargi de **2,75 mm** (0,25 mm de dérive + 0,5 mm d'expansion d'objet + peau 2 mm) pour exclure le contact dans les deux représentations. Ce choix préserve le petit contrôleur actuel et limite explicitement son usage. Le support terrain reste exact sous l'empreinte reboîtée ; l'écart des projections verticales ne signifie pas une translation physique du sol.

### Ressources et reproduction

Pendant `prepare` ou une transaction, maximum **deux CubeTerrain** CPU (quatre occurrences de patch, 16 770 sommets/32 768 triangles plus tableaux locaux/index/temporaires). `set_site` libère la paire précédente AVANT de préparer la nouvelle, y compris aux retours M/G/O/U : pas de troisième paire à l'entrée. Après préparation/remplacement : une paire, deux patches. Les métadonnées de deux cartes restent constantes ; compteur deux constructions à l'entrée, puis une par migration admise. Les deux handles et entités du terrain, les trois obstacles et la couleur du patch partagé sont réutilisés. Aucun nouveau proxy GPU ajouté ni génération par rotation/frame ordinaire. Les plafonds sont structurels, sans mesure RAM/VRAM totale ni garantie sur les buffers temporaires internes du renderer.

```bash
cargo build --release --locked
MESA_VK_WSI_PRESENT_MODE=immediate SOLAR_CUBE_OFFCENTER=1 SOLAR_CUBE_VALIDATE_DIR=docs/mon-nouveau-parcours-p34 RUST_LOG=info target/release/solar_survival
# Les quatre scénarios successifs, dans un dossier neuf pour les preuves :
python3 scripts/validate_cube_suite.py --output docs/ma-nouvelle-campagne --mesa-immediate
```

Le premier diagnostic exige un parent existant et un dossier final nouveau. Sans variable de captures, M ouvre le parcours jouable. Le scénario de 88 s adapte ses poses de préparation au plan (x=c−3 m, z=hz/2) ; huit migrations par touches ECS, dont un saut, repos, pause/×1000, deux limites terminales, M/G/O/U puis Terre. Les deux migrations explicites de préparation des limites ne comptent pas parmi les marches. Sous Windows natif, omettre Mesa et utiliser les variables PowerShell ; ce protocole n'a pas été exécuté ici.

Résultats : [VALIDATION](VALIDATION.md). P3.4 n'ajoute aucune chaîne de cartes, migration ±V, passage jouable entre faces, relief, LOD mixte ou changement du contrôleur. Une extension cumulant plusieurs orientations doit réévaluer les mêmes budgets, notamment l'expansion des obstacles depuis le **site d'entrée**, et justifier son plafond de préparation avant allocation.


## Terrain mondial, six faces

Le fournisseur `globe_terrain.rs` utilise le même `PatchKey::vertex` et les voisins orientés sur toutes les faces. Identités cubiques et sommets de bord/coin restent exacts en f64. Treize patches maximum à distance de graphe ≤2, niveau 15/32 segments, pas de planète entière ni LOD mixte. Les triangles sont réindexés localement pour les contacts, avec soustraction d’origine f64, puis un contrôle de l’aire couverte sous toute l’empreinte : un centre valide ne suffit pas à autoriser un support partiel.

Le repère transporté évite le changement d’axes U/V lors des arêtes ; génération seulement pour les nouvelles clés, géométries et meshes communs réutilisés. Les anciens couloirs/hystérésis/cartes fixes servent uniquement aux régressions P3.2–P3.4. Ressources, physique orientée et commandes dans [GLOBAL_TERRAIN](GLOBAL_TERRAIN.md) ; résultats dans [VALIDATION](VALIDATION.md).

## LOD mondial de résolution

Le fournisseur mondial conserve le niveau spatial 15 et choisit 32/16/8 cellules intérieures selon la distance avec hystérésis. Les bords utilisent toujours la même identité cubique à 32 segments ; éventails des cellules de bord, sans jupe ni T-junction. Les coins des trois faces et tous les raccords dirigés/inversés sont testés aux neuf combinaisons de résolutions. Ce mécanisme est distinct des anciennes paires et d’un quadtree à niveaux spatiaux variables. [Contrat, seuils et budgets](GLOBAL_TERRAIN.md).
