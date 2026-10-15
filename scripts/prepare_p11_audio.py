#!/usr/bin/env python3
"""Synthèse originale offline des signaux P11 ; bibliothèque standard uniquement."""
import hashlib
import json
import math
from pathlib import Path
import struct
import wave

ROOT = Path(__file__).resolve().parent.parent / 'assets' / 'audio'
RATE = 22050
# (fréquence Hz, durée s, silence après s). Aucun enregistrement tiers.
MOTIFS = {
    'ui': [(880, .045, 0)],
    'refused': [(220, .10, .035), (165, .13, 0)],
    'harvest': [(523.25, .07, .02), (783.99, .12, 0)],
    'built': [(261.63, .10, .025), (392, .10, .025), (523.25, .20, 0)],
    'survey': [(659.25, .10, .025), (880, .10, .025), (987.77, .15, 0)],
    'report': [(523.25, .09, .02), (659.25, .09, .02), (783.99, .09, .02), (1046.5, .24, 0)],
    'refill': [(392, .13, .04), (587.33, .20, 0)],
    'low': [(440, .16, .09), (440, .16, 0)],
    'critical': [(880, .09, .07), (880, .09, .07), (880, .09, 0)],
    'failed': [(392, .18, .04), (293.66, .18, .04), (196, .35, 0)],
    'saved': [(659.25, .10, .03), (783.99, .18, 0)],
}

def main():
    ROOT.mkdir(parents=True, exist_ok=True)
    manifest = {}
    for name, notes in MOTIFS.items():
        samples = []
        for frequency, duration, silence in notes:
            count = round(duration * RATE)
            for i in range(count):
                t = i / RATE
                # Attaque/fin de 12 ms, deux partiels doux, crête <= 0,11.
                fade = min(1, i / (RATE * .012), (count - 1 - i) / (RATE * .012))
                envelope = .5 - .5 * math.cos(math.pi * fade)
                phase = 2 * math.pi * frequency * t
                sample = .11 * envelope * (math.sin(phase) + .18 * math.sin(2 * phase)) / 1.18
                samples.append(round(32767 * sample))
            samples.extend([0] * round(silence * RATE))
        path = ROOT / (name + '.wav')
        with wave.open(str(path), 'wb') as audio:
            audio.setparams((1, 2, RATE, 0, 'NONE', 'not compressed'))
            audio.writeframes(struct.pack('<' + 'h' * len(samples), *samples))
        manifest[path.name] = dict(bytes=path.stat().st_size, seconds=len(samples) / RATE,
                                   sha256=hashlib.sha256(path.read_bytes()).hexdigest())
    (ROOT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps(dict(files=len(manifest), bytes=sum(v['bytes'] for v in manifest.values()))))

if __name__ == '__main__':
    main()
