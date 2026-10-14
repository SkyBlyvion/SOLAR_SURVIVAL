#!/usr/bin/env python3
"""Mesures séquentielles global/P5, sans capture ni compilation concurrente."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
from profile_suite import summarize

ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--scene', choices=['global', 'p5'], default='global')
    parser.add_argument('--lod', choices=['fixed', 'adaptive'], default='adaptive')
    parser.add_argument('--repeats', type=int, default=2)
    parser.add_argument('--gpu-probe', action='store_true')
    parser.add_argument('--camera-probe', action='store_true')
    parser.add_argument('--mesa-immediate', action='store_true')
    parser.add_argument('--focus-x11', action='store_true', help='Activer réellement la fenêtre de diagnostic sur X11, sans touches/souris OS')
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--resources', action='store_true', help='RSS/HWM Linux et DRM si disponibles, intervalle 250 ms')
    args = parser.parse_args()
    if args.repeats < 1 or args.camera_probe and args.scene != 'global':
        parser.error('repeats >=1 ; caméra probe seulement dans la scène globale')
    out = args.output.resolve()
    out.mkdir()  # préserver les campagnes précédentes
    binary = (args.binary or ROOT/'target/release'/('solar_survival.exe' if os.name == 'nt' else 'solar_survival')).resolve()
    sources = [*ROOT.glob('src/**/*.rs'), *ROOT.glob('tests/**/*.rs'), *[p for p in ROOT.glob('assets/**/*') if p.is_file()], ROOT/'Cargo.toml', ROOT/'Cargo.lock']
    metadata = dict(binary_sha256=sha(binary), source_sha256={str(p.relative_to(ROOT)): sha(p) for p in sorted(sources)},
                    options=vars(args) | {'output': str(out), 'binary': str(binary)}, display=os.environ.get('DISPLAY'))
    asset_root = binary.parent/'assets' if (binary.parent/'assets').is_dir() else ROOT/'assets'
    metadata['runtime_asset_sha256'] = {str(p.relative_to(asset_root)):sha(p) for p in sorted(asset_root.rglob('*')) if p.is_file()}
    (out/'environment.json').write_text(json.dumps(metadata, indent=2)+'\n')
    plan = [(f'run-{args.lod}-{i}', False, 'baseline') for i in range(1,args.repeats+1)]
    if args.gpu_probe:
        plan.append((f'run-{args.lod}-gpu', True, 'baseline'))
    if args.camera_probe:
        plan.append((f'run-{args.lod}-no-sky', False, 'no_sky'))
    results = []
    for name, gpu, variant in plan:
        env = {k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
        env.update(SOLAR_TERRAIN_LOD=args.lod, RUST_LOG='info,solar_survival::player::surface=debug,solar_survival::rendering::terrain=debug')
        path = out/(name+'.csv')
        if args.resources:
            if not args.focus_x11: parser.error('--resources exige --focus-x11 sur le helper Linux actuel')
            env['SOLAR_RESOURCE_SAMPLES'] = str(out/(name+'-resources.json'))
        if args.scene == 'global':
            env.update(SOLAR_PROFILE_TERRAIN='global', SOLAR_PROFILE_SECONDS='54', SOLAR_PROFILE_GPU=str(int(gpu)),
                       SOLAR_PROFILE_VARIANT=variant, SOLAR_PROFILE_OUTPUT=str(path))
        else:
            env.update(SOLAR_CONSTRUCTION_VALIDATE_DIR=str(out/(name+'-scenario')), SOLAR_P5_TIMING_OUTPUT=str(path),
                       SOLAR_P5_TIMING_GPU=str(int(gpu)))
        if args.mesa_immediate:
            env['MESA_VK_WSI_PRESENT_MODE']='immediate'
        print(name, flush=True)
        start=time.monotonic()
        with (out/(name+'.log')).open('w') as log:
            if args.focus_x11:
                from focus_diagnostic import run_focused
                code, focused = run_focused([str(binary)], env, log, 115)
            else:
                code = subprocess.run([str(binary)],cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=115).returncode
                focused = None
        content=(out/(name+'.log')).read_text()
        errors=re.findall(r'.*(?:\bERROR\b|\bWARN\b|panicked|panic:).*',content)
        results.append(dict(run=name,seconds=time.monotonic()-start,exit_code=code,focused_window=focused,diagnostics=errors))
        (out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
        if code or errors or not path.exists():
            raise SystemExit(1)
        summarize(sorted(out.glob('run-*.csv')),out/'summary.csv')
    unchanged=sha(binary)==metadata['binary_sha256'] and all(sha(ROOT/p)==h for p,h in metadata['source_sha256'].items()) and all(sha(asset_root/p)==h for p,h in metadata['runtime_asset_sha256'].items())
    (out/'concordance.json').write_text(json.dumps(dict(hashes_unchanged=unchanged),indent=2)+'\n')
    if not unchanged:
        raise SystemExit('Sources/binaire modifiés pendant les mesures')


if __name__=='__main__':
    main()
