# Lindskou’s Rust Journey

## Primer

  - Rust book: https://doc.rust-lang.org/book/
  - Rust by example: https://doc.rust-lang.org/rust-by-example/
  - Traits: https://oswalt.dev/2020/07/rust-traits-defining-behavior/
  - Macros1: https://github.com/maciejkula/rustlearn/blob/master/src%2Ffactorization%2Ffactorization_machines.rs
  - Macros2: https://www.howtocodeit.com/guides/writing-production-rust-macros-with-macro-rules
  - Gradient Boosting Third Party: https://github.com/jinlow/forust
  - ML: https://arewelearningyet.com/
  

## Compilation

In `Cargo.toml` many different binaries are define, e.g.

``` toml
[[bin]]
name = "primitives"
path = "src/01-primitives/main.rs"
```

This tells rust, that there is a main file in `src/01-primitives/` that
should be compiled. In addition it has been given the name `primitives`
and one can type `make primitives` to automatically compile and run
this. This is just sugar for the command `cargo run --bin primitives`.

## Cargo

``` bash
cargo new # Instantiate new project and make a git repo
cargo build # Build the project
cargo run # Build and run the project
cargo check # Check if the project compiles (not producing executable)
```

## Setup

A toolchain for Rust:
https://doc.rust-lang.org/book/ch01-01-installation.html

 - Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
 - Emacs Rust mode: https://github.com/rust-lang/rust-mode
 - Rust analyzer (LSP): `rustup component add rust-analyzer`
 - Background code checker: https://github.com/Canop/bacon
 - Rust linter: https://github.com/rust-lang/rust-clippy (can be used from bacon)
