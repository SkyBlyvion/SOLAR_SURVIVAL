# P4 : oxygène et abri de session

13 octobre 2026. La partie normale offre une première boucle de survie sur Mars. Après Jouer puis M, l'abri turquoise est à six mètres devant le point d'arrivée ; sa porte ouverte est du côté du joueur. Marcher à l'intérieur recharge automatiquement. Le HUD vital montre la réserve, l'abri en service ou la distance/direction du plus proche. H masque la télémétrie, mais conserve l'oxygène et l'issue de l'expédition visibles.

## Conventions de gameplay

| Paramètre | Convention |
|---|---|
| Capacité de la combinaison | 180 secondes de réserve, pleine au démarrage |
| Consommation à l'extérieur | 1 seconde de réserve par seconde réelle, constante même en course |
| Recharge dans un abri | +30 secondes de réserve par seconde réelle, limitée à 180 ; pas de consommation simultanée |
| Alerte | À 30 secondes ou moins |
| Épuisement | Expédition terminée, pilotage bloqué et curseur libre ; état verrouillé jusqu'à R |
| R après épuisement | Nouvelle expédition au centre du premier abri, réserve pleine, sans supprimer les bâtiments |
| Menu/options/crédits | Réserve et recharge suspendues ; calendrier et gravité continuent selon les règles existantes |
| Visites spatiales M/G/O/U | Réserve et échec conservés, consommation/recharge suspendues ; aucune recharge implicite |
| Retour M | Même site de session, près du premier abri ; peut être nocturne après une longue visite |

La réserve est exprimée en secondes d'autonomie, pas en masse ou pression d'oxygène. Tous ces paramètres sont des choix de jeu, sans prétention physiologique. Pas de modèle de sas/pression/température/santé. Le bâtiment ouvert constitue un service de recharge de combinaison ; il ne simule pas une pièce hermétique. Aucun nouveau paramètre astronomique.

Le système utilise **Time<Real> après Navigation**, sans multiplicateur orbital P/T, Time<Virtual> ni plafond de 0,25 s. Une interruption de 12 s consomme donc 12 s en surface hors menu, tandis que la physique locale garde son plafond habituel. Le contexte de recharge est celui des pieds résolus à la fin de la frame ; les transitions menu/espace s'appliquent à cette frame. Aucun rattrapage des passages intermédiaires pendant une interruption n'est revendiqué.

## Bâtiment réutilisable P4 → P5

`buildings::Buildings` conserve des objets `OxygenShelter` avec `BuildingId` monotone, stable et unique en session, position `DVec3` en mètres relative au centre de Mars et orientation `DQuat` dans les axes du corps tournant. Ce sont les données autoritaires, indépendantes des clés de patch, du repère local mobile, du JD et des entités de rendu. `install_shelter(position_m, orientation)` instancie le même type à une pose choisie. `install_first` choisit seulement le placement initial et le site de retour. Une deuxième instance passe par la même installation, les mêmes contacts et la même recharge ; pas de logique particulière dans le contrôleur.

L'abri mesure environ 6,3 × 3,8 × 6,3 m avec une porte de 2,4 m côté +Z. Deux murs latéraux, un fond, deux jambages et un toit composent six boîtes orientées. Les murs descendent de 0,5 m sous l'origine de pose ; pas de dalle/rampes, le terrain fait office de sol. La liste de pièces est commune aux collisions et aux proxies. Le volume utile est x/z ±2,7 m, y −0,5 à 3,15 m dans le repère du bâtiment ; les **huit coins du joueur 0,6 × 1,8 × 0,6 m** doivent tous y tenir. Une proximité au centre, une position derrière un mur ou au-dessus du toit ne suffisent pas.

Lors d'une installation, `publish_colliders` publie six `BodyBox` fixes par abri dans `MarsTerrain`. Cette copie de collision est mise à jour seulement lorsque des bâtiments sont installés. Chaque requête convertit la pose fixe en carte active en f64 ; balayage SAT continu sur 15 axes, glissement et supports triangulaires utilisent le moteur existant. Les objets éloignés sont exclus par une borne conservative utilisant leurs dimensions. Le rebasage ne réinstalle ni ne déplace un bâtiment. P5 devra appeler la même installation et publication après un placement admis, plutôt que réécrire l'abri. Les règles de terrain constructible, inventaire, recettes, interface de placement et sauvegarde restent à développer.

`survival::Oxygen` contient seulement la réserve, l'issue et l'identité de l'abri courant ; il interroge le service `shelter_at` depuis la pose corporelle des pieds. La consommation ignore le code de placement. Les commandes R et M utilisent le site de retour choisi à la première entrée, conservé pendant la session ; R est inactif avant un échec. Le plugin vital est ordonné après la navigation, avant Rendering/Interface ; l'ordre global Input → Simulation → Navigation → Rendering → Interface demeure inchangé.

## Rendu et coûts

`rendering/buildings.rs` charge uniquement les proxies des bâtiments dont le centre est à moins de 120 m des pieds. Six entités par abri local, un seul mesh de cube partagé et deux matériaux de session. Les parties communes gardent leurs entités/handles lors d'un rebasage ; les proxies sortants sont supprimés et réinstanciés au retour avec la même identité de bâtiment. Les données et boîtes corporelles restent en CPU. En P4 : un bâtiment normal, deux dans le diagnostic ; pas de génération de mesh par frame, pas de génération de planète. La complexité CPU des requêtes est linéaire en nombre d'abris installés ; un index spatial sera à évaluer selon les usages P5, sans en créer un maintenant.

Le rendu soustrait **position du bâtiment − yeux dans le corps en f64**, ajoute le centre local de pièce tourné en f64, puis applique l'inverse du repère local avant conversion explicite f32. L'orientation est convertie depuis `Q_carte⁻¹ Q_bâtiment`. Aucun Transform n'est autoritaire et aucun proxy ne modifie la simulation. Les budgets de terrain P3 restent inchangés ; les nouveaux coûts des bâtiments/HUD ne font pas l'objet d'un benchmark complet ni d'une mesure RAM/VRAM exhaustive.

Les modes cube historiques, smoke/P2 et le profilage désactivent bâtiments et survie pour conserver leurs scènes de régression. La partie normale historique `SOLAR_MARS_GLOBAL=0` conserve un abri fonctionnel. Les anciens types préparatoires Vitals/Inventory, inutilisés, sont remplacés par la fonctionnalité effective ; aucun inventaire n'est implémenté.

## Reproduire la validation ciblée

```bash
cargo test --locked
cargo build --release --locked
python3 scripts/validate_survival.py --output docs/mon-nouveau-p4 --mesa-immediate
```

Le dossier final doit être nouveau dans un parent existant. Le script lance uniquement le binaire déjà construit ; ne pas compiler ni lancer un autre jeu pendant ce parcours. Sans Python, `SOLAR_SURVIVAL_VALIDATE_DIR=<nouveau dossier> target/release/solar_survival` exécute le même diagnostic. La variable Mesa est le contournement local NVK, à omettre sur un autre pilote/Windows.

Parcours de 56 s : consommation dehors, marche par la porte, recharge, sortie, menu avec avance orbitale, course aller/retour avec rebasage, épuisement, R, sortie/retour spatial, installation de deuxième instance, marche et recharge. Treize captures, assertions et télémétrie sont conservées. Le diagnostic initialise une réserve de 80 s, puis 1 s pour l'échec et 60 s devant le second abri ; il repositionne ce dernier départ. Ces préparations raccourcissent la campagne sans changer les taux ni l'issue de production. Ce ne sont ni une construction joueur ni une attente continue de trois minutes. Les autres sorties, murs, 30/60/144 FPS, delta de 12 s et conservation de poses sont testés numériquement/ECS. Voir [VALIDATION](VALIDATION.md) pour résultats et limites réels.
