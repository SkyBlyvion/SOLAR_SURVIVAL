> P11 : écriture au format **5**, lecture P10/**4** conservée ; volume/muet et alertes. [Contrat courant audio](P11_AUDIO.md).

> P10 utilise désormais le format 4 et ajoute les objectifs, sans migration des anciens profils. Les mentions de format 3 ci-dessous décrivent la livraison P9. [Contrat courant P10](P10_EXPLORATION.md).

# P9 — base, inventaire et stockage

Implémentation du 15 octobre 2026. Boucle : **récolter → transporter → déposer /
reprendre → construire → produire / stocker → recharger → repartir**. Les valeurs
ci-dessous sont des **conventions de gameplay**, sans masse implicite des objets,
chimie, rendement photovoltaïque ou physiologie scientifiques.

## Jouer et démarrer

Créer une **nouvelle aventure format 3**, puis M. Le premier abri dispose d'une
réserve initiale **finie de 720 secondes d'autonomie**, distincte de la combinaison
pleine de 180 s. Les huit caisses dorées donnent encore 6 alliage chacune.
Quatre points de récupération durables sont derrière l'arrivée : x=±12 m,
z=+4 m pour l'alliage (ocre), x=±16 m, z=+4 m pour les circuits (cyan), dans la
carte initiale. Leur pose est ensuite fixe dans Mars. Ces dispositifs de
récupération sont artistiques ; ce ne sont pas des gisements géologiques mesurés.

**E à 2,5 m** récolte un lot entier de six objets, avec segment d'accès dégagé.
Un refus pour capacité pleine conserve le lot. Le point vide reste visible,
se recharge après **30 s actives**, puis fournit six objets à nouveau. Seuls
quatre points sont conservés ; pas de génération mondiale à chaque déplacement.
M, R et le streaming ne les recréent pas. Les circuits servent effectivement aux
recettes électriques. Aucun consommable chimique n'est requis pour le générateur.

Amorçage possible : 40 alliage et 14 circuits construisent un coffre et un
exemplaire de chacun des quatre équipements. Les caisses fournissent 48 alliage ;
les deux sources de circuits donnent 12, puis un renouvellement fournit les six
suivants. La réserve initiale laisse recharger pendant cette mise en place.
Les sources renouvelables permettent ensuite d'agrandir l'installation. En cas
d'échec, R conserve la base et les contenus, revient au premier abri et rend une
combinaison pleine : **secours de gameplay illimité**, sans remplir la base.
Une nuit sans batterie chargée ne bloque donc pas définitivement l'aventure.

## Inventaire et coffres

- Porté : **8 emplacements** ; coffre : **16 emplacements** ; **20 objets/pile**.
  Deux types utiles : alliage et circuits. Aucune limite de masse fictive.
- **I** ouvre le porté et le coffre accessible le plus proche ; **E** devant un
  coffre ouvre aussi cet écran. Sans coffre proche, le porté reste consultable.
  I / Échap ferme ; un coffre devenu distant, masqué, absent, un menu principal,
  une sortie spatiale ou un échec ferme l'accès. L'oxygène reste visible.
- Cliquer une pile sélectionne le côté source. **Quantité** propose 1/5/20 ;
  **↑/↓** règle toute quantité exacte entre 1 et 20. **Transférer quantité** ou
  **Transférer pile** refuse l'ensemble si impossible. **Maximum admissible**
  transfère explicitement le minimum entre cette pile et la place disponible,
  et annonce le nombre réellement déplacé. Maximum ne parcourt pas toutes les
  piles du type : sélectionner une autre pile pour continuer.
- **Regrouper** compacte le côté sélectionné par type. **Déplacer la pile**, puis
  clic dans le même inventaire, échange les deux emplacements (y compris vides).
  Changer de côté exige une commande de transfert.
- Portée et segment yeux→coffre sont revalidés à chaque opération ; seul le volume
  propre du coffre est ignoré dans ce segment. Les murs et obstacles bloquent.
  Le coffre est une boîte solide ; ses contenus ne sont pas liés au proxy.

Les mutations de `inventory.rs` vérifient quantités et capacités avant débit.
Une demande exacte refusée ne publie aucune mutation. Les actions répétées ne
peuvent retirer une pile vide, dupliquer ni dépasser les capacités. Piles et ordre
sont sauvegardés. L'UI et ses sélections/transactions en attente ne le sont pas.

## Catalogue et admission

**B** ouvre six recettes. Sélectionner la fiche, **Prévisualiser / Entrée**, puis
flèches et J/L, **Entrée** pour confirmer ; X/B/Échap annule sans coût. Le paiement
vient **uniquement du porté**, après revalidation du terrain, des volumes et de la
recette. Retirer les objets d'un coffre avant de construire ; pas de ponction à
distance. La confirmation crée une seule identité et ferme la preview.

| Installation | Coût porté | Boîte solide (m, x/y/z) | Usage |
|---|---|---|---|
| Abri | 12 alliage | six pièces P4, emprise 6,3 × 6,3 | Recharge depuis la base s'il est desservi |
| Coffre | 6 alliage | 1,4 × 0,9 × 1 | 16 piles indépendantes |
| Panneau solaire | 8 alliage + 4 circuits | 2,8 × 0,65 × 2 | 160 W crête conventionnels |
| Batterie | 8 alliage + 4 circuits | 1 × 1,3 × 0,8 | +4 800 000 J, installée vide |
| Générateur O₂ | 12 alliage + 6 circuits | 1,4 × 1,6 × 1 | 40 W → 4 s d'autonomie/s |
| Réservoir O₂ | 6 alliage | 1 × 1,8 × 1 | +3 600 s de réserve, installé vide |

Plafonds : **32 abris et 64 équipements/coffres** cumulés. Chaque installation a
un BuildingId commun monotone et une pose DVec3/DQuat dans le corps de Mars.
Les coffres peuvent être éloignés ; les quatre équipements électriques/O₂
doivent se trouver à **40 m du premier abri**. Tous les abris à 40 m de ce premier
abri sont desservis. Cette base utilise **un bus collectif sans câbles** : stockage
énergétique et O₂ agrégés, sans réseau, pertes de transport ni connexion
transitive entre abris. Une batterie augmente la capacité sans créer d'énergie ;
un réservoir augmente la capacité sans créer d'oxygène. Pas de démantèlement.

Le carré conservateur de support garde les extrema des triangles fins et la
variation ≤0,4 m. Demi-largeur 4,5 m pour l'abri, demi-diagonale horizontale +0,1 m
pour un équipement. L'abri garde son origine au milieu des extrema ; les boîtiers
reposent sur le maximum pour éviter leur enfouissement. Admission SAT contre
joueur, abris, équipements, obstacles, caisses pleines et sources renouvelables
(même vides), séparation 0,2 m. Pas de terrassement ni contacts de collectible.

## Électricité et oxygène

Les deux réserves de base sont séparées des objets. **1 Wh = 3 600 J** ; une
batterie ajoute 1 333,33 Wh. L'O₂ est mesuré en **secondes d'autonomie P4** :
une seconde transférée retire une seconde de la base et en ajoute une à la
combinaison. Le service retire au plus 30 s/s et remplit au plus 180 s. Hors abri
desservi ou réserve vide : consommation 1 s/s, même dans un abri distant. Si la
réserve se vide au milieu d’une longue frame, le temps restant consomme de
nouveau la combinaison après la durée du transfert effectif.

Pour chaque panneau fixé au corps, la normale +Y de sa pose est comparée à la
direction corporelle du Soleil depuis ce panneau, avec le catalogue et la
rotation martienne existants. Puissance de jeu :

```text
P = 160 W × max(0, normale · direction_soleil)
          × exp(−0,65 × poussière_P8) × (1 − 0,25 × nuages_P8)
```

La poussière/tempête réduit donc la production ; la nuit donne zéro. Les lux et
couleurs de rendu ne servent pas à calculer l'électricité. Pas de loi solaire
inverse carrée, ombre de terrain/bâtiment, suivi mécanique, encrassement permanent
ou modèle d'ingénierie photovoltaïque. Les coefficients artistiques de P8 sont
réutilisés comme **entrée de gameplay**, pas comme opacité instrumentale.

Le générateur demande au plus 40 W par instance ; il consomme seulement l'énergie
nécessaire à la place libre en réserve. Production disponible et charge de
batterie servent cette demande ; le surplus charge la batterie, puis est perdu.
Une alimentation partielle produit proportionnellement ; batterie vide sans
soleil : arrêt. La reprise est automatique dès qu'une énergie est disponible.
La réserve pleine coupe la consommation du générateur. Les réserves restent
entre zéro et leurs capacités. Une batterie pleine peut alimenter un générateur
40 W pendant 120 000 secondes réelles, donc plus qu'une nuit martienne à ×1 ;
P/T accélère l'alternance astronomique, pas les débits de production.

## Horloges et propriétaires

Survie, production et renouvellement utilisent les **secondes réelles intégrales**
de Time<Real>, sans plafond 0,25 s. Le menu principal, l'espace et l'échec
suspendent les trois ; catalogue/inventaire/preview les laissent fonctionner.
Ces écrans libèrent la souris et bloquent la marche/visée et P/T ; la gravité et
le calendrier continuent. P/T commandé en jeu change les conditions solaires et
météo, sans multiplier les débits. Une longue interruption est intégrée une fois
avec **les conditions de fin de frame** ; pas d'intégration des levers/couchers
intermédiaires. Application fermée : **aucun rattrapage**. R et M conservent réserves,
contenus, poses et renouvellements ; seul le secours R remplit la combinaison.

`inventory.rs` : piles et transactions pures ; `Construction` : porté, caisses et
preview ; `Buildings` : identités/poses, coffres/contenus et types d'équipements ;
`Base` : sources, charge collective et O₂ collectif ; `Oxygen` : combinaison et
échec ; `ui/inventory.rs` : écran et commandes transitoires ; `persistence.rs` :
instantané validé avant remplacement. Navigation → production/survie → construction,
puis rendu/UI dans les GameSet existants. Aucun proxy n'écrit les états physiques.

Rendu : cube/13 textures P7 partagés, teintes et matériaux réutilisés ; un proxy
par équipement/source et six pièces maximum de fantôme, dans 120 m. Les premiers
boîtiers d'équipements sont simples ; l'UI donne leur nom, recette et usage.
Pas de nouveau mesh ni texture généré par frame, ni nouvelle dépendance. Cache
terrain ≤13 patches inchangé. Plafond 32×7+64+8+4+6 = **306 proxies** hors terrain,
ciel et obstacles. Ces bornes ne sont pas une mesure de VRAM totale.

## Sauvegardes et vérification

**SOLAR_SURVIVAL 3** : porté et ordre, coffres et capacités, équipements/identités,
poses, énergie J, O₂ s, lots et délais des quatre sources ; caisses, joueur,
calendrier et options existants. Fichiers ≤256 Kio. Types, quantités, capacités,
identités, charges, délais et poses sont validés avant publication. Formats 1/2
refusés explicitement, sans migration ni suppression. Secours et profils P6 conservés.

```bash
cargo fmt --check
cargo check --all-targets --locked
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
python3 scripts/validate_base.py --output docs/mon-p9 --mesa-immediate --focus-x11
```

Le scénario graphique utilise des poses de départ préparées et les vraies touches /
boutons ECS. Il ne télécommande pas les périphériques OS. Il conserve les refus et
assertions, deux processus et deux profils ; ne pas le confondre avec une partie
humaine sans repositionnement. Les scénarios P5 antérieurs restent des régressions
de l'abri et ne prouvent pas seuls P9. Résultats/limites effectivement mesurés dans
[VALIDATION](VALIDATION.md) et [PERFORMANCE](PERFORMANCE.md).
