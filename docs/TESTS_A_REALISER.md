# Tests à réaliser — campagne groupée

Créé le **15 octobre 2026**, à la demande de SkyBlyvion. Référence des tests
reportés : **les réaliser lors d'une campagne commune, corriger les défauts puis
rejouer les cas concernés et leurs régressions**. Les lancements GPU se font
successivement, sans compilation concurrente : « ensemble » ne signifie pas
exécuter plusieurs jeux/benchmarks en parallèle.

**VALIDATIONS ARRÊTÉES à la demande utilisateur le 15 octobre 2026.**
Ne plus lancer de campagne, reprise de benchmark ni répétition sans nouvelle
demande explicite. La passe d’expédition déjà réussie est conservée et ne doit
pas être répétée. Des cases ouvertes ne donnent pas l’autorisation de relancer.

**Créneau initialement autorisé le 15 octobre 2026 après P12.** SkyBlyvion demande toutes les
validations en attente et laisse la main sur le poste, sans nouvelle question.
La campagne utilise le vrai bureau Ubuntu, successivement ; aucune preuve Windows
ou cible ne peut en être déduite. Les cases restent ouvertes jusqu’aux résultats.
**Ajustement utilisateur pendant la campagne : ne pas refaire la passe
d’expédition, trop longue.** La passe graphique 900p déjà réussie est conservée ;
aucune répétition des trois trajets au sol n’est autorisée par défaut.
Cette liste ne programme aucun rappel ni lancement futur automatique.

## Résultats déjà acquis

P10 : **153 tests CPU/ECS**, fmt/check/Clippy/release réussis ; archive Ubuntu
extraite et empreintes contrôlées, **sans lancement du jeu**. P9 possède déjà ses
parcours fonctionnels et captures. Ne pas les compter comme une validation visuelle
P10 ou comme une comparaison de performance réussie. Les cas fonctionnels ci-dessous
déjà couverts en CPU restent à confirmer dans le jeu et/ou après vrai redémarrage.

P11 : **159 tests CPU/ECS**, fmt/check/Clippy/release réussis ; archive803 fichiers
extraite/vérifiée sans lancement, décodage des onze WAV,
alertes, plafonds et options/persistance4/5 contrôlés sans périphérique. Écoute
et rendu des options ne sont pas validés par ces tests.

[Validation](VALIDATION.md), [performance](PERFORMANCE.md),
[règles P10](P10_EXPLORATION.md), [règles P9](P9_BASE.md).

## Préparation et preuves

- [x] **PRE-01 — Créneau et machines.** Confirmer le créneau et les hôtes disponibles.
  Relever OS, CPU exact, RAM, GPU/VRAM, pilote, résolution/MSAA, options, révision Git
  et empreintes binaire/assets. Windows et cible indisponibles restent ouverts.
- [x] **PRE-02 — Versions et profils.** Préserver les changements/fichiers utilisateur,
  utiliser une racine de sauvegarde d'essai neuve. P10 utilise le format 4. Chaque
  référence P8/P9 conserve son binaire, ses assets et ses profils compatibles ; ne
  pas charger une sauvegarde ancienne dans P10 pour fabriquer une comparaison.
- [x] **PRE-03 — Preuves.** Créer un dossier daté neuf sous `docs/validation-groupee/`.
  Conserver commandes, codes de sortie, journaux, captures inspectées, CSV et
  empreintes. Distinguer partie humaine, pilotage ECS et poses/JD/réserves préparés.

## P10 — parcours dans le vrai jeu

| Fait | ID | Scénario | Réussite attendue et preuve |
|---|---|---|---|
| [x] | P10-01 | Nouvelle aventure, M, les trois expéditions ≈120/240/360 m au sol, recharge entre missions et passage par porte | Trois balises et retours réalisables sans téléportation/repositionnement ; objectif, cap et déblocage compréhensibles. Conserver durées/O₂ et vues départ/site/rapport/3 sur 3. Signaler un parcours automatisé comme tel. |
| [x] | P10-02 | F hors portée puis à 3 m, balise future et F répété après relevé | Seule la balise active à portée donne ses données ; refus clair, ni paiement ni duplication. |
| [x] | P10-03 | Rapport dehors, derrière un mur, dans un autre abri puis dans le premier ; répéter F | Seul le joueur entièrement dans le premier abri valide ; récompense exacte et unique : 8+4, 12+6, 16+8 alliage/circuits. Capturer stock/progression. |
| [ ] | P10-04 | Porté plein, puis cas où l'alliage tiendrait mais pas les circuits ; ranger dans un coffre et réessayer | Refus du lot entier sans ajout partiel ni perte des données ; un seul paiement après place libérée. |
| [x] | P10-05 | Données en cours : essayer séparément M, G, O et U, revenir puis F | Données non rapportées perdues, découverte et anciens rapports conservés ; aucune livraison par téléportation, nouveau relevé possible. |
| [ ] | P10-06 | Épuisement O₂ avec relevé, R puis F, refaire le relevé | Échec lisible, données perdues/découverte gardée ; R ne paie rien et ne remplit pas la base. Mission récupérable. |
| [ ] | P10-07 | Menu, I, catalogue B, placement ; essayer F ; ensuite P/T en jeu | F bloqué dans les écrans ; menu suspend la survie, inventaire/construction la laissent active ; données conservées. P/T ne déplace pas les sites ni ne multiplie les débits réels. |
| [ ] | P10-08 | Abri construit autour d'une balise, relevé à proximité ; rapport avec réserve de base vide | Radio à 3 m utilisable sans visée/segment, aucun blocage définitif ; rapport au premier abri indépendant de son O₂. |

## P10 — rendu et ergonomie non inspectés

| Fait | ID | Scénario | Réussite attendue et preuve |
|---|---|---|---|
| [ ] | VIS-01 | HUD/messages longs à **1280×720 et 1440×900**, I/B/placement/H/N, mission finale | Texte lisible sans débordement/chevauchement, O₂ visible ; panneau d'objectif masqué/restauré correctement. Aide F cohérente espace/surface. Captures aux deux tailles. |
| [ ] | VIS-02 | Cap devant/gauche/droite/derrière, pitch haut/bas, mouvement et rebasage | Direction/distance cohérentes, base retrouvable ; avertissement O₂ lisible, estimation de retour sans garantie d'itinéraire. |
| [ ] | VIS-03 | Mâts actifs/validés sur pentes, jour, lever/coucher, nuit et tempête | Orange/turquoise identifiables ; pas de flottement/enfouissement ou scintillement gênant. Brouillard/terrain occultent normalement ; visibilité à travers une tempête non exigée. Vues et JD/météo conservés. |
| [ ] | VIS-04 | Rebasages répétés, éloignement/retour, espace/surface et changement de profil | Aucun saut de pose, doublon ou ancien mât résiduel ; transitions de chargement inspectées. Au plus six proxies de balises dans 600 m, aucun asset généré continuellement. |

## Sauvegardes et archive

| Fait | ID | Scénario | Réussite attendue et preuve |
|---|---|---|---|
| [x] | SAVE-01 | Sauvegarder à une balise avec données en cours, fermer le processus, relancer et charger | Même pose, données/découverte, O₂, inventaire et base ; retour/rapport possibles, aucune récompense au chargement. Fichiers et journaux des deux processus conservés. |
| [ ] | SAVE-02 | Sauvegarder après paiement, redémarrer et F ; alterner deux aventures puis créer une neuve | Aucun double paiement ni mélange de progression/stock/base ; profil neuf vide. Sauvegarde explicite avant Quitter, sans supposer un autosave. |
| [ ] | SAVE-03 | Profil corrompu, progression incohérente, ancien format 3 et principal manquant avec secours | Refus explicite sans mutation de la partie active ni suppression ; secours selon le contrat de persistance. |
| [ ] | DIST-01 | Archive du code final corrigé, extraite hors checkout ; lancer avec ses assets depuis un CWD extérieur | Démarrage/shaders/UI/police/crédits corrects ; création, relevé, rapport, sauvegarde puis chargement après redémarrage réussis. Empreintes, codes et vues conservés. Une extraction seule ne coche pas le lancement. |

Archive préparée : `dist/solar-survival-ubuntu-p10.tar.gz` ; son
[intégrité](p10-validation/2026-10-15/archive-verification.json) est vérifiée mais
son lancement reste ouvert. Si le code change, produire une nouvelle archive et
identifier le build réellement testé ; garder les anciennes preuves historiques.

Archive P11 préparée : `dist/solar-survival-ubuntu-p11.tar.gz` ;
[intégrité](p11-validation/2026-10-15/archive-verification.json) contrôlée après
extraction hors checkout (803 fichiers, onze WAV). **Non lancée, aucune écoute.**
AUD-06 garde ouvert son essai réel ; les archives P9/P10 restent historiques.

## Régressions P8/P9 avec P10

| Fait | ID | Scénario | Réussite attendue et preuve |
|---|---|---|---|
| [x] | REG-01 | Récoltes finies/renouvelables, transferts quantité/pile/maximum, rangement, construction depuis le porté | Quantités/ordre/capacités conservés, refus sans perte, accès coffre local, renouvellement et paiement unique ; F n'interfère pas avec E/I/B. |
| [ ] | REG-02 | Panneaux/batterie/générateur/réservoir : jour/nuit/poussière, panne/reprise, réserve pleine | États/réserves cohérents à l'écran, O₂ fini, aucune énergie/O₂ créé par installation ou chargement ; débits indépendants de P/T. |
| [ ] | REG-03 | Marche/course/saut, portes, construction, streaming/rebasages et retours spatiaux | Aucune fissure nouvelle, rupture de support/contact, texture glissante ou équipement déplacé ; ≤13 patches et couronne réutilisée. Cibler les zones touchées sans rejouer tout l'historique sans motif. |

## Performance reportée et budgets

Mesurer **après les corrections fonctionnelles**, sans captures, compilation ou
autre jeu/charge GPU concurrente. Référence identifiable, mêmes réglages/conditions
pour comparer, au moins deux répétitions. Un essai interrompu par perte de focus
avant la phase utile ne mesure pas le coût du gameplay ; conserver cet échec.

| Fait | ID | Mesure | Résultat à consigner |
|---|---|---|---|
| [ ] | PERF-01 | Reprendre **P8→P9**, interrompu pendant P9 : repos, marche/collecte/construction et vrai streaming sur parcours commun | CSV complets, codes 0, moyenne/médiane/p95/p99 des frames et CPU/GPU disponibles. Distinguer les spans du temps de frame et les incertitudes. Ne rien déduire des anciennes références P8 en échec. |
| [ ] | PERF-02 | Comparer **P9→P10** sur scène commune ; mesurer séparément HUD, mâts, rapport et rebasages propres à P10 | Surcoût comparable distingué du coût d'un parcours différent ; résolution/MSAA/JD/météo contrôlés. Pas de FPS extrapolés des captures. |
| [ ] | PERF-03 | Base chargée, notamment **64 équipements/coffres**, et cas défavorable avec abris/balises ; approches/retraits répétés | Frames/CPU/services/contacts/rendu, pics et ressources. Contrôler plafonds 32 abris/64 équipements et six proxies P10 ; pas de croissance non bornée. Décrire les poses préparées. |
| [ ] | PERF-04 | RAM/VRAM totale et pics de chargement/staging ; cycles espace/surface/profils | Outil/pilote/unités et réglages ; distinguer 40,168 Mio d'images P7 du total du jeu. Capteur absent : noter « non mesuré » et garder ouvert, sans assimiler à zéro ou au budget textures. |

## P11 — audio et options, à réaliser pendant le même créneau

Ces cas s'ajoutent à la campagne autorisée ultérieurement. **Pas d'écoute ni de
fenêtre maintenant.** Le format courant d'écriture est désormais 5 ; conserver
les références P9/P10 et leurs profils compatibles pour leurs propres mesures.
Lecture P10 format 4 conservée par P11, volumes initiaux 50 %/son activé.

| Fait | ID | Scénario | Réussite attendue et preuve |
|---|---|---|---|
| [ ] | AUD-01 | Écouter les onze motifs sur Ubuntu : récolte, transfert, construction, relevé/rapport, refus, profils, boutons et O₂ | Motifs distincts et confortables, aucun clic/craquement gênant ou son au survol ; pas de succès sonore sur transaction refusée, aucun double paiement/son permanent. Décrire matériel et volume, ne pas enregistrer le bureau sans besoin. |
| [ ] | AUD-02 | O₂ 31→30→10→0, recharge effective, R, menu, I/B, espace, P/T et longue interruption | Rappels 5 s puis 1,5 s réelles, une mort, recharge sans boucle ; pas de rafale après interruption, alertes actives en I/B, suspendues en menu/espace ; R ne simule pas un transfert O₂ de base. |
| [ ] | AUD-03 | Options à 1280×720 et 1440×900, volume 0/25/50/75/100, muet puis changement de volume et réactivation, Reset | Boutons lisibles et accessibles ; modification immédiate des sons actifs, silence à zéro/muet, aucun vieux son à réactivation ; Reset 50 %/activé ; HUD vital conservé. |
| [ ] | AUD-04 | Deux aventures avec volumes/muet différents, fermeture/rechargement, profil P10 format 4 puis sauvegarde | Réglages indépendants, aucune voix héritée ni mort rejouée au chargement ; profil 4 lu avec défaut audio puis écrit 5 au save explicite, secours préservé ; format 3 toujours refusé. |
| [x] | AUD-05 | Absence de périphérique audio au démarrage puis nombreuses actions, transitions et base chargée | Jeu utilisable, avertissement de backend explicite, huit voix au plus et expiration ≤2 s sans sink ; pas d'accumulation, de crash ou de baisse non expliquée. Le retour à chaud d'un périphérique n'est pas implémenté. |
| [ ] | AUD-06 | Nouvelle archive P11 extraite hors checkout, CWD extérieur ; sons, options et reprise | WAV/crédits/licences et bibliothèque ALSA runtime disponibles ; vraie lecture, save/load et UI réussis. Empreintes du nouveau build. Une archive P10 ne prouve pas P11. |
| [ ] | AUD-07 | Windows 11 natif : build/package/lecture, options et profils 4/5 | Backend Windows CPAL, aucune dépendance ALSA, signaux et silence fonctionnels ; aucune validation attribuée depuis Linux. |
| [ ] | PERF-05 | Mesure P10→P11 sur scène commune et actions fréquentes, huit voix, muet/activé, même périphérique et réglages | Coût CPU/frame/mémoire et incertitudes, au moins deux répétitions sans captures ; comparer séparément le backend audio réel. Les anciens profilers désactivent volontairement l'audio et ne mesurent donc pas à eux seuls son coût. |

## Windows 11 et cible — autre hôte requis

Même liste de campagne, mais ces cas restent ouverts sans les machines requises.
Linux ou une compilation croisée ne valent pas validation Windows native.

| Fait | ID | Scénario | Réussite attendue et preuve |
|---|---|---|---|
| [ ] | WIN-01 | Sur Windows 11 natif : Rust stable 2024/Bevy exactement 0.19.1, fmt/check/test/release | Commandes/codes et environnement MSVC/SDK/pilote conservés ; aucune dépendance OS dans la logique centrale. |
| [ ] | WIN-02 | Cycle P9/P10, souris/capture/libération, options/fenêtre et profils après redémarrage | Contrôles et UI française utilisables, aucune capture bloquée/perte de progression ; chemins/noms et refus des formats cohérents. |
| [ ] | WIN-03 | Construire l'archive Windows native, extraire dans un dossier neuf, lancer avec assets/licences | Exécution indépendante du checkout et profils conservés ; empreintes, codes et vues. |
| [ ] | CIBLE-01 | Relever CPU exact (« i5 12700K » à confirmer), RAM, GPU/puissance/pilote ; mesurer sur cible RTX4050 6 Go au mieux en **1920×1080** | Objectif 60 FPS évalué sur base/exploration/streaming avec moyenne/p95/p99, mémoire et réglages. Corriger/régler selon résultats ; ne pas extrapoler la GTX1060. |

## Corrections et clôture

- [ ] **FIN-01 — Corriger et rejouer.** Chaque défaut a son ID, sa reproduction,
  attendu/observé et sa preuve. Après changement significatif : fmt/check/test/release
  et contrôles adaptés ; rejouer le cas et les régressions concernées. Refaire
  l'inspection si le rendu change, les mesures si le coût ou la charge changent.
- [x] **FIN-02 — Concordance.** L'archive finale contient le binaire/assets réellement
  testés après correction ; aucun succès d'un ancien build attribué au nouveau.
- [ ] **FIN-03 — Bilan.** Mettre à jour cette checklist, VALIDATION, PERFORMANCE,
  ROADMAP et NEXT_CHAT ; cocher seulement les réussites avec preuve. Échecs et cas
  impossibles restent visibles, avec cause et suite.

### Registre à remplir pendant la campagne

Statuts : **à faire**, **en échec**, **corrigé à rejouer**, **validé avec preuve**,
**reporté avec raison**. Case cochée = validé avec preuve. Ajouter les futurs cas
nécessaires ; ne pas supprimer un cas ouvert pour raccourcir la liste.

| ID | Build / machine | Statut | Preuve ou défaut | Correction / nouvelle preuve |
|---|---|---|---|---|
| À renseigner | — | À faire | Aucune exécution lors de cette planification | — |


## Bilan de la campagne du 15 octobre 2026 — arrêt demandé

[Notes et limites](validation-groupee/2026-10-15/NOTES.md),
[index JSON](validation-groupee/2026-10-15/summary.json).

| Cas / groupe | Statut et preuve |
|---|---|
| P10-01 | Validé par un seul parcours ECS graphique des trois missions, visée automatique, sans téléportation des trajets ; trois paiements, total36 alliage/18circuits. 268,66s, code0,21captures. Pas une partie humaine. Aucune répétition après la consigne utilisateur. |
| P10-02/03/05 | Portée, balise future, répétition du relevé/rapport, mur extérieur/autre abri et sorties M/G/O/U contrôlés par vraies commandes ECS ; poses ciblées préparées. `ui-cases-720/assertions.txt`, passes initiale et sans audio. |
| P10-04/06/07/08 | Couverture partielle : refus atomique du lot152+8+4, échec/reprise et perte de données, I/B/menu bloquent F, rapport sans O₂ de base. Toutes les variantes (placement, abri entourant une balise avant relevé, etc.) ne sont pas confirmées graphiquement. Restent ouvertes. |
| VIS-01–04 | Options/commandes et inventaire inspectés à720p/900p ; jour/crépuscules/nuit/tempête à12m, aides remappées et parcours avec rebasages. Pas toutes les variantes de pitch, panneaux et pentes. Les poses ciblées réinitialisent aussi les trois obstacles de diagnostic ; aucune nouvelle donnée de terrain mesurée. |
| SAVE-01 | Deux processus : relevé en cours et C/V restaurés, rapport payé une fois après retour préparé dans l’abri. `restart-load`, code0. |
| SAVE-02/03 | Rapport déjà payé rechargé dans le binaire d’archive, vraie touche V/F5 sans nouveau lot ; stock/progression/touches identiques. Deux profils/refus Broken dans la régression P9. Variantes progression corrompue/anciens formats/secours couvertes en CPU, pas toutes rejouées dans le renderer. |
| DIST-01/AUD-06 | Archive803 fichiers vérifiée et extraite ; vrais lancements depuis CWD extérieur, deux processus P9 et entrées natives X11. Les relevés/rapports P10 du parcours complet utilisent le diagnostic construit avec la même bibliothèque ; le parcours complet de création/relevé/rapport propre au binaire d’archive n’a pas été rejoué. |
| REG-01 | Boucle P9 de l’archive,116,40s+26,47s, deux codes0 et36captures inspectées : récoltes, stockage/transferts, construction et profils. |
| REG-02/03 | Jour/nuit, réserve/production, portes/recharge, streaming et transitions exécutés ; contrôles CPU conservés. Couverture visuelle ciblée, pas toutes les variantes. |
| AUD-01–04 | Onze motifs demandés et vrais AudioSink ; rappels faibles5,020s et critiques1,507s observés, options/volume/muet/reset testés. Sortie du jeu seule enregistrée :22050Hz,13,78s, crête0,0982, aucun échantillon écrêté. Ni microphone ni mix des autres applications. Confort/distinction subjective non établis ; variantes4/5/deux profils audio surtout en CPU. |
| AUD-05 | Absence de périphérique provoquée dans l’environnement ALSA du seul enfant : avertissement attendu,57,15s code0, pic5 lecteurs/0sink, expiration vérifiée, sans accumulation ni crash. Aucune configuration système modifiée. |
| PERF-01/02/05 | Références P8 en échec avant export :8,608s puis26,424s. Aucun benchmark comparatif valide ; arrêt demandé avant les versions suivantes. Ne pas relancer pour « finir ». |
| PERF-03/04 | Scénario64 équipements/audio actif-muet préparé mais non exécuté. RSS échantillonnée pendant validations, aucune attribution de coût ni mesure VRAM/staging complète. |
| Windows/cible | Non exécutés : aucun hôte Windows11 ou RTX4050 accessible. |

La limite des deux aides E/B fixes constatée le 15 octobre est corrigée dans
les sources le **16 octobre 2026** : distance de caisse et rappel du catalogue
utilisent les marqueurs des touches réaffectées. Diff contrôlé uniquement ; aucun
nouveau test, build ou lancement. L’archive du 15 octobre est antérieure à cette
retouche. Les cases ouvertes ci-dessus restent ouvertes, sans relance automatique.

Aucun jeu, benchmark ou enregistrement laissé actif. Les validations ont cessé ;
la mise à jour finale est documentaire, sans nouvelle exécution de tests.


## Contrôle rapide distinct — 16 octobre 2026

Sur demande explicite de lancer le jeu, un build dev optimisé et un lancement
normal confirment Mars/HUD/menu et les deux aidesK/Y corrigées. Aucun test Cargo,
benchmark ou trajet d’expédition répété. Le jeu est laissé au menu sur une partie
d’essai isolée. [Résultat](VALIDATION.md). Aucune case de campagne supplémentaire
cochée sur la base de ce seul démarrage.
