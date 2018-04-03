# Rust-Crypto

This forked branch only includes the needed files to use sha2 project that needs
to be compiled in wasm.

The current version of rust-crypto does not compile
correctly into wasm  because it relies on os bindings and a deprecated
rust-serialize.

As such only the sha2 files and its dependencies have been kept and
rust-serialize has been replaced by hex.


## Usage

To use Rust-Crypto, add the following to your Cargo.toml:

```toml
[dependencies]
rust_crypto = {git = 'https://github.com/agunde406/rust-crypto', branch = "wasm_sha2"}
```

and the following to your crate root:

```rust
extern crate crypto;
```

## License

Rust-Crypto is dual licensed under the MIT and Apache 2.0 licenses, the same
licenses as the Rust compiler.

## Algorithms

* Sha2 (All fixed output size variants)
