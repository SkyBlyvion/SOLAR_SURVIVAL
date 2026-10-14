# P5 : récupérer, stocker, construire et utiliser

13 octobre 2026. La partie normale ajoute un inventaire de session et rend constructible **le même OxygenShelter P4**. Les bâtiments et la recharge restent ceux de [SURVIVAL](SURVIVAL.md) ; aucune zone de recharge spécifique au placement n'est créée.

## Jouer

Après création ou chargement d’une aventure et M, huit caisses dorées se trouvent à gauche et à droite du premier abri, à x=±8 m et z=0/−6/−12/−18 m dans le site d'arrivée. Ce sont des objets de récupération artistiques, sans donnée de minerai martien. Approcher une caisse à moins de 2,5 m puis presser **E** stocke ses six unités d'alliage. La caisse disparaît ; elle ne réapparaît pas lors de M, R, d'un menu ou d'un rebasage. Le HUD indique le stock et la distance/direction de la caisse restante la plus proche. Un mur ou obstacle coupant le segment yeux→centre de caisse interdit la récupération. Les caisses sont des collectibles sans collision de déplacement.

L'inventaire commence vide. Deux caisses financent un abri : **12 unités d'alliage**, recette de gameplay sans masse SI implicite. Les huit caisses donnent 48 unités, donc quatre constructions supplémentaires au maximum dans cette première boucle. Ressource finie, pas de renouvellement, réseau mondial de dépôts, autre recette ou démantèlement.

**B** ouvre le catalogue de construction en jeu : fiche de l’abri, coût, stock et disponibilité. Cliquez **Prévisualiser** ou pressez Entrée pour ouvrir la prévisualisation. L'abri suit la visée horizontale et les déplacements du joueur, initialement neuf mètres devant lui, porte vers lui. **Flèches haut/bas** règlent la distance (4–24 m), **gauche/droite** le décalage latéral (±16 m), à 4 m/s réel. **J/L** tournent le bâtiment à 1 rad/s réel. La souris continue de viser en pilotage ; les deltas souris ne sont pas multipliés par le temps. La porte et les six pièces sont visibles dans le fantôme. Le HUD et le fantôme affichent vert/autorisée ou rouge/interdite avec le motif.

**Entrée** confirme. Une confirmation interdite conserve stock et prévisualisation. **X**, **B** ou **Échap** annulent sans dépense ; dans le catalogue ils le ferment. Échap pendant le placement annule et libère le curseur avant le menu. Ouvrir le menu, quitter la surface ou épuiser l'oxygène annule aussi. Aucune confirmation par clic souris ; le clic droit garde sa fonction de capture. Le catalogue libère la souris et bloque le pilotage ; l’oxygène continue pendant le catalogue et le placement en jeu et se recharge dans les abris existants selon les règles P4.

Une fois construit, entrer par la porte recharge à +30 s/s réel. Marche, mur, toit et service utilisent les mêmes pièces et contacts P4. H conserve le HUD d'inventaire/placement et le HUD vital. Les retours spatiaux, menus et R gardent inventaire, dépôts épuisés et bâtiments pendant la session. **Sauvegarde P6** : le menu conserve maintenant cette session sur disque, avec reprise après fermeture ; sauvegarder explicitement avant Quitter. [Contrat](PERSISTENCE.md).

## Admission et transaction f64

`construction::Construction` possède le stock, les huit dépôts corporels DVec3/DQuat et les commandes de placement. `Placement` est une proposition corporelle f64 recalculée après la navigation et la survie ; elle ne devient un bâtiment qu'à confirmation. La projection horizontale de la visée donne la base et le décalage, l'orientation est `Q_carte × rotation_Y(yaw + angle)`.

L'admission prend les **extrema exacts des triangles** sous un carré conservateur de demi-largeur **4,5 m** dans la carte active, enveloppant l'abri de 6,3 m même tourné à 45°. Le fournisseur mondial réutilise l'index 8 m, déduplique par première case commune, découpe les triangles et certifie l'aire couverte à 10⁻⁹ m². La requête de construction accepte une demi-largeur ≤5 m ; la requête de contact joueur garde sa limite historique ≤1 m. Elle ne construit aucun patch. Le carré de placement entier doit rester dans le disque de couverture de 66 m ; un placement près du bord peut être refusé jusqu'au rebasage suivant, sans borner le déplacement mondial.

Le terrain artistique utilise ses mêmes triangles locaux, dans ±4 096 m ; les modes de régression cube/scénarios et le profilage restent sans construction jouable. Variation verticale maximale admise : **0,4 m** sur le carré entier. L'origine du bâtiment est au milieu des extrema, donc le terrain reste entre −0,2 et +0,2 m : murs enterrés jusqu'à −0,5 m, sol sans dalle, volume de recharge et hauteur utile préservés. Cette politique conservative peut refuser un carré dont seule une petite partie extérieure à l'abri est trop irrégulière ; elle ne prétend pas admettre les pentes arbitraires.

Une enveloppe OBB de **6,3 × 3,95 × 6,3 m**, centrée à y=1,475 m (bas −0,5 / haut 3,45), réserve l'ensemble de l'abri. SAT sur 15 axes en f64 vérifie l'intersection avec l'enveloppe complète de chaque bâtiment, les trois obstacles fixes, la boîte du joueur et les caisses non récupérées, avec **0,2 m** de séparation minimale sur les axes testés. Le bâtiment ne peut donc ni emprisonner le joueur ni partager le volume utile d'un autre abri. La réserve est une convention d'admission, pas la peau de contact de 2 mm. Le test est conservateur, pas une distance euclidienne exacte entre volumes.

À Entrée, `confirm` **recalcule** la candidate et tous les refus sur l'état actuel, vérifie le stock et le plafond de 32 abris, puis appelle `Buildings::install_shelter` et `publish_colliders`. Les 12 unités sont déduites après l'installation admise, une seule fois ; la preview est fermée. Identités monotones et poses corporelles deviennent autoritaires dans Buildings. Ni terrain, ni patch, ni proxy ne possède la nouvelle instance. Pas de nouvelle dépendance ni refonte de la simulation.

## Ordre, rendu et budgets

GameSet Input→Simulation→Navigation→Rendering→Interface inchangé. Navigation résout le mouvement/rebasage ; Oxygen applique le temps réel complet ; Construction lit ensuite cet état et peut installer. La publication des contacts a donc lieu après la résolution de cette frame, et ces contacts participent à la suivante. L'exclusion du volume joueur à confirmation empêche une pose déjà en intersection.

Le rendu P5 partage **le cube de l'abri**, avec trois matériaux supplémentaires de session : caisses et fantômes vert/rouge transparents. Six entités de preview sont réutilisées tant qu'elle reste active et supprimées à l'annulation/confirmation. Au plus huit proxies de caisse, chargés dans 120 m et supprimés après récupération/en espace/hors portée. Les entités conservées survivent au rebasage ; seuls leurs Transform changent. Différences corporelles position−yeux calculées en f64, rotation vers la carte, puis conversion f32 explicite. Aucun mesh créé par frame.

Plafond d'admission : 32 abris, donc ≤192 proxies d'abri, six de preview et huit de caisse (206 proxies P5/P4, hors terrain/obstacles). La quantité normale de ressource permet seulement cinq abris avec celui du départ, soit ≤44 proxies avec preview et caisses ; ce plafond conservateur additionne aussi des caisses qui seraient déjà récoltées. Au repos après construction, le nombre réel est plus petit. CPU : requêtes linéaires sur ces collections bornées, pas d'index spatial spéculatif. Les budgets terrain restent ≤13 patches, niveau spatial 15, avec résolutions intérieures 32/16/8 et bords 32. Ce sont des budgets structurels, sans mesure exhaustive RAM/VRAM ni benchmark.

## Vérifier et reproduire

Après compilation, sans autre scénario GPU/compilation concurrent :

```bash
cargo fmt --check
cargo check --all-targets --locked
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
python3 scripts/validate_construction.py --output docs/mon-nouveau-p5 --mesa-immediate
```

Dossier final neuf dans un parent existant ; Mesa immediate concerne seulement ce poste NVK. Le diagnostic utilise `SOLAR_CONSTRUCTION_VALIDATE_DIR`, exige un scénario mondial isolé et conserve seize captures avec télémétrie. Il récolte deux caisses par les touches E après marche réelle, teste refus/annulation, déplace et tourne le fantôme, confirme par Entrée, entre et recharge, pousse contre un mur, court jusqu'à un rebasage, quitte/revient via M et vérifie le menu.

Les départs devant la porte, de course et devant l'abri après retour sont des **poses de diagnostic préparées**, les réserves de 60 s raccourcissent les essais de recharge ; aucun alliage injecté ni installation directe dans ce parcours. Les touches sont injectées dans l'ECS, pas dans les périphériques OS. Tests numériques/ECS et résultats graphiques effectivement observés dans [VALIDATION](VALIDATION.md). Windows 11/RTX 4050/Core i5 restent à valider nativement.

## Terrain adaptatif

Le parcours mondial utilise désormais des résolutions 32/16/8 avec bords communs fins. La zone de déplacement/admission garde les triangles 32 et leurs extrema exacts ; le nouvel index local couvre toutes les requêtes admises. Bâtiments, colliders, services, dépôts et inventaire restent fixes dans leurs propriétaires corporels ; aucun changement de résolution ne les réinstalle ni ne les recentre. Les régressions numériques et le parcours graphique P5 sont vérifiés sur ce fournisseur. [Architecture et limites](GLOBAL_TERRAIN.md), [résultats](VALIDATION.md).


## Caméra et ciel uniques

13 octobre 2026 : les proxies locaux sont désormais rendus par la même GameCamera que les visites spatiales, sur la couche 1. [Ciel et masquage](SURFACE_SKY.md). Les poses corporelles, contacts, horloges, budgets terrain et services de jeu décrits ici restent autoritaires ; aucun objet de ciel ne fournit du support ou ne réinstalle un bâtiment/dépôt. Les nouvelles preuves sont distinctes des campagnes historiques.

## Reprise P6

P6 restaure le stock, l’épuisement des dépôts et les identités/poses corporelles des mêmes abris. La preview est annulée ; aucune transaction de placement ni nouveau débit ne se produit au chargement. Les contacts/proxies sont reconstruits depuis les objets restaurés ; les vieux proxies de dépôts sont libérés même au chargement d’une session antérieure à l’arrivée sur Mars. [PERSISTENCE](PERSISTENCE.md).

## Profils et interface P6

Chaque aventure nommée possède ses propres caisses, stock et abris. Une nouvelle aventure ne reprend aucun objet de la précédente. Le catalogue B et les instructions de placement utilisent la palette commune du HUD ; l’état de disponibilité ne remplace pas l’admission finale et aucun coût n’est dépensé pendant la consultation/prévisualisation. [PERSISTENCE](PERSISTENCE.md).
