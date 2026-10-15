# Prochain chat Solar Survival — P12 intégré, validations arrêtées

Mise à jour : **15 octobre 2026**. Chef de projet : **SkyBlyvion**.

## Consignes utilisateur prioritaires

**Ne plus lancer de tests, de benchmark ou de campagne pour compléter les cases.**
L’utilisateur avait autorisé la main sur le poste et toutes les validations en
attente, puis a précisé « ne refais pas la passe d’expédition, c’est trop long »,
et enfin « arrête de revalider […] les mêmes tests ». Cette dernière consigne
arrête les validations. La passe graphique unique des trois trajets a réussi et
n’a pas été répétée. Les résultats ouverts ne donnent aucune autorisation de
relance. Aucun jeu, benchmark ou enregistrement n’est laissé actif.

## Prompt autonome

Nous poursuivons Solar Survival dans `/home/cfa_m2_cs/Documents/SOLAR_SURVIVAL`.
Lire AGENTS.md, README.md, ROADMAP.md, cette passation, ASTRONOMY.md,
TESTS_A_REALISER.md, P12_CONTROLS.md, P11_AUDIO.md, PERSISTENCE.md et les premières
sections de VALIDATION.md/PERFORMANCE.md ; détails dans
`docs/p12-validation/2026-10-15/NOTES.md` et
`docs/validation-groupee/2026-10-15/NOTES.md`.
Examiner Git : départ de ce chat propre à `ecc711c` (P11), P12 et la campagne sont
maintenant présents dans le workspace **non commité**. Aucun commit demandé/créé.
Préserver les changements et preuves ; ne pas attribuer les anciens travaux à P12.

**Objectif suivant : suivre la nouvelle demande de SkyBlyvion, sans relancer de
validation par défaut. P13 n’est pas défini.** Si une prochaine demande porte
sur les commandes, deux aides secondaires restent à corriger : dans
`src/ui/construction.rs`, distance de caisse (~ligne220, « E à moins… ») et rappel
de catalogue (~ligne226, « B : construction ») n’utilisent pas encore les marqueurs
de touches. Cette limite est documentée ; elle n’a pas été modifiée après l’arrêt.
Ne pas transformer cette passation en ordre de relancer une campagne ou un trajet.

## État réel P12

- `controls.rs` : 26 commandes physiques, table de touches admissibles/ids disque
  explicites, conflits refusés sans mutation, modificateurs gauche/droite liés.
  Menus, chiffres0–9, flèches, Entrée/Échap, F5/F9/F12, molette et souris fixes.
- Options → Commandes : cinq pages, six lignes maximum, saisie/annulation/reset,
  nom de la commande en conflit. QWERTY/AZERTY indiqués pour les positions qui
  diffèrent. Pas de combos, manette ou remappage de souris ajouté.
- `GameplayKeys` traduit une fois les entrées brutes après le menu, avant
  inventaire/caméra. Navigation espace/surface, construction, inventaire,
  expéditions et HUD utilisent cette vue. Touches maintenues dans un menu
  bloquées jusqu’au relâchement. Champs de nom et menus restent sur le brut.
- Aides par marqueurs `<key:KeyF>` et petit cache des modèles statiques, nettoyé
  des entités retirées après Interface. Deux rappels E/B encore fixes, ci-dessus.
- **Écriture format6 ; lecture4/5/6.** Affectations dans PlayerSettings par
  aventure ;4/5 utilisent les commandes par défaut. Ancien binaire P10/P11 ne lit
  pas6. Formats1/2/3 toujours refusés sans supprimer les fichiers. Validation du
  nombre, des ids et de l’unicité avant publication ; checksum/secours conservés.
- Aucun ajout de dépendance, de physique ou d’astronomie. Bevy exactement0.19.1,
  Rust stable2024, Cargo.lock inchangé ; main.rs inchangé.

## Résultats acquis et limites

**165 tests CPU/ECS**, fmt/check all-targets, Clippy -D warnings, release et
compilation release du diagnostic réussis **avant l’arrêt**. Aucun nouveau test
relancé après. Les erreurs intermédiaires de compilation/fixtures/Clippy ont été
corrigées et leurs journaux conservés.

- Une seule passe graphique au sol des trois expéditions avec C/V :268,660s,
  code0,21captures,36alliage/18circuits. Entrées ECS et visée automatique ; pas
  une partie humaine. Rebasages/patches et recharge contrôlés.
- Options/commandes720p/900p, conflits/pages, aides, inventaire/catalogue et cas
  ciblés : sortie0. Sites futurs/hors portée, murs/autre abri, rapport sans O₂
  de base, sorties, épuisement/R, jour/crépuscules/nuit/tempête contrôlés selon
  les assertions. Les cas préparés réinstallent aussi les obstacles du site ;
  ne pas les confondre avec les trajets non repositionnés.
- Deux processus : données en cours et C/V restaurés, rapport unique après retour
  préparé à l’abri. Archive803 fichiers/11WAV vérifiée et lancée hors checkout/CWD
  extérieur ; P9 base/profils116,403s+26,472s,deux codes0,36captures inspectées.
  Entrées natives XTest sur l’archive,sortie0,progression/stock/touches conservés
  après rapport déjà payé. Pas de nouveau trajet d’expédition.
- Backend audio réel avec AudioSink ; onze motifs demandés, faible5,020s et
  critique1,507s. Sortie du jeu seule enregistrée (pas de microphone/autres apps),
  13,78s,22050Hz,crête0,0982,aucun échantillon écrêté. pw-record interrompu
  volontairement par SIGINT/code1,WAV lisible ; aucune écoute subjective validée.
- Absence de périphérique limitée à l’environnement ALSA du seul processus :
  avertissement attendu,57,153s,code0,pic5 lecteurs/0sink,expiration sans accumulation.
-101captures produites, inspection ciblée et quatre planches P9 ; pas101 vues
  individuellement approuvées. Aucun aval esthétique/sonore de l’utilisateur.
- **Performances non comparées** : deux références P8 en échec avant export,
  8,608s puis26,424s. La seconde passe la collecte et échoue au placement ; cause
  complète non établie. Le scénario64 équipements/audio activé-muet est préparé
  mais non exécuté. VRAM totale/staging, Windows11 natif, RTX4050/CPU cible exact
  et1080p60 restent ouverts. Ne pas relancer ces essais sans nouvelle demande.

Archive : `dist/solar-survival-ubuntu-p12.tar.gz`,SHA256
`1506af817db4cc20f41e8b34b040c0dfab9e41ca98fa4e6adcfc5262ef5d65c5`.
Binaire `36bb3f85c1c764c9ade64ec15285569ab3f41cc1b12d6288d69bd4b669c7fbba`.
Sources de jeu/assets/lock inchangés durant les essais ; preuves dans le JSON de
concordance. Les dernières mises à jour après l’arrêt sont documentaires.

## Contexte permanent et compilation

P9 : porté8/coffre16/pile20, six recettes, base collective40m ; temps réel pour
production/renouvellement, P/T pour conditions. P10 : trois missions120/240/360m,
F de base pour relevé/rapport, récompenses uniques, perte des données en cours
sur échec/sortie. P11 : onze WAV originaux187020octets,huit voix maximum, alertes
O₂30s/10s,volume/muet. Profils6 actuels, lecture4/5 conservée.

Ubuntu de compilation : pkg-config/libasound2-dev absents ; sysroot local P11
sous `/tmp/solar-p11-build-deps`, `source /tmp/solar-p11-build-deps/env.sh` pour
Cargo si un prochain travail le requiert. Temporaire, peut disparaître ; procédé
reproductible dans les notes P11. Aucune installation système faite. Runtime ALSA
standard installé ; aucun chemin temporaire distribué. Windows utilise CPAL natif.

SI/f64/DVec3/DQuat, soustraction de l’origine avant f32, proxies en lecture.
Input→Simulation→Navigation→Rendering→Interface ; Time<Real> orbital indépendant,
souris sans facteur delta time. Visites spatiales translatoires, surface tournante
et forces d’inertie omises conservées. Ubuntu/X11,i7-6700K/32Go/GTX1060 3Go NVK.
En fin de prochain chat de développement : actualiser roadmap, validations/limites
et cette passation, puis fournir son lien. Respecter en priorité l’arrêt des tests.
