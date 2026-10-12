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

Le catalogue conserve les inclinaisons axiales publiées. Pour Vénus/Uranus, la rotation négative décrit déjà le mouvement rétrograde ; l'inclinaison visuelle utilise son complément à 180° afin de ne pas inverser deux fois la rotation. Les longitudes de pôles et la phase de rotation de départ sont conventionnelles. L'obliquité est appliquée graphiquement autour d'un axe fixe ; ce n'est pas une orientation IAU complète référencée au plan de chaque orbite.

`ReferenceFrame` permet composition et conversion de repères rigides en f64. Le mode espace utilise un repère héliocentrique ou un repère **translatoire** attaché au corps visité ; il ne tourne pas avec sa surface. La future surface demandera un repère tournant et des zones locales. `local_to_render` refuse les coordonnées hors de ±100 km avant conversion en f32. Le rendu de sphères globales ne garantit pas la précision au centimètre d'un futur terrain ; les zones locales devront le remplacer à proximité.

## Terre et Lune

La ligne Terre de JPL représente le barycentre Terre–Lune. La Terre est décalée de `−r_lune × M_lune/(M_terre+M_lune)` ; la Lune reçoit l'autre fraction. Le barycentre est donc conservé.

L'orbite lunaire est une ellipse moyenne non perturbée. La phase à J2000 est **choisie au périastre**, et le nœud et l'argument du périastre sont **conventionnellement nuls** : ces trois valeurs ne sont pas des mesures, ni une position réelle de la Lune à cette date. La période, le demi-grand axe, l'excentricité et l'inclinaison proviennent de la NASA. Une vraie phase lunaire/éphéméride et une orientation de rotation synchrone devront être ajoutées avant un usage d'observation.

## Temps et précision scientifique

L'époque est J2000, JD 2451545.0. La date est exprimée en jour julien du modèle, proche de TDB ; on ne convertit pas UTC, TT, TDB ni les secondes intercalaires. `SOLAR_JD` est une date du modèle, pas une API d'horloge civile.

Le GM du Soleil permet de vérifier la troisième loi de Kepler. Les périodes évaluées à partir de L̇−ϖ̇ sont des périodes anomalistiques approximatives, et peuvent différer des périodes sidérales tabulées. Pas de calcul de vitesse orbitale stockée dans cette version : la position est analytique en fonction du temps. Le déplacement de caméra ne correspond pas à une intégration gravitationnelle.

La précision numérique f64 ne supprime pas l'erreur du modèle JPL. Les positions ajustées peuvent avoir des écarts importants aux vraies éphémérides, en particulier pour les géantes. Les besoins futurs de positionnement exact devront introduire des données d'éphémérides **locales**, sans imposer un service réseau pour jouer.
