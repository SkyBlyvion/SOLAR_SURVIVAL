from pathlib import Path
import os,subprocess,json,time,re,hashlib
root=Path.cwd(); out=root/'docs/lod-validation/graphics';out.mkdir()
sources=[*root.glob('src/**/*.rs'),*root.glob('tests/*.rs'),*root.glob('examples/*.rs'),root/'Cargo.toml',root/'Cargo.lock']
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
meta=dict(source_sha256={str(p.relative_to(root)):sha(p) for p in sources},binary_sha256=sha(root/'target/release/solar_survival'),example_sha256=sha(root/'target/release/examples/lod_seams'))
(out/'environment.json').write_text(json.dumps(meta,indent=2)+'\n')
results=[]
for name,binary,variables in [
 ('seams',root/'target/release/examples/lod_seams',{}),
 ('p5',root/'target/release/solar_survival',{'SOLAR_CONSTRUCTION_VALIDATE_DIR':str(out/'p5-captures')}),
 ('global',root/'target/release/solar_survival',{'SOLAR_GLOBAL_VALIDATE_DIR':str(out/'global-captures'),'SOLAR_LOD_DEBUG':'1'})]:
 env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
 env.update(variables,MESA_VK_WSI_PRESENT_MODE='immediate',RUST_LOG='info,solar_survival::player::surface=debug,solar_survival::rendering::terrain=debug')
 cmd=[str(binary)]
 if name=='seams':cmd.append(str(out/'seams-captures'))
 print(name,flush=True);start=time.monotonic()
 with (out/(name+'.log')).open('w') as log:code=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=180).returncode
 content=(out/(name+'.log')).read_text();errors=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',content)
 results.append(dict(name=name,exit_code=code,seconds=time.monotonic()-start,diagnostics=errors,captures=len(list((out/(name+'-captures')).glob('*.png')))))
 (out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
 if code or errors:raise SystemExit(1)
unchanged=all(sha(root/p)==h for p,h in meta['source_sha256'].items()) and sha(root/'target/release/solar_survival')==meta['binary_sha256'] and sha(root/'target/release/examples/lod_seams')==meta['example_sha256']
(out/'concordance.json').write_text(json.dumps(dict(hashes_unchanged=unchanged),indent=2)+'\n')
assert [r['captures'] for r in results]==[6,16,36]
assert unchanged
