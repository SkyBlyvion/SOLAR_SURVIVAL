"""Échantillonnage Linux opt-in du processus, sans capture/GPU recorder.
RSS/HWM processus ; compteurs DRM uniquement si le pilote les publie.
Les valeurs absentes restent null, jamais zéro présumé. Pas de mesure de staging isolé.
"""
from pathlib import Path
import json
import re
import threading
import time

class ResourceMonitor:
    def __init__(self, pid, output):
        self.pid = pid
        self.output = Path(output)
        self.samples = []
        self.start = time.monotonic()
        self.stop = threading.Event()
        self.thread = threading.Thread(target=self.run, daemon=True)
        self.thread.start()

    def run(self):
        root = Path('/proc') / str(self.pid)
        while not self.stop.is_set():
            record = dict(seconds=time.monotonic()-self.start, rss_kib=None, hwm_kib=None, drm={})
            try:
                status=(root/'status').read_text()
                for field,key in [('VmRSS','rss_kib'),('VmHWM','hwm_kib')]:
                    match=re.search(r'^'+field+r':\s+(\d+) kB',status,re.M)
                    if match:record[key]=int(match.group(1))
                clients={}
                for info in (root/'fdinfo').glob('*'):
                    try:
                        text=info.read_text()
                        client=re.search(r'^drm-client-id:\s*(.*)',text,re.M)
                        if client:
                            clients[client.group(1)] = dict(re.findall(r'^(drm-(?:memory|resident|shared|purgeable)-[^:]+):\s+(\d+)\s+KiB',text,re.M))
                    except OSError:pass
                for values in clients.values():
                    for key,value in values.items():record['drm'][key]=record['drm'].get(key,0)+int(value)
            except OSError:pass
            self.samples.append(record)
            self.stop.wait(.25)

    def finish(self):
        self.stop.set();self.thread.join()
        summary=dict(samples=self.samples,peak_rss_kib=max((s['rss_kib'] for s in self.samples if s['rss_kib'] is not None),default=None),
                     process_hwm_kib=max((s['hwm_kib'] for s in self.samples if s['hwm_kib'] is not None),default=None),
                     drm_available=any(s['drm'] for s in self.samples),interval_seconds=.25,
                     limitations='RSS processus pas allocation textures seule. HWM noyau et échantillonnage. VRAM/staging absents si pilote non instrumenté. Linux uniquement.')
        self.output.write_text(json.dumps(summary,indent=2)+'\n')
