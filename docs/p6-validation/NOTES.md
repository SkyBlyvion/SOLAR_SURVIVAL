# Passe P6 — 13 octobre 2026

Départ HEAD 22efa72, uniquement NEXT_CHAT modifié par l'utilisateur. Il contenait la reprise ancienne P4/P5, alors que Git/README/ROADMAP et la passation du HEAD indiquaient P6. La première vérification P5 est conservée dans ../p5-recheck-2026-10-13. L'utilisateur a corrigé explicitement ce mauvais cadrage ; aucun commit/push n'avait alors été effectué. Les paragraphes documentaires provisoires de reprise P5 ont été retirés. Le travail effectif de cette passe est P6.

## Implémentation et essais progressifs

Aucune dépendance ajoutée, Cargo.toml/Cargo.lock inchangés, Bevy exactement 0.19.1/Rust stable 2024. Instantané explicite, format UTF-8 version 1, contrôle d'intégrité FNV-1a, limites 64 Kio/32 abris/8 dépôts, validation puis reconstruction et publication des ressources. Écriture exclusive/synchronisée, renommages et secours, chargement sans restauration partielle, contrôles/menu/paramètres et assets de distribution.

Les cinq tests de persistance ciblés et le test menu ont réussi progressivement. Le premier check du nouveau diagnostic a signalé E0308 : comparaison de String avec une liste de &str, corrigée par as_str. Le premier contrôle all-targets a identifié la nouvelle propriété de GameConfig manquante dans un constructeur de test de profilage, corrigée. Clippy-initial.txt conserve aussi l'initialisation suivie de réaffectations, remplacée par une initialisation directe. Clippy-second.txt passe. Aucun de ces contrôles intermédiaires n'est une validation finale.

Build initial complet : six commandes réussies, 126 tests dont 46 internes, release 147,882 s. Journaux et durées dans build-initial. Les tests antérieurs terrain, contacts, survie, construction, horloges et caméra restent réussis. Le test de rendu P5 est renforcé pour vérifier la suppression des anciens proxies lors du remplacement des dépôts par une session vide.

## Deux processus et archive

Parcours graphique initial : save 18,346 s/code 0/huit captures, puis load 30,367 s/code 0/douze captures. Aucun ERROR/WARN/panic, hashes inchangés pendant la campagne. La première partie récolte deux caisses par E et construit par B/Entrée, puis prépare explicitement un rebasage et le départ dans l'abri ; réserve 60 s et changement de JD/options documentés. La seconde démarre réellement dans une application neuve, charge par F9, vérifie les propriétaires corporels et les paramètres, reprend la recharge et teste sauvegarde/chargement espace/échec, R, erreur de fichier. Aucune injection d'alliage ou installation directe dans ce parcours.

Les commandes sont ECS ; --focus-x11 active le client sans simuler clavier/souris. Les temps de parcours et rebasage sous captures ne sont pas des benchmarks. Le menu est inspecté avec chemin long et messages lisibles. Les deux entités caméra 270v0 et 271v0 appartiennent à deux processus différents ; chaque processus conserve sa propre identité pendant tous les chargements.

Packaging Linux natif : archive 23 322 401 octets, assets/police/shaders/licences/lanceur et manifest. Extraite dans /tmp/solar-survival-p6-t4qow3dp, hors checkout ; package-test.json conserve chemins et empreintes. Le script de packaging n'effectue aucune compilation croisée et ne crée aucun résultat Windows depuis Linux.

## Limites

Pas d'autosave à Quitter, un emplacement plus secours, pas de réseau/cloud/migration de schéma ni de rattrapage offline. Pas de garantie de coupure matérielle universelle, absence de fsync portable du dossier ; écritures concurrentes de plusieurs instances exclues. Ce n'est pas un benchmark, aucune nouvelle RAM/VRAM exhaustive, campagne P2/suspension/DPI ou vérification de périphériques OS. Windows 11 natif et RTX 4050/Core i5 restent non validés.


## Dernier cas de restauration et validations finales

Après les premiers parcours réussis et le test de l'archive initiale, la revue a identifié un cas réel : les diagnostics historiques désactivent la création des assets de survie à l'installation des plugins ; accepter un chargement de partie normale dans ces modes aurait réactivé des données sans leur rendu. Restore refuse désormais ces destinations avant mutation. Un septième test dédié vérifie oxygène désactivé et mode cube, avec maintien de la session. Les campagnes initiales restent conservées ; le binaire final est reconstruit et les parcours ciblés sont répétés, sans rejouer les campagnes mondiales/P2 anciennes.

Le script de packaging écrit le lanceur Windows en octets CRLF et les chemins zip avec séparateurs d'archive ; ceci ne constitue pas un essai natif Windows. Le programme et ses dépendances restent inchangés hors de ce dernier garde de chargement.
