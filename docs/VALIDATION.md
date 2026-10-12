# Validation Solar Survival 0.2 — 12 octobre 2026

Environnement exécuté : **Ubuntu 24.04.5 LTS**, session X11 (`DISPLAY=:1`), Rust stable **1.99.0**, édition 2024, Bevy **exactement 0.19.1**. Le lancement release sélectionne **NVIDIA GeForce GTX 1060 3GB (NVK GP106)**, Vulkan, **Mesa 25.2.8-0ubuntu0.24.04.2**. Aucune dépendance ajoutée ; `Cargo.lock` ne change que la version de l'application (0.2.0).

| Vérification exécutée | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check --locked` | Réussi |
| `cargo test --locked` | **40 tests réussis** : 7 tests internes, 21 fondations, 12 tests de surface |
| `cargo clippy --all-targets --locked -- -D warnings` | Réussi, aucune alerte |
| `cargo tree -i bevy --depth 1` | Bevy 0.19.1 confirmé, une seule version |
| `cargo build --release --locked` | Réussi sur les sources finales, 1 min 55 s |
| `SOLAR_SURFACE_SMOKE_SECONDS=50 RUST_LOG=info cargo run --release --locked` | Réussi avec présentation Mesa immediate : 10 captures, sortie normale |
| `SOLAR_SMOKE_SECONDS=66 RUST_LOG=info cargo run --release --locked` | Réussi avec présentation Mesa immediate : 11 captures, sortie normale |
| Windows 11 natif | **Non exécuté** : aucun hôte Windows disponible |

Les deux scénarios release finaux ajoutent **`MESA_VK_WSI_PRESENT_MODE=immediate`** à leur environnement. Ce réglage de diagnostic est propre au pilote Mesa ; aucune branche Linux n'est introduite dans la logique du jeu.

Journaux : [check](validation-v02-check.txt), [tests](validation-v02-tests.txt), [Clippy](validation-v02-clippy.txt), [release](validation-v02-release.txt), [surface](validation-v02-surface.txt), [espace](validation-v02-space.txt).

## Couverture nouvelle

Les 23 tests précédents restent réussis. Les 17 nouveaux tests couvrent : chute de 20 m puis repos sans dérive, gravité et hauteur du saut martien, contacts du volume avec murs/plafonds/boîtes et glissement, réception stable sur une boîte et saut depuis son dessus, vitesse résolue nulle devant un obstacle, déplacement à 30/60/144 FPS, saut mis en attente entre deux pas physiques sans répétition, arrêt des commandes sans arrêt de la gravité, limitation des longues interruptions, limite du site, contacts continus aux coordonnées négatives et limites de cellules, verticale et hauteur de caméra dans un site arbitrairement orienté, concordance du mesh avec ses sommets physiques et raccords entre tuiles.

Les tests ECS vérifient aussi les quatre sorties de surface (M/G/O/U), la reprise d'une visite terrestre, l'indépendance de la physique vis-à-vis de P/T et l'absence de multiplication des deltas souris par le delta time. Le test de chargement vérifie neuf meshes résidents, leur réutilisation sans déplacement, six meshes conservés après franchissement d'une tuile, puis libération totale en quittant la surface.

Le contrôleur et l'architecture sont décrits dans [README](../README.md) et [ASTRONOMY](ASTRONOMY.md). Les attentes Clippy nouvelles concernent uniquement les paramètres et filtres d'emprunt des systèmes ECS.

## Inspection graphique finale

Les deux lancements release finaux ont produit leurs 21 captures attendues et terminé avec le code de sortie 0. Leurs journaux ne contiennent aucun ERROR, WARN ou panic. Le scénario surface montre l'arrêt devant la boîte, le saut, la course jusqu'à z = −150,969 m (franchissement de la limite de tuile z = −128 m), le repos après libération des commandes, puis les visites spatiales Mars et Terre. Le scénario spatial visite les dix corps et la vue globale.

Captures représentatives effectivement inspectées : [site au repos](v02-surface-00.png), [approche/contact](v02-surface-01.png), [saut](v02-surface-02.png), [course après changement de tuile](v02-surface-06.png), [retour spatial Mars](v02-surface-08.png), [Terre après sortie](v02-surface-09.png), [Saturne et anneaux](v02-space-03.png), [Lune](v02-space-06.png), [Soleil](v02-space-09.png) et [vue globale](v02-space-10.png). Le terrain apparaît en 3D, les obstacles restent à l'échelle humaine, l'aide et le HUD changent avec le mode et les sphères spatiales réapparaissent après sortie. Des marqueurs se superposent encore dans la vue globale, limitation préexistante.

Ces captures affichent environ **22–23 FPS en surface** et **42–55 FPS sur les vues spatiales inspectées**, avec la présentation Mesa immediate et sans compilation concurrente. Ce sont des valeurs ponctuelles du HUD, pas un benchmark ; **60 FPS n'est pas atteint ni revendiqué**. Le coût des deux caméras et le profilage GPU/CPU restent à examiner.

## Diagnostic de présentation sur cette session

Le premier essai release en présentation AutoVsync a montré environ **1 FPS**, avec un timeout d'acquisition de texture dans le lancement spatial et une capture asynchrone non reçue avant sa fermeture. À ce rythme, le plafonnement de 0,25 s par frame ralentissait volontairement la physique : cet essai ne validait pas le parcours prévu. Les journaux sont conservés : [surface VSync](validation-v02-surface-vsync.txt), [espace VSync](validation-v02-space-vsync.txt).

Le nouvel essai avec `MESA_VK_WSI_PRESENT_MODE=immediate`, [variable officielle Mesa](https://docs.mesa3d.org/envvars.html), a rétabli environ **23 FPS** sur la première capture de surface. Le journal montre ensuite l'arrêt devant l'obstacle (z = −10,698 m, vitesse nulle), puis le saut. Cela indique un problème de présentation dans les conditions de cette session ; ce n'est pas une preuve générale de la cause ni un benchmark. La configuration normale du jeu conserve AutoVsync. Le contournement est décrit dans le README ; Windows n'a pas été testé.

## Limites du prototype et de la validation

Le terrain est une grille locale de hauteur courbée sur la sphère moyenne, **pas encore un réseau mondial cube-sphere**. Le site translatoire est borné à ±4 096 m ; pas de rotation du terrain, de corps dynamiques, de grottes ni de pentes arbitraires. Le joueur utilise une boîte alignée sur le site ; sa caméra et la gravité restent radiales. La marche impose l'adhérence sur le relief doux. Les collisions avec les trois obstacles sont balayées ; les contacts terrain reposent sur les triangles f64 sous l'empreinte du joueur. Les sous-pas de 1/120 s acceptent au plus 0,25 s par frame, sans rattrapage des longues interruptions.

Les scénarios graphiques injectent des commandes dans l'ECS ; ils ne simulent pas les touches, souris ou clics au niveau de l'OS. Les captures servent à vérifier le rendu 3D et les transitions, pas à prouver tous les contacts physiques. Pas de validation native Windows 11 ni de benchmark dédié à 60 FPS. Le rendu astronomique compressé garde ses limites d'occultation et d'éclairage. La survie reste inactive.

---

# Historique : validation Solar Survival 0.1 — 12 octobre 2026

Environnement réel : Ubuntu Linux, session X11, Rust stable 1.99.0, Cargo 1.99.0, édition Rust 2024, Bevy **0.19.1**. GPU sélectionné par wgpu : **NVIDIA GeForce GTX 1060 3GB (NVK GP106)**, Vulkan, pilote **Mesa 25.2.8**, nouveau/NVK. VS Code : rust-analyzer et Even Better TOML installés. Aucun paquet système avec sudo n'a été nécessaire pour compiler et lancer sur ce poste.

| Vérification exécutée | Résultat |
|---|---|
| `cargo fmt --check` | Réussi |
| `cargo check` | Réussi |
| `cargo check --locked` | Réussi |
| `cargo test` | **23 tests réussis** : 2 tests du contrôleur ECS + 21 tests des fondations |
| `cargo clippy --all-targets -- -D warnings` | Réussi, aucune alerte non traitée |
| `cargo tree -i bevy --depth 1` | Bevy 0.19.1 confirmé |
| `cargo build --release` final | Réussi sur les sources finales (2 min 05 s) |
| `SOLAR_SMOKE_SECONDS=66 cargo run` | Réussi : lancement graphique, onze vues capturées, sortie normale |
| `SOLAR_SMOKE_SECONDS=66 cargo run --release` final | Réussi : onze étapes, captures PNG, sortie normale |
| Windows 11 natif | **Non exécuté** : aucun hôte Windows dans cet environnement |

Les signatures à huit paramètres de systèmes Bevy et le ParamSet de textes possèdent des attentes Clippy documentées : ce sont des paramètres ECS injectés, pas des appels métier à huit arguments.

## Tests et vérifications

[Sortie des tests](validation-tests.txt) et [sortie Clippy](validation-clippy.txt), [compilation release](validation-release.txt) et [journal de lancement release](validation-graphics.txt).

Les tests couvrent unités SI/UA, distances à plusieurs milliers de milliards de mètres, résidu de Kepler, inclinaisons, périastre/apoastre, périodes, catalogue et dimensions, barycentre Terre–Lune, orbite lunaire, stabilité sur la plage JPL, dates limites grégoriennes, repères rigides et conversions, préservation angulaire du proxy, pause/accélération temporelle, collision balayée supraluminique, gravité radiale, visites et zones cube-sphere reproductibles.

Les deux tests du contrôleur exécutent les vrais systèmes de caméra Bevy dans une App avec temps manuel : déplacement à 250 m/s pendant une seconde à 60 et 144 FPS, sélection de Mars, visite, capture puis libération du curseur. Ils ne simulent pas de touches au niveau de l'OS et n'équivalent pas à un test manuel de périphérique. Les clics sur les boutons n'ont pas fait l'objet d'un test manuel automatisé au niveau du système d'exploitation.

## Inspection graphique réalisée

Le lancement de validation visite les dix corps, puis la vue globale. Onze captures ont été produites et inspectées : Terre, Mars, Jupiter, Saturne, Uranus, Neptune, Lune, Mercure, Vénus, Soleil, système. Pas de panic, crash ou erreur de shader signalés dans le lancement de 66 secondes. Le renderer utilise bien le GPU discret Vulkan, et non un affichage console ou un schéma 2D.

| Capture | Vue |
|---|---|
| [screenshot.png](screenshot.png) | Terre |
| [smoke-01.png](smoke-01.png) | Mars |
| [smoke-02.png](smoke-02.png) | Jupiter |
| [smoke-03.png](smoke-03.png) | Saturne et anneaux |
| [smoke-04.png](smoke-04.png) | Uranus |
| [smoke-05.png](smoke-05.png) | Neptune |
| [smoke-06.png](smoke-06.png) | Lune et Terre distante |
| [smoke-07.png](smoke-07.png) | Mercure |
| [smoke-08.png](smoke-08.png) | Vénus |
| [smoke-09.png](smoke-09.png) | Soleil émissif avec halo |
| [smoke-10.png](smoke-10.png) | Vue globale et marqueurs |

Le test graphique a permis de corriger les accents manquants de la police initiale, d'aligner les sphères UV Z-up de Bevy avec les rotations planétaires Y-up et de réduire le halo sur les planètes non émissives. Les tests ECS ont permis de corriger la hausse de vitesse à molette immobile (`signum(0.0)` valait 1). La borne du calendrier a été recalculée et vérifiée indépendamment.

Les captures de développement indiquaient environ 23–50 FPS suivant la vue, pendant une compilation release concurrente. Il ne s'agit pas d'un benchmark dédié ; l'objectif 60 FPS n'est pas déclaré atteint sur ce poste. Le lancement release sans compilation concurrente a également fonctionné ; les captures Terre/Mars montrent environ 43–46 FPS sur ce pilote. Le seuil de 60 FPS reste à atteindre par profilage GPU/CPU et mesures dédiées.

## Limites restant à valider

Pas de FPS de surface, de terrain collisionnable ou de survie active : ces fonctionnalités appartiennent à la prochaine étape Mars. Pas d'éphéméride lunaire réelle, de cartographie terrestre réelle, de photométrie physique ni d'occultations exactes après compression du rendu. Des marqueurs peuvent se superposer dans la vue globale. Le prototype utilise des sphères et textures artistiques, avec une simulation physique 1:1 distincte.

La compatibilité Windows est prévue par les dépendances et le code portable ; sa compilation et son lancement natifs doivent encore être vérifiés sur Windows 11. La session graphique Linux a été effectivement testée.
