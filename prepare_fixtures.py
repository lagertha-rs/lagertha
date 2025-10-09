#!/usr/bin/env python3
import argparse
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Dict, List, Optional

DEFAULT_EXPECTED = "javac 24.0.2"
DEFAULT_OUT_DIR = "target/test-classes"
DEFAULT_CONFIG = "class_file/fixtures.toml"

RUNTIME_SRC_ROOT = Path("runtime/testdata")
VM_SRC_ROOT = Path("vm/testdata")

SUBDIR_JDK = "jdk"
SUBDIR_RUNTIME = "runtime"
SUBDIR_VM = "vm"


def eprint(*a, **k):
    print(*a, file=sys.stderr, **k)


def check_tool(name: str):
    if shutil.which(name) is None:
        eprint(f"ERROR: required tool '{name}' not found in PATH")
        sys.exit(2)


def check_javac_version(expected: str):
    check_tool("javac")
    got = subprocess.run(["javac", "--version"], capture_output=True, text=True)
    ver = (got.stdout or got.stderr).strip().replace("\r", "")
    if ver != expected:
        eprint(f"Error: expected {expected}, got: {ver}")
        sys.exit(1)


def find_java_sources(root: Path) -> List[Path]:
    """Find all .java files under a root, excluding anything under a path containing 'java.base'."""
    if not root.exists():
        return []
    files: List[Path] = []
    for p in root.rglob("*.java"):
        parts = p.parts
        if "java.base" in parts:
            continue
        files.append(p)
    return sorted(files)


def compile_sources(sources: List[Path], out_dir: Path, label: str):
    out_dir.mkdir(parents=True, exist_ok=True)
    if not sources:
        print(f"[{label}] No .java sources found (excluding java.base).")
        return
    for s in sources:
        eprint(f"[{label}] Compiling {s}")
    with tempfile.TemporaryDirectory(prefix="javac_args_") as td:
        argfile = Path(td) / "sources.txt"
        argfile.write_text("".join(f"\"{str(s)}\"\n" for s in sources), encoding="utf-8")
        cmd = ["javac", "-g", "-d", str(out_dir), f"@{argfile}"]
        subprocess.run(cmd, check=True)
    print(f"[{label}] Compiled {len(sources)} file(s) to {out_dir}")


def load_toml(config_path: Path) -> Dict:
    try:
        import tomllib
    except ModuleNotFoundError:
        try:
            import tomli as tomllib
        except ModuleNotFoundError:
            eprint("ERROR: Need Python 3.11+ (tomllib) or `pip install tomli`")
            sys.exit(2)
    with config_path.open("rb") as f:
        return tomllib.load(f)


def fqn_to_rel(class_fqn: str) -> Path:
    return Path(*class_fqn.split(".")).with_suffix(".class")


def extract_fixtures(config_path: Path, out_dir: Path, java_home: Optional[str]):
    if not config_path.exists():
        print(f"No {config_path} found; skipping JDK extraction.")
        return
    cfg = load_toml(config_path)
    modules = cfg.get("modules") or {}
    if not modules:
        print(f"{config_path} has no [modules.*] entries; nothing to extract.")
        return

    if not java_home:
        java_home = os.environ.get("JAVA_HOME")
    if not java_home:
        eprint("ERROR: JAVA_HOME is not set (and --java-home not provided)")
        sys.exit(2)
    jmods_dir = Path(java_home) / "jmods"
    if not jmods_dir.is_dir():
        eprint(f"ERROR: {jmods_dir} not found")
        sys.exit(2)

    check_tool("jmod")

    out_dir.mkdir(parents=True, exist_ok=True)
    copied = 0

    with tempfile.TemporaryDirectory(prefix="jmod_extract_") as td:
        td = Path(td)
        for mod_name, payload in modules.items():
            classes: List[str] = payload.get("classes") or []
            if not classes:
                continue
            jmod = jmods_dir / f"{mod_name}.jmod"
            if not jmod.is_file():
                eprint(f"ERROR: missing {jmod}")
                continue
            extract_dir = td / mod_name
            subprocess.run(["jmod", "extract", "--dir", str(extract_dir), str(jmod)], check=True)
            classes_root = extract_dir / "classes"
            if not classes_root.is_dir():
                eprint(f"WARNING: no classes/ in {jmod}")
                continue
            for fqn in classes:
                rel = fqn_to_rel(fqn)
                src = classes_root / rel
                dst = out_dir / rel
                if not src.is_file():
                    eprint(f"ERROR: {mod_name}:{fqn} not found at {src}")
                    continue
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(src, dst)
                print(f"Extracted {mod_name}:{fqn} -> {dst}")
                copied += 1

    if copied == 0:
        print("No classes extracted from JDK modules.")
    else:
        print(f"Extracted {copied} class file(s) to {out_dir}")


def main():
    ap = argparse.ArgumentParser(
        description="Prepare Java fixtures: compile sources from runtime/vm testdata and extract JDK classes."
    )
    ap.add_argument("--expected-javac", default=DEFAULT_EXPECTED, help="Exact `javac --version` to require")
    ap.add_argument("--out-dir", default=DEFAULT_OUT_DIR, help="Root output directory for class files")
    ap.add_argument("--config", default=DEFAULT_CONFIG, help="TOML config with modules/classes to extract")
    ap.add_argument("--java-home", default=None, help="JAVA_HOME override (defaults to env)")
    args = ap.parse_args()

    check_javac_version(args.expected_javac)

    out_root = Path(args.out_dir)
    out_jdk = out_root / SUBDIR_JDK
    out_runtime = out_root / SUBDIR_RUNTIME
    out_vm = out_root / SUBDIR_VM

    # Compile runtime sources
    runtime_sources = find_java_sources(RUNTIME_SRC_ROOT)
    compile_sources(runtime_sources, out_runtime, label="runtime")

    # Compile vm sources
    vm_sources = find_java_sources(VM_SRC_ROOT)
    compile_sources(vm_sources, out_vm, label="vm")

    # Extract JDK fixtures
    extract_fixtures(Path(args.config), out_jdk, args.java_home)

    print(f"Fixtures ready in: {out_root}")
    print(f"  runtime classes: {out_runtime}")
    print(f"  vm      classes: {out_vm}")
    print(f"  jdk     classes: {out_jdk}")


if __name__ == "__main__":
    main()
