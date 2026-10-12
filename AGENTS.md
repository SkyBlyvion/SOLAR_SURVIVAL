# Conventions Solar Survival

- Développer le jeu en Rust stable, édition 2024. Bevy est épinglé **exactement à 0.19.1** ; aucune API 0.20 ni changement de version sans instruction explicite de l'utilisateur.
- Lire le README et docs/ASTRONOMY.md avant de modifier les calculs physiques ou le rendu astronomique.
- Au début d'un chat de développement, lire aussi docs/ROADMAP.md et docs/NEXT_CHAT.md. Suivre l'étape courante ; justifier toute réorganisation par un besoin utilisateur ou des résultats observés.
- À la fin de chaque chat de développement, actualiser l'état et les critères de la roadmap, consigner les validations/limites, et réécrire docs/NEXT_CHAT.md avec un prompt autonome : état réel, objectif suivant, périmètre, contraintes et vérifications. Fournir son lien dans la réponse finale.
- Maintenir Ubuntu Linux et Windows 11. Pas de dépendance propre à un OS dans la logique centrale. X11 est une option du backend Bevy, pas une API de gameplay.
- Positions, distances, temps orbital et transformations globales en f64 (`DVec3`, `DQuat`). SI : mètres, secondes, kilogrammes. Préserver l'échelle physique 1:1.
- Ne jamais alimenter directement un Transform f32 avec une position astronomique. Soustraire l'origine flottante en f64, puis convertir explicitement vers le proxy de rendu. Les proxies ne doivent jamais modifier les données physiques.
- Séparer simulation (`universe`), rendu (`rendering`), joueur/navigation, physique locale, UI, survie et terrain. Ne pas grossir main.rs ; n'ajouter un module que lorsqu'il porte une fonctionnalité réelle.
- Préserver l'ordre des GameSet. L'horloge orbitale utilise Time<Real>, indépendant des contrôles et de l'UI. Les déplacements utilisent delta time ; les deltas souris sont déjà intégrés et ne sont pas multipliés par delta time.
- Les visites attachent la caméra au repère translatoire d'un corps ; documenter toute évolution vers un repère tournant.
- Documenter sources et unités des nouveaux paramètres astronomiques ; distinguer valeurs mesurées, moyennes, exactes et conventions de gameplay. Ne pas présenter un bruit procédural comme une donnée scientifique.
- Vérifier la compatibilité de chaque dépendance avant ajout. Éviter les dépendances inutiles ; Avian n'est ajouté que pour une vraie fonctionnalité de physique terrain.
- Ne pas générer une planète entière, un mesh ou des ressources GPU à chaque frame. Réutiliser les assets, limiter le LOD et charger seulement les zones locales nécessaires.
- Tester les changements significatifs avec `cargo fmt --check`, `cargo check`, `cargo test`, `cargo build --release`. Garder Cargo.lock versionné pour cette application. Vérifier le lancement graphique lorsqu'une session est disponible ; distinguer compilation et validation visuelle.
- Ne pas prétendre avoir validé Windows depuis Linux. Consigner l'environnement, les résultats et les limitations dans docs/VALIDATION.md.
- Documenter les changements d'architecture. Préférer une petite fonction opérationnelle à une abstraction spéculative. Ne pas ajouter de modules vides pour des fonctionnalités futures.
