#!/usr/bin/env python3
"""Compile a Java or RNS source and run it with Lagertha."""

from __future__ import annotations

import argparse
import os
import re
import shlex
import shutil
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CACHE_ROOT = ROOT / ".cache" / "java-run"
FIXTURES_ROOT = ROOT / "vm" / "tests" / "testdata"
EXCLUDED_DIRS = {".cache", ".git", ".github", "docs", "features", "target"}
PACKAGE_PATTERN = re.compile(
    r"^\s*package\s+([A-Za-z_$][\w$]*(?:\.[A-Za-z_$][\w$]*)*)\s*;",
    re.MULTILINE,
)
RNS_CLASS_PATTERN = re.compile(r"^\s*\.class\s+([^\r\n;]+)", re.MULTILINE)
RNS_PACKAGE_PATTERN = re.compile(
    r"^\s*\.package\s+([A-Za-z_$][\w$]*(?:[/.$][A-Za-z_$][\w$]*)*)\s*(?:;|$)",
    re.MULTILINE,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Find, compile, and run a Java or RNS source with Lagertha."
    )
    parser.add_argument(
        "source",
        nargs="?",
        help="source path, filename, or filename without its extension",
    )
    parser.add_argument(
        "--mode",
        choices=("vm", "jvm", "both", "integration"),
        help="skip the launch-mode prompt",
    )
    return parser.parse_args()


def find_sources() -> list[Path]:
    sources: list[Path] = []
    for current, directories, files in os.walk(ROOT):
        directories[:] = sorted(
            directory
            for directory in directories
            if directory not in EXCLUDED_DIRS
        )
        current_path = Path(current)
        sources.extend(
            current_path / name
            for name in files
            if name.endswith((".java", ".rns"))
        )
    return sorted(sources, key=lambda path: path.relative_to(ROOT).as_posix())


def choose_source(sources: list[Path], heading: str) -> Path:
    if not sources:
        raise ValueError("no Java or RNS sources found")

    print(heading)
    for index, source in enumerate(sources, start=1):
        print(f"  {index}. {source.relative_to(ROOT)}")

    while True:
        try:
            answer = input("Select source: ").strip()
        except EOFError as error:
            raise ValueError("source selection requires interactive input") from error
        if answer.isdigit() and 1 <= int(answer) <= len(sources):
            return sources[int(answer) - 1]
        print(f"Enter a number from 1 to {len(sources)}.", file=sys.stderr)


def resolve_source(query: str | None, sources: list[Path]) -> Path:
    if query is None:
        return choose_source(sources, "Java and RNS sources:")

    requested_path = Path(query).expanduser()
    path_candidates = [requested_path]
    if not requested_path.is_absolute():
        path_candidates = [Path.cwd() / requested_path, ROOT / requested_path]

    for candidate in path_candidates:
        try:
            resolved = candidate.resolve(strict=True)
        except OSError:
            continue
        if resolved in sources:
            return resolved
        if resolved.is_file() and resolved.suffix in (".java", ".rns"):
            raise ValueError(f"source is outside scanned paths: {resolved}")

    name = requested_path.name
    stem = name.removesuffix(".java").removesuffix(".rns")
    matches = [source for source in sources if source.stem == stem]
    if not matches:
        raise ValueError(f"Java or RNS source not found: {query}")
    if len(matches) == 1:
        return matches[0]
    return choose_source(matches, f"Multiple sources match {query!r}:")


def is_fixture(source: Path) -> bool:
    return source.is_relative_to(FIXTURES_ROOT)


def has_integration_test(source: Path) -> bool:
    return is_fixture(source) and source.stem.endswith("Test")


def choose_mode(source: Path, requested_mode: str | None) -> str:
    modes = [
        ("vm", "Lagertha VM"),
        ("jvm", "Real JVM (JAVA_HOME)"),
        ("both", "Lagertha VM + Real JVM (JAVA_HOME)"),
    ]
    if has_integration_test(source):
        modes.insert(
            0,
            ("integration", "Integration test (Lagertha + Real JVM)"),
        )

    available_modes = {mode for mode, _ in modes}
    if requested_mode:
        if requested_mode not in available_modes:
            raise ValueError(
                "integration mode requires a *Test.java or *Test.rns source "
                "under vm/tests/testdata"
            )
        return requested_mode

    print()
    print("=" * 72)
    print(" Launch source")
    print(f" {source.relative_to(ROOT)}")
    print("=" * 72)
    for index, (_, label) in enumerate(modes, start=1):
        print(f"  {index}. {label}")

    while True:
        try:
            answer = input("Select launch mode [1]: ").strip()
        except EOFError as error:
            raise ValueError(
                "launch-mode selection requires interactive input"
            ) from error
        if answer == "":
            return modes[0][0]
        if answer.isdigit() and 1 <= int(answer) <= len(modes):
            return modes[int(answer) - 1][0]
        print(f"Enter a number from 1 to {len(modes)}.", file=sys.stderr)


def print_and_run(command: list[str], extra_env: dict[str, str] | None = None) -> int:
    environment = os.environ.copy()
    environment.update(extra_env or {})
    env_prefix = ""
    if extra_env:
        env_prefix = shlex.join(f"{key}={value}" for key, value in extra_env.items()) + " "
    print(f"$ {env_prefix}{shlex.join(command)}", flush=True)
    try:
        return subprocess.run(
            command, cwd=ROOT, env=environment, check=False
        ).returncode
    except OSError as error:
        raise ValueError(f"cannot execute {command[0]}: {error}") from error


def run_integration_test(source: Path) -> int:
    if not source.stem.endswith("Test"):
        raise ValueError(
            "integration harness only discovers *Test.java or *Test.rns sources"
        )
    print_runtime_banner("Integration test: Lagertha + Real JVM")
    command = [
        "cargo",
        "test",
        "--quiet",
        "-p",
        "vm",
        "--test",
        "integration_test",
        source.stem,
    ]
    return print_and_run(command, cargo_environment())


def cargo_environment() -> dict[str, str]:
    rustflags = os.environ.get("RUSTFLAGS", "").strip()
    return {"RUSTFLAGS": f"{rustflags} -Awarnings".strip()}


def javac_path() -> str:
    java_home = os.environ.get("JAVA_HOME")
    if java_home:
        candidate = Path(java_home) / "bin" / "javac"
        if candidate.is_file():
            return str(candidate)
    return "javac"


def java_path() -> str:
    java_home = os.environ.get("JAVA_HOME")
    if java_home:
        candidate = Path(java_home) / "bin" / "java"
        if candidate.is_file():
            return str(candidate)
    return "java"


def main_class_name(source: Path) -> str:
    contents = source.read_text(encoding="utf-8")
    if source.suffix == ".rns":
        class_match = RNS_CLASS_PATTERN.search(contents)
        if not class_match:
            raise ValueError(f"RNS class declaration not found: {source}")
        class_name = class_match.group(1).split()[-1]
        package_match = RNS_PACKAGE_PATTERN.search(contents)
        if package_match:
            class_name = f"{package_match.group(1)}/{class_name}"
        return class_name.replace("/", ".")

    package_match = PACKAGE_PATTERN.search(contents)
    if package_match:
        return f"{package_match.group(1)}.{source.stem}"
    return source.stem


def compile_source(source: Path) -> tuple[int, str]:
    shutil.rmtree(CACHE_ROOT, ignore_errors=True)
    CACHE_ROOT.mkdir(parents=True)
    main_class = main_class_name(source)

    if source.suffix == ".java":
        sibling_sources = sorted(source.parent.glob("*.java"))
        compile_command = [
            javac_path(),
            "-encoding",
            "UTF-8",
            "-g",
            "-d",
            str(CACHE_ROOT),
            *(str(sibling) for sibling in sibling_sources),
        ]
    else:
        class_file = CACHE_ROOT.joinpath(
            *main_class.split(".")
        ).with_suffix(".class")
        class_file.parent.mkdir(parents=True, exist_ok=True)
        compile_command = [
            "rnsc",
            "asm",
            str(source),
            "-o",
            str(class_file),
        ]
    compile_status = print_and_run(compile_command)
    if compile_status != 0:
        return compile_status, main_class

    class_file = CACHE_ROOT.joinpath(*main_class.split(".")).with_suffix(".class")
    if not class_file.is_file():
        raise ValueError(f"compiler did not produce expected main class: {class_file}")
    return 0, main_class


def print_runtime_banner(title: str) -> None:
    print()
    print("=" * 72)
    print(f" {title}")
    print("=" * 72, flush=True)


def run_compiled_vm(main_class: str) -> int:
    print_runtime_banner("Lagertha VM")
    run_command = [
        "cargo",
        "run",
        "--quiet",
        "--package",
        "vm",
        "--bin",
        "vm",
        "--",
        "--classpath",
        str(CACHE_ROOT),
        main_class,
    ]
    return print_and_run(run_command, cargo_environment())


def run_compiled_jvm(main_class: str) -> int:
    print_runtime_banner("Real JVM (JAVA_HOME)")
    run_command = [
        java_path(),
        "-ea",
        "-cp",
        str(CACHE_ROOT),
        main_class,
    ]
    return print_and_run(run_command)


def run_with_vm(source: Path) -> int:
    compile_status, main_class = compile_source(source)
    if compile_status != 0:
        return compile_status
    return run_compiled_vm(main_class)


def run_with_jvm(source: Path) -> int:
    compile_status, main_class = compile_source(source)
    if compile_status != 0:
        return compile_status
    return run_compiled_jvm(main_class)


def run_with_both(source: Path) -> int:
    compile_status, main_class = compile_source(source)
    if compile_status != 0:
        return compile_status

    vm_status = run_compiled_vm(main_class)
    jvm_status = run_compiled_jvm(main_class)
    return vm_status if vm_status != 0 else jvm_status


def main() -> int:
    args = parse_args()
    try:
        sources = find_sources()
        source = resolve_source(args.source, sources)
        mode = choose_mode(source, args.mode)
        if mode == "integration":
            return run_integration_test(source)
        if mode == "jvm":
            return run_with_jvm(source)
        if mode == "both":
            return run_with_both(source)
        return run_with_vm(source)
    except (OSError, UnicodeError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
