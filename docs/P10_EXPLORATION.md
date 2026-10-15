> Depuis P12 : écriture **SOLAR_SURVIVAL 6**, lecture **4/5/6** ; commandes par aventure, valeurs par défaut pour4/5. Formats1/2/3 refusés sans suppression. [Contrat courant](P12_CONTROLS.md). Les formats cités dans les sections historiques ci-dessous décrivent leurs livraisons propres.

> Depuis P11 : écriture au format **5**, lecture P10/**4** conservée avec réglages audio par défaut. Les règles P10 ci-dessous restent en vigueur. [Audio et compatibilité](P11_AUDIO.md).

# P10 — expéditions, balises et objectifs

15 octobre 2026. Boucle : **partir de la base → relever une balise → rentrer au
sol → transmettre → recevoir une récompense → débloquer la suivante**. Trois
expéditions par aventure. Installations/noms fictifs, aucune cartographie mesurée.

## Jouer

Créer une **nouvelle aventure au format 4**, puis M. Le HUD à droite donne mission,
cap relatif à la visée, distance et récompense. Mâts sombres à tête orange,
turquoise après validation du rapport ; seules les balises débloquées sont rendues.
H/N, inventaire et construction masquent ce panneau pour libérer leurs écrans.
Le HUD d'oxygène reste indépendant. Aucun menu supplémentaire n'est nécessaire.

| Objectif, dans l'ordre | Distance approximative de la base | Récompense portée |
|---|---:|---|
| Relais Aurore | 120 m | 8 alliage + 4 circuits |
| Station Horizon | 240 m | 12 alliage + 6 circuits |
| Sonde Lointaine | 360 m | 16 alliage + 8 circuits |

**F à moins de 3 m** relève la balise active. Liaison radio de proximité, sans
visée ni test de segment : un abri construit autour ne bloque pas la mission.
Les balises n'ont ni collision, ni O₂, ni inventaire. Une balise future ne peut
être relevée avant validation de la précédente. Relevé instantané, sans minuterie
ou coût électrique implicite. Ces paramètres sont des conventions de gameplay.

Revenir **dans le premier abri**, par sa porte, puis **F** pour transmettre. Le
même volume utile P4 (boîte joueur entière à l'intérieur) est vérifié. Un autre
abri ou la proximité derrière un mur ne valident pas le rapport. La transmission
reste disponible si la réserve O₂ de la base est vide.

Le lot entier doit tenir dans le porté : copie candidate, ajout des deux types,
puis publication unique avec avancement. Refus sans ajout partiel, sans perte de
données et sans déblocage si le porté est plein. Déposer dans un coffre P9 puis
réessayer. Ni récompense répétable ni ponction distante. Total : 36 alliage et
18 circuits. Après le troisième rapport : 3/3 et exploration libre.

## Risque et navigation

Combinaison 180 s, marche 4,5 m/s, course 8 m/s, gravité et recharge finie P9
conservées. Les trajets admettent un aller-retour en course, avec marge pour
tourner/entrer par la porte. Le dernier représente environ 90 s de course en ligne
droite, davantage avec détours. Pas de réserve portable ou de desserte éloignée
requise ; se recharger entre missions. Nuit/poussière/relief P8 restent présents.

Le cap indique devant/derrière ou gauche/droite et l'angle en degrés, projeté
horizontalement depuis la visée. Le HUD garde aussi direction/distance du premier
abri. La **borne inférieure** de retour à pied (`distance corporelle / 4,5 m/s`) et
la marge conventionnelle de 20 s ne calculent pas d'itinéraire : murs, relief,
détours et manipulation ne sont pas évalués. Un avertissement conseille de rentrer
ou courir lorsque l'O₂ passe sous cette estimation. Ce n'est pas une garantie.

**Épuisement d'O₂ ou sortie spatiale M/G/O/U : données non rapportées perdues.**
La balise reste connue mais doit être relevée à nouveau ; rapports payés conservés.
R garde le secours P9 sans validation ni récompense. M ne livre donc pas le relevé
par téléportation. Le menu conserve les données et suspend la survie ; inventaire
et catalogue laissent l'O₂ actif mais bloquent F. P/T ne change ni mission ni pose.
Sauvegarder/charger garde un relevé en cours sur place, sans rattrapage du temps fermé.

## Architecture et bornes

`expedition.rs` possède trois découvertes, le préfixe des rapports validés et le
booléen de données en cours. Son système suit navigation, survie et construction
dans GameSet::Navigation ; rendu/UI viennent ensuite. Ordre global, horloges,
souris et données astronomiques conservés.

Les poses DVec3/DQuat sont dérivées une fois du premier abri, de son orientation,
du rayon et de la graine P8. Décalages locaux en mètres : `(72, −96)`, `(192, −144)`,
`(−216, −288)` en X/Z, puis projection radiale et relief artistique P8. Aucun patch
lointain construit pour ces points. Mât de −0,3 à +2,5 m autour de cette hauteur ;
les petits écarts champ analytique/triangles ne changent pas le support du joueur.
Poses fixes dans Mars, indépendantes du streaming/JD. Modes historiques
`SOLAR_MARS_GLOBAL=0` et diagnostics sans survie : P10 désactivé.

`rendering/expedition.rs` : deux proxies de cube par balise débloquée dans 600 m,
supprimés en espace/hors portée/profil vide. Cube existant partagé, trois matériaux
constants, aucune nouvelle image/lumière/passe. Au plus six proxies, donc **312**
avec le plafond P9, hors terrain/ciel/obstacles. Entités communes conservées au
rebasage ; soustraction position corporelle−yeux et rotation locale en f64 avant
conversion explicite f32. Aucun mesh créé par frame, aucun proxy autoritaire.

`ui/expedition.rs` : HUD et cap séparés. `persistence.rs` : **SOLAR_SURVIVAL 4**, trois
découvertes, nombre de rapports, données en cours. Poses reconstruites depuis la
base restaurée. Validation avant publication : 0–3 rapports, découvertes dans
l'ordre, données seulement pour la mission active connue en surface sans échec,
progression vide avant installation de la base. Formats 1/2/3 refusés sans
migration ni suppression, selon la convention des étapes précédentes. Profils,
checksum, secours et plafond 256 Kio conservés.

## Vérifications et limites

Tests CPU/ECS sans renderer ni fenêtre OS : trois allers-retours par vraies entrées
de course, porte/recharge finie, récompenses uniques, refus atomique pour porté
plein, portée/ordre, écrans, quatre sorties spatiales et échec/R, cap/pitch/rebasage,
deux profils et nouvelle App, relevé en cours et récompense conservés. Certains
cas ciblés préparent leurs poses ; seul le test des trois trajets fait tout le
parcours au sol, avec visée automatiquement orientée par le test. Un test de rendu
sans GPU vérifie précision, réutilisation et déchargement des proxies.

[Résultats](VALIDATION.md), [notes P10](p10-validation/2026-10-15/NOTES.md).
**Aucune fenêtre de test automatique** : consigne utilisateur sans focus prioritaire.
Aspect/lisibilité dans le jeu, transitions des mâts, GPU et archive lancée restent
à vérifier dans un créneau explicite. Les tests ECS ne remplacent pas une partie
humaine ni une inspection esthétique. Windows 11 et RTX4050/1080p60 non validés.
