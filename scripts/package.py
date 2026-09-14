#!/usr/bin/env python3
"""Build both crates together, including Cargo's isolated package verification."""
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[1]


def run(*args):
    subprocess.run(args, cwd=ROOT, check=True)


run(sys.executable, "scripts/release.py", "check")
run("cargo", "fmt", "--all", "--", "--check")
run("cargo", "test", "--locked", "--workspace")
run("cargo", "doc", "--no-deps", "--workspace")
run("cargo", "package", "--locked", "--workspace")
output = ROOT / "dist"
output.mkdir(exist_ok=True)
version = (ROOT / "VERSION").read_text().strip()
for name in ("schematic-supertest-macros", "schematic-supertest"):
    package = ROOT / "target/package" / f"{name}-{version}.crate"
    with tarfile.open(package) as archive:
        names = archive.getnames()
        assert any(name.endswith("/LICENSE-MIT") for name in names)
        assert any(name.endswith("/LICENSE-APACHE") for name in names)
    shutil.copyfile(package, output / package.name)
