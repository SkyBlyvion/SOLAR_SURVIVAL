# P8 — campagne du 15 octobre 2026

Départ réel `496e26a7269dcdc5c61705ddb0e93d805ed6f6f4`, dépôt initial propre.
Périmètre utilisateur : P8 relief/horizons, étendu à une atmosphère martienne
visuelle dynamique. Pas de compatibilité avec les anciennes sauvegardes.
Accord explicite reçu dans le chat avant tout lancement prenant le focus.
Aucun écran virtuel, périphérique simulé ou reprise de focus en boucle ajouté.

## Implémentation et vérifications numériques

Contrat dans `../../MARS_ENVIRONMENT.md`. Bruit artistique mondial f64, même
géométrie pour rendu/contact, couronne raccordée sans jupe aux patches locaux.
Même LOD 32/16/8 et contacts fins, nouvelles normales du relief, caméra unique.
Météo dérivée du JD/de la position/graine, sans état accumulé ni texture nouvelle.
Format de sauvegarde 2, refus explicite du format 1, fichiers anciens intacts.

`final-checks.json` : fmt, check all-targets, **139 tests**, Clippy -D warnings,
probe CPU et build release réussis. Build release 161,732 s. Après correction
WGSL seule : build courant confirmé en 0,20 s (`shader-rebuild.log`), même Rust
et même exécutable ; le shader est chargé depuis les assets au lancement.

Les premiers tests (`tests-first.log`, `tests-second.log`) ont révélé une assertion
qui supposait encore un sol mondial plat, puis un écart aller-retour de repère
2,1965 nm dépassant la tolérance initiale de 2 nm d'un test d'abri. L'assertion
plate est remplacée par une vérification d'extrema sur le relief ; le calcul
`chart` soustrait maintenant l'origine corporelle avant rotation inverse.
La tolérance de 2 nm est conservée et passe (`survival-targeted.log`). Les deux
avertissements Clippy initiaux sont corrigés ; journaux antérieurs conservés.

Probe CPU **dev optimisé, hors GPU** : 200 rebasages de 64,1 m sur un trajet de
12,8 km. Sphère : moyenne 0,7525 / p95 0,9744 / max 1,1275 ms ; relief+couronne :
2,2795 / 5,1449 / 5,6155 ms. Ce n'est ni un coût de frame complet ni un benchmark
release : les normales/uploads de rendu ne sont pas inclus. Couronne maximale
6400 sommets / 11520 triangles ; cache fin ≤13 patches, couronne unique réutilisée
si le patch central ne change pas. Le test topologique couvre les six faces,
coutures et coins ; les autres tests conservent marches/courses/sauts/rebasages,
construction, survie et profils.

## Inspection graphique

`sky-first` : code 0, 66,397 s, 22 captures, aucun diagnostic et empreintes
inchangées. Quelques vues et la première planche inspectées : les bandes des
nuages étaient trop géométriques. Cette campagne reste une version intermédiaire.
Le motif sinus initial est remplacé par deux échelles de bruit de valeur 3D
à hash entier, sans texture/volume GPU. Pas de modification du relief, du
calendrier ou des directions astronomiques pour cette correction.

`sky-final` : code 0, 66,411 s, 22 captures, aucun diagnostic et empreintes
inchangées. Les **22 captures sont inspectées sur quatre planches** dans
`sky-final/inspection`, ainsi que l'original ciel nuageux `sky-06.png`.
Lever et coucher : halo bleu localisé autour du Soleil ; nuit : étoiles et sol
sombre ; tempête : ciel plus uniforme, Soleil fortement atténué et visibilité
réduite. Sol, abri, course/rebasage, aller-retour orbital, recharge, menu et
autres faces/coin vus. Les marqueurs UI demeurent des annotations, perceptibles
même lorsque la poussière éteint presque le disque. Les trois boîtes de diagnostic
restent présentes, sans être présentées comme roches naturelles.

Le relief est doux ; le paysage s'étend à plusieurs kilomètres, sans montagnes
mesurées ni morphing de LOD. La galerie ne valide pas un transit de tempête
observé intégralement en temps réel : elle prépare des dates naturelles distinctes.
Continuité, événements rares, pause/reproductibilité et invariance de rebasage
sont également vérifiés numériquement. Pas d'approbation esthétique utilisateur.

## Mesures et distribution

Référence P7 :59,494s ; P8 :59,461 /59,495s sans recorder,59,506s GPU séparé,
quatre codes 0, aucun diagnostic, empreintes inchangées. Comparaison et limites
mises à jour dans PERFORMANCE.md : surcoût moyen 1,59–1,90 ms/frame selon phase,
rebasage observé≤5,725ms et synchronisation meshes≤9,361ms (retours inclus).
Ressources processus et spans GPU conservés, aucune mesure de VRAM totale.

Archive finale `dist/solar-survival-ubuntu-p8.tar.gz`,36 907 688 octets,747 fichiers
vérifiés après extraction. SHA256 `96c28d2c71351278b5fa36207627c668b0730a6c271d098e7264a8c3d102f003`.
Binaire `b27bdbd4207a0685132897ddac2b11c416d1a6df91aaca7ba5f8569efd00d5c3`.
Deux processus de profils depuis ce binaire/ces assets extraits :48,414+54,388s,
codes 0,32 captures inspectées sur six planches, empreintes inchangées. Alpha garde
ses deux abris/stock0 après construction et rebasage ; Bravo son stock6 et un
seul abri ; chargement/refus Broken, catalogue, espace, échec/R et recharge vérifiés.
Les profils sont neufs au format 2 ; ce n'est pas un essai de migration P6.
Le répertoire de travail du helper était le checkout, les assets venaient de
l'archive à côté de son exécutable ; racine des sauvegardes explicite dans les preuves.

Un smoke supplémentaire a ensuite été lancé avec exécutable, assets **et CWD**
hors checkout. Premier essai code 0 mais erreurs IO de captures : le vieux diagnostic
smoke écrit dans `docs/` sans créer ce dossier, absent d'une archive normale.
Préparation corrigée : création explicite de ce dossier de diagnostic, puis
relance8,418s,code 0,aucun diagnostic,11 captures et audit13textures GPU réussi ;
747 fichiers distribués toujours identiques. Aucun code de jeu modifié pour cet
essai ; le lancement normal ne demande pas ce dossier. Les logs/JSON des deux
essais sont conservés, les11 captures de relance copiées dans les preuves mais
non comptées dans les54 captures P8/profils inspectées. Cette limite du diagnostic
historique est documentée ; l'archive n'a pas été retouchée pour la masquer.
La référence P7 utilise son exécutable **et ses assets d'archive propres**, et non
un ancien binaire chargeant le nouveau shader. Les métadonnées du lanceur
identifient les hashes du binaire/runtime ; les hashes de checkout décrivent le
checkout courant pendant cette mesure, pas les sources historiques du binaire P7.

## Limites permanentes

Ubuntu de développement : i7-6700K / 32 Go / GTX 1060 3 Go, NVK Mesa25.2.8,
1440×900/MSAA4 pour ciel/mesures ; profils prévus à 1280×720. Windows 11 natif,
RTX 4050 6 Go et objectif 1080p60 non validés ; CPU cible indiqué « i5 12700K »
à confirmer. VRAM totale, allocations pilote et staging non disponibles sur
ce pilote ; RSS/HWM processus et budget utile des texels sont distincts.
Pas de climat scientifique, force de vent, pression locale, ombres de nuage
ou de relief, changement de survie, énergie P9 ou objectifs P10 ajoutés.
