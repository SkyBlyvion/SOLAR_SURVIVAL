# P6 : profils et interface — 14 octobre 2026

## Périmètre corrigé

Le HEAD de départ est 22efa72 : P4/P5, LOD adaptatif et caméra unique existaient déjà. La première vérification P5, faite à partir de la passation périmée, n’était pas le développement demandé ; preuves conservées dans ../p5-recheck-2026-10-13. L’utilisateur a rappelé P6, puis précisé que la sauvegarde unique ne répondait pas à sa demande de plusieurs aventures. La passe à emplacement unique reste dans ../p6-validation, sans être présentée comme validation des profils. Aucun commit/push intermédiaire n’a été fait. NEXT_CHAT-before.md et PERSISTENCE-before.md préservent la documentation reçue avant cette correction.

Le périmètre actuel est concret : nom obligatoire à Nouvelle aventure, un répertoire par aventure, nouvelles progressions indépendantes et sélection à charger ; refonte du menu principal, catalogue de construction en jeu et HUD harmonisés. Palette sobre commune, boutons/texte alignés, centre de l’écran libre ; pas de nouvelle mécanique spéculative ni dépendance. L’apparence inspectée ne vaut pas une approbation esthétique de l’utilisateur.

## Tests progressifs et corrections

Les essais tests-initial/second/third et clippy-initial/second/third sont conservés. La première compilation du menu utilisait BorderColor.0, API incorrecte pour Bevy 0.19.1 ; remplacée par les champs/bords appropriés. Clippy a signalé la complexité de requêtes ECS ; filtres disjoints explicites et expectations locales motivées, pas de désactivation globale. Les tests de menu et construction ont été adaptés à B → catalogue → prévisualisation, puis confirmation distincte. La touche Entrée qui ouvre la prévisualisation ne peut pas la confirmer dans la même frame.

Deux tests de profils vérifient Alpha/Bravo à travers une application neuve, les stocks/abris/épuisements distincts, la réinitialisation, noms interdits/réservés, casse, Unicode et refus sans mutation. Le test UI saisit les noms avec les vrais messages KeyboardInput, crée deux aventures et restaure le profil sélectionné. Le catalogue teste blocage de mouvement/capture, oxygène toujours actif et annulation sans dépense. Les tests antérieurs vérifient les abris construits, contacts/portes/recharge et rebasages ; le test de rendu vérifie également qu’une nouvelle session vide retire les proxies de caisses.

build-ui-initial précède les derniers ajustements de hauteur/pagination ; ses sources ont évolué pendant la compilation. Il ne sert donc pas de référence finale. layout-check passe. build-responsive est le bilan final : six commandes réussies, **130 tests** (49 internes), release **152,440 s**, Cargo.toml/Cargo.lock inchangés. Les scénarios GPU commencent après cette compilation et se succèdent ; les empreintes de chaque campagne attestent la stabilité des sources Rust/tests/shaders/Cargo et du binaire.

## Parcours graphique final

Dans graphics-responsive : deux processus distincts, 1440×900. Save **48,373 s**, load **54,377 s**, codes 0, seize captures chacun, aucun ERROR/WARN/panic. Alpha récolte deux caisses et construit le même abri via le nouveau catalogue ; un rebasage et une recharge sont vérifiés. Bravo démarre sans stock/abri/site ancien, puis garde six unités et un seul abri. Le choix du profil ramène les deux abris d’Alpha et son stock nul. Le second processus charge Bravo, vérifie ses données, ouvre le catalogue avec stock insuffisant, revient à Alpha, sauvegarde/recharge l’espace et l’échec, utilise R, puis refuse Broken tout en gardant Alpha actif.

Les poses proches des caisses, le seuil de rebasage, les réserves, le JD et les options sont préparés explicitement par le diagnostic. Récolte, catalogue, construction, menus, sélection et sauvegardes passent par les systèmes de jeu. Les clics sont des Interaction ECS et la saisie utilise KeyboardInput ; pas de validation de périphériques OS. Ces parcours ne prouvent pas une marche continue de trois minutes ou un benchmark. La campagne historique de contacts/marche reste référencée dans VALIDATION ; elle n’est pas répétée sans modification de la physique.

## Archive et limites

package.json conserve archive/empreintes/chemin d’extraction. L’archive Linux finale est extraite dans un dossier /tmp neuf, hors checkout ; les **731 fichiers** du manifeste sont vérifiés, le binaire correspond au release. packaged-720 rejoue les deux processus avec assets adjacents, dossier de travail extrait et fenêtre 1280×720. Le packaging inclut lanceurs, licences et manifeste ; l’archive binaire reste dans dist ignoré par Git.

Un seul profil actif réside dans le World. Pas de chargement de toutes les planètes des profils, cloud, renommage/suppression de profil, saisie riche/IME, synchronisation concurrente de processus ou garantie universelle de coupure électrique. Quitter ne sauvegarde pas automatiquement ; changer/créer sauvegarde d’abord la partie quittée. Secours .bak par profil, checksum de corruption accidentelle. Pas de nouveau benchmark/RAM/VRAM/DPI/suspension/P2 complet. Windows natif et cible RTX 4050/Core i5 restent non validés ; le zip produit uniquement par le script sur Windows n’est pas une preuve Windows depuis Linux.

Bilan de l’archive : save **48,358 s**, load **54,431 s**, codes 0, 32 captures, aucun ERROR/WARN/panic ; concordance source/binaire vraie. Les deux campagnes finales totalisent **64 captures inspectées sur seize planches**, avec originaux accueil/saisie/catalogue/liste/erreur également vus. Aucun chevauchement ou texte coupé constaté dans ces poses. Les originaux et leur SHA-256 sont énumérés dans chaque inspection/verified.json. Aucun changement du jeu après build-responsive ; seulement scripts de preuves/packaging et documentation.
