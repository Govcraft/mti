# Magic Type ID (MTI): Empowering Distributed Systems with Intelligent Identifiers

[![Crates.io](https://img.shields.io/crates/v/mti.svg)](https://crates.io/crates/mti)
[![Documentation](https://docs.rs/mti/badge.svg)](https://docs.rs/mti)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

Welcome to `mti`, a Rust crate that brings the power of type-safe, prefix-enhanced identifiers to your distributed
systems. Built on the [TypeID Specification](https://github.com/jetpack-io/typeid), `mti` combines the uniqueness of
UUIDs with the readability and type safety of prefixed identifiers, offering a robust solution for managing identifiers
across your applications.

## Acknowledgments

This crate implements version 3 of the [TypeID Specification](https://github.com/jetpack-io/typeid) created and
maintained by [Jetpack](https://www.jetpack.io/). I'm grateful for their work in developing and managing this
specification.

## Features

- **Type Safety**: Embed type information directly in your identifiers.
- **Readability**: Human-readable prefixes make identifiers self-descriptive.
- **Uniqueness**: Utilizes UUIDs (including UUIDv7) for guaranteed global uniqueness.
- **Sortability**: When using UUIDv7, identifiers are inherently time-sortable.
- **Flexibility**: Support for various UUID versions and custom prefixes.
- **Ease of Use**: Intuitive API with extension methods for effortless creation and manipulation.
- **Performance**: Zero-cost abstractions for string-like operations.
- **Reliability**: Built on thoroughly tested and verified `TypeIdPrefix` and `TypeIdSuffix` crates.

## Quick Start

Add `mti` to your `Cargo.toml`:

```toml
[dependencies]
mti = "1.0.0-beta.1"
```

Then, in your Rust code:

```rust
use mti::prelude::*;

// Create a MagicTypeId for a user
let user_id = "user".create_type_id::<V7>();
println!("New User ID: {}", user_id); // e.g., "user_01h455vb4pex5vsknk084sn02q"

// Parse an existing MagicTypeId
let order_id = MagicTypeId::from_str("order_01h455vb4pex5vsknk084sn02q").unwrap();
assert_eq!(order_id.prefix().as_str(), "order");
```

## Usage Examples

### Creating MagicTypeIds

```rust
use mti::prelude::*;

// Create with UUIDv7 (sortable, recommended)
let product_id = "product".create_type_id::<V7>();

// Create with UUIDv4 (random)
let user_id = "user".create_type_id::<V4>();

// Create with custom suffix
let custom_suffix = TypeIdSuffix::new::<V7>();
let order_id = "order".create_type_id_with_suffix(custom_suffix);
```

### Flexible Prefix Handling

```rust
use mti::prelude::*;

// Sanitized creation (always produces a valid prefix)
let sanitized_id = "Invalid Prefix!".create_type_id::<V7>();
assert!(sanitized_id.to_string().starts_with("invalidprefix_"));

// Strict creation (returns an error for invalid prefixes)
let result = "Invalid Prefix!".try_create_type_id::<V7>();
assert!(result.is_err());
```

### String-Like Behavior

```rust
use mti::prelude::*;

let id = "user".create_type_id::<V7>();
assert!(id.starts_with("user_"));
assert_eq!(id.len(), 31);

// Use in string comparisons
assert_eq!(id.as_str(), id.to_string());
```

### Parsing and Component Extraction

```rust
use mti::prelude::*;

let id_str = "product_01h455vb4pex5vsknk084sn02q";
let magic_id = MagicTypeId::from_str(id_str).unwrap();

assert_eq!(magic_id.prefix().as_str(), "product");
assert_eq!(magic_id.suffix().to_string(), "01h455vb4pex5vsknk084sn02q");

// Extract UUID
let uuid = magic_id.suffix().to_uuid();
println!("Extracted UUID: {}", uuid);
```
### Sorting
When `MagicTypeId` is created with a `V7` UUID, it provides a natural sorting order:
1. **Primary Sorting**: By the timestamp in the `UUIDv7` suffix. This means that identifiers
   generated later will appear after those generated earlier.
2. **Secondary Sorting**: If the timestamps are equal, then sorting is based on the lexicographical order
   of the prefix. This ensures consistent ordering even when identifiers are created at the same time.
```rust
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use mti::prelude::*;
use typeid_prefix::prelude::*;
use typeid_suffix::prelude::*;

let prefix1 = TypeIdPrefix::from_str("user").unwrap();
let prefix2 = TypeIdPrefix::from_str("admin").unwrap();
let id1 = MagicTypeId::new(prefix1.clone(), TypeIdSuffix::new::<V7>());

sleep(Duration::from_millis(10));  // Ensure different timestamps

let id2 = MagicTypeId::new(prefix1.clone(), TypeIdSuffix::new::<V7>());
let id3 = MagicTypeId::new(prefix2.clone(), TypeIdSuffix::from_str(&id2.suffix().to_string()).unwrap());

assert!(id1 < id2, "Expected id1 to be less than id2 due to earlier timestamp");
assert_eq!(id2.suffix(), id3.suffix(), "Suffixes for id2 and id3 should be the same");
assert!(id3 < id2, "Expected id3 to be less than id2 due to lexicographically smaller prefix when timestamps are equal");
```
## Use Cases

MagicTypeId is versatile and can be applied in various scenarios:

1. **Distributed Systems**: Generate globally unique, type-safe identifiers across microservices.
   ```rust
   let order_id = "order".create_type_id::<V7>();
   // Send to another service: "order_01h455vb4pex5vsknk084sn02q"
    ```
2. **Database Records**: Create readable, sortable primary keys.

``` rust
let user_id = "user".create_type_id::<V7>();
// MagicTypeIds behave like strings
db.insert_user(user_id, user_data);
```

3. **API Development**: Use as resource identifiers in REST or GraphQL APIs.

```rust
#[get("/users/{id}")]
async fn get_user(id: Path<MagicTypeId>) -> impl Responder {
    // Retrieve user with id
}
```

4. **Non-unique, Repeatable IDs**: Use UUIDv5 for generating consistent IDs based on input.

```rust
let namespace = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
let name = "example.com";
let v5_uuid = Uuid::new_v5( & namespace, name.as_bytes());
let domain_id = MagicTypeId::new(
TypeIdPrefix::from_str("domain").unwrap(),
TypeIdSuffix::from(v5_uuid)
);
// Always produces the same ID for "example.com"
assert_eq!(domain_id.uuid_str().unwrap(), "cfbff0d1-9375-5685-968c-48ce8b15ae17");
```

5. **Logging and Tracing**: Embed type information in log entries for easier debugging.

```rust 
log::info!("Processing order {}", "order".create_type_id::<V7>());
```

## Advanced Usage

### Custom Type-Safe ID Types

```rust
use mti::prelude::*;

struct UserId(MagicTypeId);

struct OrderId(MagicTypeId);

impl UserId {
    fn new() -> Self {
        Self("user".create_type_id::<V7>())
    }
}

impl OrderId {
    fn new() -> Self {
        Self("order".create_type_id::<V7>())
    }
}

// Compile-time type safety
fn process_user(id: UserId) { /* ... */ }

fn process_order(id: OrderId) { /* ... */ }

let user_id = UserId::new();
let order_id = OrderId::new();

process_user(user_id);
process_order(order_id);
// process_user(order_id); // This would cause a compile-time error!
```

### Database Integration

```rust
use mti::prelude::*;

#[derive(Debug)]
struct User {
    id: MagicTypeId,
    name: String,
}

fn create_user(name: &str) -> User {
    User {
        id: "user".create_type_id::<V7>(),
        name: name.to_string(),
    }
}

// In your database operations
let user = create_user("Alice");
// MagicTypeId behaves like a string
db.insert(user.id, & user);

// Retrieving
let retrieved_id = MagicTypeId::from_str("user_01h455vb4pex5vsknk084sn02q").unwrap();
let retrieved_user = db.get::<User>(retrieved_id.to_string());
```

## Performance and Safety

`mti` is designed with performance and safety in mind:

- Zero-cost abstractions for string-like operations.
- Built on top of the thoroughly tested and verified `TypeIdPrefix` and `TypeIdSuffix` crates.
- Extensive use of Rust's type system to prevent errors at compile-time.
- Comprehensive test suite ensuring reliability and correctness.

## Beta Status

While this crate is feature-complete and thoroughly tested, it is currently in beta to gather wider community feedback.
I encourage you to use it in your projects and provide feedback. If you encounter any issues or have suggestions,
please file them on my [GitHub repository](https://github.com/GovCraft/mti/issues).

## Contributing

I welcome contributions! Please see my [GitHub repository](https://github.com/GovCraft/mti) for issues, feature
requests, and pull requests.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))

at your option.

---

## About the Author

I'm [@rrrodzilla](https://github.com/rrrodzilla), a technologist with 30 years of industry experience. I'm a former SOA and cloud architect, and former Principal Technical Product Manager at AWS for the Rust Programming Language. Currently, I'm the owner and operator of Govcraft, building and consulting on Rust and AI solutions.

For more information, visit [https://www.govcraft.ai](https://www.govcraft.ai)
