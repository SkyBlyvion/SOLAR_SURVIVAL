# Signaux audio Solar Survival — P11

Onze motifs originaux composés et synthétisés pour Solar Survival le 15 octobre
2026, avec l'assistance de Codex, sous la direction de SkyBlyvion. Licence MIT du
projet (voir ../../LICENSE). Aucun enregistrement, échantillon, musique ou voix tiers.

Sources : `scripts/prepare_p11_audio.py`, bibliothèque standard Python. WAV PCM
mono, 16 bits, 22 050 Hz ; enveloppes douces et deux partiels sinusoïdaux. Motifs
et fréquences artistiques, aucune donnée acoustique martienne ni alarme médicale.
Ce sont des retours de combinaison/interface, sans propagation dans l'atmosphère.

`ui`, `refused`, `harvest`, `built`, `survey`, `report`, `refill`, `low`, `critical`,
`failed`, `saved`. Empreintes, tailles et durées dans `manifest.json`.
Regénération offline : `python3 scripts/prepare_p11_audio.py` ; aucun besoin de Python
pour jouer. Les WAV exportés sont versionnés et partagés par toutes les occurrences.
