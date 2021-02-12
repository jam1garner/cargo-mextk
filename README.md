# cargo-mextk

[![](https://docs.rs/cargo-mextk/badge.svg)](https://docs.rs/cargo-mextk)

A cargo subcommand for build and working with melee mods.

## Building From Source

Requirements:

* Rust - https://www.rust-lang.org/tools/install
* LLVM (for libclang)
  * Windows: `choco install llvm`
  * MacOS: `brew install llvm`
  * Ubuntu/Debian/etc - `sudo apt install libclang-10-dev`

Install from crates.io:

```
cargo install cargo-mextk
```

Build locally:

```
git clone https://github.com/jam1garner/cargo-mextk
cd cargo-mextk
cargo build --release
```

Build Windows Installer (requires cargo-wix):

```
cargo wix
```
