# workflow-test

Sample Rust project template with GitHub actions, local Git hooks, and other development baselines suitable for small-scale Rust projects.

## setup

First, make sure `rustc` and `cargo` are installed.

To ensure local compliance with repository level policies, it's recommended to set up local `git` hooks:

```
cargo run -p xtask -- install-hooks # populate git hooks (commit, commit message, push)
cargo run -p xtask -- install-tools # locally install dev dependencies
```

## development utilities

As this repo is a template for real projects, development environment examples are included (currently VS Code only).

Plugins in use that apply are the following:

- `rust-analyzer` version 0.4.2438
- `black formatter` version 2025.2.0
- `CodeLLDB` version 1.11.4

Rust source for this project was created with the following toolchain versions in use:

```
rustc --version --verbose
rustc 1.86.0 (05f9846f8 2025-03-31)
binary: rustc
commit-hash: 05f9846f893b09a1be1fc8560e33fc3c815cfecb
commit-date: 2025-03-31
host: x86_64-pc-windows-msvc
release: 1.86.0
LLVM version: 19.1.7
```
