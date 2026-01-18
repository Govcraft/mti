# TypeID for Rust: Human-Readable, Type-Safe, Globally Unique Identifiers

[![Crates.io](https://img.shields.io/crates/v/mti.svg)](https://crates.io/crates/mti)
[![Documentation](https://docs.rs/mti/badge.svg)](https://docs.rs/mti)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

Transform opaque UUIDs like `f47ac10b-58cc-4372-a567-0e02b2c3d479` into readable identifiers like `user_01h455vb4pex5vsknk084sn02q` that indicate type and remain globally unique.

## Overview

UUIDs provide global uniqueness but obscure entity types. Debugging production issues requires determining whether `f47ac10b-58cc-4372-a567-0e02b2c3d479` represents a user, order, or payment—impossible without database queries or code searches. TypeIDs solve this by prefixing UUIDs with human-readable type information, transforming opaque identifiers into self-documenting ones like `user_01h455vb4pex5vsknk084sn02q` while maintaining global uniqueness and adding time-sortability through UUIDv7.

This workspace provides three complementary Rust crates for working with TypeIDs:

- **Start in two lines of code** with the high-level `mti` crate
- **Production-ready**: implements [TypeID Specification v0.3.0](https://github.com/jetify-com/typeid), zero unsafe code, extensively tested
- **Time-sortable by default**: UUIDv7 provides database-friendly chronological ordering
- **Modular architecture**: use the full API or just prefix validation or suffix encoding

## Crates

| Crate | Version | Purpose | Use When |
|-------|---------|---------|----------|
| [mti](crates/mti) | [![Crates.io](https://img.shields.io/crates/v/mti.svg)](https://crates.io/crates/mti) | High-level TypeID API with ergonomic creation, parsing, and manipulation | You need complete TypeID functionality (most developers start here) |
| [typeid-prefix](crates/typeid-prefix) | [![Crates.io](https://img.shields.io/crates/v/typeid_prefix.svg)](https://crates.io/crates/typeid_prefix) | Validation and sanitization of TypeID prefixes | You need standalone prefix validation or custom TypeID implementations |
| [typeid-suffix](crates/typeid-suffix) | [![Crates.io](https://img.shields.io/crates/v/typeid_suffix.svg)](https://crates.io/crates/typeid_suffix) | Base32 encoding/decoding of UUID suffixes (26 URL-safe characters) | You need standalone suffix handling or custom encoding schemes |

## Quick Start

Add the main crate to your project:

```toml
[dependencies]
mti = "1.1"
```

Create and parse TypeIDs:

```rust
use mti::prelude::*;

// Create a new TypeID (defaults to UUIDv7 - IDs sort chronologically)
let user_id = "user".create_type_id();
println!("{}", user_id); // user_01h455vb4pex5vsknk084sn02q

// Parse existing TypeIDs
let order_id = MagicTypeId::from_str("order_01h455vb4pex5vsknk084sn02q")?;
assert_eq!(order_id.prefix_str(), "order");
```

## Why TypeIDs

TypeIDs address four common problems with traditional UUID-based identifiers:

**Debuggability**: Raw UUIDs provide no context. In a production log showing `Failed to process f47ac10b-58cc-4372-a567-0e02b2c3d479`, you cannot determine the entity type without querying databases or searching code. TypeIDs make this immediate: `Failed to process user_01h455vb4pex5vsknk084sn02q` shows a user identifier at a glance.

**Type Safety**: Accepting all identifiers as raw UUIDs or strings allows passing a `product_id` where a `user_id` is expected. These errors surface only at runtime. TypeIDs embed type information in the identifier, enabling runtime validation of prefix correctness. Combined with Rust newtypes wrapping `MagicTypeId`, you can prevent mixing different ID types (like `UserId` vs `OrderId`) at compile time.

**Sortability**: Traditional UUIDv4 identifiers are random, causing poor database index locality and making chronological ordering impossible without separate timestamp columns. TypeIDs default to UUIDv7, which encodes creation time in the first 48 bits. Records sort naturally by insertion time, improving clustered index performance and simplifying time-based queries.

**Consistency**: Without a standard, teams invent ad-hoc schemes: prefixed strings, composite keys, or UUID columns with separate type fields. TypeIDs provide a specification-backed format that works across services, languages, and systems. The format `prefix_suffix` is simple, unambiguous, and increasingly adopted.

## Architecture

The three crates compose cleanly:

```
┌─────────────────────────────────────────────┐
│             mti (High-Level API)            │
│  ┌───────────────────────────────────────┐  │
│  │         MagicTypeId                   │  │
│  │  - create_type_id()                   │  │
│  │  - parse from strings                 │  │
│  │  - string-like operations             │  │
│  └────────────┬──────────────┬───────────┘  │
└───────────────┼──────────────┼──────────────┘
                │              │
      ┌─────────▼────┐    ┌────▼─────────┐
      │ typeid-prefix│    │ typeid-suffix│
      ├──────────────┤    ├──────────────┤
      │ TypeIdPrefix │    │ TypeIdSuffix │
      │              │    │              │
      │ • Validates  │    │ • Base32     │
      │   prefixes   │    │   encoding   │
      │ • Sanitizes  │    │ • UUID       │
      │   input      │    │   versions   │
      │ • Max 63     │    │ • 26 chars   │
      │   chars      │    │   (128 bits) │
      └──────────────┘    └──────────────┘
```

This separation allows using `typeid-prefix` for input validation without UUID generation, or `typeid-suffix` for base32 encoding in non-TypeID contexts. Most developers need only `mti`.

## Key Features

**Specification Compliance**: Implements TypeID v0.3.0 from Jetify, ensuring format compatibility with other TypeID implementations across languages and platforms—your Rust services can exchange TypeIDs with TypeScript, Go, or Python services.

**UUID Version Support**: Handles UUIDv1, v3, v4, v5, v6, v7, and Nil. Defaults to v7 for new identifiers, providing time-sortability and global uniqueness. Use v5 for deterministic IDs (same input always produces same ID) and v4 for simple random identifiers. Built-in `NamespaceId` with RFC 4122 constants (DNS, URL, OID, X500) makes creating deterministic IDs ergonomic—no need to add `uuid` as a direct dependency.

**Zero Unsafe Code**: All three crates forbid unsafe code via `#![deny(unsafe_code)]`, eliminating entire classes of memory safety vulnerabilities. Safety is guaranteed by the compiler, not runtime checks—critical for production systems handling sensitive identifiers.

**Optional Serde Support**: Enable the `serde` feature to serialize and deserialize TypeIDs as JSON, YAML, TOML, or any format Serde supports. TypeIDs serialize as their string representation for human-readable output.

**Optional Tracing**: Enable the `instrument` feature to emit trace events for creation, parsing, and validation, making production debugging easier when you need to understand ID generation patterns.

**Comprehensive Testing**: Each crate includes unit tests, property-based tests with `proptest`, specification compliance tests, and fuzz testing. The test suites verify prefix rules, suffix encoding, roundtrip conversions, and edge cases.

## Usage Examples

### Creating TypeIDs with Different UUID Versions

```rust
use mti::prelude::*;

// UUIDv7 (time-sortable, recommended for most use cases)
let product_id = "product".create_type_id(); // v7 is default
println!("{}", product_id); // product_01h455vb4pex5vsknk084sn02q

// UUIDv4 (random, no timestamp - use when creation time shouldn't be inferable)
let session_id = "session".create_type_id::<V4>();

// UUIDv5 (deterministic - same namespace+name always produces same ID)
// Use for content-addressable storage or idempotent operations
let domain_id = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
println!("{}", domain_id); // Always the same for this namespace+name

// Well-known namespaces: DNS, URL, OID, X500 (RFC 4122)
let url_id = "page".create_type_id_v5(NamespaceId::URL, b"https://example.com/about");

// Custom namespace from UUID string
let custom_ns = NamespaceId::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8")?;
let resource_id = "resource".create_type_id_v5(custom_ns, b"unique_resource_name");
```

### Parsing and Extracting Components

```rust
use mti::prelude::*;

let id_str = "order_01h455vb4pex5vsknk084sn02q";
let order_id = MagicTypeId::from_str(id_str)?;

assert_eq!(order_id.prefix_str(), "order");
assert_eq!(order_id.suffix_str(), "01h455vb4pex5vsknk084sn02q");

// Extract the underlying UUID
let uuid = order_id.uuid()?;
println!("UUID: {}", uuid);
```

### Building Type Safety with Newtypes

Wrap `MagicTypeId` in domain-specific types to prevent mixing different ID types at compile time:

```rust
use mti::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserId(MagicTypeId);

impl UserId {
    fn new() -> Self {
        Self("user".create_type_id())
    }

    fn parse(s: &str) -> Result<Self, MagicTypeIdError> {
        let id = MagicTypeId::from_str(s)?;
        if id.prefix_str() != "user" {
            return Err(MagicTypeIdError::Validation(
                "Expected 'user' prefix".to_string()
            ));
        }
        Ok(Self(id))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderId(MagicTypeId);

impl OrderId {
    fn new() -> Self {
        Self("order".create_type_id())
    }
}

fn process_user(id: UserId) {
    println!("Processing user: {}", id.0);
}

let user_id = UserId::new();
let order_id = OrderId::new();

process_user(user_id); // Compiles
// process_user(order_id); // Compile error: expected `UserId`, found `OrderId`
```

## When to Use Each Crate

| Scenario | Use Crate | Reason |
|----------|-----------|---------|
| Building application with TypeID identifiers | `mti` | Complete API handles all common operations |
| Validating user input for prefix-only fields | `typeid-prefix` | Lightweight validation without UUID dependencies |
| Custom identifier format using base32 encoding | `typeid-suffix` | Reuse encoding logic in different contexts |
| Implementing custom TypeID variant | `typeid-prefix` + `typeid-suffix` | Build on validated components with custom logic |
| Adding TypeIDs to existing UUID-based system | `mti` | Drop-in replacement with migration path |

## Development

Build all crates:

```bash
cargo build --workspace
```

Run tests:

```bash
cargo nextest run --workspace
```

Check with clippy:

```bash
cargo clippy --workspace --all-features
```

Format code:

```bash
cargo fmt --workspace
```

## Workspace Structure

```
mti/
├── Cargo.toml              # Workspace configuration
├── README.md               # This file
└── crates/
    ├── mti/                # High-level TypeID API
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── src/
    │   └── tests/
    ├── typeid-prefix/      # Prefix validation and sanitization
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── src/
    │   ├── tests/
    │   └── fuzz/
    └── typeid-suffix/      # Suffix encoding and decoding
        ├── Cargo.toml
        ├── README.md
        ├── src/
        ├── tests/
        └── fuzz/
```

## Contributing

Contributions are welcome. This project follows these standards:

- **Conventional Commits**: All commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.
- **No Unsafe Code**: The workspace denies unsafe code. Submit safe implementations.
- **Linting**: Code must pass `cargo clippy` with no warnings. Fix underlying issues rather than suppressing lints.
- **Testing**: Add tests for new features. Maintain or improve code coverage.
- **Documentation**: Public APIs require documentation. Add examples for complex features.

Submit pull requests to the `main` branch. The CI pipeline checks formatting, linting, tests, and documentation builds.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Acknowledgments

This implementation follows the TypeID Specification v0.3.0 created and maintained by [Jetify](https://www.jetify.com/). The specification defines the format, validation rules, and encoding schemes implemented in these crates.
