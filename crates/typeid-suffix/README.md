# `TypeID` Suffix

[![Crates.io](https://img.shields.io/crates/v/typeid_suffix.svg)](https://crates.io/crates/typeid_suffix)
[![Documentation](https://docs.rs/typeid_suffix/badge.svg)](https://docs.rs/typeid_suffix)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/Govcraft/mti#license)

A Rust library that implements the suffix portion of the [TypeID Specification](https://github.com/jetpack-io/typeid). It provides functionality to work with `TypeID` suffixes, which are base32-encoded representations of UUIDs used in the `TypeID` system.

Combined with the [TypeIdPrefix crate](https://crates.io/crates/typeid_prefix) to comprise the [mti (Magic Type Id) crate](https://crates.io/crates/mti).

Use the [mti (Magic Type Id) crate](https://crates.io/crates/mti) for a holistic implementation of the TypeID specification.

## Features

- **UUID Version Support**: Implements support for `UUIDv7` and other UUID versions.
- **Flexible Architecture**: Generic implementation allows for handling various UUID versions.
- **Base32 Encoding/Decoding**: Efficient encoding and decoding of UUIDs to/from base32 `TypeID` suffixes.
- **Error Handling**: Comprehensive error types for invalid suffixes and UUIDs.
- **Validation**: Checks the 26-character suffix encoding, alphabet, and first-character limit. Parsing preserves UUID bits without restricting the UUID version or variant.
- **Zero-cost Abstractions**: Designed to have minimal runtime overhead.
- **Optional Tracing**: Integrates with the `tracing` crate for logging (optional feature `instrument`).
- **Optional Serde Support**: Enables serialization and deserialization with `serde` (optional feature `serde`).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typeid_suffix = "1.3"
```

To enable optional features:

```toml
[dependencies]
typeid_suffix = { version = "1.3", features = ["instrument", "serde"] }
# Or select specific features, e.g., just serde:
# typeid_suffix = { version = "1.3", features = ["serde"] }
```

## Usage

The prelude re-exports `TypeIdSuffix`, UUID version types, and `Uuid`. Examples that import `uuid::Uuid` directly require `uuid = { version = "1", features = ["v4"] }` in your dependencies. `TypeIdSuffix::default()` uses V7; `TypeIdSuffix::new::<V4>()` explicitly selects another version.

### Basic Usage

```rust
use std::str::FromStr;
use typeid_suffix::prelude::*;
use uuid::Uuid;

fn main() {
    // Create a `TypeIdSuffix` from a `UUIDv7` (default)
    let suffix_v7 = TypeIdSuffix::default();
    assert_eq!(suffix_v7.to_uuid().get_version(), Some(Version::SortRand));
    println!("TypeID suffix (v7 default): {}", suffix_v7);

    // Create a `TypeIdSuffix` from a specific UUID
    let uuid_v4 = Uuid::new_v4();
    let suffix_v4: TypeIdSuffix = uuid_v4.into();
    assert_eq!(suffix_v4.to_uuid(), uuid_v4);
    println!("TypeID suffix (from v4): {}", suffix_v4);

    // Parse a `TypeIdSuffix` from a string
    let parsed_suffix = TypeIdSuffix::from_str("01h455vb4pex5vsknk084sn02q").expect("Valid suffix");
    
    // Convert back to a UUID
    let recovered_uuid: Uuid = parsed_suffix.into();
    // UUID conversion preserves the version bits as well as the remaining bytes.
    println!("Recovered UUID from parsed suffix: {}", recovered_uuid);
}
```

### Working with Other UUID Versions

```rust
use typeid_suffix::prelude::*;

fn main() {
    // Creating a new suffix for a specific version (e.g., V4)
    let suffix_v4 = TypeIdSuffix::new::<V4>();
    println!("TypeID suffix for new UUIDv4: {}", suffix_v4);

    let suffix_v1 = TypeIdSuffix::new::<V1>();
    println!("TypeID suffix for UUIDv1: {}", suffix_v1);
}
```

### Error Handling

The crate provides detailed error types for various failure cases:

```rust
use typeid_suffix::prelude::*;
use std::str::FromStr;

fn main() {
    assert_eq!(
        TypeIdSuffix::from_str("invalid_suffix"),
        Err(DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidLength))
    );
    // Exactly 26 characters, but the first character exceeds '7'.
    assert_eq!(
        TypeIdSuffix::from_str("80000000000000000000000000"),
        Err(DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidFirstCharacter))
    );
}
```

## Optional Features

### Optional Tracing (`instrument`)

When the `instrument` feature is enabled, the crate will log operations using the `tracing` crate:

```toml
[dependencies]
typeid_suffix = { version = "1.3", features = ["instrument"] }
```

### Serde Support (`serde`)

When the `serde` feature is enabled, `TypeIdSuffix` implements `serde::Serialize` and `serde::Deserialize`. This allows `TypeIdSuffix` instances to be easily serialized to and deserialized from various formats like JSON, YAML, CBOR, etc., that are supported by Serde.

`TypeIdSuffix` is serialized as its string representation and deserialized from a string.

To enable this feature:

```toml
[dependencies]
typeid_suffix = { version = "1.3", features = ["serde"] }
```

**Example:**

```rust
# #[cfg(feature = "serde")] {
use typeid_suffix::prelude::*;
// Note: serde_json is used here as an example and would be a separate dependency.
// Add `serde_json = "1.0"` to your [dependencies] or [dev-dependencies] in Cargo.toml.
use serde_json;

fn serde_example() -> Result<(), serde_json::Error> {
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
# serde_example().unwrap();
# }
```

## Use Cases

- **Distributed Systems**: Generate globally unique, sortable identifiers for distributed systems.
- **Database Systems**: Create compact, base32-encoded identifiers for database records.
- **API Development**: Use `TypeID` suffixes as part of API responses or for resource identification.
- **Time-based Sorting**: Leverage the sortable nature of `UUIDv7`-based `TypeID` suffixes for time-ordered data.

## Safety and Correctness

This crate has been thoroughly tested and verified:

- Comprehensive unit tests
- Property-based testing with `proptest`
- Fuzz testing

These measures ensure that the crate behaves correctly and safely under various inputs and conditions.

## Minimum Supported Rust Version (MSRV)

No minimum Rust version is currently declared in the package manifest. Use the stable toolchain tested by the repository CI.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

This crate implements a portion of the [TypeID Specification](https://github.com/jetpack-io/typeid) created by Jetpack.io.

## Sponsor

Govcraft is a one-person shop, with no corporate backing or investors, just me building useful tools. If this project helps you, [sponsoring](https://github.com/sponsors/Govcraft) keeps the work going.

[![Sponsor on GitHub](https://img.shields.io/badge/Sponsor-%E2%9D%A4-%23db61a2?logo=GitHub)](https://github.com/sponsors/Govcraft)
