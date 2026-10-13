from pathlib import Path
import subprocess,os,time,json,re,hashlib,csv,statistics
root=Path.cwd(); out=root/'docs/global-validation'; binary=root/'target/release/solar_survival'
env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
env.update(RUST_LOG='info,solar_survival::player::surface=debug',MESA_VK_WSI_PRESENT_MODE='immediate')
results=[]
def run(name,command,variables,timeout):
 current=env|variables; start=time.monotonic()
 print('Lancement',name,flush=True)
 with (out/(name+'.txt')).open('w') as log:
  code=subprocess.run(command,env=current,cwd=root,stdout=log,stderr=subprocess.STDOUT,timeout=timeout).returncode
 content=(out/(name+'.txt')).read_text()
 record=dict(name=name,command=command,variables=variables,exit_code=code,seconds=time.monotonic()-start,diagnostics=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',content),completion=re.findall(r'GLOBAL terminé[^\n]*',content))
 results.append(record);(out/'graphics-results.json').write_text(json.dumps(results,indent=2)+'\n')
 print(name,code,record['completion'],flush=True)
 if code or record['diagnostics']:raise SystemExit('Examiner '+name)
run('global',[str(binary)],dict(SOLAR_MARS_GLOBAL='1',SOLAR_GLOBAL_VALIDATE_DIR=str(out/'global')),160)
for name,terrain in [('historical-before','historical'),('global-profile','global'),('historical-after','historical')]:
 run(name,[str(binary)],dict(SOLAR_PROFILE_SECONDS='42',SOLAR_PROFILE_GPU='0',SOLAR_PROFILE_VARIANT='baseline',SOLAR_PROFILE_TERRAIN=terrain,SOLAR_PROFILE_OUTPUT=str(out/(name+'.csv'))),100)
run('legacy-suite',['python3','scripts/validate_cube_suite.py','--output',str(out/'legacy'),'--mesa-immediate'],450)
metadata=json.loads((out/'environment.json').read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(binary)==metadata['binary_sha256']
assert all(sha(root/p)==digest for p,digest in metadata['source_sha256'].items())
summary={}
for name in ['historical-before','global-profile','historical-after']:
 values={}
 for row in csv.DictReader((out/(name+'.csv')).open()): values.setdefault((row['phase'],row['metric']),[]).append(float(row['value_ms']))
 summary[name]={}
 for (phase,metric),samples in values.items():
  if metric not in ['frame','cpu/navigation','cpu/rendering_sync']:continue
  samples.sort();n=len(samples)
  percentile=lambda p:samples[int((n-1)*p)]
  summary[name][phase+'/'+metric]=dict(count=n,mean_ms=statistics.mean(samples),median_ms=statistics.median(samples),p95_ms=percentile(.95),p99_ms=percentile(.99),max_ms=max(samples))
content=(out/'global.txt').read_text()
timings=[float(v) for v in re.findall(r'GLOBAL rebasage CPU ([\d.]+) ms',content)]
summary['global_rebase_cpu']=dict(count=len(timings),min_ms=min(timings),median_ms=statistics.median(timings),max_ms=max(timings))
summary['source_and_binary_hashes_unchanged']=True
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print('Toutes les campagnes terminées, empreintes inchangées',flush=True)
