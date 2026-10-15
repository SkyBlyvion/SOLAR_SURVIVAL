# P11 — audio, alertes et contrôles sans bureau — 15 octobre 2026

Périmètre explicitement choisi par SkyBlyvion : **Audio et alertes de jeu**.
Au départ `fec803a` (P9 fini), P10 et sa documentation étaient déjà présents dans
le workspace non commité ; ils ont été conservés. Aucun commit demandé ou créé.
Lecture préalable AGENTS/README/roadmap/passation/astronomie, P10/P9/persistance,
survie/construction, checklist et derniers résultats de validation/performance.

## Livré

[Contrat P11](../../P11_AUDIO.md) : onze sons originaux de combinaison/interface,
récolte/transfert, construction, relevé/rapport, refus, menus et profils. Signaux
liés aux résultats réels, sans analyse de textes. Alertes O₂ ≤30s/5s actives et
≤10s/1,5s actives, recharge observée et échec unique. Pas de cadence orbitale,
rafale après longue frame ou recharge fictive au secours R. Messages visuels gardés.

Options volume0/25/50/75/100 %, défaut50 %, muet mémorisant la sélection ;
application aux voix en cours et nettoyage au silence. Banque partagée onzeWAV,
**187020octets**, PCM16bits mono22050Hz. Génération offline standard Python,
crédits originaux MIT ; pas de ressource tiers ni nouvelle texture/mesh/passeGPU.
Huit voix maximum, onze demandes distinctes, délai150ms/motif, alertes prioritaires
avec préemption et suppression des voix après2s même sans sink. Menu/espace coupent
les sons de gameplay, reprise sans stock de sons ; profil chargé nettoie les transitoires.

Bevy **=0.19.1** conservé. Fonctionnalités `bevy_audio` et `wav` concrètement
utilisées ; Cargo.lock ajoute les dépendances compatibles dont bevy_audio0.19.1,
rodio0.22.2,cpal0.17.3,hound3.5.1/ALSA sur Linux. Pas de logique OS dans le jeu.
Sources locales/API et documentation de la version vérifiées avant implémentation.
Aucun changement d'astronomie/terrain/physique/horloges/GameSet, aucun Avian.

Persistance : écrit5, lit4/5 ; nouvelle ligne audio validée avant publication.
P10/4 a50 %/son activé, conservation de son état et pas de réécriture au simple
load. F5 ou sauvegarde avant changement d’aventure écrit5, secours habituel. Formats1/2/3 refusés sans suppression.
Le binaire P10 ne lit pas le format5. Voix et timers jamais sérialisés.

## Environnement de compilation

[Relevé](environment.json) : Ubuntu Linux, Ruststable1.99.0, édition2024,
i7-6700K/32Go. Session X11 disponible, **aucun jeu ni renderer/audio matériel,
fenêtre, focus, capture, écoute ou benchmark lancé**. Tests MinimalPlugins sans
AudioPlugin/AudioOutput ; décodage CPU avec le vrai décodeur Bevy/rodio.

Le poste n'avait ni pkg-config ni les fichiers de développement ALSA. Tentative
`sudo -n apt-get install -y pkg-config libasound2-dev` refusée : mot de passe requis.
Aucune installation système effectuée, aucun mot de passe demandé/enregistré.
Solution locale sans privilège : `apt-get download pkgconf-bin libpkgconf3
libasound2-dev`, puis `dpkg-deb -x` de chaque paquet dans
`/tmp/solar-p11-build-deps/root`. Paquets Ubuntu noble : pkgconf1.8.1-2build1,
libasound2-dev1.2.11-1ubuntu0.3, version runtime déjà installée identique.

Compilation exécutée après `source /tmp/solar-p11-build-deps/env.sh` :

```sh
export PKG_CONFIG=/tmp/solar-p11-build-deps/root/usr/bin/pkgconf
export LD_LIBRARY_PATH=/tmp/solar-p11-build-deps/root/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
export PKG_CONFIG_SYSROOT_DIR=/tmp/solar-p11-build-deps/root
export PKG_CONFIG_LIBDIR=/tmp/solar-p11-build-deps/root/usr/lib/x86_64-linux-gnu/pkgconfig
```

Dans ce sysroot, `usr/lib/x86_64-linux-gnu/libasound.so` est fourni par le paquet
et pointe vers `libasound.so.2.0.0`. Ajouter localement un lien portant **ce nom
exact** vers `/usr/lib/x86_64-linux-gnu/libasound.so.2.0.0` pour résoudre le linker.
Un lien `.so.2` seul ne résout pas cette cible. Ce montage temporaire sert
uniquement à compiler, n'est pas distribué et peut disparaître. Sur une machine
Ubuntu configurée normalement : installer `pkg-config libasound2-dev` ; runtime
`libasound.so.2` requis pour exécuter. Windows utilise son backend CPAL natif.

## Résultats sans bureau

[Commandes et codes](checks.json), [version Bevy](bevy-version.txt).

- `cargo fmt --check` : code0.
- `cargo check --all-targets --locked` : code0.
- `cargo test --locked` : code0, **159 tests réussis**.
- `cargo clippy --all-targets --locked -- -D warnings` : code0.
- `cargo build --release --locked` : code0, **7min27s**, recompilation des composants
  affectés par les nouvelles fonctionnalités comprise ; [journal](release.txt).
- Packaging natif Ubuntu et vérification intégrale après extraction : codes0, sans
  lancer le jeu. Liens documentaires, scripts Python et diff contrôlés.

Six nouveaux tests : décodage des onze fichiers (format/durée/échantillons bornés,
silence aux extrémités, volume utile non nul), logique d'alertes/temps, mixeur/
priorités/plafonds/expiration sans sink, muet/menu/espace/missing clips/reset,
vrais boutons Options, snapshot5/lecture4/valeurs audio invalides. Les tests de
construction et des trois trajets P10 observent aussi les événements émis par les
vrais systèmes : deux récoltes/un bâtiment et trois relevés/trois rapports,
refus présents et aucun double succès. Certains états sont préparés comme dans
les tests existants ; les trajets P10 restent au sol avec visée ECS automatique.
Ce n'est pas une partie humaine ni un test de périphérique.

Échecs intermédiaires conservés et corrigés :
[check initial](check-initial.txt), AudioSink doit être emprunté mutable pour régler
son volume ; [premier test](tests-initial.txt), détection de modification de Window
via `Ref<Window>` nécessaire ; [second test](tests-second.txt), cible de symlink
ALSA locale manquante au linker ; [Clippy initial](clippy-initial.txt), deux
assertions de tests remplacées par `contains`. Aucun refus de test masqué ni
paramètre de gameplay assoupli. Les journaux finaux remplacent ces essais.

## Archive préparée, sans lancement

`dist/solar-survival-ubuntu-p11.tar.gz` : **37773347 octets**,
**803 fichiers** du manifeste contrôlés après extraction hors checkout.
SHA256 archive : `284f45eb0a0ffd2dbaf6d4768ce6454ebcae0134596b6c929ee6c8e9ce345477`.
SHA256 binaire : `9fccb1e750171d7b95db90fd2a50ac041310ea8dad48e854d574eb17f7b25cf3`.
Extraction : `/tmp/solar-survival-p11-2026-10-15-p8gxdns3/solar-survival-ubuntu-p11`.

[Packaging](package.json), [vérification](archive-verification.json),
[bibliothèques runtime](runtime-libraries.txt). Binaire identique au release,
WAV/crédits/manifeste conformes et LIRE-MOI format5/lecture4/P11 vérifié. Sources
et assets inchangés pendant la clôture, [empreintes](source-sha256.json).
`ldd` résout toutes les bibliothèques dont `libasound.so.2` depuis le système ;
aucune dépendance runtime vers pkgconf ou le sysroot temporaire. Cette inspection
n'exécute ni main, ni renderer, ni backend sonore. **Archive non lancée**, aucune
validation réelle du chargement de ses assets ou du périphérique à l'exécution.

## Limites et suite

**Aucune écoute ni validation graphique/audio matérielle P11.** Décoder des
échantillons n'établit ni confort ni bon choix de périphérique. Aucun essai réel
d'absence de périphérique/reconnexion ; la poursuite silencieuse est celle du
code Bevy et le bornage sans sink est testé. Pas de reconnexion à chaud ajoutée.
Options compactées sur petite fenêtre mais aspect/accessibilité à inspecter à
1280×720/1440×900. Pas de mesure CPU/audio/GPU, RAM/VRAM globale ou gain FPS.

La [campagne groupée](../../TESTS_A_REALISER.md) ajoute AUD-01–07/PERF-05.
Créneau utilisateur explicite requis ; conserver les cas P9/P10 ouverts. Les
profiler/diagnostics existants désactivent AudioPlugin et les sons pour ne pas
ouvrir un périphérique ni changer leurs scènes ; ils ne prouvent pas le coût
d'une lecture audio active. Écoute Ubuntu et Windows, archive réellement lancée,
RTX4050/CPU exact/1080p60 et VRAM totale/staging restent ouverts. Aucune approbation
sonore/esthétique utilisateur. Aucun jeu/benchmark/audio laissé actif.
