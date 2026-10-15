# P12 — commandes remappables — 15 octobre 2026

Départ propre `ecc711c` (P11). Périmètre choisi par SkyBlyvion : ergonomie et
commandes remappables. Aucun commit demandé/créé. [Contrat](../../P12_CONTROLS.md).

26 commandes physiques, cinq pages de six lignes au plus, saisie avec refus des
conflits et touches réservées, Échap annule, reset local/général. Traduction commune
`GameplayKeys` après le menu et avant inventaire/caméra ; menus et saisie des noms
conservent les entrées brutes. Barrière des touches maintenues dans un menu.
Aides statiques/dynamiques avec marqueurs explicites et cache nettoyé des entités
retirées. Quelques alias fixes/documentés et deux côtés des modificateurs liés.
Format6 avec affectations validées avant publication ; lecture4/5 avec défauts,
formats1/2/3 refusés. Aucune dépendance ajoutée ; Bevy exactement0.19.1 conservé.
Aucun changement de physique, horloges ou GameSet. main.rs n’a pas grossi.

## Résultats obtenus avant la demande d’arrêt

- fmt/check all-targets, **165 tests CPU/ECS**, Clippy all-targets -D warnings et
  release réussis ; `tests.txt`, `check.txt`, `clippy-with-campaign.txt` et `release.txt`.
- Six nouveaux tests : conflits/ids/modificateurs, maintien/menu, aides,
  vrais boutons/pages/capture, sauvegarde/invalides et navigation remappée espace/
  surface/pause. Test audio étendu à la lecture4/5 avec écriture6.
- Le premier check utilisait un mauvais nom de variante SoundCue et une fixture
  MenuState incomplète ; corrigés. Un test d’inventaire isolé avait besoin du
  traducteur d’entrées ; fixture corrigée. Clippy demandait4..=6 ; corrigé.
  Journaux intermédiaires conservés, aucun test retiré ou assoupli.
- Diagnostic graphique/audio compilé en release ; Clippy all-targets inclut cet
  exemple. Les scripts ajoutent des entrées réelles/natives opt-in, une détection
  de la fenêtre du diagnostic et la prise en charge d’un CWD extérieur.
- Archive803 fichiers/11WAV, SHA256
  `1506af817db4cc20f41e8b34b040c0dfab9e41ca98fa4e6adcfc5262ef5d65c5`, binaire
  `36bb3f85c1c764c9ade64ec15285569ab3f41cc1b12d6288d69bd4b669c7fbba`.
  [Vérification](archive-verification.json), [runtime](runtime-libraries.txt).

La compilation utilise le sysroot ALSA temporaire de P11, sans installation
système : `source /tmp/solar-p11-build-deps/env.sh`. Voir P11 pour le reconstruire
si /tmp est nettoyé ; ne pas distribuer ces chemins.

[Campagne groupée et limites](../../validation-groupee/2026-10-15/NOTES.md).
Les validations sont **arrêtées par l’utilisateur** ; ne plus les relancer pour
cocher des cases. La passe d’expédition réussie n’a pas été répétée. P12 reste
avec deux aides secondaires E/B non remappées ; performance, Windows/cible et
certains scénarios sont ouverts. Aucun processus de test laissé actif.
