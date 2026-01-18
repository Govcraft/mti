# `TypeId`Suffix

[![Crates.io](https://img.shields.io/crates/v/typeid_suffix.svg)](https://crates.io/crates/typeid_suffix)
[![Documentation](https://docs.rs/typeid_suffix/badge.svg)](https://docs.rs/typeid_suffix)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A Rust library that implements the suffix portion of the [TypeID Specification](https://github.com/jetpack-io/typeid). It provides functionality to work with `TypeId`suffixes, which are base32-encoded representations of UUIDs used in the `TypeId`system.

Combined with the [TypeIdPrefix crate](https://crates.io/crates/typeid_prefix) to comprise the [mti (Magic Type Id) crate](https://crates.io/crates/mti).

Use the [mti (Magic Type Id) crate](https://crates.io/crates/mti) for a holistic implementation of the TypeID specification.

## Features

- **UUID Version Support**: Implements support for `UUIDv7` and other UUID versions.
- **Flexible Architecture**: Generic implementation allows for handling various UUID versions.
- **Base32 Encoding/Decoding**: Efficient encoding and decoding of UUIDs to/from base32 `TypeId`suffixes.
- **Error Handling**: Comprehensive error types for invalid suffixes and UUIDs.
- **Validation**: Robust validation for `TypeId`suffixes and UUIDs.
- **Zero-cost Abstractions**: Designed to have minimal runtime overhead.
- **Optional Tracing**: Integrates with the `tracing` crate for logging (optional feature `instrument`).
- **Optional Serde Support**: Enables serialization and deserialization with `serde` (optional feature `serde`).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typeid_suffix = "1.2.0"
```

To enable optional features:

```toml
[dependencies]
typeid_suffix = { version = "1.2.0", features = ["instrument", "serde"] }
# Or select specific features, e.g., just serde:
# typeid_suffix = { version = "1.2.0", features = ["serde"] }
```

## Usage

### Basic Usage

```rust
use std::str::FromStr;
use typeid_suffix::prelude::*;
use uuid::Uuid;

fn main() {
    // Create a `TypeIdSuffix` from a `UUIDv7` (default)
    let suffix_v7 = TypeIdSuffix::default();
    println!("TypeID suffix (v7 default): {}", suffix_v7);

    // Create a `TypeIdSuffix` from a specific UUID
    let uuid_v4 = Uuid::new_v4();
    let suffix_v4: TypeIdSuffix = uuid_v4.into();
    println!("TypeID suffix (from v4): {}", suffix_v4);

    // Parse a `TypeIdSuffix` from a string
    let parsed_suffix = TypeIdSuffix::from_str("01h455vb4pex5vsknk084sn02q").expect("Valid suffix");
    
    // Convert back to a UUID
    let recovered_uuid: Uuid = parsed_suffix.try_into().expect("Valid UUID");
    // Note: We don't know the original UUID version from the suffix alone without context,
    // but we can recover the UUID bytes.
    println!("Recovered UUID from parsed suffix: {}", recovered_uuid);
}
```

### Working with Other UUID Versions

```rust
use typeid_suffix::prelude::*;
use uuid::Uuid;

fn main() {
    // Creating a new suffix for a specific version (e.g., V4)
    let suffix_v4 = TypeIdSuffix::new::<V4>();
    println!("TypeID suffix for new UUIDv4: {}", suffix_v4);

    let uuid_v1 = Uuid::new_v1([1,2,3,4,5,6]); // Example, requires v1 feature on uuid crate
    let suffix_v1: TypeIdSuffix = uuid_v1.into();
    println!("TypeID suffix for UUIDv1: {}", suffix_v1);
}
```

### Error Handling

The crate provides detailed error types for various failure cases:

```rust
use typeid_suffix::prelude::*;
use std::str::FromStr;

fn main() {
    let result = TypeIdSuffix::from_str("invalid_suffix"); // Invalid length and characters
    match result {
        Ok(_) => println!("Valid suffix"),
        Err(e) => println!("Invalid suffix: {}", e), // e.g., InvalidSuffix(InvalidLength)
    }

    let result_bad_first_char = TypeIdSuffix::from_str("81h455vb4pex5vsknk084sn02q"); // First char > '7'
    match result_bad_first_char {
        Ok(_) => println!("Valid suffix"),
        Err(e) => println!("Invalid suffix: {}", e), // e.g., InvalidSuffix(InvalidFirstCharacter)
    }
}
```

## Optional Features

### Optional Tracing (`instrument`)

When the `instrument` feature is enabled, the crate will log operations using the `tracing` crate:

```toml
[dependencies]
typeid_suffix = { version = "1.2.0", features = ["instrument"] }
```

### Serde Support (`serde`)

When the `serde` feature is enabled, `TypeIdSuffix` implements `serde::Serialize` and `serde::Deserialize`. This allows `TypeIdSuffix` instances to be easily serialized to and deserialized from various formats like JSON, YAML, CBOR, etc., that are supported by Serde.

`TypeIdSuffix` is serialized as its string representation and deserialized from a string.

To enable this feature:

```toml
[dependencies]
typeid_suffix = { version = "1.2.0", features = ["serde"] }
```

**Example:**

```rust
# #[cfg(feature = "serde")] {
use typeid_suffix::prelude::*;
// Note: serde_json is used here as an example and would be a separate dependency.
// Add `serde_json = "1.0"` to your [dependencies] or [dev-dependencies] in Cargo.toml.
use serde_json;

fn main() -> Result<(), serde_json::Error> {
    let suffix = TypeIdSuffix::default();

    // Serialize
    let json_string = serde_json::to_string(&suffix)?;
    println!("Serialized suffix: {}", json_string); // e.g., "\"01h455vb4pex5vsknk084sn02q\""

    // Deserialize
    let deserialized_suffix: TypeIdSuffix = serde_json::from_str(&json_string)?;
    assert_eq!(suffix, deserialized_suffix);
    println!("Deserialized suffix: {}", deserialized_suffix);

    // Example of deserialization error
    let invalid_json = "\"invalid_suffix_string\"";
    let result: Result<TypeIdSuffix, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
    if let Err(e) = result {
        println!("Error deserializing invalid suffix: {}", e);
    }
    Ok(())
}
# }
```

## Use Cases

- **Distributed Systems**: Generate globally unique, sortable identifiers for distributed systems.
- **Database Systems**: Create compact, base32-encoded identifiers for database records.
- **API Development**: Use `TypeId`suffixes as part of API responses or for resource identification.
- **Time-based Sorting**: Leverage the sortable nature of `UUIDv7`-based `TypeId`suffixes for time-ordered data.

## Safety and Correctness

This crate has been thoroughly tested and verified:

- Comprehensive unit tests
- Property-based testing with `proptest`
- Fuzz testing

These measures ensure that the crate behaves correctly and safely under various inputs and conditions.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on Rust 1.60.0 and later.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

This crate implements a portion of the [TypeID Specification](https://github.com/jetpack-io/typeid) created by Jetpack.io.## Sponsor

Govcraft is a one-person shop—no corporate backing, no investors, just me building useful tools. If this project helps you, [sponsoring](https://github.com/sponsors/Govcraft) keeps the work going.

[![Sponsor on GitHub](https://img.shields.io/badge/Sponsor-%E2%9D%A4-%23db61a2?logo=GitHub)](https://github.com/sponsors/Govcraft)
