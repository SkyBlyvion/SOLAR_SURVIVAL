# P10 — implémentation et contrôles sans bureau — 15 octobre 2026

Départ : `fec803a — P9 fini`, checkout propre. Lecture des conventions, README,
roadmap/passation, contrats astronomie, terrain, P8, survie/construction, base,
profils, direction visuelle et derniers résultats/limites P9. Passation reçue
avec HEAD P9 historique09f7027 ; Git réel utilisé. Aucun commit demandé ni créé.
Bevy exactement=0.19.1, Cargo.toml/Cargo.lock inchangés, aucune dépendance nouvelle.

## Périmètre réalisé

Trois balises locales séquentielles à≈120/240/360m, F à3m pour relever puis F
**dans le premier abri** pour transmettre et recevoir une récompense portée
unique8+4/12+6/16+8 alliage/circuits. Lot entier ou refus intact. Données perdues
si échec ou sortie spatiale, découverte connue conservée ; R ne paie rien.
HUD objectif/récompense/cap/distance/base et estimation minimale de retour avec
marge20s. Poses corporelles f64 dérivées du premier abri et du relief artistique
P8, indépendantes du streaming et du calendrier. Radio courte portée sans segment
pour ne pas bloquer un site entouré d'une construction. Pas de nouvelle autonomie
portable, d'O₂ distant, d'inventaire de balise ni de logistique abstraite.

Modules concrets `expedition.rs`, `rendering/expedition.rs`, `ui/expedition.rs`.
Deux cubes/balise débloquée dans600m, six proxies maximum, cube existant et trois
matériaux constants ; aucun nouvel asset image/mesh/passe par frame. Différences
corporelles et rotation locales en f64 avant conversion explicite f32. P9+P10≤312
proxies hors terrain/ciel/obstacles. Horloges, GameSet, physique et terrain conservés.
[Contrat complet](../../P10_EXPLORATION.md).

Format4 sans migration des1/2/3 ni suppression, suivant la convention précédente.
Découvertes, rapports payés et données en cours par profil, poses déterministes
reconstruites. Validation avant publication ; checksum/secours/256Kio conservés.
Nouveaux profils requis ; une archive P9 ne prouve pas P10.

## Contrôles exécutés

[Codes et commandes](checks.json), [environnement](environment.json).
Ubuntu Linux, Ruststable1.99.0, édition2024, i7-6700K. Session X11 disponible mais
**aucune fenêtre OS, aucun jeu/benchmark/GPU lancé**. Tests MinimalPlugins avec
entité Window en mémoire, aucun Winit/renderer ni périphérique OS.

- `cargo fmt --check` : code0.
- `cargo check --all-targets --locked` : code0.
- `cargo test --locked` : code0, **153 tests réussis**, dont six nouveaux P10
  (cinq scénarios d'intégration et un test des proxies). [Journal](tests.txt).
- `cargo clippy --all-targets --locked -- -D warnings` : code0, aucune alerte.
- `cargo build --release --locked` : code0, 2min48s. [Journal](release.txt).
- Packaging natif Linux et extraction/empreintes : succès sans exécution du jeu.

P10 couvre trois **trajets complets au sol** avec vraies touches ECS, visée orientée
par le test, entrée/sortie par porte et réserve finie/recharge. O₂ minimal avant
porte84,6s ; ce chiffre appartient au trajet automatique en course sur ce site,
pas à toute expédition humaine. Récompenses uniques, F répété hors base et après
rapport, porté plein où seul le premier type tiendrait, portée/ordre des sites,
quatre sorties M/G/O/U, échec/R, menus/inventaire/catalogue/P/T, cap/pitch/rebasage.
Deux profils dans une nouvelle App : relevé en cours, pose/identité déterministes,
récompense déjà reçue non dupliquée, progression isolée et incohérences refusées.
Ancien profil3 réel et fixtures P6 refusés. Proxies : entités conservées au rebasage,
max six, précision<20µm dans le cas testé, déchargement espace/profil vide ; aucun
mesh supplémentaire. Les régressions existantes restent réussies.

Certains cas ciblés préparent les poses ; seul le test des trois allers-retours
marche réellement tout le parcours sans repositionnement. Aucun test graphique
n'a été discrètement remplacé par un écran virtuel.

Premiers essais conservés : [check initial](check-initial.txt), [tests initiaux](tests-initial.txt).
Quatre scénarios P10 passaient ; la boucle bloquait en quittant l'abri en diagonale
contre un jambage. Le test sort maintenant par la porte puis contourne le mur ;
**aucune collision modifiée ni tolérance physique élargie**. Une affectation de
variable inutilisée dans le test retirée. [Deuxième essai ciblé](tests-p10-second.txt)
réussi, puis suite complète et contrôles finaux ci-dessus.

## Archive préparée, sans lancement

`dist/solar-survival-ubuntu-p10.tar.gz` : **37,025,421 octets**,
747 fichiers du manifeste vérifiés après extraction hors checkout.
SHA256 archive : `ebcd8a6dd0e47e483019d3191c27629b03fca37e1ee71130f027fb1fa11640ed`.
SHA256 binaire : `f27d64e774b7926ae9952628c1527b2b20fbdda4cbc4994418a60f0af729f78d`.
Extraction : `/tmp/solar-survival-p10-2026-10-15-uk2lbnxm/solar-survival-ubuntu-p10`.
Binaire identique au build release ; LIRE-MOI format4 et commandesP10 contrôlés.
[Packaging](package.json), [vérification](archive-verification.json).
**L'archive n'a pas été exécutée** : intégrité/distribution des fichiers seulement.

## Limites et suite

Aucune capture P10, inspection esthétique, vérification effective des shaders/UI
à l'écran ou comparaison de performance P9→P10. Le code des proxies est testé sans
GPU, pas leur apparence ni leur transition visuelle en streaming. Six proxies et
textures conservées sont des bornes structurelles, pas une mesure VRAM/pilote.
Mâts simples, trois missions finies, radio sans occlusion, estimation de retour
sans itinéraire. Relevés/terrain sont artistiques, pas scientifiques.

**Consigne sans focus maintenue** : aucune campagne visible automatique ; attendre
un créneau explicitement demandé pour inspecter jeu/archive, jour/nuit/tempête,
petite/grande fenêtre, transitions et profils. Le travail de code utile reste
possible sans ce créneau. Aucun jeu/benchmark laissé actif. Windows11 natif,
RTX4050 6Go, CPU cible exact,1080p60 et total VRAM/staging restent non validés.
