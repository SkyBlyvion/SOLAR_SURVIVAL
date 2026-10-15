# P9 — base, inventaire et stockage

Cadrage du 15 octobre 2026, après P8 commité dans `ee87770`. **P9 reste à
implémenter.** L'utilisateur ajoute explicitement la gestion d'inventaire et le
stockage aux panneaux solaires, batteries, générateur/réserve d'oxygène, récolte
durable, interface et sauvegarde. Ces éléments font partie des critères de P9.

## Boucle attendue

**Explorer → récolter → transporter → déposer/reprendre → fabriquer/construire →
produire et stocker énergie/oxygène → préparer une expédition plus longue.**

Le joueur doit gérer ce qu'il emporte et ce qu'il conserve à la base. La première
installation doit être réalisable avec les moyens de départ : définir et tester
l'amorçage pour éviter une dépendance circulaire bloquant l'aventure.

## Périmètre fonctionnel

| Élément | Résultat à livrer |
|---|---|
| Panneaux solaires | Production utile variant avec l'ensoleillement et la poussière/tempête de P8 ; état et puissance lisibles |
| Batteries | Capacité finie, charge/décharge et réserve permettant de traverser la nuit ou une baisse de production |
| Générateur et réserve d'oxygène | Consommation électrique, production et stockage bornés ; recharge de combinaison reliée à cette réserve |
| Récolte durable | Sources accessibles au-delà des huit caisses initiales, selon une règle déterministe et persistée |
| Inventaire du joueur | Plusieurs types d'objets/ressources utiles, quantités, piles et capacité finie ; consulter et organiser ce qui est porté |
| Stockage de base | Coffres/conteneurs constructibles et accessibles à proximité, avec identité, capacité et contenu propres ; déposer et reprendre des objets |
| Interface et sauvegarde | Inventaire et conteneur consultables ensemble, commandes de transfert, coûts/disponibilités, énergie/oxygène et erreurs lisibles ; toute la progression conservée par aventure |

## Inventaire et conteneurs

L'état actuel P8 est un compteur d'alliage dans `Construction`, avec huit caisses
épuisables. Il ne fournit pas encore l'inventaire à piles ni les coffres demandés.
Examiner ce propriétaire et ses transactions avant de remplacer le compteur.

Pour la première implémentation, privilégier des emplacements contenant des piles
typées, avec limite par pile et nombre d'emplacements borné. Fixer les valeurs
lors du cadrage concret des recettes ; ne pas inventer une masse physique à partir
d'un nombre d'objets. Les types retenus doivent servir à une recette ou un usage
effectivement livré. Prévoir rangement/regroupement et transfert d'une quantité
choisie, d'une pile ou du maximum admissible, avec commandes explicites.

Un coffre appartient à la base, indépendamment du joueur, du patch et du proxy
de rendu. Il possède une identité stable, une pose corporelle DVec3/DQuat, une
capacité finie et un contenu autoritaire. Réutiliser le placement/admission et les
principes de contacts existants selon sa géométrie réelle. Sa consultation et les
transferts exigent une proximité définie ; fermer l'interface s'il devient
inaccessible. Un rebasage ou un déchargement visuel ne doit pas déplacer ou vider
le coffre.

Chaque mouvement d'objets doit préserver les quantités : vérifier disponibilité,
type de pile, capacité et accès avant de publier la transaction. Une demande
précise impossible est refusée sans débit ; une commande « maximum » calcule et
annonce la quantité effectivement transférée. Une récolte refusée pour inventaire
plein ne doit pas épuiser la source. Aucun dépassement, perte ou duplication lors
d'un clic répété, d'une annulation, d'un changement d'écran ou du chargement.

Rendre explicite la provenance des matériaux des recettes. Proposition initiale :
fabrication/construction depuis l'inventaire porté ; le joueur retire les objets
du coffre avant usage. Toute utilisation directe d'un stockage proche doit être
définie et validée comme une règle supplémentaire, sans ponction distante implicite.
Revalider et payer une seule fois à confirmation, jamais à la prévisualisation.

Le stockage d'objets, les batteries et la réserve d'oxygène ont des contenus et
unités distincts. Ne pas représenter une énergie ou une réserve respiratoire par
une pile d'objets sans définir une conversion et un usage réels.

## Énergie, oxygène et ressources

Définir puissances, capacités, rendements, coûts et débits avant d'équilibrer la
boucle. Pour l'électricité, employer des unités explicites et cohérentes (W et
J ou Wh avec conversion documentée). P8 fournit des conditions visuelles ; ses
couleurs, opacités et lux de présentation ne sont pas une puissance de panneau.
Documenter le modèle de production comme convention de gameplay, ainsi que les
arrêts pour batterie vide, réserve pleine ou énergie insuffisante.

L'oxygène P4 est exprimé en secondes d'autonomie de combinaison. Définir l'unité
de réserve de base et son lien avec cette autonomie avant de modifier la recharge.
Décrire le devenir du service actuellement gratuit de l'abri et les moyens de
démarrer ou de récupérer d'une panne. Ne pas revendiquer une chimie ou une pression
scientifique que le jeu ne calcule pas.

Définir une politique temporelle commune à la production/consommation : proposition
en secondes réelles comme la survie, P/T changeant les conditions solaires. Préciser
et tester menus, interfaces d'inventaire, espace et longues interruptions, sans
changer silencieusement les deux horloges existantes. La politique de progression
application fermée doit aussi être explicite ; P8 n'effectue aucun rattrapage.

Les nouvelles sources de récolte nécessitent des identités et règles de
disponibilité persistantes : rendement, épuisement ou renouvellement, placement
et chargement local bornés. Un retour spatial, R ou un changement de patch ne doit
pas réinitialiser les stocks/dépôts. Conserver par défaut le comportement actuel
de R, qui garde les ressources et bâtiments, tant qu'une autre règle n'est pas
demandée.

## Interface, persistance et architecture

Prévoir une interface lisible pour le contenu porté et celui du conteneur ouvert,
les capacités restantes, les quantités transférées et les motifs de refus. Montrer
production/consommation électrique, charge de batterie, stock d'oxygène et état
des équipements. Définir les commandes et l'effet des écrans sur la capture souris,
le déplacement et l'oxygène ; garder les instruments vitaux lisibles.

Sauvegarder inventaire/piles, identités/poses/contenus des conteneurs, équipements,
charges/réserves et état des ressources propres à l'aventure. Valider les types,
quantités, capacités et identités avant restauration ; annuler les transactions
et interfaces transitoires. Vérifier deux profils indépendants après fermeture.
Le format livré reste **2 jusqu'à l'implémentation de P9** ; choisir alors sa version
appropriée. Aucune compatibilité ancienne ni migration n'est requise pour le moment.

Séparer les données d'inventaire et les transactions de la preview de construction,
du rendu et de l'UI. Réutiliser bâtiments, admission et persistance lorsqu'ils
portent réellement le besoin ; créer un module uniquement avec sa fonctionnalité.
Conserver SI/f64, GameSet, contacts, assets partagés et budgets de P8. Pas de
génération mondiale de gisements ou de conteneurs à chaque frame.

## Critères de fin et contrôles

1. Réaliser la boucle complète depuis une aventure neuve : récolter, gérer la
   capacité portée, construire un coffre, déposer/reprendre, fabriquer et installer
   les équipements, produire/stocker, puis recharger la combinaison.
2. Observer l'effet jour/nuit et poussière/tempête sur la production, la décharge
   et la reprise après panne ; aucune réserve négative ou supérieure à sa capacité.
3. Tester inventaire plein, piles pleines, source insuffisante, transfert partiel
   explicite/maximal, coffre distant, commandes répétées et confirmation périmée ;
   quantités conservées et dépenses uniques.
4. Vérifier accès/contacts des coffres et équipements, rebasages, streaming,
   retours spatiaux, menu et R ; aucun déplacement de contenu ou réinitialisation.
5. Retrouver inventaire organisé, contenus de plusieurs coffres, charges/réserves
   et récoltes après redémarrage ; profils indépendants et fichier incohérent refusé
   sans mutation de la partie active.
6. Exécuter fmt, check, tests, Clippy et build release ; vérifier graphiquement la
   boucle et l'UI dans un créneau autorisé, mesurer les coûts ajoutés sur des scènes
   comparables, et documenter environnement, résultats et limites. Windows reste
   à valider nativement.

Les quantités, capacités, recettes et détails d'interaction restent à fixer pendant
l'implémentation. Ce document constitue le périmètre de travail et ses critères,
pas une annonce de fonctionnalités disponibles. P10 exploration/objectifs reste
ultérieur ; un réseau logistique automatique n'est pas nécessaire à cette boucle.
