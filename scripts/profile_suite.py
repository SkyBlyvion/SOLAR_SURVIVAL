#!/usr/bin/env python3
"""Lancer séquentiellement le binaire release et résumer les CSV, sans dépendance Python."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess

ROOT = Path(__file__).resolve().parent.parent
VARIANTS = ("baseline", "no_space_camera", "no_surface_camera", "half_resolution", "msaa_off", "surface_unlit")


def percentile(values, fraction):
    index = (len(values) - 1) * fraction
    lower = int(index)
    upper = min(lower + 1, len(values) - 1)
    return values[lower] + (values[upper] - values[lower]) * (index - lower)


def summarize(paths, output):
    rows = []
    for path in paths:
        groups = {}
        with path.open(newline="") as stream:
            for row in csv.DictReader(stream):
                groups.setdefault((row["phase"], row["metric"]), []).append(float(row["value_ms"]))
        for (phase, metric), values in sorted(groups.items()):
            values.sort()
            mean = statistics.fmean(values)
            rows.append(dict(run=path.stem, phase=phase, metric=metric, samples=len(values),
                             mean_ms=mean, p50_ms=statistics.median(values),
                             p95_ms=percentile(values, .95), p99_ms=percentile(values, .99),
                             max_ms=values[-1], fps=1000 / mean if metric == "frame" and mean > 0 else ""))
    fields = ("run", "phase", "metric", "samples", "mean_ms", "p50_ms", "p95_ms", "p99_ms", "max_ms", "fps")
    with output.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--seconds", type=float, default=42)
    parser.add_argument("--repeats", type=int, default=1)
    parser.add_argument("--output", type=Path, default=ROOT / "docs" / "profiles")
    parser.add_argument("--mesa-immediate", action="store_true", help="Contournement local Mesa/NVK déjà validé")
    parser.add_argument("--gpu-probe", action="store_true", help="Ajouter un baseline avec diagnostics GPU pour mesurer leur effet")
    parser.add_argument("--summarize-only", action="store_true")
    args = parser.parse_args()
    if not (30 <= args.seconds <= 300) or args.repeats < 1:
        parser.error("seconds doit être entre 30 et 300, repeats >= 1")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    if args.summarize_only:
        summarize(sorted(output.glob("run-*.csv")), output / "summary.csv")
        return
    binary = ROOT / "target" / "release" / ("solar_survival.exe" if os.name == "nt" else "solar_survival")
    if not binary.exists():
        parser.error("Construire d'abord le binaire avec cargo build --release --locked")
    if any(output.glob("run-*")):
        parser.error("Choisir un dossier vide pour préserver les mesures précédentes")
    source_hash = hashlib.sha256()
    sources = sorted(ROOT.glob("src/**/*.rs")) + sorted(ROOT.glob("assets/shaders/*")) + [ROOT / "Cargo.toml", ROOT / "Cargo.lock"]
    for path in sources:
        source_hash.update(str(path.relative_to(ROOT)).encode())
        source_hash.update(path.read_bytes())
    metadata = dict(platform=platform.platform(), processor=platform.processor(), source_sha256=source_hash.hexdigest(),
                    seconds=args.seconds, repeats=args.repeats, mesa_immediate=args.mesa_immediate,
                    gpu_probe=args.gpu_probe, variants=VARIANTS,
                    binary=str(binary), binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
    (output / "environment.json").write_text(json.dumps(metadata, indent=2) + "\n")
    plan = [(variant, False, repeat) for repeat in range(1, args.repeats + 1) for variant in VARIANTS]
    # Un retour au baseline en fin de suite aide à repérer une dérive du poste.
    plan.append(("baseline", False, args.repeats + 1))
    if args.gpu_probe:
        plan.append(("baseline", True, 1))
    paths = []
    for variant, gpu, repeat in plan:
        name = f"run-{variant}-gpu{int(gpu)}-{repeat}"
        path = output / f"{name}.csv"
        env = os.environ.copy()
        for variable in ("SOLAR_JD", "SOLAR_SMOKE_SECONDS", "SOLAR_SURFACE_SMOKE_SECONDS", "SOLAR_ROTATION_SMOKE_SECONDS"):
            env.pop(variable, None)
        env.update(SOLAR_PROFILE_SECONDS=str(args.seconds), SOLAR_PROFILE_VARIANT=variant,
                   SOLAR_PROFILE_GPU=str(int(gpu)), SOLAR_PROFILE_OUTPUT=str(path), RUST_LOG="info")
        if args.mesa_immediate:
            env["MESA_VK_WSI_PRESENT_MODE"] = "immediate"
        print(f"Lancement {name}", flush=True)
        with (output / f"{name}.log").open("w") as log:
            subprocess.run([str(binary)], cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, check=True,
                           timeout=args.seconds + 60)
        paths.append(path)
        rows = summarize(paths, output / "summary.csv")
        for row in rows:
            if row["run"] == name and row["metric"] == "frame":
                print(f"  {row['phase']}: {row['fps']:.1f} FPS, moyenne {row['mean_ms']:.2f} ms, p95 {row['p95_ms']:.2f} ms", flush=True)
    print(f"Synthèse : {output / 'summary.csv'}", flush=True)


if __name__ == "__main__":
    main()
