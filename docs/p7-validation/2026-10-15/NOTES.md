# Clôture P7 sur Ubuntu — 15 octobre 2026

La reprise commence à `eeaff80`, propre et identique à origin/main. L’audit confirme que l’implémentation était intégrée, mais que la galerie des six autres planètes, la mesure finale et le lancement de l’archive restaient ouverts. L’utilisateur a explicitement réservé le bureau aux campagnes, puis autorisé la dernière revue du raccord. Aucun écran virtuel ni changement des contrôles ajouté.

## Résultats réellement obtenus

- Galerie six planètes/crédits : 75,556 s, code 0, 25 captures, aucun diagnostic, sources/assets/binaire inchangés (`planets/results.json`). La revue a révélé une couture visible de Jupiter, donc cette première galerie n’est pas le résultat visuel final.
- P7 sans captures : deux répétitions sans recorder, 59,471 / 59,449 s, puis GPU séparé 59,483 s, trois codes 0, aucune erreur/alerte/panic (`performance/results.json`, `concordance.json`). Même récolte, stock, confirmation/refus, porte/recharge, rebasage, espace et menu P5.
- Référence P6 actuelle : un lancement sans recorder du binaire réellement distribué avant P7, SHA identique à before-p5 du 14 octobre, code 0. Même poste, 1440×900, MSAA4, LOD adaptatif et Mesa immediate, sans captures (`baseline-p6`). Cette référence supplémentaire évite de comparer uniquement deux sessions de bureau différentes.
- Archive initiale P7 extraite hors checkout : sauvegarde puis second processus de chargement à 1280×720, 48,410 + 54,376 s, deux codes 0 et 32 captures, aucun diagnostic (`packaged-720`). Alpha construit/rebase/recharge ; Bravo neuf et indépendant ; retour à Alpha, fermeture réelle, catalogue, espace, échec/R, profil Broken refusé avec Alpha préservé. Même GameCamera et poses/stock/services vérifiés.
- Premier parcours de construction à captures échoué à la porte (`construction`), conservé. Relance de 59,424 s réussie, 16 captures (`construction-retry`). Observation XGetInputFocus à 100 ms : focus conservé de la création à la fermeture (`construction-focus.json`). Aucun focus forcé en boucle ni contrôle modifié. La cause exacte de l’échec initial n’était pas instrumentée ; le scénario reste dépendant du focus, comme les anciennes campagnes. Ne pas présenter tous les essais comme réussis.

## Correction réellement apportée pendant cette reprise

Le composite Voyager de Jupiter présente des bords différents : écart RGB moyen 12,706/255. `periodic_longitude` corrige l’export sur huit colonnes de chaque bord, en lumière linéaire, poids smoothstep ; maître intact, 704 colonnes intérieures identiques, bords finaux égaux. Cette retouche artistique est déclarée dans CREDITS ; pas de donnée scientifique corrigée. Seul jupiter.p7tex change, puis son SHA dans le manifeste et les crédits ; les douze autres textures et tous les fichiers Rust/shaders/Cargo restent identiques. 720×360, dix niveaux et total GPU P7 inchangés (`jupiter-seam-fix.json`, `texture-audit-final.json`, `final-concordance.json`).

Nouvelle archive `dist/solar-survival-ubuntu-p7-final.tar.gz` : 36 876 876 octets, 747 fichiers vérifiés après extraction. Même binaire release SHA256 `96ef1731fdc29a540ca31858b8e7cd876b2fa31f8142bac8ec84533d45d813ea`. Cette archive finale a exécuté les 25 vues planètes/crédits hors checkout en 75,427 s, code 0, aucun diagnostic, sources/runtime/binaire inchangés. Original de la couture de Jupiter corrigée et toutes les vues inspectés (`planets-final-package`, `package-final.json`). Le parcours de profils ci-dessus précède la retouche du seul albédo Jupiter ; même exécutable, mêmes matériaux de surface et mêmes règles. Il n’est pas réattribué à un second lancement des profils dans la nouvelle archive.

## Inspection et tests

**98 captures inspectées sur 26 planches**, plus sept originaux de captures et le JPEG maître de Jupiter (`inspection/manifest.json`, `verified.json`). Les 34 vues surface/jour/nuit/rebasage/Terre/Mars inspectées le 14 restent dans inspection-first, sans les présenter comme une nouvelle campagne. Ghost vert/rouge, stock insuffisant, caisses, plaque et porte, retour d’aventure chargée/rebasée, phases/coutures/pôles des six planètes et crédits SkyBlyvion revus. Les vues de pôle très proches peuvent couper le bord des anneaux de Saturne ; la visite standard les cadre entièrement.

Les six commandes Cargo et **134 tests** du 14 octobre restent valides pour les sources Rust inchangées. Après le nouvel export : fmt --check, diff --check, py_compile et les **deux tests Rust de textures** réussis ; assertions offline de raccord/intérieur inchangé et audit SHA/taille des 13 payloads réussis (`targeted-tests.json`). Pas de nouveau build release attribué à cette reprise.

## Mesures et limites

[Comparaison exploitable](comparison.json). Moyenne P7 versus P6 : +2,38 ms au repos, +2,35 ms au placement, +2,49 ms en streaming (moyenne des deux répétitions P7). Fenêtres de repos courtes : p95 P7 32,829 / 36,557 ms, pas une garantie de performance générale. HWM P6 253 384 Kio ; P7 273 328–278 136 Kio. Ce sont des pics processus, pas une allocation de textures isolée. CPU de préparation des payloads, spans CPU/GPU et ressources brutes conservés. Spans GPU, notamment main opaque, ne mesurent pas le temps GPU complet. Les mesures précèdent la seule retouche RGB de Jupiter ; tous formats/dimensions/mips, shaders, algorithmes et douze autres exports sont identiques.

P7 est clôturé **sur Ubuntu de développement**, avec budget utile 42 118 856 octets = 40,168 Mio / 128 Mio, cohérence fonctionnelle et visuelle contrôlée. Windows 11 natif, matériel RTX4050/CPU exact/RAM/pilote/puissance et objectif1080p60 restent des validations transversales non réalisées. VRAM totale, alignement/allocation pilote et staging non disponibles sur NVK ; ne pas déduire zéro des compteurs absents ou de la sonde externe. Pas d’approbation esthétique donnée par l’utilisateur. P8 relief/horizons reste la prochaine étape, non implémentée ; ne pas ajouter des fonctionnalités pour masquer ces limites.

La préparation complète des treize exports a ensuite été relancée hors jeu :
code0,7,403s, tous les fichiers (manifestes/crédits compris) byte-identiques aux
exports/conditions présents dans l’archive finale (`reproduction.json`).
Aucun lancement GPU supplémentaire après libération du bureau utilisateur.

Les CSV de synthèse conservent les fins de ligne CRLF émises par le module
Python csv. Le contrôle Git des ajouts est passé avec `core.whitespace` incluant
`cr-at-eol`, sans réécriture des données de preuve.
