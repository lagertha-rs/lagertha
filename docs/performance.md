# Performance tools

Very important topic. This file is completely TODO. I don't know really how to investigate performance in rust programs.

Added hotpath crate, it looks not bad, but need to configure it properly.

Command for myself to remember how to run with hotpath:

```shell
cargo run --package vm --bin vm --features=hotpath
```

```shell
HOTPATH_ALLOC_SELF=true; cargo run --package vm --bin vm --features='hotpath,hotpath-alloc'
```