# Overview
- [📦 crates.io](https://crates.io/crates/jsonice)
- [⚖ zlib license](https://opensource.org/licenses/Zlib)

CLI tool to read JSON from stdin and pretty-print it to stdout. It does not load the whole JSON document into memory.

# Documentation
```
Pretty-prints JSON without requiring to load it all in memory

USAGE:
    jsonice [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --indent-size <indent-size>    Indentation size in number of spaces [default: 2]
```

# Dependencies
[Rust](https://www.rust-lang.org/tools/install)

# Install
```sh
cargo install jsonice
```

# Build
```sh
cargo build
```

# Contribute
All contributions shall be licensed under the [zlib license](https://opensource.org/licenses/Zlib).
