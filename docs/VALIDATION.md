# Validation Solar Survival 0.1 — 12 octobre 2026

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
