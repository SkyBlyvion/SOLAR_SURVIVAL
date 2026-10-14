# Après le MVP : direction visuelle et textures

14 octobre 2026. **Cadrage de P7, pas une fonctionnalité déjà implémentée.** Demande utilisateur : poursuivre après P1–P6, améliorer la qualité et remplacer les surfaces provisoires par des textures sur mesure ou des ressources NASA/JPL. [Ordre des travaux](ROADMAP.md), [état livré](VALIDATION.md).

## État observé

Au HEAD 6898bdb, `assets/` contient deux shaders et la police, aucune bibliothèque de textures de surface. `planet.wgsl` dessine les continents, bandes et couleurs par bruit procédural ; `rendering/terrain.rs` utilise un matériau presque uni. Abris/caisses/obstacles sont des volumes simples avec couleurs unies. Le panorama stellaire est artistique. Le terrain mondial est une sphère sans relief, avec LOD de résolution 32/16/8 ; la brume de surface est limitée à 40–60 m. Ces limites expliquent une partie de l'apparence actuelle, même après la refonte UI.

Le premier objectif est une scène martienne cohérente et lisible à hauteur humaine, avec une amélioration visible des visites Terre/Mars. Le remplacement de toutes les planètes, l'horizon lointain et le relief ne sont pas nécessaires pour livrer cette première amélioration complète. Aucun changement des règles de survie, des identités d'abris ou des sauvegardes n'est requis pour des matériaux seuls.

## Deux échelles de textures

| Usage | Choix prévu | Limite à respecter |
|---|---|---|
| Terre/Mars depuis l'espace | Cartes planétaires issues de sources officielles, ramenées à une résolution adaptée au jeu | Une texture globale ne fournit pas le détail sous les pieds ; documenter projection, orientation, pôle et méridien de référence |
| Sol proche | Matériaux sur mesure : poussière/sable et roche, couleur, normales et rugosité | Détail artistique inspiré d'observations, sans prétendre une reconstruction scientifique du site |
| Abri et caisses | Matériaux industriels cohérents : métal peint, métal exposé, joints et marquages ; détails visuels lisibles | Garder le même bâtiment, sa porte utile et ses volumes de contact ; ne pas remplacer une pose corporelle par un objet attaché au patch |
| Ciel/lumière | Harmonisation de l'exposition, des couleurs et des contrastes avec les matériaux | Préserver directions solaires, masquage et caméra unique ; les ombres ou effets supplémentaires doivent être mesurés |

Une photographie de rover comporte sa propre perspective et son éclairage. Elle sert d'abord de référence ; la couleur de base d'un matériau doit éviter de cuire à nouveau des ombres que le moteur recalculera. Les normales/rugosités créées à partir d'une photo ou dessinées restent des interprétations artistiques. Ne pas utiliser une image en couleurs rehaussées comme preuve de la couleur naturelle de Mars.

## Sources candidates vérifiées

Pages consultées le **14 octobre 2026**. Aucun nouveau fichier d'image téléchargé ou intégré lors de ce cadrage.

- [NASA — Mars Image Texture](https://science.nasa.gov/3d-resources/mars/) : texture destinée aux modèles 3D, issue d'images Viking traitées à l'USGS et de cartes JPL/Caltech ; JPEG/TIFF proposés. Candidate pratique pour une première Mars orbitale ; dimensions et conventions exactes à contrôler sur le fichier avant intégration. Crédit de la fiche : NASA/Jet Propulsion Laboratory & Caltech.
- [NASA — Blue Marble Next Generation, Base Map](https://science.nasa.gov/earth/earth-observatory/blue-marble-next-generation/base-map/) : composites mensuels terrestres, déclinaisons globales et tuilées. Choisir un composite et une taille raisonnables, sans simuler automatiquement les saisons à partir d'une image statique.
- [USGS — Mars Viking Colorized Global Mosaic 232m](https://astrogeology.usgs.gov/search/map/mars_viking_colorized_global_mosaic_232m) : environ 232 m/pixel à l'équateur, mosaïque explicitement colorisée, coordonnées planétocentriques/longitudes est. La fiche déclare public domain ; le produit intégral annoncé est volumineux. Source de référence cartographique, pas une texture de sol au centimètre ni un gros téléchargement à imposer au jeu.
- [NASA/JPL-Caltech/MSSS — Raw, Natural and White-Balanced Views](https://science.nasa.gov/resource/raw-natural-and-white-balanced-views-of-martian-terrain/) : comparaison utile pour séparer image brute, estimation de couleur naturelle et balance des blancs sous éclairage terrestre. Référence pour décider la palette, pas un jeu de matériaux PBR prêt à utiliser.
- [JPL — Image Use Policy](https://www.jpl.nasa.gov/jpl-image-use-policy/) : vérifier le crédit et les exceptions de chaque image, notamment les propriétaires tiers ; ne pas supposer qu'un crédit générique NASA remplace les informations propres à l'asset.

Le catalogue actuel emploie des axes/phase de rotation conventionnels, pas une orientation IAU complète. Plaquer une carte authentique ne corrige pas automatiquement cette convention. P7 doit documenter la correspondance visuelle retenue et ne pas revendiquer un atterrissage géographique exact. Une intégration de relief mesuré/cartographie demanderait un contrat séparé.

## Production et intégration

Choisir quelques références concrètes et produire un premier ensemble cohérent, puis juger dans le jeu. Les matériaux doivent rester propres et fonctionnels, avec usure et poussière à une échelle crédible ; les couleurs doivent distinguer ressources, abris et sol sans rendre tout brillant ou uniformément orange.

Pour chaque asset effectivement retenu, enregistrer source/page et fichier, auteur/crédit, conditions applicables, date, empreinte SHA-256, dimensions, espace colorimétrique, échelle métrique et transformations réalisées. Les sources maîtres et exports doivent pouvoir être retrouvés ; éviter les jeux de données massifs dans Git. Préparer hors exécution les conversions, raccords, variantes de résolution et niveaux de texture nécessaires. Le jeu reste autonome hors ligne, avec assets locaux et licences incluses dans l'archive.

L'échantillonnage du sol doit rester fixe dans le corps martien : aucun glissement ou changement d'échelle aux rebasages, aux frontières de patches et aux coins. Calculer les références/phasages nécessaires en f64 avant leur représentation locale f32. Ne pas échantillonner une position astronomique brute convertie en f32. Couleur de base en sRGB ; normales et paramètres de matériau traités comme données linéaires, suivant les API locales Bevy 0.19.1. N'ajouter un format/décodeur ou une dépendance que si l'asset retenu le nécessite réellement.

Charger et partager textures/matériaux, sans génération d'image/mesh par frame. Choisir résolution, filtrage et niveaux de texture à partir de la taille à l'écran et des mesures ; pas de « 8K partout ». Établir le budget textures CPU/GPU et les temps de chargement avant/après sur le poste actuel ; la cible RTX 4050/Core i5 reste à relever et à mesurer séparément. Ne pas annoncer un plafond VRAM universel sans mesure.

## Contrainte matérielle et budgets de départ

Précision utilisateur du **14 octobre 2026** : le jeu doit viser des machines modestes ; la **RTX 4050 avec 6 Go de VRAM constitue le haut de la cible**, pas un minimum à dépasser. Le CPU indiqué est « i5 12700K » ; la référence [12700K est officiellement un Core i7](https://www.intel.com/content/www/us/en/products/sku/134594/intel-core-i712700k-processor-25m-cache-up-to-5-00-ghz/specifications.html). Conserver le modèle exact à confirmer, sans supposer un autre i5. RAM, puissance/configuration GPU et alimentation restent à relever. L'objectif de travail est **1080p/60 FPS sur cette cible**, à vérifier sur le matériel ; aucune garantie ni nouvelle mesure n'est produite par ce cadrage.

Budgets initiaux choisis pour P7, révisables seulement avec des mesures documentées :

| Élément | Budget/choix de départ |
|---|---|
| Sol et matériaux proches | Textures répétables **1 024×1 024** par défaut ; petits objets **512×512** ; atlas et matériaux partagés entre instances |
| Planètes prioritaires | Cartes Terre/Mars **2 048×1 024** au départ ; pas de chargement des maîtres scientifiques haute résolution dans le jeu |
| Exception de résolution | **2 048×2 048 maximum** pour un matériau proche uniquement si le gain est visible et le budget respecté ; aucune texture 4K/8K prévue en P7 |
| Nouvelles textures P7 résidentes | **128 Mio maximum estimés**, toutes cartes/niveaux de texture compris dans leur format GPU réellement chargé ; compter aussi une représentation décompressée si la compression n'est pas utilisée |
| Pics de chargement | Suivre séparément copies CPU, staging et allocations transitoires ; le budget résident n'est pas une preuve de pic RAM/VRAM |
| Cartes de matériau | Couleur de base ; ajouter normales et rugosité seulement si utiles à l'écran ; paramètres constants ou regroupés quand ils suffisent |
| Éclairage et shaders | Réutiliser la lumière/caméra existantes ; aucun ray tracing ni effet coûteux requis ; limiter les couches de mélange et mesurer leur coût GPU |
| Chargement | Réutilisation entre patches et bâtiments, pas de duplication par instance ; libération des assets spécifiques devenus inutiles ; ni téléchargement ni traitement des sources maîtres en jeu |

Ce sont des **budgets de production**, pas des limites déjà contrôlées dans le code. Le total VRAM du jeu inclut aussi meshes, buffers, profondeur, rendu HDR/MSAA et pilote ; ne pas confondre le poids du PNG/JPEG sur disque avec l'occupation une fois chargé. Garder une marge sur les 6 Go et relever consommation réelle/pics quand l'outil et le pilote le permettent. Aucune baisse de poids de fichier ne suffit à démontrer un gain GPU.

Les photos de rovers déterminent la palette et les détails à reproduire, pas la résolution des fichiers distribués. Fournir des exports plus petits pour le réglage réduit si nécessaire, avec filtrage/niveaux de texture adaptés et sans scintillement gênant. Les profils qualité sont à implémenter selon les besoins observés ; ne pas les déclarer disponibles maintenant. La GTX 1060 3 Go reste utile pour détecter tôt les excès, sans extrapoler ses FPS vers la RTX 4050.

Comparer avant/après sans captures, à résolution/MSAA/site/visée identiques : frame moyenne/p95, CPU/GPU si disponibles, temps de chargement et nombre/octets d'assets résidents. Si les textures provoquent dépassement, saccades ou coût de shader important, réduire taille/cartes/couches et corriger avant de livrer P7. L'aspect visuel doit s'améliorer dans ces contraintes ; la résolution et les effets ne servent pas de mesure de qualité.

## Critères de fin P7

1. Terre et Mars portent des cartes sourcées et correctement orientées dans la convention documentée ; pas de couture ou de pôle manifestement cassé.
2. Sol martien proche avec poussière/roche à échelle cohérente, détails de surface stables et répétition visuellement acceptable ; pas de texture qui nage au rebasage.
3. Le même abri et les caisses ont une apparence cohérente, porte et ressources immédiatement identifiables ; placement autorisé/interdit toujours lisible.
4. Captures comparables avant/après : espace, sol, abri extérieur/intérieur, jour/nuit, placement, rebasage et retour depuis une aventure chargée. Inspection des originaux et corrections avant de déclarer l'étape faite.
5. Mesures sans captures aux mêmes réglages : temps de frame, chargement et budgets d'assets ; budget P7 de 128 Mio de nouvelles textures respecté, pics et régressions traités ; objectif cible 1080p/60 FPS à vérifier nativement.
6. Tests adaptés aux matériaux/chargement/f64, régressions survie/construction/profils, contrôles Cargo et archive hors checkout avec crédits. L'approbation esthétique de l'utilisateur reste distincte des vérifications techniques.

Le relief mondial, les grands horizons et le streaming asynchrone sont prévus en P8. Une extension de sauvegarde ultérieure devra préserver les aventures P6 via une migration explicite ; aucun nouveau contenu ne doit rendre silencieusement les profils existants illisibles.
