# P13 — outils, construction et première production

Cadrage révisé le **16 octobre 2026**, **non implémenté**. Référence de direction
donnée par l’utilisateur : un jeu comparable dans son principe à *Planet Crafter*,
avec plusieurs matériaux à récolter, des bâtiments et des machines productives.
Demandes explicites : barre rapide d’outils, catalogue enrichi, machines posables
à l’intérieur et à l’extérieur. Ce document est la liste de travail courante.

## Direction et premier résultat jouable

**Récolter → fabriquer → construire → alimenter → produire → améliorer → explorer
plus loin.** La base et sa production portent la progression. Les expéditions
approvisionnent cette base et donnent accès à de nouvelles ressources/recettes.
L’ancien scénario proposé de liaison de secours est retiré comme objectif central :
il pourra devenir un objectif secondaire, sans imposer une fin après trois sorties.

Premier résultat P13 : sélectionner un outil dans la barre rapide, récolter
plusieurs matériaux, construire un atelier, y poser une machine, en alimenter une
autre à l’extérieur et récupérer une production qui finance une amélioration.
Les machines doivent être utilisables et reconnaissables dans le vrai jeu.

Les quantités ci-dessous sont un **périmètre initial de conception**, à équilibrer,
pas un inventaire déjà implémenté. Aujourd’hui : deux objets (alliage/circuit),
six recettes, pas de barre rapide, pas de fabrication de matériaux par machine.
La production énergie/O₂ P9 existe et doit être réutilisée.

## Lots d’exécution P13

| Ordre | Livrable visible | Dépendance et fin du lot |
|---|---|---|
| **A — outils et inventaire** | Barre rapide, sélection, outil utilisé, matériaux différenciés à récolter | Inventaire/UI/commandes et sauvegarde livrés ensemble ; sélectionner puis récolter une ressource dans une partie normale |
| **B — construire et aménager** | Catalogue enrichi, atelier, fondations et machines installées dedans/dehors | Réutilise A ; support intérieur et collisions corrects, déplacement/démantèlement sans perte ; accès aux machines conservé |
| **C — production et progression** | Recettes, machines alimentées, entrées/sorties, premier extracteur et amélioration d’autonomie | Réutilise A/B et énergie P9 ; produire un composant, l’utiliser pour agrandir/améliorer, sauvegarder et reprendre |

Chaque lot reste jouable et se valide en fin d’ensemble cohérent. P13 est une
étape conséquente : les lots évitent une livraison monolithique, sans multiplier
les sous-phases de documentation ou les campagnes identiques.

## A — outils, ressources et inventaire

- [ ] Ajouter une **barre rapide de huit cases**, visible pendant le jeu.
- [ ] Afficher case sélectionnée, icône/nom de l’outil, raccourci et quantité si utile.
- [ ] Sélection par touches1–8 et molette en aventure ; rendre ces actions remappables.
- [ ] Réserver la sélection numérique des planètes au parcours libre spatial ;
      éviter les conflits avec les raccourcis P12 et la vitesse de vol.
- [ ] Affecter/désaffecter un objet depuis l’inventaire ; la barre référence les
      vrais objets, sans ajouter huit stocks gratuits ni dupliquer les quantités.
- [ ] Effacer/résoudre une référence si l’objet est consommé, déplacé ou retiré.
- [ ] Agrandir et rendre lisible l’inventaire : départ proposé **24 emplacements**,
      piles/capacités explicites, barre rapide distincte du rangement.
- [ ] Introduire un outil de récolte et un outil de construction avec une action
      réelle ; prévoir leur équipement de départ pour éviter un amorçage impossible.
- [ ] Donner un retour visible/sonore à l’équipement et à l’utilisation ; un modèle
      simple en main peut suffire au premier lot, s’il se lit correctement.
- [ ] Bloquer utilisation/roulette de jeu pendant menus, inventaire et catalogue ;
      clics UI et activation de l’outil ne doivent pas se déclencher ensemble.
- [ ] Commencer avec cinq ressources brutes : **minerai de fer, minerai de titane,
      silice, cobalt, glace**. Ces dépôts sont fictifs/artistiques, sans carte
      géologique réelle revendiquée.
- [ ] Différencier aspect, nom, icône et usages ; afficher la ressource visée,
      portée et place nécessaire avant récolte.
- [ ] Disperser des dépôts locaux persistants ; premières ressources accessibles,
      certaines plus éloignées. Retirer les distributeurs voisins toutes les30s
      de la progression principale, tout en conservant les diagnostics historiques.
- [ ] Une récolte refusée ne vide pas le dépôt ; les stocks épuisés ne renaissent
      pas au streaming, au chargement ou à la sortie spatiale.
- [ ] Préparer des réserves/gisements suffisants pour construire et se relever
      après un échec, sans farm infini par retourR ni verrou de recette circulaire.

## B — bâtiments, catalogue et aménagement

- [ ] Réorganiser B en catégories **Bâtiments / Stockage / Énergie / Survie /
      Production**, avec coût, fonction, alimentation et état de déblocage.
- [ ] Conserver abri, coffre, panneau, batterie, générateurO₂ et réservoir ; adapter
      leurs recettes aux nouveaux matériaux au lieu de tout payer en alliage/circuits.
- [ ] Ajouter des **fondations** stables et un **module atelier** assez spacieux
      pour aménager plusieurs postes ; silhouettes et accès différents de l’abri.
- [ ] Ajouter **établi**, **four de traitement** et **extracteur** au catalogue,
      avec effets réels livrés au plus tard au lotC. Ne pas laisser ces recettes
      annoncées comme terminées si elles ne construisent que des décors.
- [ ] Améliorer le placement à la visée : distance, rotation, fantôme vert/rouge,
      support choisi et motif de refus lisibles ; garder les contrôles existants utiles.
- [ ] Prendre en charge **terrain, fondation et plancher intérieur** comme supports.
- [ ] Remplacer, pour l’admission des machines, l’exclusion actuelle du volume
      complet de l’abri par les vrais solides : murs, plancher, plafond, montants.
      Le plancher porte la machine ; il ne la fait pas rejeter comme une collision.
- [ ] Orienter la machine dans le repère de son support en f64 ; conserver son
      placement au rebasage, au changement de LOD et à la reprise de sauvegarde.
- [ ] Vérifier toute l’emprise, la hauteur libre et les murs ; empêcher pose dans
      le vide, sous le sol, dans un plafond, à travers une paroi ou dans le joueur.
- [ ] Garder une porte et l’accès d’interaction praticables ; annoncer les règles
      spécifiques (extracteur sur gisement, panneau recevant le soleil).
- [ ] Autoriser les équipements compatibles sur un sol intérieur **et** extérieur,
      sans les limiter arbitrairement à une seule zone.
- [ ] Vérifier aussi la pose d’un bâtiment autour d’objets déjà présents ; ne pas
      résoudre l’intérieur en désactivant les contrôles d’intersection.
- [ ] Partager géométrie de rendu, collision et surfaces constructibles ; éviter
      des boîtes invisibles qui interdisent des espaces visuellement libres.
- [ ] Ajouter déplacement/démantèlement avec prévisualisation, contenu préservé et
      remboursement annoncé ; refuser atomiquement si le stockage ne peut recevoir.
- [ ] Empêcher démontage d’un support occupé tant que ses machines n’ont pas été
      retirées ; conserver identité/stock lors d’un déplacement valide.
- [ ] Aucune création d’énergie, d’O₂ ou d’objets en construisant/démontant en boucle.

## C — fabrication, machines et progression

- [ ] Fournir quatre premiers produits utiles : **alliage, plaques de titane,
      circuits, eau**. Définir recettes et quantités ensemble ; ce sont des
      conventions de gameplay simplifiées, pas des procédés industriels mesurés.
- [ ] L’établi permet la fabrication manuelle initiale (pièces/circuits/équipement)
      et une première amélioration d’autonomie ; présenter coût et effet avant paiement.
- [ ] Le four transforme les minerais, avec recette sélectionnée, alimentation,
      stock d’entrée et stock de sortie. Une sortie est utilisable par l’inventaire.
- [ ] L’extracteur récupère automatiquement la ressource du gisement ciblé,
      consomme de l’électricité et stocke réellement les objets produits.
- [ ] Donner une utilité à la glace/eau dès P13, par une recette de productionO₂ ;
      préserver un amorçage de secours accessible, sans imposer une nouvelle barre
      de soif pour justifier artificiellement la ressource.
- [ ] Afficher **En marche / Sans courant / Entrée vide / Sortie pleine / Arrêtée**,
      progression du cycle, consommation et production dans l’interface de machine.
- [ ] Ajouter démarrage/arrêt et sélection de recette ; gérer un changement de
      recette en cours sans perte ou duplication d’un lot déjà engagé.
- [ ] Stock plein : production suspendue avant consommation ; entrée manquante ou
      alimentation insuffisante : état clair, reprise sans lot gratuit.
- [ ] Réutiliser l’électricité P9, afficher portée et budget ; pour P13, garder
      un réseau local borné explicite plutôt qu’inventer des câbles sans gameplay.
- [ ] Définir une priorité stable en cas de pénurie (survie, puis ateliers) et
      afficher pourquoi une machine est coupée ; batterie vide ne signifie pas
      destruction de matériel.
- [ ] Faire avancer les productions en secondes réelles, indépendamment de P/T.
      Définir le comportement de longue frame, menu et espace avec les horloges P9.
- [ ] Suspendre la production quand l’application est fermée pour cette première
      version ; ne pas ajouter implicitement une simulation hors ligne.
- [ ] Faire progresser le joueur : récolte manuelle → raffinage → première machine
      → extraction régulière → autonomie accrue → ressource ou lieu plus éloigné.
- [ ] L’objectif du palier est une base autonome et productive ; montrer les
      déblocages et leur utilité. Le plaisir ne se mesure pas au nombre de recettes.
- [ ] Rééquilibrer ensemble coûts, débits, énergie, capacité et distance pour que
      construire une machine change le jeu sans longue attente obligatoire.

## À intégrer à chaque lot

- [ ] Sauvegarder les nouvelles données du lot : objets/outils/raccourcis,
      gisements, supports/poses, recettes, stocks, cycles et déblocages concernés.
- [ ] Définir le traitement des profils4/5/6 avant de changer le format ; aucune
      conversion silencieuse, écrasement ou suppression des anciennes aventures.
- [ ] Démarrer la nouvelle aventure sur Mars ; garder visite libre/diagnostics
      accessibles séparément, sans téléportation de cargaison dans la progression.
- [ ] Rendre récolte, machines et catalogue lisibles à720p ; conserver remappage,
      barrières UI et alertes vitales. Ajouter des raccourcis utiles, pas des menus
      qui exigent de manipuler une machine à chaque unité produite.
- [ ] L’échec conserve la base et les équipements acquis, mais a un coût annoncé
      sur la cargaison non déposée ; ressources critiques récupérables à nouveau,
      aucun blocage définitif ni génération gratuite par répétition du secours.
- [ ] Réutiliser meshes/matériaux, donner des formes distinctes aux machines,
      charger les dépôts proches seulement ; aucun monde entier généré par frame.
- [ ] Garder Rust2024/Bevy exactement0.19.1, Ubuntu/Windows, SI/f64 et GameSet.
      Ne pas modifier le modèle astronomique pour ajouter de la production.
- [ ] Vérifier transactions, support intérieur, inventaires et reprise avec des
      cas ciblés ; regrouper fmt/check/test/release requis en fin de lot significatif.
      Un nouveau défaut justifie un contrôle ciblé, pas une nouvelle campagne complète.
- [ ] Inspecter le lot dans le vrai jeu selon le créneau autorisé ; ne pas répéter
      les trois anciennes expéditions ni annoncer du plaisir de jeu validé par les tests.

## Après P13 : liste de progression

- [ ] **P14 — habitat et fabrication avancée :** modules connectables, portes,
      aménagement, établis spécialisés, assembleur automatique, équipements
      améliorés et recettes avec plusieurs usages concurrents des matériaux.
- [ ] **P15 — progression de transformation planétaire :** définir l’objectif de
      longue durée, premiers producteurs et indicateurs ; effets visibles graduels
      et déblocages. La terraformation est une piste de conception inspirée de la
      référence donnée, pas encore une simulation livrée ni une exigence détaillée
      explicitement définie par l’utilisateur. Distinguer indices de jeu et physique.
- [ ] **P16 — exploration et rover :** lieux distincts, ressources régionales,
      carte/découvertes, transport, avant-postes et raison de quitter la base.
- [ ] **P17 — logistique et gestion de base :** alimentation entre installations,
      transferts automatisés choisis, priorités, maintenance/météo et optimisation
      de production ; ajouter chaque réseau lorsqu’il produit un choix utile.
- [ ] **P18 — alpha martienne :** contenu, rythme, prise en main, visuels, audio,
      équilibre, reprise et distribution Ubuntu/Windows sur matériel cible.
- [ ] **Ensuite — progression spatiale :** premier vol utile et second monde ;
      préserver le système solaire physique existant sans confondre visite caméra
      et voyage jouable. Pas de calendrier de livraison promis.

## Ce qui clôt réellement P13

Une partie neuve permet d’utiliser la barre rapide, récolter les cinq ressources,
construire/aménager l’atelier, poser des machines dedans et dehors, produire des
objets sous contrainte d’énergie, agrandir la base avec cette production et
reprendre le tout après fermeture. Déplacer/démonter ne duplique rien. Les
machines servent réellement la progression ; leur intérêt et leur équilibre
restent à observer en jouant. Aucun de ces critères n’est acquis par ce cadrage.
