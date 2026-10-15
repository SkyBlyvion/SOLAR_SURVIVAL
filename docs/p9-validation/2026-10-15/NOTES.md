# P9 — base, inventaire et stockage — 15 octobre 2026

Départ : `09f7027cabb2057bb48f98d728cee40a1118aad1`, checkout propre ; P8
et son cadrage P9 documentaire présents. Travail P9 conservé dans le workspace,
sans commit demandé. Aucun changement de Bevy (=0.19.1), de Cargo.lock ou de dépendance.

## Fonctionnement livré

Porté 8 piles / coffre 16 / 20 objets par pile ; alliage et circuits ; quantités
exactes 1–20, pile, maximum admissible, regroupement et échange d'emplacements.
Transferts par portée/segment dégagé, atomiques ; récolte pleine refusée sans
épuiser la source. Six recettes depuis le porté, admission/contact corporels
f64 et identités communes aux abris/coffres/équipements. Quatre sources durables,
30 secondes actives par lot de six, au-delà des huit caisses finies.

Panneaux 160 W crête conventionnels, incidence corporelle et atténuation poussière/
nuages P8 ; batteries 4,8 MJ, générateur 40 W → 4 s O₂/s, réservoir +3 600 s.
Réserve initiale 720 s, combinaison 180 s, recharge 30 s/s puis consommation si
la réserve se vide au milieu d'une longue frame. Bus collectif à 40 m du premier
abri, sans câbles ; boîtiers visuels simples réutilisant les matériaux P7.
Menu/espace/échec suspendent base et survie ; inventaire/catalogue restent actifs.
Aucun rattrapage application fermée. R est un secours de gameplay, sans remplir
la base ni réinitialiser les contenus. [Contrat complet](../../P9_BASE.md).

Format 3, sans migration des anciens formats 1/2, conformément à la décision
utilisateur. Organisation des piles, coffres/poses/identités, énergie J, O₂ s et
renouvellements sérialisés ; états d'UI et preview annulés. Validation avant
publication ; checksum et secours P6 conservés, limite 256 Kio.

## Vérifications de compilation et données

[checks.json](checks.json), journaux `fmt-final.txt`, `check-final.txt`,
`clippy-budget-final.txt`, `tests-complete-final.txt`, `release-concordance.txt` :
fmt, check all-targets, **147 tests**, Clippy -D warnings et release réussis.
Rust stable 1.99.0, Ubuntu/X11, i7-6700K, GTX1060 3 Go/NVK Mesa 25.2.8 ;
[environnement](host.json). Les temps de build incluent parfois une attente du
verrou Cargo ; ils ne constituent pas un benchmark du compilateur.

Tests P9 : transactions exactes refusées, capacités pleines, maximum partiel,
répétitions et conservation, deux types et recettes, vraie UI de transfert,
source refusée intacte, renouvellement/portée/mur, boucle d'amorçage, admission et
contacts partagés, énergie/O₂ bornés, nuit/tempête/arrêt/reprise, réserve pleine,
interruption longue et budget O₂ fini, panneaux stables au rebasage ; deux profils
avec deux coffres, ordre des piles, charges/délais et refus de données incohérentes
sans mutation. Régressions terrain, précision f64, marche/course/saut, menus et
persistance existantes conservées.

Les premiers cycles sont conservés : erreurs de compilation corrigées, anciennes
attentes de recharge gratuite d'un abri distant actualisées ; le premier abri
continue de recharger après rebasage. Le test solaire initial utilisait le pôle Y,
dont la normale ne tourne pas : le test jour/nuit emploie désormais un site
équatorial X. Aucune tolérance physique/numérique n'a été élargie.

## Jeu et archive

Premier parcours `graphics-initial` : 116,429 +26,410 s, codes 0, 27+9 captures,
aucun diagnostic. L'inspection de l'UI a révélé le marquage ALLIAGE des caisses
réutilisé sur les nouveaux coffres/circuits. Correction par matériaux industriels
partagés sans ce faux marquage ; indication du motif d'arrêt du générateur,
commande I dans l'aide et E sans fermeture/récolte involontaire de l'écran ouvert.

Archive extraite : [packaged-final/results.json](packaged-final/results.json),
**116,404 +26,465 s, deux codes 0**, aucun WARN/ERROR/panic. Binaire et assets propres
à l'archive, **CWD extérieur au checkout**, aucun focus imposé pour ce parcours
sans marche injectée. Deux profils P9 Alpha/Bravo, collecte réelle des huit caisses
et de circuits, renouvellement réel attendu, coffre payé, transferts de quantité/
pile/max, quatre équipements payés, production/recharge, menu figé, rebasage,
passage à une date nocturne, redémarrage, fichier Broken refusé, échec/R et M.
Les dates, réserves de test et poses d'approche sont préparées : ce n'est pas une
partie humaine sans téléportation ni l'observation d'une nuit entière.

**36 captures de l'archive inspectées sur quatre planches** ; UI lisible,
contenus et états conservés, réserve/charge cohérentes, menus et nuit.
[Manifeste des vues](inspection/manifest.json). Les vues d'équipement du recorder
standard tombent à la confirmation, avant la pose d'inspection suivante : une
campagne supplémentaire utilise le même binaire, sans entrée clavier/souris OS,
et capture exclusivement sa fenêtre X11 aux quatre poses déjà prévues. Le focus
initial est explicite ; aucun écran virtuel. Cette campagne visuelle est distincte
des mesures de performance sans captures.

Archive : `dist/solar-survival-ubuntu-p9.tar.gz`, **36 994 649 octets**, 747 fichiers
vérifiés. SHA256 `8fb7c54a2928a07ee378d24f53d8aeeaac95c071a3fc265b037bad61c72611ea`.
Binaire `b8436414b812a9e453c1c8346dfe54b99db24577c65be2d3bd240f102d125499`.
[Manifeste d'archive](archive.json), [concordance](packaged-final/concordance.json).

## Limites

Pas de chimie, pression de pièce, câbles ou logistique automatique ; réserve et
charge collectives, pas de charge attribuée individuellement à chaque batterie.
Pas de démantèlement, équipements de proximité limités à la base initiale ;
coffres et abris peuvent être éloignés, mais un abri distant n'a pas de service
O₂. Boîtiers visuels simples, sans animation de machine. Les conditions solaires
sont échantillonnées en fin de frame, sans intégrer une alternance entière lors
d'une interruption. Les capteurs de VRAM/staging et un pire cas à 64 équipements
ne sont pas établis. Windows 11 natif, RTX4050 6 Go, CPU cible exact et 1080p60 restent
ouverts. L'inspection de captures n'est pas une approbation esthétique utilisateur.
P10 (exploration/objectifs) n'est pas implémenté.

## Arrêt des mesures demandé par l’utilisateur

Deux essais P8, conservés dans `performance` et `performance-final`, échouent avant
l’export CSV : codes 101, 26,385 /8,612s. L’entrée dans l’abri, puis la première
récolte ne sont pas atteintes. La trace en lecture seule du deuxième essai observe
une perte de focus ; ce n’est pas une mesure de régression P9. Aucun chiffre de
surcoût P9 n’est déduit de ces essais incomplets.

**Décision utilisateur après ces essais : arrêter les tests qui prennent le focus,
la souris ou interrompent son autre travail ; les reporter, ou employer seulement
des vérifications sans interaction avec son bureau.** Plus aucun processus de jeu,
benchmark ou test graphique n’est en cours. Pas de contournement par écran virtuel,
ni modification des règles de capture du gameplay. Les lanceurs P9 exigent désormais
une option de créneau visible explicite ; ils ne lancent plus silencieusement une
campagne avec focus. Aucune campagne graphique supplémentaire ne doit être lancée
automatiquement dans le prochain chat. Les 147 tests et la validation fonctionnelle
précèdent cette décision et restent acquis pour ce code.

La campagne de quatre vues supplémentaires a réussi : 118,061s, code 0, aucun
diagnostic, même binaire. **40 captures finales inspectées** : 36 de l’archive et
quatre vues des boîtiers panneau/batterie/générateur/réservoir. Les modèles restent
simples, identifiés par couleur/type et l’UI, sans animation. Les comparaisons de
performance, pire cas de base et Windows natif restent à effectuer plus tard.
