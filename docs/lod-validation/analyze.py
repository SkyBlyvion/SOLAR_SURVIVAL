"""Résumer les campagnes sans modifier les preuves brutes."""
from pathlib import Path
import csv,json,re,statistics
root=Path(__file__).resolve().parent
summary={}
for folder in ['baseline-global','baseline-p5','final-fixed','final-adaptive','final-p5']:
 p=root/folder
 if not (p/'summary.csv').exists():continue
 frames=[]
 for row in csv.DictReader((p/'summary.csv').open()):
  if row['metric']=='frame' or row['phase'] in ('streaming','placement','confirm') and row['metric'].startswith('cpu/'):
   frames.append(row)
 transactions=[]; uploads=[]
 for path in p.glob('*.log'):
  content=path.read_text()
  transactions.extend(dict(run=path.stem,total_ms=float(m[0]),constructions=int(m[1]),details=m[2],changes=int(m[3]),geometry_ms=float(m[4]),index_ms=float(m[5])) for m in re.findall(r'GLOBAL rebasage CPU ([\d.]+) ms, patches \d+, constructions (\d+), LOD (.+?), changements (\d+), géométrie ([\d.]+) ms, index ([\d.]+) ms',content))
  uploads.extend(dict(run=path.stem,cpu_ms=float(m[0]),meshes=int(m[1]),details=m[2]) for m in re.findall(r'GLOBAL meshes sync CPU ([\d.]+) ms, uploads (\d+), LOD ([^\n]+)',content))
 summary[folder]=dict(metrics=frames,transactions=transactions,uploads=uploads)
(root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
for folder,data in summary.items():
 print('\n'+folder)
 for row in data['metrics']:
  if row['metric']=='frame' and row['phase'] in ('rest','contact','streaming','placement','built_rest'):
   print(row['run'],row['phase'],'mean',round(float(row['mean_ms']),3),'p95',round(float(row['p95_ms']),3),'max',round(float(row['max_ms']),3))
 for row in data['transactions']:print('transaction',row)
