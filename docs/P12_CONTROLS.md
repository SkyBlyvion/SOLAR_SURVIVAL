# P12 — ergonomie et commandes remappables

15 octobre 2026. Périmètre choisi par SkyBlyvion : ergonomie et commandes
remappables. L'utilisateur autorise ensuite la campagne complète en attente sur
ce poste, avec fenêtres, focus, souris et audio, après l'implémentation P12.
Aucune validation native Windows ou de la cible ne peut être déduite d'Ubuntu.

## Jouer

Options → **Commandes / changer les touches**. Les 26 commandes sont réparties
sur cinq pages de six lignes maximum. Cliquer une commande puis presser une
touche ; Échap annule la saisie. Une touche déjà affectée est refusée en nommant
la commande concernée, sans changer aucune affectation. Une touche réservée
est refusée ; la saisie reste active pour réessayer ou annuler. Les côtés gauche
et droit de Maj/Ctrl/Alt correspondent à une même affectation.

**Rétablir les commandes** restaure seulement les touches ; le bouton général
**Rétablir les valeurs par défaut** restaure aussi les autres options.
Échap revient de Commandes à Options, puis au menu principal. Les commandes de
jeu sont bloquées pendant la saisie ; une touche maintenue dans le menu doit
être relâchée avant de pouvoir agir en jeu.

Les noms désignent des **positions physiques**, pas des caractères saisis.
Les positions qui diffèrent sont montrées sous les deux noms QWERTY / AZERTY
(W / Z, A / Q, Q / A, Z / W, M / virgule). La saisie du nom d'aventure reste
fondée sur les caractères logiques de l'OS. Les aides du HUD, des bâtiments,
de construction, d'inventaire et d'expédition suivent les affectations.

Remappables : marcher/voler horizontalement, monter/sauter, descendre, course,
récolte/coffre, relevé/rapport/centrage, inventaire, construction, annulation,
rotation du placement, surface/retour, visite, vue globale, détachement,
reprise après échec, temps/pause, télémétrie/navigation, cible suivante et vitesse.
Les usages contextuels d'une commande gardent la même touche (F par défaut
pour relever, transmettre et centrer).

Fixes : clic droit, souris et molette, Échap, Entrée, flèches (placement et menus),
0–9, +/− (alias de vitesse), F5/F9 (profils) et F12 (capture de diagnostic).
Touches disponibles : lettres physiques, Espace, Maj/Ctrl/Alt, Tab, Pg↑/Pg↓,
F1–F4/F6–F8/F10–F11. Pas de combinaison, manette ou remappage de souris ajouté.

## Architecture et persistance

`controls.rs` traduit le clavier brut vers `GameplayKeys` une seule fois dans
GameSet::Input, après le menu et avant inventaire/caméra. Navigation spatiale,
contrôleur de surface, construction, inventaire, expédition et HUD lisent cette
même vue. Le menu et les champs de nom conservent le clavier brut. Les diagnostics
injectent leurs touches avant traduction ; le profiler historique publie ses
entrées canoniques après les contrôles comme auparavant.

Les textes contenant des marqueurs explicites `<key:KeyF>` sont résolus après
GameSet::Interface. Un petit cache conserve uniquement leurs modèles, pour
actualiser aussi les aides statiques ; les entités disparues sont retirées.
Aucun remplacement des lettres ordinaires du texte ou du nom des aventures.

`PlayerSettings` conserve les affectations par aventure. Écriture
**SOLAR_SURVIVAL 6**, lecture **4/5/6** ; les anciens formats compatibles reçoivent
les touches par défaut. Charger n'écrit pas le fichier ; la prochaine sauvegarde
écrit 6 avec le secours habituel. Le format stocke un nombre strict de commandes
et des identifiants explicites de touches, indépendants des discriminants Bevy.
Touches inconnues, doublons, compte incorrect et données supplémentaires sont
refusés avant publication. Formats 1/2/3 toujours refusés sans suppression.
Un ancien binaire P10/P11 ne lit pas le format 6.

Aucune dépendance ajoutée ; Rust stable 2024, Bevy =0.19.1 et Cargo.lock conservés.
Aucun changement de pose, de physique, d'horloge, d'astronomie ou d'ordre GameSet.
Les deltas souris restent intégrés sans facteur delta time.

## Vérifications

165 tests CPU/ECS réussis : vraie navigation spatiale/surface et pause avec
remappage, ancienne touche inactive, conflit transactionnel, modificateurs,
touche maintenue dans un menu, aides statiques/dynamiques, boutons/pages/saisie,
annulation/reset, sauvegarde 6 et lecture 4/5, données invalides sans mutation.
Les validations graphiques/audio et mesures sont consignées séparément dans
[VALIDATION](VALIDATION.md), [la checklist](TESTS_A_REALISER.md) et les dossiers
P12/campagne groupée. Les tests CPU ne valent pas inspection ni essai matériel.


## Clôture et limites après arrêt utilisateur

Menus inspectés à720p/900p, parcours graphique unique avec C/V, sauvegarde/reprise,
archive et backend audio réel/absent ont des preuves dans les notes de campagne.
**Ne plus lancer de validation ni répéter la passe d’expédition sans nouvelle
demande explicite.** Deux rappels secondaires de `ui/construction.rs` (distance
de caisse et rappel de catalogue) conservent encore E/B par défaut. Les autres
aides principales sont remappées ; ne pas annoncer une couverture exhaustive.
Comparaisons de performance et variantes restantes ouvertes, Windows/cible non
validés. Les dernières modifications après l’arrêt sont documentaires.
