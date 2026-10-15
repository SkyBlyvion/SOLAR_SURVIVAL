# État courant — P7 clôturé sur Ubuntu le 15 octobre 2026

La reprise a terminé les contrôles manquants, corrigé le raccord visible de Jupiter et vérifié une nouvelle archive. [Bilan exact, preuves et limites](2026-10-15/NOTES.md). Windows/RTX4050/1080p60 et VRAM totale/staging restent ouverts en transversal.

---

## État reçu le 14 octobre, conservé comme historique

# P7 — état de livraison du 14 octobre 2026

Chef de projet et orchestrateur : **SkyBlyvion**. Départ f748faf/main identique à origin. Implémentation : surface artistique, panneaux/caisses/plaque et huit cartes planétaires, treize textures partagées/mipmaps offline. 40,168 Mio, physique et Bevy/Cargo/profils conservés. [Crédits](../../assets/textures/CREDITS.md), [masters/prompt](../../art/p7-masters/PROMPT.md).

Six contrôles réussis, 134 tests : build-all-planets-final/results.json. Archive dist/solar-survival-ubuntu-p7.tar.gz, 36 875 096 octets, 747 fichiers vérifiés hors checkout, même release ; package.json. Lancement de cette archive non validé. Concordance finale sources/assets/binaire vérifiée dans final-source-concordance.json.

34 vues inspectées du premier périmètre au shader albedo dans inspection-first, avant ajout des six autres planètes. Conserver before-sky/before-construction, sky-first et sky-albedo et les essais de performance : pas de remplacement des preuves historiques. Tests Cargo antérieurs en erreur suivis des réparations réussies, conservés. Les fixtures P6 livrées Alpha et Bravo sont restaurées par tests/p7_profile_compatibility.rs.

Deux campagnes finales P5 sans captures ne vont pas au bout : final-performance et final-performance-retry. Les assertions échouent sur un parcours dépendant de captured/focus. L’utilisateur confirme que les campagnes prennent souris/focus pendant qu’il travaille à côté : conflit avéré, causalité de chaque assertion non isolée. Ne pas présenter ces runs comme succès ni performance finale. L’essai Xvfb virtual-before échoue avant présentation Vulkan (pas de DRI3), conservé comme diagnostic ; aucun fonctionnement d’écran virtuel/adaptation des contrôles retenu. À sa demande, arrêter ces essais et finir rapidement la livraison existante.

**Encore à faire :** inspecter les 25 vues six planètes/crédits avec validate_sky.py --planets, refaire la mesure finale comparable sans capture et sans travail concurrent/focus perdu, validation graphique archive/profils. Ne pas lancer spontanément --focus-x11 sur son bureau. Vérifier VRAM totale/staging par outil du processus jeu disponible sur cible ; Windows 11 natif et RTX 4050/CPU exact restent non validés. P7 est intégré mais sa validation finale reste ouverte, P8 non implémenté.
