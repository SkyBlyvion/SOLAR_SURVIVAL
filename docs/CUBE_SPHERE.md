# Cube-sphere : contrat P3.1 et contacts bornés P3.2

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

Le triplet cubique entier est l'identité du sommet dans la paire. Il reste exactement représentable en f64 jusqu'au niveau 24 et 64 segments. Les deux patches utilisent **les mêmes indices** au raccord : aucune moyenne de positions, tolérance de soudure ou jupe de dissimulation. Une construction séparée avec les mêmes paramètres fournit également des positions de bord exactement identiques. Cette garantie porte sur un même niveau/résolution ; aucun raccord de LOD différent n'est implémenté.

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
