# Contrat géométrique cube-sphere P3.1

La géométrie opérationnelle est dans `src/planets/cube_sphere.rs`, réexportée par `planets` pour conserver les chemins `CubeFace` et `PatchKey`. Elle ne dépend d'aucun système ECS, mesh GPU ou contrôleur. Le terrain jouable de `planets/terrain.rs` reste le site tournant de neuf tuiles borné à ±4 096 m. Ce contrat prépare son extension sans la réaliser.

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
