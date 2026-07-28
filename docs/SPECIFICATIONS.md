# Java SE Specification Cache

Lagertha uses the official Java SE 25 JVMS and JLS as its source of truth. A
generated local cache supports repeated research without fetching specification
pages for every task.

## Cache Contents

The ignored `.cache/java-se-25/` directory contains:

```text
.cache/java-se-25/
├── jvms/
│   ├── html/          # original Oracle HTML pages
│   ├── sections/      # searchable text split by section heading
│   ├── index.tsv      # section metadata and canonical source URLs
│   └── manifest.json  # source, refresh time, counts, and page hashes
└── jls/
    └── ...
```

The cache is local generated data, not project documentation or a source of
truth. Canonical references in feature definitions, Issues, and documentation
remain direct Oracle Java SE 25 URLs.

## Build And Refresh

Build both caches:

```bash
python3 tools/java-spec-cache.py refresh
```

Refresh one specification:

```bash
python3 tools/java-spec-cache.py refresh jvms
python3 tools/java-spec-cache.py refresh jls
```

Only `refresh` requires network access. It builds a replacement directory first
and preserves the previous cache if download or extraction fails.

Inspect cache state:

```bash
python3 tools/java-spec-cache.py status
```

## Search And Read

Search section titles and text with a case-insensitive regular expression:

```bash
python3 tools/java-spec-cache.py search 'method resolution'
python3 tools/java-spec-cache.py search 'checkcast' --spec jvms
```

Read one section using its fragment or canonical URL:

```bash
python3 tools/java-spec-cache.py show jvms-6.5.checkcast
python3 tools/java-spec-cache.py show \
  'https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.checkcast'
```

`show` includes named child headings such as an instruction's Description,
Linking Exceptions, Run-time Exceptions, and Notes. It does not recursively load
numbered child sections from broad chapter sections.

Section files under `.cache/java-se-25/*/sections/` can also be searched with
`rg` and read directly. Start with `index.tsv` or `search`; do not load all
sections into one LLM context.

## Verify References

Verify a canonical page and fragment against the original cached HTML:

```bash
python3 tools/java-spec-cache.py verify \
  'https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.checkcast'
```

This check is offline. Refresh the cache explicitly when Java SE version changes
or when current Oracle content must be reconfirmed.
