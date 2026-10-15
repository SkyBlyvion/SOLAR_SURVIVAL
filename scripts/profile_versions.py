#!/usr/bin/env python3
"""Deux passes courtes P5 par version archivée, sans trajet d'expédition ni captures."""
import argparse,csv,hashlib,json,os,re,statistics,time
from pathlib import Path
from focus_diagnostic import run_focused

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--version',action='append',required=True,help='nom=chemin_du_binaire')
    p.add_argument('--output',type=Path,required=True);p.add_argument('--allow-window',required=True,action='store_true')
    a=p.parse_args();out=a.output.resolve();out.mkdir(parents=True)
    versions={k:Path(v).resolve() for k,v in (item.split('=',1) for item in a.version)}
    fingerprints={}
    for name,binary in versions.items():
        manifest=binary.parent/'manifest.json'
        if manifest.is_file():
            m=json.loads(manifest.read_text())
            assert all(hashlib.sha256((binary.parent/f).read_bytes()).hexdigest()==h for f,h in m['files'].items()),name
        fingerprints[name]={'binary':str(binary),'sha256':hashlib.sha256(binary.read_bytes()).hexdigest()}
    (out/'versions.json').write_text(json.dumps(fingerprints,indent=2)+'\n')
    results={}
    for repeat in range(1,3):
        order=list(versions.items())
        if repeat==2:order.reverse()
        for name,binary in order:
            label=f'{name}-{repeat}'
            env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
            env.update(SOLAR_CONSTRUCTION_VALIDATE_DIR=str(out/(label+'-scene')),SOLAR_P5_TIMING_OUTPUT=str(out/(label+'.csv')),SOLAR_P5_TIMING_GPU='0',SOLAR_RESOURCE_SAMPLES=str(out/(label+'-resources.json')),MESA_VK_WSI_PRESENT_MODE='immediate',RUST_LOG='info,solar_survival::player::surface=debug')
            start=time.monotonic()
            with (out/(label+'.log')).open('w') as log:code,focus=run_focused([str(binary)],env,log,100)
            log=(out/(label+'.log')).read_text()
            record={'exit_code':code,'seconds':time.monotonic()-start,'focused_window':focus,'complete':'P5 terminé' in log,'diagnostics':re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',log)}
            results[label]={'run':record}
            if code or not record['complete']:
                (out/'results.json').write_text(json.dumps(results,indent=2)+'\n');raise SystemExit(1)
            groups={}
            with (out/(label+'.csv')).open() as data:
                for row in csv.DictReader(data):
                    if row['phase'] not in ['built_rest','preview','streaming']:continue
                    groups.setdefault(row['phase']+'/'+row['metric'],[]).append(float(row['value_ms']))
            for group,values in groups.items():
                values.sort();results[label][group]={'samples':len(values),'mean_ms':statistics.mean(values),'median_ms':statistics.median(values),'p95_ms':values[min(len(values)-1,int(len(values)*.95))],'p99_ms':values[min(len(values)-1,int(len(values)*.99))],'max_ms':max(values)}
            (out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
            print(json.dumps({label:record}),flush=True)
    assert all(hashlib.sha256(binary.read_bytes()).hexdigest()==fingerprints[name]['sha256'] for name,binary in versions.items())
    (out/'concordance.json').write_text(json.dumps({'binaries_unchanged':True,'versions':fingerprints},indent=2)+'\n')
if __name__=='__main__':main()
