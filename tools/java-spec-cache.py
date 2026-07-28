#!/usr/bin/env python3
"""Build and query an offline Java SE 25 specification cache."""

from __future__ import annotations

import argparse
import csv
import hashlib
import html.parser
import json
import re
import shutil
import sys
import tempfile
import urllib.parse
import urllib.request
from datetime import datetime, timezone
from pathlib import Path


SPECS = {
    "jvms": "https://docs.oracle.com/javase/specs/jvms/se25/html/",
    "jls": "https://docs.oracle.com/javase/specs/jls/se25/html/",
}
CACHE_ROOT = Path(__file__).resolve().parents[1] / ".cache" / "java-se-25"
USER_AGENT = "Lagertha Java SE specification cache"


class LinkParser(html.parser.HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.links: list[str] = []

    def handle_starttag(
        self, tag: str, attrs: list[tuple[str, str | None]]
    ) -> None:
        if tag != "a":
            return
        href = dict(attrs).get("href")
        if href:
            self.links.append(href)


class SectionParser(html.parser.HTMLParser):
    BLOCK_TAGS = {
        "blockquote",
        "br",
        "dd",
        "div",
        "dl",
        "dt",
        "li",
        "p",
        "table",
        "td",
        "th",
        "tr",
        "ul",
    }

    def __init__(self, prefix: str) -> None:
        super().__init__(convert_charrefs=True)
        self.prefix = prefix
        self.sections: list[tuple[str, str, str]] = []
        self.heading_tag: str | None = None
        self.heading_anchor: str | None = None
        self.heading_text: list[str] = []
        self.current_anchor: str | None = None
        self.current_title = ""
        self.current_text: list[str] = []

    def handle_starttag(
        self, tag: str, attrs: list[tuple[str, str | None]]
    ) -> None:
        if re.fullmatch(r"h[1-6]", tag):
            self.heading_tag = tag
            self.heading_anchor = None
            self.heading_text = []
            return

        attributes = dict(attrs)
        anchor = attributes.get("name") or attributes.get("id")
        if self.heading_tag and anchor and anchor.startswith(f"{self.prefix}-"):
            self.heading_anchor = anchor

        if not self.heading_tag and tag in self.BLOCK_TAGS:
            if tag == "li":
                self.current_text.append("\n- ")
            else:
                self.current_text.append("\n")

    def handle_endtag(self, tag: str) -> None:
        if tag == self.heading_tag:
            title = normalize_text("".join(self.heading_text))
            if self.heading_anchor:
                self._finish_section()
                self.current_anchor = self.heading_anchor
                self.current_title = title
                self.current_text = []
            elif self.current_anchor and title:
                self.current_text.extend(("\n", title, "\n"))
            self.heading_tag = None
            self.heading_anchor = None
            self.heading_text = []
        elif not self.heading_tag and tag in self.BLOCK_TAGS:
            self.current_text.append("\n")

    def handle_data(self, data: str) -> None:
        if self.heading_tag:
            self.heading_text.append(data)
        elif self.current_anchor:
            self.current_text.append(data)

    def close(self) -> None:
        super().close()
        self._finish_section()

    def _finish_section(self) -> None:
        if not self.current_anchor:
            return
        text = normalize_text("".join(self.current_text), preserve_lines=True)
        self.sections.append((self.current_anchor, self.current_title, text))
        self.current_anchor = None


class AnchorParser(html.parser.HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.anchors: set[str] = set()

    def handle_starttag(
        self, tag: str, attrs: list[tuple[str, str | None]]
    ) -> None:
        attributes = dict(attrs)
        for key in ("id", "name"):
            if attributes.get(key):
                self.anchors.add(attributes[key])


def normalize_text(value: str, preserve_lines: bool = False) -> str:
    if not preserve_lines:
        return re.sub(r"\s+", " ", value).strip()
    lines = [re.sub(r"[ \t\r\f\v]+", " ", line).strip() for line in value.split("\n")]
    return "\n".join(line for line in lines if line).strip()


def decode_html(data: bytes) -> str:
    match = re.search(br"charset=([A-Za-z0-9._-]+)", data[:4096], re.IGNORECASE)
    encoding = match.group(1).decode("ascii") if match else "utf-8"
    return data.decode(encoding, errors="replace")


def download(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read()


def discover_pages(index_data: bytes, base_url: str) -> list[str]:
    parser = LinkParser()
    parser.feed(decode_html(index_data))
    base_path = urllib.parse.urlparse(base_url).path
    pages = ["index.html"]
    seen = set(pages)

    for href in parser.links:
        url = urllib.parse.urljoin(base_url, href)
        parsed = urllib.parse.urlparse(url)
        if parsed.netloc != "docs.oracle.com" or not parsed.path.startswith(base_path):
            continue
        name = Path(parsed.path).name
        if not name.endswith(".html") or name in seen:
            continue
        seen.add(name)
        pages.append(name)
    return pages


def refresh_spec(spec: str) -> None:
    base_url = SPECS[spec]
    CACHE_ROOT.mkdir(parents=True, exist_ok=True)
    temporary = Path(tempfile.mkdtemp(prefix=f".{spec}-", dir=CACHE_ROOT))
    target = CACHE_ROOT / spec
    backup = CACHE_ROOT / f".{spec}-previous"

    try:
        html_dir = temporary / "html"
        sections_dir = temporary / "sections"
        html_dir.mkdir()
        sections_dir.mkdir()

        index_data = download(urllib.parse.urljoin(base_url, "index.html"))
        pages = discover_pages(index_data, base_url)
        page_data: dict[str, bytes] = {"index.html": index_data}
        manifest_pages: dict[str, str] = {}

        for page in pages:
            data = page_data.get(page)
            if data is None:
                data = download(urllib.parse.urljoin(base_url, page))
                page_data[page] = data
            (html_dir / page).write_bytes(data)
            manifest_pages[page] = hashlib.sha256(data).hexdigest()

        rows: list[dict[str, str]] = []
        for page in pages:
            parser = SectionParser(spec)
            parser.feed(decode_html(page_data[page]))
            parser.close()
            for anchor, title, text in parser.sections:
                section_file = f"sections/{anchor}.txt"
                source_url = f"{base_url}{page}#{anchor}"
                content = (
                    f"{spec.upper()} {anchor.removeprefix(f'{spec}-')}: {title}\n"
                    f"Source: {source_url}\n\n{text}\n"
                )
                (temporary / section_file).write_text(content, encoding="utf-8")
                rows.append(
                    {
                        "spec": spec,
                        "section": anchor.removeprefix(f"{spec}-"),
                        "title": title,
                        "page": page,
                        "fragment": anchor,
                        "source_url": source_url,
                        "section_file": section_file,
                    }
                )

        with (temporary / "index.tsv").open("w", encoding="utf-8", newline="") as file:
            writer = csv.DictWriter(
                file,
                fieldnames=(
                    "spec",
                    "section",
                    "title",
                    "page",
                    "fragment",
                    "source_url",
                    "section_file",
                ),
                dialect="excel-tab",
            )
            writer.writeheader()
            writer.writerows(rows)

        manifest = {
            "spec": spec,
            "version": 25,
            "source": base_url,
            "refreshed_at": datetime.now(timezone.utc).isoformat(),
            "pages": manifest_pages,
            "sections": len(rows),
        }
        (temporary / "manifest.json").write_text(
            json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

        if backup.exists():
            shutil.rmtree(backup)
        if target.exists():
            target.rename(backup)
        temporary.rename(target)
        if backup.exists():
            shutil.rmtree(backup)
        print(f"Cached {spec.upper()}: {len(pages)} pages, {len(rows)} sections")
    except Exception:
        shutil.rmtree(temporary, ignore_errors=True)
        if backup.exists() and not target.exists():
            backup.rename(target)
        raise


def read_index(spec: str) -> list[dict[str, str]]:
    path = CACHE_ROOT / spec / "index.tsv"
    if not path.exists():
        raise SystemExit(f"Missing {spec.upper()} cache. Run: python3 tools/java-spec-cache.py refresh {spec}")
    with path.open(encoding="utf-8", newline="") as file:
        return list(csv.DictReader(file, dialect="excel-tab"))


def search_cache(query: str, specs: list[str], limit: int) -> None:
    pattern = re.compile(query, re.IGNORECASE)
    count = 0
    for spec in specs:
        for row in read_index(spec):
            section_path = CACHE_ROOT / spec / row["section_file"]
            content = section_path.read_text(encoding="utf-8")
            if not pattern.search(f"{row['title']}\n{content}"):
                continue
            print(f"{row['fragment']}\t{row['title']}\t{row['source_url']}")
            count += 1
            if count >= limit:
                return


def resolve_reference(reference: str) -> tuple[str, str]:
    if reference.startswith("http://") or reference.startswith("https://"):
        parsed = urllib.parse.urlparse(reference)
        fragment = parsed.fragment
    else:
        fragment = reference.lstrip("#")
    spec = fragment.split("-", 1)[0]
    if spec not in SPECS or not fragment:
        raise SystemExit("Reference must contain a JVMS or JLS fragment, such as jvms-6.5.iadd")
    return spec, fragment


def show_section(reference: str) -> None:
    spec, fragment = resolve_reference(reference)
    rows = read_index(spec)
    matches = []
    for row in rows:
        candidate = row["fragment"]
        if candidate == fragment:
            matches.append(row)
            continue
        if not candidate.startswith(f"{fragment}."):
            continue
        suffix = candidate[len(fragment) + 1 :].split(".", 1)[0]
        if suffix and not suffix[0].isdigit():
            matches.append(row)

    if not matches:
        raise SystemExit(f"Section not found in local cache: {fragment}")
    for index, row in enumerate(matches):
        if index:
            print()
        path = CACHE_ROOT / spec / row["section_file"]
        sys.stdout.write(path.read_text(encoding="utf-8"))


def verify_reference(reference: str) -> None:
    spec, fragment = resolve_reference(reference)
    parsed = urllib.parse.urlparse(reference)
    if parsed.scheme:
        expected_prefix = urllib.parse.urlparse(SPECS[spec])
        if (
            parsed.netloc != expected_prefix.netloc
            or str(Path(parsed.path).parent) != expected_prefix.path.rstrip("/")
        ):
            raise SystemExit(f"URL is not an official Java SE 25 {spec.upper()} HTML link")
        page = Path(parsed.path).name
    else:
        rows = read_index(spec)
        matches = [row for row in rows if row["fragment"] == fragment]
        if not matches:
            raise SystemExit(f"Fragment not found in local cache: {fragment}")
        page = matches[0]["page"]

    path = CACHE_ROOT / spec / "html" / page
    if not path.exists():
        raise SystemExit(f"Page not found in local cache: {page}")
    parser = AnchorParser()
    parser.feed(decode_html(path.read_bytes()))
    if fragment not in parser.anchors:
        raise SystemExit(f"Fragment not found in {page}: {fragment}")
    print(f"Verified offline: {page}#{fragment}")


def show_status() -> None:
    for spec in SPECS:
        path = CACHE_ROOT / spec / "manifest.json"
        if not path.exists():
            print(f"{spec.upper()}: missing")
            continue
        manifest = json.loads(path.read_text(encoding="utf-8"))
        print(
            f"{spec.upper()}: {manifest['sections']} sections, "
            f"refreshed {manifest['refreshed_at']}"
        )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    refresh = subparsers.add_parser("refresh", help="download and rebuild cache")
    refresh.add_argument("spec", choices=(*SPECS, "all"), default="all", nargs="?")

    search = subparsers.add_parser("search", help="search section titles and text")
    search.add_argument("query", help="case-insensitive regular expression")
    search.add_argument("--spec", choices=(*SPECS, "all"), default="all")
    search.add_argument("--limit", type=int, default=20)

    show = subparsers.add_parser("show", help="print one section by fragment or URL")
    show.add_argument("reference")

    verify = subparsers.add_parser("verify", help="verify a page and fragment offline")
    verify.add_argument("reference")

    subparsers.add_parser("status", help="show local cache state")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    if args.command == "refresh":
        specs = list(SPECS) if args.spec == "all" else [args.spec]
        for spec in specs:
            refresh_spec(spec)
    elif args.command == "search":
        specs = list(SPECS) if args.spec == "all" else [args.spec]
        search_cache(args.query, specs, args.limit)
    elif args.command == "show":
        show_section(args.reference)
    elif args.command == "verify":
        verify_reference(args.reference)
    elif args.command == "status":
        show_status()


if __name__ == "__main__":
    main()
