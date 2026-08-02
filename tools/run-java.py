#!/usr/bin/env python3
"""Compile a Java source and run it with Lagertha."""

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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Find, compile, and run a Java source with Lagertha."
    )
    parser.add_argument(
        "source",
        nargs="?",
        help="source path, filename, or filename without .java",
    )
    parser.add_argument(
        "--mode",
        choices=("vm", "integration"),
        help="skip the run-mode prompt for integration fixtures",
    )
    return parser.parse_args()


def find_java_sources() -> list[Path]:
    sources: list[Path] = []
    for current, directories, files in os.walk(ROOT):
        directories[:] = sorted(
            directory
            for directory in directories
            if directory not in EXCLUDED_DIRS
        )
        current_path = Path(current)
        sources.extend(current_path / name for name in files if name.endswith(".java"))
    return sorted(sources, key=lambda path: path.relative_to(ROOT).as_posix())


def choose_source(sources: list[Path], heading: str) -> Path:
    if not sources:
        raise ValueError("no Java sources found")

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
        return choose_source(sources, "Java sources:")

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
        if resolved.is_file() and resolved.suffix == ".java":
            raise ValueError(f"source is outside scanned paths: {resolved}")

    name = requested_path.name
    stem = name.removesuffix(".java")
    matches = [source for source in sources if source.stem == stem]
    if not matches:
        raise ValueError(f"Java source not found: {query}")
    if len(matches) == 1:
        return matches[0]
    return choose_source(matches, f"Multiple sources match {query!r}:")


def is_fixture(source: Path) -> bool:
    return source.is_relative_to(FIXTURES_ROOT)


def choose_mode(source: Path, requested_mode: str | None) -> str:
    if requested_mode == "integration" and not is_fixture(source):
        raise ValueError("integration mode requires a source under vm/tests/testdata")
    if requested_mode:
        return requested_mode
    if not is_fixture(source):
        return "vm"

    print("Run mode:")
    print("  1. Integration test (Lagertha and reference JDK)")
    print("  2. Lagertha directly")
    while True:
        try:
            answer = input("Select mode [1]: ").strip()
        except EOFError as error:
            raise ValueError("run-mode selection requires interactive input") from error
        if answer in ("", "1"):
            return "integration"
        if answer == "2":
            return "vm"
        print("Enter 1 or 2.", file=sys.stderr)


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
        raise ValueError("integration harness only discovers *Test.java sources")
    command = [
        "cargo",
        "test",
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


def main_class_name(source: Path) -> str:
    contents = source.read_text(encoding="utf-8")
    package_match = PACKAGE_PATTERN.search(contents)
    if package_match:
        return f"{package_match.group(1)}.{source.stem}"
    return source.stem


def run_with_vm(source: Path) -> int:
    shutil.rmtree(CACHE_ROOT, ignore_errors=True)
    CACHE_ROOT.mkdir(parents=True)

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
    compile_status = print_and_run(compile_command)
    if compile_status != 0:
        return compile_status

    main_class = main_class_name(source)
    class_file = CACHE_ROOT.joinpath(*main_class.split(".")).with_suffix(".class")
    if not class_file.is_file():
        raise ValueError(f"javac did not produce expected main class: {class_file}")

    run_command = [
        "cargo",
        "run",
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


def main() -> int:
    args = parse_args()
    try:
        sources = find_java_sources()
        source = resolve_source(args.source, sources)
        mode = choose_mode(source, args.mode)
        if mode == "integration":
            return run_integration_test(source)
        return run_with_vm(source)
    except (OSError, UnicodeError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
