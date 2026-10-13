# Données astronomiques et conventions du prototype

Sources consultées le 12 octobre 2026. Toutes les données nécessaires au jeu sont embarquées dans `src/universe/catalog.rs` ; aucune requête réseau n'est exécutée par le jeu.

## Sources primaires

- [JPL — Approximate Positions of the Planets, table 1](https://ssd.jpl.nasa.gov/planets/approx_pos.html) : six éléments J2000 et leurs taux séculaires. Domaine d'ajustement 1800–2050. Le moteur borne conservativement l'horloge au 1 janvier 2050. Les éléments sont des ajustements approximatifs, pas des valeurs exactes ni des éléments moyens physiques. Le modèle calcule les variations séculaires, résout Kepler en radians en f64 et effectue les rotations orbitales.
- [JPL — Planetary Physical Parameters](https://ssd.jpl.nasa.gov/planets/phys_par.html) : rayons volumiques moyens, masses et périodes de rotation sidérale des huit planètes. Ce sont des valeurs publiées arrondies avec incertitudes, non des constantes exactes. Les masses publiées et les GM modernes proviennent de références différentes et ne sont pas forcées artificiellement à vérifier exactement GM=G×M.
- [JPL — Astrodynamic Parameters](https://ssd.jpl.nasa.gov/astro_par.html) : unité astronomique exacte IAU 2012, durée du jour, siècle julien, G CODATA 2018, GM Soleil et paramètres GM DE440. Les GM de Mars et des géantes représentent leurs systèmes, employés ici comme approximations pour les corps principaux ; le code n'est pas une solution N-corps.
- [NASA — Moon Fact Sheet](https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html) : rayon moyen lunaire, masse, demi-grand axe de 384 400 km, excentricité moyenne 0,0549, inclinaison écliptique de 5,145°, période sidérale 27,3217 jours et inclinaison axiale.
- [NASA — Sun Fact Sheet](https://nssdc.gsfc.nasa.gov/planetary/factsheet/sunfact.html) : rayon moyen solaire, masse, inclinaison et rotation adoptée à 16° de latitude. Le Soleil réel possède une rotation différentielle ; ici on utilise une rotation rigide de 609,12 heures.
- [NASA — Planetary Fact Sheet](https://nssdc.gsfc.nasa.gov/planetary/factsheet/planet_table_british.html) et fiches individuelles : inclinaisons axiales indicatives, catégories d'atmosphère. Les catégories sont qualitatives et ne constituent pas des modèles thermodynamiques.
- [NASA — Saturnian Rings Fact Sheet](https://nssdc.gsfc.nasa.gov/planetary/factsheet/satringfact.html) : limites C–A de 74 658 à 136 780 km du centre, division de Cassini entre 117 507 et 122 340 km. Opacité, couleurs et bandes intermédiaires du mesh sont artistiques. Anneaux D/F/G/E omis.

## Repères et échelle

Le repère héliocentrique est fixe au Soleil. Les coordonnées écliptiques J2000 NASA/JPL `(x,y,z)` sont transformées vers le repère direct Y-up du moteur par `(x,z,-y)`. Toutes les positions sont en mètres et tous les calculs astronomiques en f64. Les rayons sont moyens sphériques, sans ellipsoïde.

Le catalogue conserve les inclinaisons axiales publiées. Pour Vénus/Uranus, la rotation négative décrit déjà le mouvement rétrograde ; l'inclinaison visuelle utilise son complément à 180° afin de ne pas inverser deux fois la rotation. Les longitudes de pôles et la phase de rotation de départ sont conventionnelles. L'obliquité est appliquée autour d'un axe fixe par `CelestialBody::rotation`, commun à la simulation de surface et aux proxies graphiques ; ce n'est pas une orientation IAU complète référencée au plan de chaque orbite.

`ReferenceFrame` permet composition et conversion de repères rigides en f64. Le mode espace utilise un repère héliocentrique ou un repère **translatoire** attaché au corps visité ; il ne tourne pas avec sa surface. Le site de surface 0.2 est désormais fixé dans un repère **tournant** avec Mars, utilisant la même rotation que la sphère planétaire. Les repères des visites spatiales ne changent pas. `local_to_render` refuse les coordonnées hors de ±100 km avant conversion en f32. Le terrain local remplace la sphère globale de Mars en surface, avec une caméra en mètres et une profondeur indépendante. Les différences entre origine de tuile et caméra sont calculées dans le repère local f64 avant conversion, sans passage aller-retour par les coordonnées héliocentriques.

## Terre et Lune

La ligne Terre de JPL représente le barycentre Terre–Lune. La Terre est décalée de `−r_lune × M_lune/(M_terre+M_lune)` ; la Lune reçoit l'autre fraction. Le barycentre est donc conservé.

L'orbite lunaire est une ellipse moyenne non perturbée. La phase à J2000 est **choisie au périastre**, et le nœud et l'argument du périastre sont **conventionnellement nuls** : ces trois valeurs ne sont pas des mesures, ni une position réelle de la Lune à cette date. La période, le demi-grand axe, l'excentricité et l'inclinaison proviennent de la NASA. Une vraie phase lunaire/éphéméride et une orientation de rotation synchrone devront être ajoutées avant un usage d'observation.

## Temps et précision scientifique

L'époque est J2000, JD 2451545.0. La date est exprimée en jour julien du modèle, proche de TDB ; on ne convertit pas UTC, TT, TDB ni les secondes intercalaires. `SOLAR_JD` est une date du modèle, pas une API d'horloge civile.

Le GM du Soleil permet de vérifier la troisième loi de Kepler. Les périodes évaluées à partir de L̇−ϖ̇ sont des périodes anomalistiques approximatives, et peuvent différer des périodes sidérales tabulées. Pas de calcul de vitesse orbitale stockée dans cette version : la position est analytique en fonction du temps. Le déplacement de caméra ne correspond pas à une intégration gravitationnelle.

La précision numérique f64 ne supprime pas l'erreur du modèle JPL. Les positions ajustées peuvent avoir des écarts importants aux vraies éphémérides, en particulier pour les géantes. Les besoins futurs de positionnement exact devront introduire des données d'éphémérides **locales**, sans imposer un service réseau pour jouer.

## Prototype de surface Mars 0.2

Le rayon moyen de 3 389 500 m et le GM de 42 828,375816 × 10⁹ m³/s² sont réutilisés depuis le catalogue JPL décrit ci-dessus. L'accélération est `−GM × r / |r|³`, où `r` est la position relative au centre de Mars, en f64. Au rayon moyen elle vaut environ 3,728 m/s². Le contrôleur travaille en mètres/seconde réelles, indépendamment de l'accélération du calendrier orbital ; ce repère tournant reste une convention de gameplay, sans intégration inertielle du joueur ni forces d’inertie du repère.

Le site est choisi dans l'hémisphère éclairé au moment de la visite, sans latitude/longitude géographique revendiquée. Sa base fixe `Q_site` est un `DQuat` dans les axes du corps tournant. À l’entrée, la normale éclairée choisie dans les axes globaux est convertie par l’inverse de `Q_Mars(t)` ; elle est ensuite conservée dans le corps, et non réorientée vers le Soleil chaque frame. En coordonnées du site, la courbure est l'intersection avec la sphère moyenne : `sqrt(R² − x² − z²) − R`, évaluée sous forme rationnalisée pour éviter la perte numérique. Un relief sinusoïdal déterministe, de quelques mètres, est ajouté aux sommets ; il ne représente aucune donnée de cartographie martienne. Les contacts interpolent exactement les triangles du mesh, y compris sous l'empreinte du joueur, au lieu d'échantillonner une autre surface lisse.

Paramètres de gameplay : tuiles de 128 m, grille de 4 m, voisinage 3 × 3 à LOD unique, site borné à ±4 096 m, volume joueur 0,6 × 1,8 × 0,6 m, yeux à 1,65 m radialement au-dessus des pieds, marche 4,5 m/s, course 8 m/s, impulsion de saut 4 m/s, marge de contact 2 mm et adhérence aux petites variations de sol jusqu'à 10 cm par sous-pas. Le volume de contact est aligné sur la normale fixe du site, tandis que caméra et gravité utilisent la verticale radiale ; ce compromis est limité à ce site de quelques kilomètres. Les trois boîtes sont des obstacles de test, pas des roches observées.

La physique est cinématique, avec sous-pas de 1/120 s ; elle ne simule pas des corps dynamiques, des surplombs ou des pentes arbitraires. Aucun paramètre astronomique supplémentaire n'est introduit. Deux caméras composent fond astronomique compressé et terrain en mètres : l'occultation astronomique globale demeure approximative. Brouillard, albédo apparent, lumière ambiante et exposition sont des choix visuels.

Compatibilité de dépendance vérifiée le 12 octobre 2026 : [table officielle Avian](https://github.com/avianphysics/avian/blob/main/README.md), Bevy 0.19 / Avian 0.7. Avian n'a pas été ajouté ; Bevy reste épinglé exactement à 0.19.1 et aucune dépendance n'a changé.

## Composition du repère martien tournant

Aucun nouveau paramètre astronomique : la période sidérale signée de Mars est celle du catalogue JPL déjà embarqué, `1,02595676 × 86 400` secondes, et l’inclinaison indicative reste 25,19°. Ces valeurs publiées arrondies ne deviennent pas exactes par leur emploi en f64. On réutilise la convention historique :

```text
t = (JD − 2451545,0) × 86 400 secondes du modèle
angle(t) = 2π × rem_euclid(t / période_sidérale, 1)
Q_Mars(t) = rotation_Z(25,19°) × rotation_Y(angle(t))
Q(t) = Q_Mars(t) × Q_site
p_global = centre_Mars(JD) + Q(t) × (R × Y + p_site)
q_global = Q(t) × q_site
```

`p_site` est ici la position des yeux : pieds + normale radiale locale × 1,65 m. `q_site` compose alignement radial, yaw et pitch. Le pôle est fixe dans les axes du moteur, la phase est nulle à J2000 et la longitude du pôle est conventionnelle. Il n’y a ni orientation IAU complète, ni précession/nutation, ni longitude/latitude cartographique du site. Les textures et le relief demeurent artistiques.

`MarsTerrain::inertial_frame` contient l’origine du site **relative au centre de Mars** et son orientation vers les axes héliocentriques, toutes deux en f64. `SurfaceState` est l’état cinématique autoritaire en coordonnées du site ; `Explorer.local_m` reçoit la position des yeux relative au centre de Mars **dans les axes inertiels**, et `Explorer.rotation` l’orientation globale. `Explorer.global` ajoute seulement le centre orbital comme en navigation spatiale. Cette publication a lieu après la physique dans `GameSet::Navigation`, même si aucun sous-pas n’est exécuté, puis alimente l’origine flottante et la caméra du fond spatial.

Les triangles, contacts et obstacles sont fixes dans le site. La gravité est évaluée directement comme `−GM × (pieds + R × Y) / |pieds + R × Y|³` dans ces axes ; une rotation rigide laisse cette force centrale équivalente à sa conversion depuis les axes globaux. La caméra de surface reçoit la visée locale, et les proxies soustraient la position locale des yeux en f64 avant conversion bornée en f32. Aucun aller-retour depuis la position héliocentrique n’intervient dans ces différences métriques. La direction vers le Soleil depuis les yeux est convertie par `Q(t)⁻¹` pour la lumière locale. La lumière directe est coupée lorsque cette direction passe sous l’horizon radial du site, pour empêcher les parois de recevoir un Soleil situé sous le terrain en l’absence d’ombres. Cet horizon idéal utilise la verticale radiale des yeux ; il ignore la dépression de l’horizon due à l’altitude, l’occlusion du relief, la taille angulaire solaire et la réfraction. L’éclairage ambiant et le brouillard artistiques subsistent même la nuit ; 11 000 lux reste une intensité de présentation choisie, pas une photométrie martienne mesurée. Le terrain n’est pas régénéré quand Mars tourne : la révision change uniquement lors du choix d’un nouveau site, et le voisinage reste 3 × 3 à LOD unique.

### Deux temps et forces omises

L’horloge `SimulationClock` avance avec `Time<Real>` dans `GameSet::Simulation`, avant la navigation. T multiplie cette avance par 1, 10, 100 ou 1000 : l’orbite et `Q_Mars` utilisent ce même JD, donc le terrain, les obstacles, le joueur et sa visée globale sont entraînés à cette vitesse. La direction solaire locale évolue en conséquence. P, ainsi que la pause à la borne supérieure du calendrier, fige le centre orbital et la rotation ; la marche, le saut, la gravité, la visée souris et les contacts continuent avec le temps réel. Reprendre P ou changer T ne réinitialise ni pieds, vitesse, visée, site ni assets. Les deltas souris restent intégrés une seule fois, sans facteur delta time.

Le contrôleur accepte au plus 0,25 s de temps réel par frame en pas de 1/120 s ; le calendrier et le suivi rotatoire n’ont pas cette limitation. Après une interruption longue, la pose globale suit donc immédiatement le JD courant, tandis que la physique locale abandonne l’excédent. Aucun balayage ni interpolation du trajet d’entraînement astronomique n’est réalisé.

Les vitesses stockées et affichées sont relatives au site en m/s réelles, sans composante orbitale ni `ω × r`. Même lors d’un saut ou d’une chute, les coordonnées du joueur restent dans le site tournant : il est entraîné cinématiquement avec lui. Le modèle **omet** Coriolis `−2ω × v`, centrifuge `−ω × (ω × r)`, Euler `−dω/dt × r` et l’accélération translatoire du centre orbital. Il ne calcule pas les impulsions inertiales qu’un changement de T ou P impliquerait, ni les marées solaires. Il n’est donc pas une simulation dynamique rigoureuse d’un observateur tournant, surtout sous accélération temporelle. La gravité demeure uniquement martienne et radiale.

M/G/O/U restent des transitions de gameplay vers des poses spatiales de sécurité, sans transfert de vitesse inertielle. Les visites spatiales restent dans un repère translatoire ; le site demeure borné à ±4 096 m, sans déplacement mondial cube-sphere.

### Menu et horloges

Le menu principal et les options bloquent les entrées de navigation et libèrent le curseur ; ils ne modifient pas `SimulationClock`. `Time<Real>` continue d'entraîner orbites et rotation avec le réglage P/T choisi en jeu. La physique locale continue sans déplacement demandé, y compris la chute d'un joueur en l'air. Reprendre conserve le site et l'état physique. Les réglages de sensibilité et de champ de vision affectent les commandes de visée et la projection ; MSAA et plein écran affectent seulement la présentation. Aucune coordonnée physique ni unité n'est changée.

## Géométrie cube-sphere P3.1

Le [contrat cube-sphere](CUBE_SPHERE.md) rend opérationnelles les conventions historiques de faces et de graines : conversions f64, voisins orientés et deux patches de même niveau partageant leurs sommets. Les positions sont en mètres dans les axes du corps tournant, au rayon moyen du catalogue. La projection radiale du cube, le niveau 15 et les 32 segments de la démonstration sont des choix numériques/de diagnostic, sans nouvelle donnée astronomique. Aucune cartographie ni relief mesuré n'est introduit. La démonstration P3.1 est indépendante du FPS. Le terrain par défaut conserve les conventions ci-dessus ; le parcours optionnel P3.2 est décrit ci-dessous.


## Contacts cube-sphere P3.2 optionnels

Aucun paramètre astronomique ni dépendance ajouté. `SOLAR_CUBE_SURFACE=1` utilise une paire de patches centraux d'une même face, au rayon moyen martien du catalogue, sans relief. La face dominante éclairée est sélectionnée dans les axes du corps à l'entrée ; le site au milieu du raccord est ensuite fixe dans le corps tournant. Niveau 15, 64 segments, rectangle intérieur avec réserve de 1 m, couleurs distinctes et position initiale à x=−8 m sont des conventions numériques/de gameplay, détaillées dans [CUBE_SPHERE](CUBE_SPHERE.md). La taille n'est pas extrapolable à tous les patches de la projection.

Un seul `Q_site` sert aux deux patches, aux obstacles et au joueur en l'air ou au sol. La composition `Q_Mars(t) × Q_site`, les visites translatoires, la publication après physique, la verticale radiale et les forces d'inertie omises ne changent pas. Les triangles f64 communs au rendu fournissent exactement le maximum sous l'empreinte rectangulaire alignée au site. Le rendu conserve les mètres 1:1 et soustrait les yeux dans ce même repère en f64 avant conversion f32 ; aucune position astronomique ne remplit directement un Transform. Les proxies ont une erreur numérique testée inférieure à 20 µm sur cette paire.

La paire et le joueur sont bornés au rectangle résident, sans migration du site, passage jouable entre faces ou chargement suivant. Le FPS historique garde par défaut ses neuf tuiles, son relief artistique et ±4 096 m. Les marches/sauts et gravité de la paire restent en secondes réelles, indépendantes de P/T et limitées à 0,25 s par frame ; rotation et calendrier suivent toujours `Time<Real>`. La paire sphérique ne revendique ni cartographie, ni hauteur mesurée, ni orientation IAU exacte.


Le bornage annule désormais la composante horizontale de vitesse sur chaque axe bloqué, pour la paire comme pour la borne historique. La position restait déjà contrainte ; la vitesse publiée correspond maintenant à cet arrêt. Les paramètres de marche/saut, la gravité et les horloges demeurent ceux du contrôleur existant.

## Changement de carte locale P3.3 opt-in

`SOLAR_CUBE_MIGRATION=1` permet le remplacement réversible de la paire centrale par sa voisine +U, sur la même face. Aucun paramètre astronomique nouveau : même rayon moyen, GM, `Q_Mars(JD)` et forces d'inertie omises. Les conventions et seuils bornés sont détaillés dans [CUBE_SPHERE](CUBE_SPHERE.md).

`Q_site` peut désormais changer après un sous-pas, par une transaction f64 conservant les positions des pieds/yeux, la visée et la vitesse **dans le corps tournant**. Le transfert utilise les origines relatives au centre de Mars ; il ne soustrait jamais deux positions héliocentriques pour calculer une différence métrique. Le transport d'une base de visée conserve l'orientation complète, y compris la petite composante que yaw/pitch seuls perdraient. La pose est toujours publiée après physique, même sans sous-pas ; aucune migration ne modifie les horloges ou le compromis d'entraînement en l'air.

Le support et la gravité sont recalculés dans la carte active ; la boîte du joueur reste alignée sur celle-ci. Les obstacles demeurent fixes dans le corps, avec une enveloppe conservatrice de contact et leur rotation exacte en rendu. Les meshes communs et leurs normales radiales sont reconstruits seulement lors d'un remplacement, jamais à chaque rotation. Le terrain historique par défaut et l'option P3.2 seule gardent leurs parcours sans migration. Les visites spatiales et sorties M/G/O/U restent translatoires et sans transfert de vitesse inertielle. Pas de cartographie, orientation IAU exacte, relief mesuré, passage entre faces ou forces d'inertie nouvelles.

## Migration hors centre P3.4 opt-in

`SOLAR_CUBE_OFFCENTER=1` choisit un site fixe près de u=0,5/v=0,5 de la face sélectionnée dans le corps, puis permet une seule migration réversible vers sa voisine +U. Ce choix déplace le site de gameplay par rapport au diagnostic central ; il ne représente ni une latitude/longitude observée ni une nouvelle orientation IAU. Le rayon, GM, rotation martienne, origine relative au centre, horloges et forces omises restent ceux ci-dessus. Les seuils sont maintenant calculés depuis le recouvrement des deux rectangles certifiés et exprimés dans la carte d'entrée ; ils ne sont pas des paramètres astronomiques.

Le transport f64 conserve les poses dans le corps tournant au sol et en l'air, toujours entraînées par Mars. La boîte joueur reste reboîtée selon la carte active : déplacement maximum mesuré numériquement des coins ~83 µm, admission plafonnée à 0,25 mm et exclusion des contacts d'obstacles. L'enveloppe conservatrice des objets fixes est réévaluée avant allocation (plafond d'expansion 0,5 mm par dimension). Ces petits angles ne valident pas de grand changement d'orientation. [CUBE_SPHERE](CUBE_SPHERE.md) expose les formules, réserves et limites. Les proxies continuent de soustraire les positions locales en f64 avant conversion explicite f32 ; aucun changement de simulation/rendu astronomique ou des visites translatoires.


## Terrain mondial : repère local mobile

13 octobre 2026 : le parcours normal utilise désormais une sphère martienne complète avec chargement local dynamique sur les six faces. Les paragraphes de site fixe/paire ci-dessus décrivent les anciens modes, conservés en régression. Aucun paramètre astronomique n’est ajouté : rayon moyen, GM, rotation conventionnelle et axes du corps restent ceux du catalogue. Niveau 15/32 segments, voisinage ≤13 patches, index 8 m et rebasage 64 m sont des choix numériques/de gameplay, sans relief ni cartographie mesurée.

Le repère local est mobile dans le corps : sa normale est transportée vers la direction radiale des pieds par rotation minimale, indépendamment des faces du cube. L’origine reste relative au centre de Mars. Les pieds, vitesse et base de visée sont transférés en f64 avant publication ; le rendu soustrait les origines/yeux en f64 avant conversion explicite en f32. La boîte joueur reste alignée sur le petit repère actif, avec petits changements par remplacement ; les objets fixes dans le corps utilisent maintenant leurs boîtes orientées exactes en contact pour ce parcours, sans expansion AABB cumulée. [Contrat complet](GLOBAL_TERRAIN.md).

Le calendrier utilise toujours Time<Real> ; P/T entraînent Mars mais ne changent pas marche/gravité/saut. En l’air, le joueur reste entraîné cinématiquement par le corps. Coriolis, centrifuge, Euler et accélération translatoire restent omises ; M/G/O/U et les visites spatiales restent des transitions translatoires sans transfert inertiel. Aucune grande rotation instantanée d’une boîte joueur n’est introduite aux arêtes.


## P4 : survie et site de session

Aucun paramètre astronomique nouveau. Les abris utilisent des poses DVec3/DQuat fixes dans les axes du corps martien ; leur rotation inertielle demeure Q_Mars(t) × Q_abri. Leurs proxies soustraient les yeux en f64 avant conversion f32. La réserve de 180 s, sa consommation 1 s/s et la recharge 30 s/s sont des conventions de gameplay, pas des données physiologiques. Le menu et l’espace suspendent la survie ; P/T n’en changent pas le rythme.

La première arrivée choisit le site éclairé selon la convention existante ; les retours normaux M de cette session conservent ce site au lieu de choisir à nouveau un point éclairé. R après échec retourne au premier abri. Le site peut donc être nocturne plus tard ; les visites spatiales restent translatoires sans transfert inertiel. Les scénarios de régression conservent leurs anciennes règles. [SURVIVAL](SURVIVAL.md) détaille les poses, collisions et limites.

## P5 : ressources et constructions

Aucun paramètre astronomique nouveau. Les caisses sont des objets artistiques de récupération, le stock en unités d’alliage une convention de gameplay sans masse physique implicite. Les abris construits gardent des poses DVec3/DQuat fixes dans le corps martien, rendues après soustraction corporelle f64 et conversion locale explicite f32. L’admission utilise les triangles existants et les volumes f64 ; ni rayon, GM, rotation/horloges, repères de visite ni forces omises ne changent. [CONSTRUCTION](CONSTRUCTION.md) expose les unités, seuils, ressources de session et limites. Le terrain conserve sa résolution fixe, sans LOD adaptatif.

## LOD adaptatif du terrain mondial

Aucun paramètre astronomique nouveau : le même rayon moyen et les mêmes sommets corporels f64 décrivent la sphère. 32/16/8 cellules intérieures, seuils de distance/hystérésis, bord commun 32 et index local ±72 m sont des choix numériques. La zone proche conserve les triangles fins et les mêmes contacts. Poses/origine flottante, horloges, transport minimal, forces omises et visites translatoires restent inchangés. Il n’y a ni relief mesuré ni orientation IAU supplémentaire. [GLOBAL_TERRAIN](GLOBAL_TERRAIN.md).
