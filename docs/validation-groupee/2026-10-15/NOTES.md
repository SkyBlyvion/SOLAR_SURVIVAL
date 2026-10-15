# Campagne groupée — 15 octobre 2026 — arrêt demandé

SkyBlyvion a demandé P12 (ergonomie/remappage), puis autorisé les validations en
attente et la main sur le poste, sans questions. Pendant la campagne : ne plus
refaire la passe d’expédition, puis **arrêter de répéter les tests**. La campagne
est arrêtée. Aucun nouveau test ni benchmark n’est autorisé par cette passation.
Les cases ouvertes sont un état des limites, pas un ordre de relance.

[Résumé machine](summary.json), [checklist et statut détaillé](../../TESTS_A_REALISER.md),
[environnement](environment.json), [concordance des sources](source-concordance.json).
Départ Git propre `ecc711c` (P11). Aucun commit créé. Ubuntu/X11, i7-6700K/32Go,
GTX1060 3Go/NVK Mesa25.2.8 ; Ruststable1.99.0, Bevy exactement0.19.1. Pas d’hôte
Windows11 ou RTX4050. Ni dépendance nouvelle ni changement physique/astronomique.

## Acquis

- Un seul parcours des trois expéditions au sol, commandes ECS et visée automatique,
  touches remappées C/V : **268,660s, code0,21captures**, trois récompenses uniques
  (36alliage/18circuits), recharge et patches bornés. Pas de partie humaine.
  [Résultat](initial-900.result.json), `initial-900/assertions.txt`.
- **720p,60,415s, code0,22captures** : options, cinq pages de commandes,
  conflit/saisie/annulation, aides, inventaire/catalogue, portée/site futur,
  murs/autre abri, rapport sans réserve de base, quatre sorties, échec/R, volume/
  muet/reset ; jour/crépuscules/nuit/tempête avec dates/poses préparées.
  [Résultat](ui-cases-720.result.json), `ui-cases-720/assertions.txt`.
- Deux processus pour les données en cours et C/V, rapport payé une fois après
  retour **préparé** dans l’abri :8,500s,code0. [Résultat](restart-load.result.json).
- Vrai backend audio Ubuntu, AudioSink observés (pic2 dans le parcours). Onze motifs
  demandés ; rappels faibles5,020s, critiques1,507s. Enregistrement numérique limité
  aux deux ports **de sortie du jeu**, sans microphone ni mix des autres applications :
  22050Hz,13,78s,crête0,0982,aucun échantillon écrêté. pw-record arrêté volontairement
  par SIGINT (code1), WAV finalisé/lisible ; [analyse](audio-analysis.json),
  [commandes et liaisons](audio-capture.json). Ne prouve pas le confort subjectif,
  la distinction à l’oreille ou la sortie acoustique des enceintes.
- Sans périphérique : environnement ALSA invalide du seul processus enfant, aucune
  configuration système modifiée. **57,153s,code0**, avertissement attendu, pic5
  lecteurs/0sink et expiration vérifiée. [Résultat](no-audio-device.result.json).
- Archive P12 :803 fichiers et onze WAV vérifiés, binaire identique au release,
  toutes bibliothèques runtime résolues ; extraction hors checkout. Deux processus
  depuis un **CWD extérieur** :116,403s+26,472s,codes0,36captures inspectées sur quatre
  planches. Récoltes, stockage/transferts, équipements, production/recharge,
  profils indépendants/refus Broken, nuit, rebasage, espace et R validés.
  [Résultats](archive-p9-regression/results.json),
  [planches](archive-p9-inspection/manifest.json).
- Même binaire extrait : entrées XTest réelles F9/Entrée/V/Échap/F5/clic droit,
  cinq captures, sortie0, stock/progression/touches inchangés après rapport déjà
  payé. [Résultat](native-input-archive/result.json). Pas de déplacement au sol.

**101 captures produites au total**, pas101 inspections individuelles : les36
captures P9 ont été examinées sur quatre planches, ainsi que des vues directes
900p/720p (menus, conflit, aides, balise et nuit). Les autres variantes restent
conservées sans prétendre à une inspection exhaustive ou une approbation esthétique.

Les cas ciblés utilisent `MarsTerrain::set_site` et des poses/JD/réserves préparés ;
cette préparation réinstalle aussi les trois obstacles de diagnostic près du
site choisi. Ne pas confondre ces scènes avec les trajets au sol non repositionnés.
Les anciens scénarios automatisés peuvent désactiver le backend audio ; le
nouveau diagnostic conserve les plugins renderer/audio réels.

## Échecs et arrêt

Référence P8 avant les comparaisons :8,608s code101, aucune collecte après le
segment de marche, aucun export ; [résultats](performance-versions/results.json).
Diagnostic suivant avec trace du focus :26,424s code101, collecte passée mais
assertion du placement échouée ; [résultats](p8-focus-diagnostic/comparison.json).
Cause complète non établie. Ces essais n’établissent aucune performance comparative.
La demande d’arrêt est arrivée ensuite ; aucun jeu/benchmark/capture audio n’était
encore actif au contrôle final. [Arrêt utilisateur](user-stop.json).

Pas de comparaison P8→P9/P9→P10/P10→P11/P11→P12 valide. Le scénario64 équipements/
audio activé-muet est écrit et compilé mais **n’a pas été exécuté**. RAM échantillonnée
pendant les validations ; VRAM totale/staging et GPU comparatif non mesurés.
Windows11, matériel cible/1080p60, confort sonore et variantes restantes demeurent
ouverts. Deux rappels secondaires de caisse/catalogue affichent encore E/B par
défaut dans `src/ui/construction.rs:220/226` : correction restant à faire.

Aucun code du jeu/asset/Cargo.lock n’a changé pendant les essais graphiques ;
la dernière mise à jour après l’arrêt est documentaire. Aucun nouveau test Cargo,
release, graphique ou benchmark n’a été relancé après cette instruction.
