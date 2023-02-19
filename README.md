# derive(Error) for no_std environments

This is a fork of thiserror using the nightly only, experimental error_in_core
feature in no_std environments.

## Usage

```toml
// Cargo.toml
[dependencies]
thiserror = { version = "1.0", package = "thiserror-core", default-features = false }
```

```rust
// main.rs / lib.rs
#![no_std]
#![feature(error_in_core)]
```

## Differences between thiserror and thiserror-core

Differences are kept to a minimum and changes in thiserror master will be adopted by rebasing thiserror-core.

With the default `std` feature enabled, any functional difference between thiserror and thiserror-core is considered a bug.

As soon as the matching [PR](https://github.com/dtolnay/thiserror/pull/211) is merged or equivalent changes are adopted,
this crate will be updated to re-export thiserror.