# P8 — relief, horizon et atmosphère martienne

15 octobre 2026. Périmètre étendu à la demande de SkyBlyvion : relief jouable et
paysage lointain, ciel ocre le jour, halo bleu au lever/coucher, nuit étoilée,
voiles de glace, poussière variable et tempêtes occasionnelles. Aucune migration
des anciennes sauvegardes demandée : format **SOLAR_SURVIVAL 2** uniquement.
Les fichiers historiques ne sont ni convertis ni supprimés.

## Relief artistique et support

`planets/relief.rs` définit un champ de hauteur déterministe dans les axes du
corps martien, en f64 et mètres. Bruit de valeur 3D à interpolation quintique,
quatre échelles (longueur de cellule / amplitude) : 3600/70 m, 900/14 m,
240/1,2 m, 60/0,1 m. Borne analytique |h| ≤85,3 m par rapport au rayon moyen.
Ces nombres et la graine sont des conventions artistiques ; aucun MOLA,
cratère catalogué, cartographie ou relief réel du site n'est revendiqué.

Les sommets canoniques cube-sphere sont déplacés radialement par ce même champ.
Les treize patches maximum gardent niveau spatial 15, intérieurs 32/16/8,
bords communs 32, sélection/hystérésis et index local 8 m. Rendu et contacts
utilisent exactement les mêmes sommets/indices f64. Les normales visuelles
proviennent de dérivées centrales du champ à ±1 m dans deux directions tangentes ;
elles sont communes aux faces/LOD, mais ne remplacent pas les plans de contact.
Les courbures et hauteurs sont fixes dans Mars ; la rotation ne régénère rien.

La construction réutilise les extrema des triangles sous le carré de demi-largeur
4,5 m et le refus au-delà de 0,4 m de variation. Certains emplacements sont donc
refusés sur une pente ; aucune mise à plat ni terrassement implicite. L'abri
initial et les caisses échantillonnent le nouveau terrain. Les tests de porte,
recharge, construction et retours restent nécessaires ; la capacité générale
à construire sur toute pente n'est pas promise. Le contrôleur garde son adhérence
sur pentes douces, sans surplombs ni dynamique rigide.

Le repère local demeure transporté en f64 après 64 m. `MarsTerrain::chart` soustrait
désormais l'origine corporelle avant d'appliquer la rotation inverse, évitant une
annulation supplémentaire après la rotation d'un grand vecteur. Les visites
spatiales restent translatoires ; GameSet et les deux horloges restent inchangés.

## Paysage au-delà des patches de contact

`planets/landscape.rs` extrait le contour exact de l'union des patches, en annulant
les segments partagés par identité des sommets corporels. Ce contour fermé forme
le premier anneau. Neuf anneaux supplémentaires prolongent ses rayons tangents
à 64/160/320/640/1200/2200/4000/7000/11000 m **au-delà du contour**, puis projettent
leurs directions sur le même rayon+champ de hauteur. Les triangles relient les
anneaux ; le premier bord est bit-identique au terrain local. Pas de jupe,
recouvrement volontaire, trou intérieur ou sphère entière générée.

Une couronne contient au plus6 400 sommets / 11 520 triangles. C'est une approximation
visuelle lointaine du même champ ; **seuls les triangles fins locaux fournissent
les contacts et l'admission**. À l'approche, le voisinage fin remplace cette
approximation. La couronne est reconstruite seulement au changement de patch
central ou de site ; son Arc et son mesh sont réutilisés sinon. Le cache conserve
≤13 meshes locaux+1couronne ; les assets sortants sont libérés. La génération reste
synchrone et bornée, sans quadtree ni tâche de planète entière.

La portée du brouillard varie de5 000–8 500 m par temps calme à 150 m au cœur d'une
tempête ; début 18 % de la portée, 10 % en tempête. La portée maximale reste sous
le bord extérieur même après les 64 m de déplacement avant rebasage. Projection
surface near 0,05 m / far 15 000 m. Les anciens modes de régression gardent leur brume
40–60 m. La couronne peut changer de triangulation au changement de patch central,
sans morphing temporel. Les tests topologiques ne remplacent pas l'inspection de
ces transitions. Les visites orbitales gardent la sphère/textures P7, sans ce
relief métrique : ce raccord d'échelle reste une limite connue.

## Faits martiens et sources

Sources primaires consultées le 15 octobre 2026, sans requête réseau pendant le jeu :

- [NASA, Mars Fact Sheet](https://nssdc.gsfc.nasa.gov/planetary/factsheet/marsfact.html) :
  6,36 mb au rayon moyen (636 Pa), variable saisonnièrement 4,0–8,7 mb ;
  le jeu retient l’ordre de grandeur arrondi 600 Pa, environ 0,6 % de 101 325 Pa terrestres.
  Valeur moyenne arrondie, variable réellement avec altitude/saison ; ni pression
  exacte locale ni coefficient permettant de déduire la densité de poussière.
- [NASA, Mars Facts](https://science.nasa.gov/mars/facts/) : atmosphère très mince,
  ciel rouge/brumeux dû à la poussière suspendue, tempêtes pouvant couvrir la planète.
- [NASA, Sunrises and Sunsets on Mars](https://science.nasa.gov/solar-system/planets/mars/what-does-a-sunrise-sunset-look-like-on-mars/) :
  poussière fine et lumière bleutée près du Soleil au crépuscule ; le reste du ciel
  diurne reste rouille/ocre. Le halo n'agrandit pas le disque solaire physique.
- [NASA, Water Cycle](https://www.nasa.gov/general/water-cycle/) et
  [Cloudy Sols Are Here Again](https://science.nasa.gov/blog/cloudy-sols-are-here-again/) :
  nuages de glace d'eau ; Mars possède aussi des nuages de glace de CO₂. P8 ne rend
  que les voiles artistiques de glace d'eau, pas toute cette diversité.
- [NASA, Dust Cycle](https://www.nasa.gov/general/dust-cycle/) : transport de poussière,
  effets radiatifs et interaction avec les glaces. Ce constat motive l'apparence,
  sans transformer le bruit du jeu en modèle de climat.

Le HUD H rappelle 600 Pa / 0,6 % et indique le temps visuel et la visibilité. Aucune
pression locale n'est calculée ; pas de force de vent, dégât, dépressurisation,
modification d'oxygène ou production énergétique ajoutée en P8.

## Météo déterministe et rendu

`rendering/atmosphere.rs` évalue directement l'état depuis secondes du calendrier,
période de rotation de Mars du catalogue, position corporelle et graine. Aucune
intégration accumulée, appel aléatoire par frame, nouvel asset ou météo sérialisée.
Sauver le JD et les poses suffit à retrouver le même état ; changer de profil
ne transporte aucun minuteur météo. P fige et T accélère ciel, poussière et nuages.
Le menu laisse le calendrier évoluer comme auparavant ; physique/survie gardent
leurs secondes réelles et leurs politiques de suspension propres.

Conventions artistiques, sans unités physiques cachées : poussière 0,30–0,48 au
calme, jusqu'à 2,68 avec tempête ; couverture de glace 0,10–0,60, atténuée en tempête.
Une fenêtre potentielle par six rotations, probabilité déterministe 55 %, durée
maximale 0,7 rotation et enveloppe smoothstep continue. Les événements sont communs
au monde, avec poussière et nuages modulés lentement par la position ; pas de
front météo advecté, saison ou prévision scientifique.

Une seule caméra et le triangle de ciel existant. La même palette alimente ciel
et brouillard ; lumière directe atténuée par poussière/couverture moyenne,
ambiante 15–100 selon jour/nuit puis réduite en tempête. 11 000 lux de référence et
ces intensités sont des choix de présentation. Soleil sous l'horizon radial :
lumière directe nulle. Pas d'ombres de relief/bâtiments/nuages ni de transfert
radiatif spectral ; l'occlusion des objets du ciel par le relief reste la
profondeur réelle de la vue commune.

Les rayons intersectent une coquille de glace à altitude artistique 20 km, calculée
en unités de 20 km dans les axes du corps. L'observateur est préparé en f64 avant
conversion bornée ; les phases périodiques viennent du calendrier f64. Un motif
analytique de voiles animés évite textures nouvelles, raymarching ou volumes 3D.
Étoiles et disques subissent extinction par poussière et masque des voiles.
La nuit reste sombre avec étoiles visibles par temps clair ; elles peuvent
s'effacer sous une tempête dense. Le halo bleu se concentre autour du Soleil
bas au lever **et** au coucher, sans rendre tout le ciel bleu.

Les sources physiques f64, rayon angulaire solaire et inertialité des étoiles
restent celles de [ASTRONOMY](ASTRONOMY.md). Le bloc uniforme de ciel passe de 608
à 704 octets (matrice corporelle + deux vec4), sans nouvelle image ni passe de rendu.
Textures P7 conservées, 40,168 Mio utiles ; mémoire des meshes et total VRAM distincts.

## Validation

[Résultats effectifs et limites](VALIDATION.md), [mesures](PERFORMANCE.md).
Diagnostics spécifiques :

```bash
cargo run --locked --example p8_terrain_probe
python3 scripts/validate_sky.py --p8 --focus-x11 --mesa-immediate --output docs/ma-campagne-p8
```

Le probe est un diagnostic CPU en profil dev optimisé, pas une mesure GPU/release.
La galerie cherche des dates naturelles pour jour/lever/coucher/nuages/nuit/tempête,
prépare les poses/JD et vérifie caméra unique, rebasage par course, retours,
abri/menu/calendrier accéléré et autres faces/coins. P/T est aussi couvert par
les tests ECS des commandes existantes. Aucune météo de production n'est forcée.
Le diagnostic ne remplace ni une approbation esthétique ni les mesures sans captures.
