from pathlib import Path
import subprocess,time,json,hashlib,platform,os
root=Path.cwd(); output=root/'docs/p5-validation/final'
commands=[('fmt',['cargo','fmt','--check']),('check',['cargo','check','--locked']),('all-targets',['cargo','check','--all-targets','--locked']),('tests',['cargo','test','--locked']),('clippy',['cargo','clippy','--all-targets','--locked','--','-D','warnings']),('release',['cargo','build','--release','--locked'])]
results=[]
for name,command in commands:
 start=time.monotonic()
 with (output/(name+'.txt')).open('w') as log:
  code=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT).returncode
 results.append(dict(name=name,command=command,exit_code=code,seconds=time.monotonic()-start))
 (output/'build-results.json').write_text(json.dumps(results,indent=2)+'\n')
 print(name,code,round(results[-1]['seconds'],3),flush=True)
 if code: raise SystemExit(code)
sources=sorted(root.glob('src/**/*.rs'))+sorted(root.glob('tests/*.rs'))+sorted(root.glob('examples/*.rs'))+[root/'Cargo.toml',root/'Cargo.lock']
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
metadata=dict(date='2026-10-13',platform=platform.platform(),display=os.environ.get('DISPLAY'),rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),start_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),source_sha256={str(p.relative_to(root)):sha(p) for p in sources},binary_sha256=sha(root/'target/release/solar_survival'),windows_validated=False)
(output/'environment.json').write_text(json.dumps(metadata,indent=2)+'\n')
