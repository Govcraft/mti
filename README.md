# Magic Type ID (MTI): Type-Safe, Human-Readable, Unique Identifiers for Rust

[![Crates.io](https://img.shields.io/crates/v/mti.svg)](https://crates.io/crates/mti)
[![Documentation](https://docs.rs/mti/badge.svg)](https://docs.rs/mti)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/GovCraft/mti.svg?style=social&label=Star)](https://github.com/GovCraft/mti)

Are you looking for a better way to handle identifiers in your Rust applications? Do opaque UUIDs make your logs, databases, and APIs harder to understand? Magic Type ID (MTI) offers a solution.
It provides identifiers that look like `user_01h455vb4pex5vsknk084sn02q` or `order_01h455vb4pex5vsknk084sn02q` – combining a human-readable prefix indicating the type of entity with a globally unique, sortable (by default) suffix. This approach, inspired by conventions like Stripe's API IDs, offers immediate clarity.

`mti` is a Rust crate that provides **type-safe, human-readable, and globally unique identifiers**. Based on the [TypeID Specification](https://github.com/jetify-com/typeid), MTI enhances UUIDs with descriptive prefixes. This improves debuggability, code reliability, and simplifies identifier management in various applications.

**⭐ If you find MTI useful, please consider starring the repository on [GitHub](https://github.com/GovCraft/mti)! ⭐**

## Why Use MTI? Solved Problems & Key Benefits

MTI addresses common challenges developers encounter with traditional identifiers:

*   **Problem: Opaque and Confusing IDs.**
    *   **MTI Solution:** Human-readable prefixes (e.g., `user_`, `order_`) make identifiers instantly recognizable, aiding in debugging and log analysis. You no longer need to guess what an ID like `f47ac10b-58cc-4372-a567-0e02b2c3d479` refers to.
*   **Problem: Risk of ID Misuse.**
    *   **MTI Solution:** Embedding type information directly into the ID helps prevent runtime errors where an ID for one entity (e.g., a `product_id`) is mistakenly used for another (e.g., a `user_id`). This can be further enforced by creating custom newtypes around `MagicTypeId`.
*   **Problem: Ensuring ID Uniqueness and Integrity.**
    *   **MTI Solution:** Provides a standardized, globally unique, and self-descriptive ID format that simplifies data integrity across different parts of an application or between systems.
*   **Problem: Complex ID Generation & Sorting.**
    *   **MTI Solution:** Offers an intuitive API for generating various UUID versions (including time-sortable UUIDv7 by default), abstracting the complexities of UUID generation and ensuring consistency.

**Key Benefits for Developers:**

*   **Enhanced Readability & Debuggability:** Instantly understand the type and context of an identifier.
*   **Improved Type Safety:** Reduce the risk of logical errors by making ID types explicit.
*   **Simplified Development:** Easy-to-use API for creating, parsing, and manipulating TypeIDs.
*   **Global Uniqueness:** Leverages the power of UUIDs.
*   **Inherent Sortability (with UUIDv7):** Generate IDs that can be naturally sorted by time, useful for event streams and ordered data.
*   **Performance-Oriented:** Designed with zero-cost abstractions for common string-like operations.
*   **Specification Adherence:** Implements the [TypeID Specification](https://github.com/jetify-com/typeid) for potential interoperability.
*   **Robust & Reliable:** Built on the `TypeIdPrefix` and `TypeIdSuffix` crates.

## Understanding UUID Versions in MTI

MTI supports several UUID versions, each with distinct characteristics and advantages. The choice of UUID version impacts properties like sortability and determinism:

*   **UUIDv7 (Time-Ordered - Default for new IDs):**
    *   **How it works:** Combines a high-precision Unix timestamp (usually milliseconds) with random bits to ensure uniqueness.
    *   **Advantages:**
        *   **Sortable by Time:** IDs are k-sortable by creation time, which is beneficial for database indexing (improving insert performance for clustered indexes) and for ordering records chronologically (e.g., event logs, audit trails).
        *   Globally unique.
    *   **Use Cases:** Primary keys in databases, event identifiers, any scenario where chronological ordering is valuable.

*   **UUIDv4 (Random):**
    *   **How it works:** Generated from purely random or pseudo-random numbers.
    *   **Advantages:**
        *   Simple to generate and globally unique.
        *   No information leakage (e.g., creation time).
    *   **Disadvantages:** Not time-sortable, which can lead to performance issues with database indexing on these IDs if they are primary keys in large, frequently inserted tables.
    *   **Use Cases:** General-purpose unique identifiers where time-ordering is not a requirement.

*   **UUIDv5 (Name-Based, SHA-1 Hashed):**
    *   **How it works:** Generates a deterministic UUID based on a "namespace" UUID and a "name" (a string). The same namespace and name will always produce the same UUIDv5.
    *   **Advantages:**
        *   **Deterministic:** Useful when you need to consistently derive the same ID for the same input, e.g., for content-addressable storage or identifying resources by a unique name.
        *   Globally unique within the combination of namespace and name.
    *   **Disadvantages:** Relies on SHA-1 (though for collision resistance in UUIDs, it's generally considered acceptable). Not time-sortable.
    *   **Use Cases:** Generating stable identifiers for files based on their content, deriving IDs for entities based on unique business keys, de-duplication.

MTI defaults to UUIDv7 for `create_type_id()` due to its excellent balance of global uniqueness and time-sortability, which covers a wide range of common application needs. You can explicitly choose other versions when specific characteristics are required.
Beyond V4, V5, and V7, which are commonly used and detailed above, the underlying `typeid_suffix` crate (and thus MTI through manual construction with `TypeIdSuffix::new::<V>()` and `MagicTypeId::new()`) also supports other standard UUID versions. These include `Nil` (for all-zero UUIDs), `V1` (traditional time-based), `V3` (name-based using MD5), and `V6` (another time-based variant). While these versions are available for specific needs, V7 is generally recommended for new time-based requirements, and V5 for SHA-1 based named requirements, due to their modern advantages.

## Quick Start

Add `mti` to your `Cargo.toml`:

```toml
[dependencies]
mti = "1.0" # Or the latest version
```

**Optional Serde Support:**

If you need to serialize or deserialize `MagicTypeId` instances (e.g., for use with Serde-compatible formats like JSON, YAML, etc.), enable the `serde` feature flag:

```toml
[dependencies]
mti = { version = "1.0", features = ["serde"] } # Or the latest version, ensure to match the version above
```
This will enable Serde's `Serialize` and `Deserialize` traits for `MagicTypeId`.

**Optional Tracing Instrumentation:**

For detailed operational insights, `mti` supports instrumentation via the [`tracing`](https://crates.io/crates/tracing) crate. When enabled, `mti` will emit trace events for key operations like ID creation and parsing. This is invaluable for debugging, performance analysis, and understanding the crate's behavior within your application.

To enable this feature, add `instrument` to the `features` list in your `Cargo.toml`:

```toml
[dependencies]
mti = { version = "1.0", features = ["instrument"] } # Or your current version

# Your application will also need a tracing subscriber
tracing = "0.1" # The tracing facade
tracing-subscriber = { version = "0.3", features = ["fmt"] } # Example subscriber
```

**How it Works:**

When the `instrument` feature is active, `mti` functions are annotated with `#[instrument(...)]` and contain `trace!`, `debug!`, etc., calls from the `tracing` crate. Your application can then configure a `tracing` subscriber (like `tracing-subscriber`) to collect, filter, format, and output these trace events. This gives you control over the level of detail and destination of the trace data (e.g., console, file, or a distributed tracing system).

*Example of setting up a basic subscriber in your application:*

```rust
// In your application's main.rs or initialization code:
use tracing_subscriber::fmt::format::FmtSpan;
use mti::prelude::*; // For create_type_id and MagicTypeId

fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // Adjust level as needed (TRACE, DEBUG, INFO, etc.)
        .with_span_events(FmtSpan::CLOSE)      // Configure span event reporting
        .init();                               // Initialize the global subscriber
}

fn main() {
    setup_tracing(); // Call this early in your application

    // Operations in mti will now emit traces if the "instrument" feature is enabled
    let product_id = "product".create_type_id::<V7>(); // This will be traced
    println!("Product ID: {}", product_id);

    match MagicTypeId::from_str("test_01h2xcejqg4wh1r27hsdgzeqp4") { // This parsing will be traced
        Ok(id) => println!("Parsed ID: {}", id),
        Err(e) => eprintln!("Error parsing ID: {}", e),
    }
}
```
This setup allows the host application to effectively leverage the instrumentation within `mti`.

Then, in your Rust code:

```rust
use mti::prelude::*;

// Create a MagicTypeId for a user (defaults to UUIDv7)
let user_id = "user".create_type_id();
println!("New User ID: {}", user_id); // e.g., "user_01h455vb4pex5vsknk084sn02q"

// Parse an existing MagicTypeId
let order_id_str = "order_01h455vb4pex5vsknk084sn02q";
match MagicTypeId::from_str(order_id_str) {
    Ok(order_id) => {
        assert_eq!(order_id.prefix_str(), "order");
        println!("Parsed Order ID: {}", order_id);
    }
    Err(e) => {
        eprintln!("Failed to parse TypeID '{}': {}", order_id_str, e);
    }
}
```

## Core Features

*   **Type-Safe Prefixes**: Embed type information directly in your identifiers (e.g., `user_`, `product_`).
    *   *Benefit:* Helps prevent accidental misuse of IDs, making your system more robust.
*   **Human-Readable Identifiers**: Prefixes make IDs self-descriptive and easy to understand at a glance.
    *   *Benefit:* Aids in debugging, log analysis, and database inspection.
*   **UUID-Backed Uniqueness**: Utilizes various UUID versions (V4, V5, V7) for global uniqueness. UUIDv7 is the default for new ID generation. (See "Understanding UUID Versions in MTI" above for details).
    *   *Benefit:* Helps eliminate ID collisions.
*   **Time-Sortable (with UUIDv7)**: Identifiers generated with UUIDv7 are inherently sortable by time.
    *   *Benefit:* Simplifies ordering of events, records, and logs chronologically.
*   **Flexible Prefix Handling**: Supports both strict and sanitized prefix creation.
    *   *Benefit:* Adapt to various input requirements while maintaining valid TypeID formats.
*   **Intuitive API**: Ergonomic methods for creation, parsing, and manipulation.
    *   *Benefit:* Easy to integrate and use, reducing boilerplate code.
*   **Zero-Cost Abstractions**: Efficient string-like operations without performance overhead.
    *   *Benefit:* Good performance for critical path operations.

*   **Optional Serde Support**: Easily serialize and deserialize `MagicTypeId` instances using Serde by enabling the `serde` feature flag.
    *   *Benefit:* Seamless integration with common serialization formats like JSON, YAML, TOML, etc., for data interchange and storage.

*   **Optional Tracing Instrumentation**: Enables detailed operational tracing using the `tracing` crate when the `instrument` feature is active.
    *   *Benefit:* Provides deep insights into the crate's internal workings for debugging and performance analysis, configurable by the host application's `tracing` subscriber.

## Usage Examples

### Creating MagicTypeIds with Different UUID Versions

```rust
use mti::prelude::*;
use uuid::Uuid; // For UUIDv5 example

// Create with UUIDv7 (sortable, recommended default)
let product_id_v7 = "product".create_type_id(); // V7 is default
// Or be explicit:
let explicit_product_id_v7 = "product".create_type_id::<V7>();
println!("Product ID (UUIDv7): {}", product_id_v7);

// Create with UUIDv4 (random)
let user_id_v4 = "user".create_type_id::<V4>();
println!("User ID (UUIDv4): {}", user_id_v4);

// Create with UUIDv5 (name-based, deterministic)
let prefix_for_v5 = "resource";
let namespace_uuid = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(); // Example namespace
let name_to_identify = "my_unique_resource_name";

// 1. Generate the UUIDv5
let generated_v5_uuid = Uuid::new_v5(&namespace_uuid, name_to_identify.as_bytes());

// 2. Create the TypeIdPrefix (can be sanitized or strict)
// Using sanitized prefix creation for this example:
let type_id_prefix_v5 = TypeIdPrefix::create_sanitized(prefix_for_v5);
//    Alternatively, for a strict prefix (will error if prefix_for_v5 is invalid):
//    let type_id_prefix_v5 = TypeIdPrefix::try_from(prefix_for_v5).expect("Prefix should be valid");

// 3. Create the TypeIdSuffix from the generated UUID
let type_id_suffix_v5 = TypeIdSuffix::from(generated_v5_uuid);

// 4. Combine them into a MagicTypeId
let resource_id_v5 = MagicTypeId::new(type_id_prefix_v5, type_id_suffix_v5);

println!("Resource ID (UUIDv5 for '{}'): {}", name_to_identify, resource_id_v5);

// Create with a specific suffix (if you already have a TypeIdSuffix)
// Note: The generic V on create_type_id_with_suffix is not used for suffix generation
// when a suffix is already provided, but a valid UuidVersion type is still needed.
let existing_suffix = TypeIdSuffix::new::<V7>(); // Example: new V7 suffix
let order_id_custom = "order".create_type_id_with_suffix::<V7>(existing_suffix);
println!("Order ID (custom suffix): {}", order_id_custom);
```

### Flexible Prefix Handling

```rust
use mti::prelude::*;

// Sanitized creation (attempts to produce a valid prefix from potentially invalid input)
let sanitized_id = "Invalid Prefix!".create_type_id(); // Defaults to V7
assert!(sanitized_id.to_string().starts_with("invalidprefix_"));
assert_eq!(sanitized_id.prefix_str(), "invalidprefix");

// Strict creation (returns an error for invalid prefixes)
let result = "Invalid Prefix!".try_create_type_id::<V7>();
assert!(result.is_err());
println!("Error for invalid prefix: {:?}", result.err());
```

### String-Like Behavior

```rust
use mti::prelude::*;

let id = "user".create_type_id(); // Defaults to V7
assert!(id.starts_with("user_"));
assert_eq!(id.prefix_str(), "user");
// Length can vary slightly based on prefix length, but suffix is fixed for a given UUID version's encoding.
// For a 4-char prefix and standard TypeID base32 encoding of a 128-bit UUID:
// Prefix (4) + Underscore (1) + Suffix (26 for V7/V4) = 31 characters
assert_eq!(id.len(), 31);

// Use in string comparisons
assert_eq!(id.as_str(), id.to_string());
```

### Parsing and Component Extraction

```rust
use mti::prelude::*;

let id_str = "product_01h455vb4pex5vsknk084sn02q";
let magic_id = MagicTypeId::from_str(id_str).unwrap();

assert_eq!(magic_id.prefix_str(), "product");
assert_eq!(magic_id.suffix_str(), "01h455vb4pex5vsknk084sn02q");

// Extract UUID
// You can get the underlying `uuid::Uuid` by accessing the suffix:
let uuid_val = magic_id.suffix().to_uuid();
println!("Extracted UUID via suffix(): {}", uuid_val);

// Alternatively, if the `MagicTypeIdExt` trait is in scope (e.g., via `use mti::prelude::*`),
// you can call `uuid()` or `uuid_str()` directly on the MagicTypeId instance.
// These methods return a Result, as parsing could fail if the TypeID string was malformed
// (though `magic_id` here is known to be valid).
let direct_uuid_res = magic_id.uuid(); // Returns Result<Uuid, _>
if let Ok(direct_uuid) = direct_uuid_res {
    println!("Extracted UUID directly: {}", direct_uuid);
    assert_eq!(uuid_val, direct_uuid);
}

let direct_uuid_str_res = magic_id.uuid_str(); // Returns Result<String, _>
if let Ok(direct_uuid_str) = direct_uuid_str_res {
    println!("Extracted UUID string directly: {}", direct_uuid_str);
}

// You can check its version if needed
// println!("UUID Version: {:?}", uuid_val.get_version());
```
### Sorting
When `MagicTypeId` is created with a `V7` UUID, it provides a natural sorting order:
1. **Primary Sorting**: By the timestamp in the `UUIDv7` suffix. This means that identifiers
   generated later will appear after those generated earlier.
2. **Secondary Sorting**: If the timestamps are equal (unlikely with UUIDv7's millisecond precision and random bits), then sorting is based on the lexicographical order
   of the prefix. This ensures consistent ordering.
```rust
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use mti::prelude::*;
use typeid_prefix::TypeIdPrefix; // Specific import for clarity
use typeid_suffix::TypeIdSuffix; // Specific import for clarity

let prefix1 = TypeIdPrefix::from_str("user").unwrap();
let prefix2 = TypeIdPrefix::from_str("admin").unwrap();

// Generate with default V7
let id1 = MagicTypeId::new(prefix1.clone(), TypeIdSuffix::new::<V7>());

sleep(Duration::from_millis(10));  // Ensure different timestamps

let id2 = MagicTypeId::new(prefix1.clone(), TypeIdSuffix::new::<V7>());
// Create id3 with the same suffix as id2 but different prefix for testing secondary sort
let id3_suffix_val = id2.suffix().clone();
let id3 = MagicTypeId::new(prefix2.clone(), id3_suffix_val);


assert!(id1 < id2, "Expected id1 ({}) to be less than id2 ({}) due to earlier timestamp", id1, id2);

// For secondary sort demonstration, ensure suffixes are identical if timestamps were hypothetically the same
// In practice, UUIDv7 makes identical timestamps + random bits extremely rare.
// Here, we explicitly made id3's suffix same as id2's for this test.
assert_eq!(id2.suffix_str(), id3.suffix_str(), "Suffixes for id2 and id3 should be the same for this test case");
assert!(id3 < id2, "Expected id3 ({}) to be less than id2 ({}) due to lexicographically smaller prefix ('admin' < 'user') when timestamps (and thus suffixes) are equal", id3, id2);
```

## Use Cases: Where MTI Shines

MagicTypeId is versatile and improves clarity and safety in various scenarios:

1.  **Applications with Multiple Components or Services**: Generate globally unique, type-aware identifiers that flow understandably across different parts of your application or between services.
    *Why it helps:* Reduces ambiguity when correlating events or entities.
    ```rust
    # use mti::prelude::*;
    let order_id = "order".create_type_id();
    // Send to another component/service: e.g., "order_01h455vb4pex5vsknk084sn02q"
    // The receiving part immediately knows it's an order ID.
    println!("Order ID for processing: {}", order_id);
    ```
2.  **Database Records**: Create human-readable, sortable (with UUIDv7), and type-prefixed primary or secondary keys.
    *Why it helps:* Makes database browsing and debugging easier. `user_01arZ...` is more informative than a raw UUID.
    ```rust
    # use mti::prelude::*;
    # #[derive(Debug)] struct User { id: MagicTypeId, name: String }
    # trait Db { fn insert_user(&self, id: MagicTypeId, name: &str); }
    # struct DummyDb; impl Db for DummyDb { fn insert_user(&self, id: MagicTypeId, name: &str) { println!("Inserting user {} with id {}", name, id); } }
    # let db = DummyDb;
    # let user_data = "Alice";
    let user_id = "user".create_type_id();
    // MagicTypeIds can be stored as strings in most databases
    db.insert_user(user_id, user_data);
    ```
3.  **API Development (REST/GraphQL)**: Use as clear, self-documenting resource identifiers in your API endpoints and payloads.
    *Why it helps:* Improves API ergonomics for both producers and consumers.
    ```rust
    # use mti::prelude::*;
    # #[cfg(feature = "actix_web")] // Example, not a real feature here
    # use actix_web::{get, web::Path, Responder, HttpResponse};
    # #[cfg(feature = "actix_web")]
    # async fn get_user_handler(id: Path<MagicTypeId>) -> impl Responder {
    #     // Ensure the prefix is 'user' if needed, or handle generically
    #     if id.prefix_str() == "user" {
    #         println!("Fetching user with ID: {}", *id);
    #         HttpResponse::Ok().body(format!("User data for {}", *id))
    #     } else {
    #         HttpResponse::BadRequest().body("Invalid ID type for user endpoint")
    #     }
    # }
    // Example (conceptual, actual web framework integration may vary):
    // #[get("/users/{id}")]
    // async fn get_user(id: Path<MagicTypeId>) -> impl Responder { /* ... */ }
    println!("API endpoint might look like: /users/user_01h455vb4pex5vsknk084sn02q");
    ```
4.  **Logging and Tracing**: Embed type information directly in log entries for improved debugging and event correlation.
    *Why it helps:* Quickly filter and understand logs related to specific entity types.
    ```rust
    # use mti::prelude::*;
    # fn process_order(order_id: MagicTypeId) {
    #     // Simulate logging
    #     println!("[INFO] Processing order {}", order_id);
    #     if order_id.prefix_str() != "order" {
    #         eprintln!("[WARN] Expected an order ID, but got: {}", order_id);
    #     }
    # }
    let order_id = "order".create_type_id();
    process_order(order_id);
    // log::info!("Processing order {}", "order".create_type_id());
    ```
5.  **Content-Addressable Storage (with UUIDv5)**: Generate deterministic, repeatable IDs based on content or unique names within a namespace. (See "Understanding UUID Versions in MTI" for more on UUIDv5).
    *Why it helps:* Useful for de-duplication or creating stable identifiers for specific data.
    ```rust
    # use mti::prelude::*; // Brings in MagicTypeId, TypeIdPrefix, TypeIdSuffix, etc.
    # use uuid::Uuid;    // For Uuid::new_v5 and Uuid::parse_str

    let prefix_str = "domain";
    let namespace = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(); // Example namespace
    let name = "example.com";

    // 1. Generate the UUIDv5
    let v5_uuid = Uuid::new_v5(&namespace, name.as_bytes());

    // 2. Create TypeIdPrefix
    let type_prefix = TypeIdPrefix::create_sanitized(prefix_str);
    //    Or for a strict prefix:
    //    let type_prefix = TypeIdPrefix::try_from(prefix_str).unwrap();

    // 3. Create TypeIdSuffix from the UUID
    let type_suffix = TypeIdSuffix::from(v5_uuid);

    // 4. Create MagicTypeId
    let domain_id = MagicTypeId::new(type_prefix, type_suffix);

    // Always produces the same ID for "example.com" within this namespace
    // The specific suffix depends on the namespace and name.
    println!("Content-addressable ID for '{}': {}", name, domain_id);
    ```

## Advanced Usage: Enhancing Type Safety with Newtypes

For stronger compile-time guarantees, wrap `MagicTypeId` in your own domain-specific ID types:

```rust
use mti::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] // Add necessary derives
struct UserId(MagicTypeId);

impl UserId {
    fn new() -> Self {
        // Enforce "user" prefix at construction
        Self("user".create_type_id())
    }

    // Optional: Expose the inner MagicTypeId or its parts if needed
    fn as_magic_type_id(&self) -> &MagicTypeId {
        &self.0
    }

    // Optional: Implement FromStr for UserId
    fn from_str_validated(s: &str) -> Result<Self, MagicTypeIdError> { // Corrected MTIError to MagicTypeIdError
        let mti = MagicTypeId::from_str(s)?;
        if mti.prefix_str() != "user" {
            Err(MagicTypeIdError::Validation("Invalid prefix for UserId, expected 'user'".to_string()))
        } else {
            Ok(UserId(mti))
        }
    }
}

// Implement Display, Serialize, Deserialize etc. as needed, often by delegating to self.0

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct OrderId(MagicTypeId);

impl OrderId {
    fn new() -> Self {
        Self("order".create_type_id())
    }
    // ... similar helper methods
}

// Now, your functions can demand specific ID types:
fn process_user(id: UserId) {
    println!("Processing user with ID: {}", id.as_magic_type_id());
}

fn process_order(id: OrderId) { /* ... */ }

let user_id = UserId::new();
let order_id = OrderId::new();

process_user(user_id);
process_order(order_id);

// This would now cause a compile-time error, preventing accidental misuse!
// process_user(order_id);
```

## Performance and Safety

`mti` is designed with performance and safety as priorities:

*   **Zero-Cost Abstractions**: Many string-like operations on `MagicTypeId` are designed to be efficient.
*   **Solid Foundation**: Built upon the `TypeIdPrefix` and `TypeIdSuffix` crates.
*   **Rust's Safety Guarantees**: Leverages Rust's type system and ownership model to help prevent common programming errors at compile-time.
*   **Comprehensive Test Suite**: Includes extensive unit and property-based tests to ensure correctness and reliability.

## Acknowledgments

This crate implements version 0.3.0 of the [TypeID Specification](https://github.com/jetify-com/typeid) created and
maintained by [Jetify](https://www.jetify.com/). Their work in developing and managing this
specification is appreciated. The concept of prefixing UUIDs for better readability is also notably used by services like Stripe, which served as an inspiration for the broader adoption of such patterns.

## Contributing

Contributions are welcome! If you encounter any issues, have feature requests, or want to contribute code, please visit the [GitHub repository](https://github.com/GovCraft/mti) and open an issue or pull request.

## License

This project is licensed under either of:

*   Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
*   MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))

at your option.

---

## About the Author

I'm [@rrrodzilla](https://github.com/rrrodzilla), a technologist with industry experience, including roles as an SOA and cloud architect, and Principal Technical Product Manager at AWS for the Rust Programming Language. Currently, I'm the owner and operator of Govcraft, building and consulting on Rust and AI solutions.

For more information, visit [https://www.govcraft.ai](https://www.govcraft.ai)
## Sponsor

Govcraft is a one-person shop—no corporate backing, no investors, just me building useful tools. If this project helps you, [sponsoring](https://github.com/sponsors/Govcraft) keeps the work going.

[![Sponsor on GitHub](https://img.shields.io/badge/Sponsor-%E2%9D%A4-%23db61a2?logo=GitHub)](https://github.com/sponsors/Govcraft)
