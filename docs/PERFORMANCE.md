# Performance Solar Survival

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
