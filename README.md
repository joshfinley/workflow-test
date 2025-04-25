# workflow-test

Sample Rust project template with GitHub actions, local Git hooks, and other development baselines suitable for small-scale Rust projects.

## setup

First, make sure `rustc` and `cargo` are installed.

To ensure local compliance with repository level policies, it's recommended to set up local `git` hooks:

```
cargo run -p xtask -- install-hooks
```
