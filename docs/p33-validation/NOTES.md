# Audit des essais intermédiaires P3.3

12 octobre 2026, Ubuntu. Les journaux sont conservés séparément des résultats finaux.

- `tests-first.txt` : échec du critère de largeur centrale après réduction proportionnelle des deux demi-largeurs ; correction du rectangle initial avant certification. Warnings de mut/import inutiles corrigés.
- `tests-second.txt` et `tests-third.txt` : refus numérique d'une empreinte exactement à la borne du nouveau rectangle, après addition/soustraction de 0,3 m. Tolérance de comparaison de 1 nm ajoutée dans la réserve géométrique de 1 m.
- `tests-fourth.txt` : le retour de marche de quatre secondes n'atteignait pas le seuil −120 m. Seuil ramené à −110 m, avec environ 3,12 m d'hystérésis et tests anti-oscillation.
- `tests-fifth.txt` : deux assertions du nouveau test comparaient mal la projection verticale d'un joueur à 20 m et la distance par frame à 144 FPS. Deux cartes inclinées projettent vers des points différents ; comparaison du rayon du support et de l'écart vertical. À 144 FPS, un pas fixe de 1/120 s peut dépasser le delta d'une frame ; borne fondée sur le nombre maximal de sous-pas. Conservation des pieds/yeux/visée/vitesse inchangée et maintenue par les assertions.
- `tests-sixth.txt` : 76 tests réussis avant l'ajout des limites d'admission du fournisseur et des dernières corrections du diagnostic.
- `*-before-clippy-fix` : fmt/check/all-targets/tests réussis, puis Clippy refusant un `if` imbriqué ; condition fusionnée sans exemption.
- `before-presentation-fix/` : première compilation/campagne complète avant correction de la télémétrie et de la couleur du patch conservé. Le compteur hérité `crossings` de P3.2 y compte les changements de signe de coordonnées locales lors des migrations ; ce nombre **n'est pas un franchissement physique de raccord P3.3**. Le compteur `migrations` reste distinct, avec huit migrations de marche et une en l'air. La couleur basée sur l'emplacement était changée lors du remplacement ; elle dépend désormais de la parité de la clé X. Captures intermédiaires réellement inspectées à taille originale : migration 03 et 06 seulement. Les autres restent conservées sans revendication d'inspection exhaustive.

- Après les deux parcours finaux de 88 s (codes 0), le lanceur de campagne a terminé avec code **143** pendant la fin du premier essai historique. Dix captures présentes, aucune ligne ERROR/WARN/panic, mais aucun code de sortie du jeu enregistré ; cause indéterminée. Aucun processus du jeu restant lors du contrôle. Journal/captures/planches non inspectées sous `historical-launcher-143*`, résumé dans `launcher-interruption.json`. La régression historique finale est relancée seule, sans modifier le binaire.

Les codes/temps des vérifications finales sont dans `build-results.json`, les lancements finaux dans `graphics-results.json`, les empreintes sources/binaire dans `environment.json`. Une assertion échouée ou un ancien build n'est pas présenté comme une validation finale.
