# Cible P7 et contrainte de performance — 14 octobre 2026

La RTX 4050 **6 Go** est le haut de la cible utilisateur, pas une configuration minimale exigeante. CPU annoncé « i5 12700K », modèle exact à confirmer (12700K correspond officiellement à un i7) ; RAM/puissance GPU/pilote à relever. Objectif de travail **1080p/60 FPS sur la cible**, pas une mesure acquise. Le jeu doit conserver une qualité réduite viable sur des configurations plus faibles ; seuil minimal à établir avec des essais réels.

P7 part de textures locales 1 024², petits objets 512², cartes Terre/Mars 2 048×1 024 ; exception proche limitée à 2 048² et justifiée. **Budget initial de nouvelles textures résidentes : 128 Mio**, formats GPU/niveaux de texture compris ; copies CPU et pics transitoires suivis séparément. Ce budget reste à instrumenter lors de l'implémentation. [Contrat détaillé](ART_DIRECTION.md). Le poids compressé sur disque n'est pas une mesure de VRAM. Réutiliser les assets et limiter aussi le coût des shaders, des mélanges et de l'éclairage.

La mise à jour est documentaire : pas de texture intégrée, nouvelle mesure, build ou validation sur RTX 4050/Windows. Les baselines datées ci-dessous restent celles du poste Ubuntu/GTX 1060. Avant/après P7 doit garder résolution/MSAA/site/visée comparables, sans captures pendant les fenêtres ; relever temps moyen/p95, CPU/GPU, chargement et ressources/pics. L'amélioration visuelle doit respecter ces contraintes, avec réduction/correction en cas de dépassement.

---

# P6 : profils et interface — portée des vérifications

14 octobre 2026. Les profils conservent seulement des noms en mémoire lors de la navigation ; seule l’aventure active possède terrain, objets et ressources de jeu chargés. La liste de profils est paginée selon la hauteur de fenêtre. Les panneaux de construction partagent le cube et les six pièces déjà utilisés pour le placement, sans nouveau mesh par frame. Les dossiers d’aventures n’ajoutent pas de planètes/terrains résidents.

Les diagnostics P6 avec captures vérifient les fonctionnalités et la lisibilité, **pas des performances**. Les réglages 75°/MSAA off préparés pendant le scénario distinguent les images des baselines précédentes ; aucun gain FPS n’est tiré de leur HUD. Windows/cible et RAM/VRAM exhaustive restent non validés. [Contrat](PERSISTENCE.md), [résultats](VALIDATION.md).

---

# Performance actuelle : caméra unique et ciel — 13 octobre 2026

Deux versions release comparées sur le même poste Ubuntu/i7-6700K/GTX 1060 3 Go/NVK/Mesa immediate, **1440×900, MSAA ×4**, terrain mondial adaptatif, même calendrier/site/parcours et fenêtres : deux essais sans recorder et un avec avant changement, deux sans recorder/un avec/un diagnostic no_sky après. Tous les sept essais terminent code 0 sans ERROR/WARN/panic. Pas de compilation, autre jeu GPU ou capture pendant les fenêtres. Concordances vraies [avant](sky-validation/before/concordance.json) et [après](sky-validation/after/concordance.json). Les hashes après incluent aussi les shaders WGSL. [Données détaillées et transactions](sky-validation/comparison.json), [contexte matériel](sky-validation/host.json).

## Résultat de la refonte complète

| Frame moyenne / deux répétitions sans recorder | Deux caméras avant | Caméra unique avec ciel complet |
|---|---|---|
| Repos | 51,038 / 50,983 ms · 19,59 / 19,61 FPS | **29,313 / 29,096 ms · 34,11 / 34,37 FPS** |
| Contact | 53,786 / 53,847 ms | 32,004 / 32,005 ms |
| Course avec vrai streaming | 50,735 / 50,690 ms · 19,71 / 19,73 FPS | **28,875 / 28,965 ms · 34,63 / 34,52 FPS** |
| p95 repos | 52,996 / 53,771 ms | 32,357 / 32,257 ms |
| p95 course | 52,510 / 52,381 ms | 32,121 / 32,137 ms |
| Contrôle en espace | 27,794 / 27,290 ms | 28,299 / 28,213 ms |

La refonte complète réduit ici le temps moyen de frame au repos d'environ **43 %**, soit environ **34 FPS contre 20**. Les sources diffèrent entre versions et les séries ne sont pas immédiatement alternées : pas d'attribution de tout ce gain à une seule ligne. Le contrôle spatial reste proche (~0,7 ms d'écart moyen) ; deux répétitions ne caractérisent pas toute la dérive du poste. Aucun gain supplémentaire de terrain ou performance universelle revendiqué.

Le contenu céleste est conservé sous une représentation différente en surface : panorama d'étoiles discret la nuit, disque solaire et points/disques issus des angles physiques, ciel artistique poussiéreux. Les visites spatiales conservent les sphères/anneaux et bloom. **Différences de présentation à relever** : étoiles de surface nouvelles/irrégulières ; atmosphère artistique et atténuation diurne ; sol mat (reflectance 0) ; brouillard 40–60 m au lieu de 70–125 m ; bloom du terrain absent comme sur l'ancienne caméra locale, halo solaire dans le shader. Le terrain reste rendu aux mêmes résolutions et budgets, mais sa visibilité lointaine est réduite. Comparaison de deux fonctionnalités complètes, pas de deux images strictement identiques.

## Coût du ciel et spans GPU

L'essai final avec recorder donne **29,867 ms au repos**, 29,504 ms en course ; l'ancien recorder donnait 51,552/51,628 ms. Sur une seule comparaison instrumentée : `main_opaque_pass_3d` au repos passe de **3,375 à 7,799 ms**, `tonemapping` 2,047→2,085 ms, `upscaling` 1,078→1,483 ms. Le ciel est dans la passe opaque et son traitement a un coût ; l'ancien chemin opaque était partagé entre les deux vues et n'en conservait pas forcément toutes les occurrences.

Le span `msaa_writeback` (**9,629 ms** avant) n'est plus observé dans les phases de surface finales ; bloom (**2,175 ms** avant) n'y est plus installé. Bloom reste observé lors du contrôle spatial. L'absence d'un span ne mesure pas un coût nul du GPU entier. Les spans arrivent de façon asynchrone, peuvent être imbriqués et **ne s'additionnent pas** en un total GPU fiable. L'inspection ECS/renderer vérifie indépendamment une caméra principale, sans caméra de ciel hors écran.

Le diagnostic **no_sky** donne 24,041 ms au repos et 23,934 ms en course. Écart avec le ciel complet ~5,2/~5,0 ms de frame sur ces essais successifs ; il retire tout le fond et n'est pas un réglage de production. Ce n'est pas une mesure GPU isolée du shader ou une garantie de son coût sur une autre visée/pilote. Les 32 Mio du panorama et son matériau restent résidents, seuls les pixels du fond sont retirés.

## Streaming et CPU

Chaque course mesurée renouvelle **cinq clés**, constructions 18→23, treize patches, 16 744 triangles au terme de la transaction. Les remplacements réellement dans les fenêtres coûtent **0,975/0,984 ms avant**, **0,924/1,052 ms après**. Ce sont des coûts similaires, sans optimisation LOD nouvelle. Les maxima de navigation sont 1,102/1,121→1,030/1,165 ms, synchronisation du rendu 0,441/0,154→0,131/0,143 ms. Le scénario canonique ne change pas le détail d'une clé commune pendant cette fenêtre ; les tests LOD historiques conservent ces autres cas.

Au repos, la synchronisation rendering est ~0,037 ms avant et ~0,040 ms après ; simulation ~0,008 ms. La navigation par frame baisse ~0,114→0,070 ms en moyenne avec des frames plus courtes et moins de sous-pas à effectuer par frame : **pas une nouvelle accélération du contrôleur**. Les mesures entre GameSet n'incluent pas toute l'extraction/préparation/pilote ni les uploads GPU effectifs. Les données et plafonds géométriques restent ceux de [GLOBAL_TERRAIN](GLOBAL_TERRAIN.md).

## Reproduire et variantes actuelles

Après `cargo build --release --locked`, sans compilation concurrente :

```bash
python3 scripts/profile_planetary.py --scene global --lod adaptive --repeats 2 --gpu-probe --camera-probe --focus-x11 --mesa-immediate --output docs/mon-nouveau-profil
```

Dossier final nouveau, parent existant ; omettre X11/Mesa sur les autres postes. Les quatre essais durent chacun 54 s, échauffement 10 s, phases 11 s, première seconde de chaque phase exclue. Les anciennes preuves baseline/no_space_camera/no_surface_camera conservent leur signification d'origine.

| Variante actuelle | Effet |
|---|---|
| baseline | Une caméra, ciel de surface complet, 1440×900/MSAA ×4 |
| no_sky | Cache le fond local ; caméra, UI, physique et streaming conservés |
| no_game_camera | Désactive la vue principale/UI uniquement en surface ; navigation conservée, vue restaurée en espace |
| half_resolution | 720×450 |
| msaa_off | MSAA off sur la caméra unique |
| surface_unlit | Matériaux StandardMaterial locaux unlit, même ciel/meshes |

Les variantes sont des diagnostics, sans modification automatique de qualité en partie normale. La variante no_sky est appliquée dans Update après Rendering, avant propagation de visibilité. Les anciens profils P5/historiques et toutes leurs preuves demeurent disponibles. Windows 11 natif, RTX 4050/Core i5, autres DPI, RAM/VRAM exhaustive et 60 FPS cible **non validés**.

---

# Historique : performance LOD et streaming

Phase demandée après P5, avant P6. Binaire et sources finaux inchangés pendant les mesures, Ubuntu/i7-6700K/GTX 1060 3 Go/NVK/Mesa immediate, **1440×900, MSAA ×4 et deux caméras**. Aucune compilation ni autre lancement GPU concurrent. Les réglages de qualité normaux ne changent pas. [Protocole et empreintes](lod-validation/), [synthèse des CSV et transactions](lod-validation/summary.json), [limites et essais intermédiaires](lod-validation/NOTES.md).

## Baselines avant changement

Quatre essais monde de 54 s sur le binaire P5 original (deux répétitions, recorder GPU, sans caméra spatiale), puis trois essais P5 sur un binaire contenant seulement l’instrumentation supplémentaire (deux répétitions et recorder GPU). Aucun LOD ni changement d’index encore appliqué. [Baseline monde](lod-validation/baseline-global/), [baseline P5](lod-validation/baseline-p5/). La première série mondiale dérive fortement : repos 64,566/62,353 ms sans recorder, puis 50,688 ms avec ; cela interdit d’en déduire un surcoût négatif du recorder ou un gain ultérieur de terrain.

Le profil mondial garde le parcours historique de mesure : 10 s d’échauffement, quatre phases de 11 s, première seconde exclue. La course inclut un vrai remplacement après environ huit secondes, à la différence d’un simple rebasage préparatoire. Le profiler garde le terrain artistique historique comme défaut ; **SOLAR_PROFILE_TERRAIN=global** sélectionne explicitement le monde.

Le nouveau profil P5 conserve la production (collecte, preview/rotation, confirmation, porte/recharge, mur, course/rebasage, espace, menu) et ses préparations de poses/réserves, sans captures. Les premières 0,25 s de chaque phase sont exclues ; certaines fenêtres sont courtes. Les valeurs de confirmation/construction ne chronomètrent donc pas séparément l’installation déclenchée à la première frame. Elles caractérisent la scène et ses intervalles ECS ; le streaming au milieu de la course conserve la transaction mesurée. Pas d’injection d’alliage ni d’installation directe.

## Comparaison contrôlée fixe/adaptatif finale

**Même binaire final**, deux lancements fixes puis deux adaptatifs de 54 s, successifs. Le mode fixe garde les treize patches 32 et l’index complet ; le mode adaptatif choisit 32/16/8 et indexe seulement le carré local utile. [Fixe](lod-validation/final-fixed/), [adaptatif](lod-validation/final-adaptive/). Résolution, matériaux, caméras, calendrier, trajectoires et exclusions sont identiques.

| Mesure | Fixe 1 / 2 | Adaptatif 1 / 2 |
|---|---|---|
| Frame moyenne repos | 51,619 / 51,600 ms | 51,746 / 51,724 ms |
| Frame moyenne course/streaming | 51,493 / 51,417 ms | 51,386 / 51,561 ms |
| p95 frame course | 54,606 / 54,079 ms | 53,836 / 54,303 ms |
| Maximum frame course | 56,231 / 56,781 ms | 56,367 / 59,398 ms |
| Transaction CPU du remplacement **dans la fenêtre mesurée** | 5,676 / 6,533 ms | 1,140 / 0,979 ms |
| Géométrie du même remplacement | 0,134 / 0,144 ms | 0,308 / 0,276 ms |
| Index du même remplacement | 5,263 / 6,079 ms | 0,770 / 0,659 ms |
| Maximum navigation dans la fenêtre | 5,791 / 6,669 ms | 1,282 / 1,098 ms |
| Maximum synchronisation du rendu dans la fenêtre | 0,209 / 0,485 ms | 0,148 / 0,150 ms |

Les médianes de ces **deux remplacements** sont 6,105 ms fixe et 1,060 ms adaptatif (environ 83 % de réduction CPU sur ces cas). L’index local explique l’essentiel ; générer les raccords grossiers coûte un peu plus en géométrie. Le temps de frame en course demeure proche de **51,5 ms, ~19,4 FPS** : **aucun gain FPS notable établi**. Le maximum adaptatif 59,398 ms reste conservé et n’est pas masqué par le plafonnement physique.

Les compteurs passent de 26 624 triangles fixes à 16 328 à l’entrée adaptative (6/5/2), puis 18 512 après préparation et 16 744 après remplacement (6/6/1). Ce profil renouvelle cinq clés ; il ne change pas la résolution d’une clé déjà commune dans cette fenêtre. La campagne graphique mondiale release complète ce cas par **deux changements de résolution résidente par rebasage**, douze fois, avec mesures CPU géométrie/index/mesh conservées et handles réutilisés. Les captures de ce parcours restent distinctes du benchmark sans captures.

## GPU et composition des caméras

Un essai adaptatif final de 54 s avec recorder donne 52,194 ms au repos et 52,012 ms en course. Moyennes GPU disponibles au repos : main_opaque_pass_3d **3,414 ms**, msaa_writeback **9,634 ms**, tonemapping **2,050 ms**. La baseline mondiale originale donnait 3,371/9,585/2,052 ms. Une seule mesure instrumentée par version, une dérive temporelle et les chemins partagés entre vues ne permettent pas d’attribuer un gain GPU fiable au nombre de triangles.

L’essai final sans caméra spatiale donne **32,220 ms au repos (~31,0 FPS)** et 32,174 ms en course, contre ~51,7/51,5 ms à deux caméras. Cela confirme l’intérêt d’examiner la composition des vues/MSAA et le pilote. Il retire du contenu et **n’est pas livré comme réglage de production automatique**. Les spans sont asynchrones et parfois imbriqués : ne pas les additionner pour fabriquer un temps GPU total, ni les attribuer à une caméra ou à un upload particulier.

Les temps `GLOBAL meshes sync CPU` mesurent la préparation du mesh et sa remise aux assets Bevy, **pas l’upload effectif GPU**. Géométrie et index ont leurs propres durées. Les attentes/extraction/pilote hors des GameSet demeurent hors de l’instrumentation murale du gameplay. Les coûts de frame restent dominés par le chemin de rendu dans ces conditions ; leur attribution exhaustive demande un profilage du moteur/pilote sur la cible.

## P5 avec la scène construite

Trois essais adaptatifs finaux de 59 s, deux sans recorder et un avec, terminent code 0 sans ERROR/WARN/panic, collecte/placement/abri/recharge/rebasage/retours/menu conservés. [Mesures P5 finales](lod-validation/final-p5/), empreintes inchangées. Les baselines P5 précèdent l’optimisation et la reprise de session ; les écarts ci-dessous ne sont pas une comparaison A/B immédiatement alternée sur un même binaire.

| Frame moyenne, sans recorder | P5 fixe initial 1 / 2 | P5 adaptatif final 1 / 2 |
|---|---|---|
| Placement actif | 55,925 / 55,570 ms | 56,808 / 56,614 ms |
| Repos après construction | 50,711 / 50,098 ms | 51,475 / 51,134 ms |
| Contact mur | 50,917 / 50,884 ms | 51,710 / 51,747 ms |
| Course/streaming | 45,080 / 44,983 ms | 45,945 / 46,162 ms |

Un écart ~1 ms apparaît sur ces scènes, mais la dérive observée du poste, la séparation temporelle et l’activation explicite du client X11 ne permettent pas de l’attribuer au terrain. La comparaison contrôlée mondiale du même binaire n’établit pas de régression de frame. Les réserves et les touches utilisent les mêmes règles ; les fenêtres courtes et les préparations P5 limitent les conclusions sur la transaction de construction elle-même. Le remplacement P5 final coûte 1,066/1,255 ms CPU sans recorder, contre 5,498/6,057 ms dans les deux baselines ; données complètes et phases dans les CSV.

L’essai P5 avec recorder donne 57,176 ms pendant le placement et 46,521 ms en course. Spans GPU asynchrones conservés dans le CSV ; ce n’est ni une mesure GPU totale ni une garantie sur la cible. Aucun gain de performance n’est tiré des FPS des captures.

## Budgets et reproduction

Trois résolutions effectives : **1 089/413/205 sommets** et **2 048/696/280 triangles** par patch. Bords 32 communs ; ≤13 meshes résidents, même handle à changement de détail, slots recyclés et assets libérés en espace. Plafonds conservateurs tous fins : 14 157 sommets/26 624 triangles, ≤26 occurrences CPU pendant transaction avec Arc partagés. L’index local observé sur 150 directions descend à **361 cases, 10 613 références, 5 375 triangles**. Comptes de structures et budgets de travail, sans RAM/VRAM exhaustive ni comptage des buffers internes de Bevy. [Architecture](GLOBAL_TERRAIN.md).

```bash
cargo build --release --locked
python3 scripts/profile_planetary.py --scene global --lod fixed --repeats 2 --mesa-immediate --output docs/mon-fixe
python3 scripts/profile_planetary.py --scene global --lod adaptive --repeats 2 --gpu-probe --camera-probe --mesa-immediate --output docs/mon-adaptatif
python3 scripts/profile_planetary.py --scene p5 --lod adaptive --repeats 2 --gpu-probe --mesa-immediate --output docs/mon-p5
```

Choisir des dossiers nouveaux dans un parent existant. `--focus-x11` peut activer la fenêtre cliente après deux secondes sur le poste X11, sans simuler les périphériques ; ne pas interagir avec la fenêtre pendant le parcours. Omettre cette option et Mesa sur Windows/autres pilotes. La logique centrale n’utilise aucune API X11 ; le helper de diagnostic utilise seulement ctypes/libX11 si demandé. Sans Python, le profil P5 utilise SOLAR_CONSTRUCTION_VALIDATE_DIR, SOLAR_P5_TIMING_OUTPUT et SOLAR_P5_TIMING_GPU=1 pour les timestamps ; les captures sont alors retirées, les règles P5 conservées.

Le LOD fournit des résolutions adaptatives à emprises fixes et transitions lointaines sous 0,1 mm pour une sphère sans relief. Pas de quadtree, relief, horizon complet, sauvegarde ou streaming asynchrone. Windows natif et RTX 4050/Core i5 restent à mesurer ; aucun 60 FPS cible revendiqué. Les options de qualité existantes restent au choix du joueur.

---

# Historique : performance Solar Survival

Le profilage compare des coûts dans une scène répétable. Le poste de développement Ubuntu, i7-6700K et GTX 1060 3 Go/NVK, sert à identifier ces coûts ; la cible annoncée est RTX 4050 et Intel Core i5. Les FPS du poste actuel ne prédisent pas ceux de cette cible. Le modèle exact du i5, la RAM, les réglages et la résolution seront relevés lors de sa validation. L'objectif de 60 FPS sur la cible ne bloque pas les fonctionnalités sur le poste de développement.

## Protocole de mesure

Compiler les sources avec `cargo build --release --locked`, puis fermer les autres lancements du jeu. Ne pas compiler ni lancer un autre essai GPU pendant les mesures. Ne pas interagir avec la fenêtre du jeu pendant la campagne ; les mouvements souris et les actions de navigation restent des entrées normales du moteur. Utiliser la même machine, le même pilote, la même présentation et les mêmes réglages pour comparer deux variantes. Le profiler force J2000, pause orbitale et un site d'entrée canonique sur Mars ; il injecte des commandes ECS, sans agir sur les périphériques de l'OS.

Le lancement standard de 42 secondes comprend 10 secondes d'échauffement, puis quatre phases de 8 secondes : repos, marche vers le premier obstacle, course à 8 m/s depuis x = 116 m vers la tuile suivante, puis retour à la visite spatiale de Mars. La première seconde de chaque phase est exclue pour laisser se stabiliser transitions, pipelines et diagnostics asynchrones. Le joueur franchit la limite x = 128 m pendant la fenêtre mesurée. Le calendrier est figé pour comparer exactement la lumière et la pose astronomique. Il n'y a pas de capture d'image pendant les fenêtres de mesure.

Les temps CPU `cpu/simulation`, `cpu/navigation`, `cpu/rendering_sync` et `cpu/interface` mesurent les intervalles entre les GameSet de l'Update, avec leur ordonnancement. Navigation inclut le contrôleur physique ; rendering_sync inclut le streaming et la mise à jour des proxies. Ces temps n'incluent pas tout le moteur : extraction, PostUpdate, préparation et attentes du renderer sont distincts. Ce sont des durées murales, pas des compteurs de cycles CPU.

`frame` est le delta `Time<Real>` en millisecondes : il inclut le rythme du moteur, les attentes GPU et la présentation. Les diagnostics `render/.../elapsed_cpu` et `elapsed_gpu` viennent du recorder de Bevy 0.19.1, avec requêtes de timestamps GPU lorsque le backend les permet. Ils arrivent de manière asynchrone. Le profiler extrait la dernière mesure disponible par chemin et la dédoublonne par son timestamp ; le nombre d'échantillons peut différer de celui des frames. Certains chemins Bevy sont partagés entre plusieurs vues : cette extraction ne conserve pas chaque occurrence dans un même lot et ne fournit pas une attribution par caméra. Des intervalles GPU peuvent être imbriqués : **ne pas additionner tous les spans** pour inventer un temps GPU total. Un diagnostic absent est indisponible, pas nul.

Le CSV brut conserve chaque échantillon et sa phase. `summary.csv` donne effectif, moyenne, médiane, p95, p99 et maximum. Les percentiles utilisent une interpolation linéaire sur les échantillons triés. Les FPS sont `1000 / moyenne(frame_ms)`, et non une moyenne des FPS du HUD. Le maximum et les longues frames restent dans les statistiques ; la limite physique de 0,25 s ne raccourcit pas le temps de frame mesuré.

## Variantes disponibles

| Valeur de SOLAR_PROFILE_VARIANT | Différence avec baseline |
|---|---|
| baseline | Deux caméras en surface, 1440 × 900, MSAA ×4, matériaux actuels |
| no_space_camera | Désactiver uniquement la caméra astronomique en surface ; physique et streaming continuent |
| no_surface_camera | Désactiver la caméra locale en surface ; physique et streaming continuent ; l'UI ciblant cette caméra n'est également plus rendue |
| half_resolution | Fenêtre 720 × 450, soit quatre fois moins de pixels configurés |
| msaa_off | MSAA désactivé sur les deux caméras |
| surface_unlit | Terrain et obstacles avec matériaux unlit, mêmes meshes et caméras |

Ces variantes sont des expériences qui peuvent retirer une partie de l'image. Elles ne sont pas des réglages graphiques livrés au joueur, ni des optimisations adoptées. La phase spatiale sert aussi de contrôle : les caméras désactivées uniquement en surface sont réactivées à la sortie ; half_resolution et msaa_off s'appliquent encore en espace.

`SOLAR_PROFILE_SECONDS` active le mode (30 à 300 s). `SOLAR_PROFILE_GPU=0` supprime le recorder GPU pour mesurer le rythme sans son coût ; le défaut l'active. `SOLAR_PROFILE_OUTPUT` choisit le CSV ; défaut `docs/profile-<variante>.csv`. Ce fichier est remplacé par un essai individuel. Le script de campagne protège les dossiers déjà utilisés. Le profilage est prioritaire sur les scénarios smoke et les variantes sont absentes de la partie normale.

## Reproduire une campagne

Sur le poste Mesa/NVK actuel :

```bash
cargo build --release --locked
python3 scripts/profile_suite.py --seconds 42 --mesa-immediate --gpu-probe --output docs/profiles/mon-essai
```

Le script utilise le binaire déjà compilé, lance les six variantes sans recorder GPU, répète baseline en fin de suite, puis mesure baseline avec recorder GPU. Il écrit CSV bruts, journaux, synthèse et contexte avec empreintes SHA-256 des sources et du binaire. `--repeats 2` répète toutes les variantes pour une campagne plus longue. Il n'y a aucune compilation dans les fenêtres de mesure. Choisir un dossier de sortie vide ; pour recalculer seulement la synthèse, utiliser `--summarize-only --output <dossier>`.

Sur un autre pilote, omettre `--mesa-immediate`. Cette option sert au contournement local NVK/VSync observé dans [VALIDATION](VALIDATION.md) et peut introduire du tearing. Le script conserve les autres variables de présentation de l'environnement : relever leur présence pour chaque campagne.

Sans Python, Ubuntu :

```bash
MESA_VK_WSI_PRESENT_MODE=immediate SOLAR_PROFILE_SECONDS=42 SOLAR_PROFILE_GPU=0 SOLAR_PROFILE_VARIANT=baseline SOLAR_PROFILE_OUTPUT=docs/mon-baseline.csv cargo run --release --locked
```

Sans Python, PowerShell sur la machine cible :

```powershell
$env:SOLAR_PROFILE_SECONDS="42"
$env:SOLAR_PROFILE_GPU="0"
$env:SOLAR_PROFILE_VARIANT="baseline"
$env:SOLAR_PROFILE_OUTPUT="docs/mon-baseline.csv"
cargo run --release --locked
Remove-Item Env:SOLAR_PROFILE_SECONDS, Env:SOLAR_PROFILE_GPU, Env:SOLAR_PROFILE_VARIANT, Env:SOLAR_PROFILE_OUTPUT
```

Pour la cible RTX 4050/Core i5, conserver modèle exact du CPU/GPU, RAM, pilote, OS, résolution en pixels, présentation, alimentation et réglages. Effectuer plusieurs répétitions et examiner le journal avant d'interpréter une différence. Les commandes Windows sont un protocole prévu, pas une validation native réalisée.

## Campagne du poste de développement

Campagne du 12 octobre 2026 : **huit lancements release de 42 secondes**, successifs, présentation Mesa immediate, 1440 × 900 sauf la variante de résolution, sans compilation concurrente. Tous terminent avec le code 0, quatre phases mesurées et aucun ERROR, WARN ou panic. Sources et binaire identifiés dans [environment.json](profiles/dev-nvk-2026-10-12/environment.json), mesures et journaux dans [le dossier de campagne](profiles/dev-nvk-2026-10-12/), résultats de toutes les phases dans [summary.csv](profiles/dev-nvk-2026-10-12/summary.csv).

| Essai | Échantillons repos | Moyenne frame ms | p95 ms | FPS moyens |
|---|---|---|---|---|
| Baseline initial sans recorder GPU | 142 | 49.33 | 58.67 | 20.27 |
| Sans caméra astronomique | 254 | 27.54 | 31.94 | 36.31 |
| Sans caméra locale et son UI | 308 | 22.70 | 27.17 | 44.06 |
| 720 × 450 | 384 | 18.22 | 22.04 | 54.90 |
| MSAA désactivé | 335 | 20.88 | 27.17 | 47.89 |
| Matériaux locaux unlit | 149 | 47.06 | 52.49 | 21.25 |
| Baseline final sans recorder GPU | 152 | 46.11 | 50.04 | 21.69 |
| Baseline avec recorder GPU | 148 | 47.44 | 52.42 | 21.08 |

Les deux baselines sans recorder GPU donnent 46,11–49,33 ms au repos : une dérive d'environ 6,5 % montre le bruit de cette session. Une seule répétition par variante est conservée ; le gain apparent d'unlit est de cet ordre et n'établit pas une optimisation utile. En revanche, les différences avec résolution réduite, MSAA désactivé et suppression d'une caméra sont nettement plus grandes. Elles orientent vers le chemin de rendu, sa composition et le pilote. Supprimer une caméra retire du contenu ; ce n'est pas un résultat acceptable en production sans remplacement visuel.

Dans le baseline initial au repos, les moyennes CPU mesurées sont simulation 0,008 ms, navigation/physique 0,030 ms, synchronisation du rendu 0,029 ms et interface 0,037 ms, soit environ 0,10 ms pour ces intervalles. Pendant le streaming, la synchronisation du rendu atteint au maximum 0,53 ms ; les pieds atteignent x ≈ 180 m dans tous les essais et franchissent bien la limite de tuile. Ces mesures ne désignent pas le contrôleur ou la génération de trois tuiles comme coût principal sur ce parcours. Elles n'excluent pas un coût CPU du moteur ou du pilote hors des intervalles instrumentés.

Le recorder GPU est pris en charge sur Vulkan/NVK. Le baseline avec recorder donne 47,44 ms au repos, proche des baselines sans recorder ; une seule paire et le bruit du poste ne permettent pas de chiffrer exactement son surcoût. Les derniers spans disponibles donnent notamment `msaa_writeback` 9,63 ms, `main_opaque_pass_3d` 3,32 ms, bloom 2,17 ms, tonemapping 2,05 ms, upscaling 1,08 ms et UI 0,64 ms au repos. `msaa_writeback` n'est pas présent dans la phase spatiale à une caméra. Ces diagnostics appuient l'examen futur de la composition MSAA de deux vues. Leurs chemins partagés ne permettent ni de sommer un GPU total fiable ni d'attribuer toutes les passes à chaque caméra. Le temps non couvert et les attentes de présentation/pilote restent à distinguer avec un outil plus complet sur la cible.

Les observations antérieures de 22–24 FPS dans le HUD restent des relevés ponctuels, distincts des FPS moyens de cette campagne. Le compteur `perf` du poste est inaccessible avec `perf_event_paranoid=4` ; aucun réglage de sécurité du système n'est modifié. Aucun nouveau timeout de présentation ne figure dans ces journaux, ce qui ne supprime pas le diagnostic VSync historique.

## Décision à l’issue de P1 (historique)

P1 est terminé comme étape de diagnostic : protocole transférable, données conservées et coût principal orienté vers le rendu, avec les limites CPU/GPU explicites. **P2, consolidation du prototype tournant, peut commencer.** Aucun niveau de qualité n'est réduit dans la partie normale pour atteindre 60 FPS sur le poste de développement. Sur la RTX 4050/Core i5, refaire les baselines et comparaisons avant de choisir les réglages ou une modification des deux caméras. Le profilage plus fin des attentes du moteur/pilote reste une piste conditionnelle, pas un obstacle au développement des fonctionnalités.


## Après consolidation P2

P2 est terminé sur le poste de développement ; [VALIDATION](VALIDATION.md) décrit le cycle complet, le masque solaire nocturne et les essais du menu. Les options de session permettent maintenant de choisir MSAA off/×2/×4 et le champ de vision, mais les défauts restent ×4 et 65°. Ce choix joueur n’est pas une optimisation automatique issue de P1. Les variantes de profilage gardent la priorité sur ces réglages ; le menu initial est contourné pendant les campagnes.

Un essai de régression de 30 s, msaa_off sans recorder GPU, a terminé avec code 0 : [journal](validation-p2-profile-smoke.txt), [CSV](p2-profile-smoke.csv), quatre phases et franchissement de tuile. Il vérifie le fonctionnement du scénario/export sur les nouvelles sources, sans remplacer les huit essais historiques ni justifier une comparaison de performance entre versions. Refaire la campagne complète sur la cible avant toute décision de qualité finale.


## P3.2 : budgets du premier parcours

Le parcours cube-sphere opt-in garde un index de contacts et deux meshes résidents, construits à l'entrée seulement. La [validation P3.2](VALIDATION.md) observe 8 385 sommets physiques, 16 384 triangles, 5 408 cases d'index, 53 240–53 260 références et 18 triangles maximum par case ; les plafonds et données de proxy sont dans [CUBE_SPHERE](CUBE_SPHERE.md). Les tests vérifient la réutilisation des deux handles. Ces comptes justifient un coût borné pour cette paire centrale ; ils ne mesurent pas les durées CPU/GPU, la VRAM totale ou le streaming futur.

Le parcours de 88 s réalise des captures et change de pose/calendrier ; ses relevés ponctuels de 22–25 FPS ne forment pas un benchmark comparable aux huit essais P1. Le profilage canonique conserve le terrain historique et prend priorité sur `SOLAR_CUBE_SURFACE`. Refaire une campagne dédiée lors de l'extension des zones et sur la RTX 4050/Core i5 avant toute décision de qualité.


## P3.3 : construction lors du changement de carte

La migration bornée opt-in remplace centrale↔voisine +U avec deux patches/handles après transaction. Une candidate et l'ancienne paire coexistent en CPU seulement pendant la construction ; aucun troisième mesh de terrain n'est ajouté. Les trente paires échantillonnées sur six faces donnent 1 112–5 408 cases, 34 283–53 255 références et 44 triangles/case maximum. Ces comptes ne mesurent pas la mémoire totale du moteur ou la VRAM.

La campagne release P3.3 journalise dix durées murales CPU de construction/transaction : **8,004–11,657 ms**, médiane **8,567 ms** ([synthèse](p33-validation/summary.json), [journal](p33-validation/migration.txt)). Huit migrations de parcours et deux préparations explicites, captures actives, une seule campagne sur GTX 1060/NVK. Ces durées incluent la géométrie/index et le transfert de l'état ; elles excluent la synchronisation ultérieure des meshes et le GPU. Elles ne sont ni des temps de frame complets ni un benchmark comparable aux variantes P1. Relevés HUD cube 23–25 FPS, sans prédiction de cible ou affirmation d'absence de pic GPU. Pas de changement de qualité par défaut. Le profilage canonique reste prioritaire et garde le terrain historique ; une extension hors centre demandera des mesures dédiées avant décision de budget final.


## P3.4 : construction hors centre et refus avant allocation

Le parcours isolé hors centre sur une même face conserve deux patches/handles. Le plan prépare deux cartes à l'entrée, puis ne garde qu'une géométrie et deux métadonnées ; hors admission/hystérésis/contact d'objet, le refus ne construit aucune candidate. Deux paires CPU maximum pendant préparation ou remplacement, ancienne paire libérée avant préparation lors des réentrées. Les tests vérifient les compteurs de construction et la réutilisation des handles ; ce plafond structurel ne mesure pas RAM/VRAM ou buffers transitoires internes du renderer.

Campagne release du 13 octobre 2026 : dix transactions hors centre, **6,918–9,750 ms**, médiane **7,306 ms**, captures actives. Régression centrale sur le même binaire : dix transactions, 7,758–11,458 ms, médiane 8,0705 ms. [Synthèse](p34-validation/summary.json), [journaux et limites](VALIDATION.md). Ces mesures murales CPU incluent géométrie/index/transfert, excluent la préparation initiale du plan, la mise à jour des meshes et le GPU. Une seule campagne n'établit aucun gain de FPS entre cartes ni absence de pic de frame.

L'index des deux cartes hors centre a 2 978 cases, 44 610/44 627 références et 18 triangles/case maximum ; rectangles conservateurs ~281×124 m. Les relevés HUD hors centre 19–25 FPS sont ponctuels, sans recorder GPU ou nouvelle campagne P1. Pas de mesure VRAM complète, Windows natif ou RTX 4050/Core i5. Avant une chaîne plus longue, chiffrer le coût/budget de préparation sans garder toutes ses géométries, puis mesurer les remplacements en release ; les budgets d'objets doivent tenir compte de l'orientation cumulée depuis l'entrée.


## Terrain mondial : comparaison et remplacement réel

13 octobre 2026. Le lancement normal utilise le terrain mondial ; le profiler historique reste le défaut. `SOLAR_PROFILE_TERRAIN=global` permet désormais une comparaison explicite avec les mêmes deux caméras, 1440×900, MSAA ×4, J2000, pause, parcours repos/contact/course/espace. `SOLAR_MARS_GLOBAL` ne remplace pas le choix du profiler. Niveau 15/32 segments, ≤13 patches/26 624 triangles et meshes communs réutilisés ; budgets structurels dans [GLOBAL_TERRAIN](GLOBAL_TERRAIN.md).

Trois essais release successifs de 42 s, sans recorder GPU : historique avant, mondial, historique après. CSV et journaux dans [global-validation](global-validation/), même binaire, Mesa immediate, aucune compilation/lancement GPU concurrent.

| Phase | Historique avant (frame ms / FPS) | Mondial (frame ms / FPS) | Historique après (frame ms / FPS) |
|---|---|---|---|
| Repos | 52,257 / 19,14 | 52,361 / 19,10 | 52,188 / 19,16 |
| Contact | 54,942 / 18,20 | 55,022 / 18,17 | 55,067 / 18,16 |
| Course | 51,838 / 19,29 | 51,778 / 19,31 | 51,913 / 19,26 |

Ces mesures ne montrent pas de régression notable du rythme sur ce poste. Les p95 au repos sont 55,067/55,468/55,423 ms. La navigation mondiale coûte davantage en CPU (maximum au repos 0,163 ms contre 0,054 ms avant), mais reste petite face au temps de frame. Les géométries/reliefs des fournisseurs diffèrent ; il s’agit d’une comparaison de deux parcours jouables, pas de l’attribution d’un gain à une seule ligne ou d’une preuve de performance universelle.

La phase course de 8 s du premier essai mondial termine à x≈63,8667 m après le rebasage de préparation ; elle **n’inclut pas de remplacement mesuré**. Un essai supplémentaire mondial de 54 s, phases de 11 s, inclut un véritable remplacement en course : compteur 18→23 constructions, cinq nouveaux patches. Navigation maximum **5,616 ms**, synchronisation du rendu maximum **0,165 ms** ; frame moyenne **52,005 ms**, p95 **54,932 ms**, maximum **56,741 ms** dans cette phase. Un rebasage seul, pas une garantie sur tous les pics/attentes GPU. Le rebasage de préparation de 6,066 ms est exclu par la seconde de stabilisation. [CSV](global-validation/global-streaming.csv), [journal](global-validation/global-streaming.txt).

La campagne graphique globale de 120 s effectue douze rebasages, captures actives ; leurs temps muraux CPU sont conservés dans [summary.json](global-validation/summary.json). Ils incluent construction/index/transfert/libération, excluent la synchronisation ultérieure des meshes et le GPU. Le terrain n’est généré ni par frame ordinaire ni quand Mars tourne. Les CPU/index simultanés et la réutilisation des assets sont testés, sans instrumenter l’ensemble RAM/VRAM/buffers de Bevy.

Pour reproduire un essai mondial incluant le remplacement, après compilation :

```bash
SOLAR_PROFILE_TERRAIN=global SOLAR_PROFILE_SECONDS=54 SOLAR_PROFILE_GPU=0 SOLAR_PROFILE_OUTPUT=docs/mon-profil-global.csv target/release/solar_survival
```

Ajouter Mesa immediate seulement sur le poste NVK. Mesurer nativement Windows/RTX 4050/Core i5 avec le modèle exact CPU, RAM, pilote, alimentation et réglages ; aucun 60 FPS cible ni résultat Windows n’est revendiqué ici.


## P4 : bâtiments et clarification du LOD

Le terrain conserve son voisinage de treize patches à niveau 15/32 segments. **Ce chargement dynamique n’est pas un LOD adaptatif** : aucun niveau n’est choisi selon la distance. La future extension à plusieurs niveaux devra mesurer le coût des changements/raccords, conserver des contacts fins localement et documenter les budgets CPU/GPU. Les performances historiques à résolution fixe ne valident pas cette fonctionnalité future.

P4 ajoute six boîtes de contact et six proxies par abri local, un mesh de cube partagé et deux matériaux ; un abri en partie normale, deux dans le diagnostic. Proxies supprimés au-delà de 120 m et en espace ; les données corporelles restent en CPU, interrogées avec exclusion conservative. Pas de génération de mesh par frame ni de changement des budgets du terrain. Ces plafonds structurels ne sont pas des mesures complètes RAM/VRAM.

La survie et ses assets/HUD sont désactivés dans le profilage et les scénarios historiques pour conserver leurs scènes. Le nouveau parcours de 56 s sert à valider la fonctionnalité avec captures ; ses FPS ponctuels ne constituent pas un benchmark P1 ni une garantie d’absence de régression. Une campagne dédiée aux bâtiments et, ultérieurement, au LOD devra être adaptée au nombre réel d’instances et mesurée sur la cible RTX 4050/Core i5.


## P5 : budgets et prochaine phase d’optimisation

P5 réutilise le cube de l’abri pour les caisses et le fantôme, ajoute trois matériaux et un HUD. Au plus huit caisses, six proxies de preview, six par abri local à moins de 120 m. L’admission borne la session à 32 abris : 206 proxies au plafond conservateur (hors terrain/obstacles). Les 48 unités disponibles permettent normalement seulement quatre constructions, donc cinq abris au total et ≤44 proxies avec preview/caisses. Collections linéaires bornées, sans index supplémentaire ni nouvelle dépendance.

L’admission interroge les triangles sous un carré de 9×9 m avec l’index terrain existant ; seulement pendant le placement et lors de sa confirmation. Extrema et aire couverte exacts, aucune génération de patch/mesh. Le contact joueur conserve son chemin de maximum sans calcul du minimum. Les caisses/fantômes réutilisent leurs entités au rebasage et libèrent leurs proxies à la sortie ; données corporelles f64 conservées. [CONSTRUCTION](CONSTRUCTION.md) détaille les réserves et budgets structurels, sans mesure RAM/VRAM exhaustive.

Les diagnostics P5 avec captures valident la fonctionnalité et la lisibilité, **pas un benchmark**. Aucun gain/perte de FPS attribué à la construction sans comparaison contrôlée. Les anciens profils gardent leur scène sans survie/construction. La prochaine phase dédiée reprend les mesures release avant optimisation : fenêtres et matériel identiques, repos/contact/placement/construction et streaming réel, répétitions, CPU/GPU et deux caméras. Les coûts LOD/raccords mixtes devront être instrumentés pendant les transitions et les budgets retenus comparés au terrain fixe. Plusieurs niveaux selon distance restent à implémenter ; le cache de treize patches n’en constitue pas la preuve. Windows/RTX 4050/Core i5 restent à mesurer nativement.
