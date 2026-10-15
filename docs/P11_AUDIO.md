> Depuis P12 : écriture **SOLAR_SURVIVAL 6**, lecture **4/5/6** ; commandes par aventure, valeurs par défaut pour4/5. Formats1/2/3 refusés sans suppression. [Contrat courant](P12_CONTROLS.md). Les formats cités dans les sections historiques ci-dessous décrivent leurs livraisons propres.

# P11 — audio et alertes de jeu

15 octobre 2026. Périmètre choisi par SkyBlyvion après P10 : **audio et alertes de
jeu**. Première passe de retours utiles ; écoute, confort et inspection des options
à réaliser dans la campagne groupée. Aucun test prenant le bureau ni sortie sonore
automatique pendant ce développement.

## Jouer

Options propose **Volume sonore** : 0, 25, 50, 75, 100 %, et **Son : activé / muet**.
Valeur initiale et réinitialisation : 50 %, son activé. Muet conserve le volume
choisi ; on peut modifier ce volume en restant muet. Les réglages s'appliquent aux
sons en cours et aux suivants. À zéro/muet, les voix sont supprimées, les demandes
consommées sans lecture ; aucun empilement à rejouer ensuite. Le HUD vital et les
messages de refus restent présents, indépendamment du son.

| Signal | Déclencheur |
|---|---|
| Interface | Boutons/raccourcis du menu principal et options, sans son au survol |
| Refus | Récolte/construction/relevé/rapport/transfert ou opération de profil refusés |
| Récolte / transfert | Lot réellement récupéré ou transfert d'inventaire réussi |
| Construction | Installation admise et payée, une seule fois |
| Relevé / rapport | Données effectivement acquises / récompense publiée |
| Sauvegarde / profil | Sauvegarde, création ou chargement réussis ; pas au simple rafraîchissement de liste |
| Recharge O₂ | Début observé d'une augmentation effective dans un abri, sans boucle permanente |
| O₂ faible | ≤30 s : signal immédiat puis toutes les 5 s actives |
| O₂ critique | ≤10 s : signal distinct immédiat puis toutes les 1,5 s actives |
| Échec | Passage réel à l'épuisement en surface active, une seule fois |

Débits et seuils sont des conventions de gameplay, sans signification médicale.
La recharge observée supprime les rappels de faible réserve pendant son effet.
Les alertes vitales suivent Time<Real>, après la survie, sans multiplication P/T.
Une longue frame produit au plus un rappel, sans rejouer les intervalles sautés.
Menu principal et espace suspendent ces alertes et coupent les signaux de gameplay
en cours ; inventaire et catalogue laissent la survie et ses alertes actives.
Un échec chargé depuis disque n'est pas rejoué comme une nouvelle mort. Les
transitoires sont supprimés à la restauration/changement de profil. Une réserve
faible déclenche son avertissement lors de la reprise active.

Les sons sont des signaux de **combinaison/interface non spatialisés** : aucune
propagation acoustique martienne, position astronomique ou Transform requis.
Pas de musique, voix, ambiance, pas, son de machine continu ou nouveau modèle
physique. Cette première passe ne remplace aucun message visuel par le seul son.

## Architecture et ressources

`audio.rs` contient l'événement SoundCue, l'observation O₂, la banque de clips et
le mixage. Les commandes déclenchent les événements après le résultat réel de
leur transaction. Aucun examen de texte traduit ni déduction depuis les proxies.
L'observation vitale lit Oxygen/Explorer/MenuState après Navigation, dans Rendering.
La lecture est préparée en PostUpdate ; l'ordre des cinq GameSet est conservé.
Les commandes restent fonctionnelles dans les tests sans plugin audio.

**Onze WAV mono PCM 16 bits à 22 050 Hz, 187 020 octets au total**, chargés une fois,
partagés par handle. Source originale offline : `scripts/prepare_p11_audio.py`,
bibliothèque standard ; exports versionnés, crédits MIT et manifeste dans
`assets/audio/`. Pas de synthèse, écriture de fichier ou génération GPU par frame.
Crête de chaque fichier ≤0,11 ; confort et niveau perçu restent à écouter.

Au plus **huit voix** et onze demandes distinctes en attente. Déduplication par
frame et intervalle minimal 150 ms par motif. Priorité échec → critique → faible
→ actions → interface ; une nouvelle alerte peut interrompre une voix moins
prioritaire. PlaybackSettings::DESPAWN plus expiration de secours après 2 s,
y compris sans périphérique/sink : aucune accumulation de lecteurs en attente.
Un clip encore absent des Assets n'est pas mis en lecture différée. Les voix et
timers ne sont pas sauvegardés. Ces plafonds ne mesurent ni RAM totale ni CPU réel.

## Dépendances et plateformes

Bevy reste **exactement =0.19.1**. Activation de ses fonctionnalités `bevy_audio`
et `wav` ; aucune bibliothèque audio directe supplémentaire. Cargo.lock ajoute
notamment bevy_audio 0.19.1, rodio 0.22.2, cpal 0.17.3, hound 3.5.1 et le backend
ALSA sous Linux. Contrat/API vérifiés dans les sources de ces versions et la
[documentation Bevy 0.19.1](https://docs.rs/bevy/0.19.1/bevy/audio/index.html).
Les dépendances système du backend restent hors logique centrale.

Compilation Ubuntu standard : `sudo apt install pkg-config libasound2-dev` en
plus des prérequis précédents. Exécution : bibliothèque système `libasound.so.2`,
présente sur le poste, et périphérique audio configuré. Windows : backend natif
CPAL, sans ALSA ; build/lecture natifs encore à vérifier. Si aucun périphérique
n'est trouvé à l'initialisation, Bevy journalise un avertissement et poursuit sans
son ; le jeu ne tente pas de changer le périphérique ni de le reconnecter à chaud.

Sur ce poste les outils de développement ne sont pas installés et sudo demande
un mot de passe. Les paquets Ubuntu ont donc été téléchargés/extraits sous `/tmp`
pour la compilation uniquement, sans modifier `/usr`. Les notes de validation
conservent le procédé et l'environnement. Les lanceurs/archives n'en dépendent pas.
Tous les scénarios automatisés existants désactivent le backend AudioPlugin dans
main et le mixage de gameplay ; les tests CPU n'installent aucun backend matériel.

## Persistance

Écriture **SOLAR_SURVIVAL 5**, nouvelle ligne `audio index_volume muet` validée
avant toute publication ; index 0–4 et booléen strict. Le schéma P10 est autrement
conservé. **Lecture du format 4 conservée**, avec volume 50 % et son activé ; la
prochaine écriture (F5 ou sauvegarde avant changement d’aventure) écrit 5 et garde le secours habituel. Le chargement
ne réécrit pas immédiatement le fichier. Formats 1/2/3 toujours refusés et laissés
intacts. Une aventure écrite en 5 n'est pas lisible par l'ancien binaire P10.
Le volume suit les réglages de chaque aventure ; une nouvelle aventure utilise
les préférences courantes comme les autres options. Pas de fichier global ajouté.

## Vérifications

[Résultats réels](VALIDATION.md), [notes P11](p11-validation/2026-10-15/NOTES.md),
[campagne reportée](TESTS_A_REALISER.md). Décodage par le vrai décodeur Bevy sans
périphérique, signaux issus de transactions ECS, seuils/temporisations, priorités,
plafonds, muet, expiration sans sink, suppression espace/menu/restauration,
options et persistance 4/5 couverts par les contrôles sans bureau. Une lecture
numérique ne vaut pas écoute. Aucun fonctionnement audio Windows ni niveau
sonore final n'est annoncé sans essai natif et écoute autorisée.
