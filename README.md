# TypeID Workspace

A collection of Rust crates implementing the [TypeID Specification](https://github.com/jetpack-io/typeid).

## Crates

| Crate | Description | crates.io |
|-------|-------------|-----------|
| [mti](crates/mti) | Magic Type ID - Human-readable, prefixed, globally unique identifiers | [![crates.io](https://img.shields.io/crates/v/mti.svg)](https://crates.io/crates/mti) |
| [typeid_prefix](crates/typeid-prefix) | Type-safe TypeID prefix implementation | [![crates.io](https://img.shields.io/crates/v/typeid_prefix.svg)](https://crates.io/crates/typeid_prefix) |
| [typeid_suffix](crates/typeid-suffix) | Base32-encoded UUID suffix implementation | [![crates.io](https://img.shields.io/crates/v/typeid_suffix.svg)](https://crates.io/crates/typeid_suffix) |

## Quick Start

```toml
[dependencies]
mti = "1.0"
```

```rust
use mti::prelude::*;

let user_id = "user".create_type_id::<V7>();
println!("User ID: {}", user_id);
```

## Development

Build all crates:

```bash
cargo build --workspace
```

Run all tests:

```bash
cargo nextest run --workspace
```

Check with clippy:

```bash
cargo clippy --workspace --all-features
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
