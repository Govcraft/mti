use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::FromStr;
use typeid_prefix::{TypeIdPrefix, ValidationError};
use typeid_suffix::prelude::*;
use crate::errors::MagicTypeIdError;

/// A type-safe identifier combining a prefix and a UUID-based suffix.
///
/// `MagicTypeId` represents a `TypeID` as specified in [Version 3 of the TypeID](https://github.com/jetify-com/typeid/tree/main/spec) specification.
/// It consists of a [`TypeIdPrefix`] and a [`TypeIdSuffix`], providing a unique,
/// type-safe identifier that can be used across distributed systems.
///
/// [`TypeIdPrefix`]: typeid_prefix::TypeIdPrefix
/// [`TypeIdSuffix`]: typeid_suffix::TypeIdSuffix
///
/// # Examples
///
/// Creating a new `MagicTypeId`:
///
/// ```
/// use std::str::FromStr;
/// use mti::prelude::*;
/// use typeid_prefix::prelude::*;
/// use typeid_suffix::prelude::*;
///
/// let prefix = TypeIdPrefix::from_str("user").unwrap();
/// let suffix = TypeIdSuffix::new::<V7>();
/// let type_id = MagicTypeId::new(prefix, suffix);
///
/// assert!(type_id.to_string().starts_with("user_"));
/// ```
///
/// Parsing a `MagicTypeId` from a string:
///
/// ```
/// use mti::prelude::*;
/// use std::str::FromStr;
///
/// let type_id = MagicTypeId::from_str("product_01h455vb4pex5vsknk084sn02q").unwrap();
/// assert_eq!(type_id.prefix().as_str(), "product");
/// ```
/// # Sorting
///
/// When `MagicTypeId` is created with a `V7` UUID, it provides a natural sorting order:
/// 1. **Primary Sorting**: By the timestamp in the `UUIDv7` suffix. This means that identifiers
///    generated later will appear after those generated earlier.
/// 2. **Secondary Sorting**: If the timestamps are equal, then sorting is based on the lexicographical order
///    of the prefix. This ensures consistent ordering even when identifiers are created at the same time.
///
/// ```rust
/// use std::str::FromStr;
/// use std::thread::sleep;
/// use std::time::Duration;
/// use mti::prelude::*;
/// use typeid_prefix::prelude::*;
/// use typeid_suffix::prelude::*;
///
/// let prefix1 = TypeIdPrefix::from_str("user").unwrap();
/// let prefix2 = TypeIdPrefix::from_str("admin").unwrap();
///
/// let id1 = MagicTypeId::new(prefix1.clone(), TypeIdSuffix::new::<V7>());
/// sleep(Duration::from_millis(10));  // Ensure different timestamps
/// let id2 = MagicTypeId::new(prefix1.clone(), TypeIdSuffix::new::<V7>());
/// let id3 = MagicTypeId::new(prefix2.clone(), TypeIdSuffix::from_str(&id2.suffix().to_string()).unwrap());
///
/// assert!(id1 < id2, "Expected id1 to be less than id2 due to earlier timestamp");
/// assert_eq!(id2.suffix(), id3.suffix(), "Suffixes for id2 and id3 should be the same");
/// assert!(id3 < id2, "Expected id3 to be less than id2 due to lexicographically smaller prefix when timestamps are equal");
/// ```
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct MagicTypeId {
    prefix: TypeIdPrefix,
    suffix: TypeIdSuffix,
    string_repr: String,
}

impl Ord for MagicTypeId {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.suffix.cmp(&other.suffix) {
            Ordering::Equal => self.prefix.cmp(&other.prefix),
            other => other,
        }
    }
}

impl PartialOrd for MagicTypeId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement PartialOrd for &str
impl PartialOrd<str> for MagicTypeId {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.string_repr.as_str().partial_cmp(other)
    }
}

impl PartialOrd<MagicTypeId> for str {
    fn partial_cmp(&self, other: &MagicTypeId) -> Option<Ordering> {
        self.partial_cmp(other.string_repr.as_str())
    }
}

// Implement PartialOrd for String
impl PartialOrd<String> for MagicTypeId {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        self.string_repr.partial_cmp(other)
    }
}

impl PartialOrd<MagicTypeId> for String {
    fn partial_cmp(&self, other: &MagicTypeId) -> Option<Ordering> {
        self.partial_cmp(&other.string_repr)
    }
}

impl Hash for MagicTypeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string_repr.hash(state);
    }
}

impl PartialEq<String> for MagicTypeId {
    fn eq(&self, other: &String) -> bool {
        &self.string_repr == other
    }
}

impl PartialEq<MagicTypeId> for String {
    fn eq(&self, other: &MagicTypeId) -> bool {
        self == &other.string_repr
    }
}

impl MagicTypeId {
    /// Creates a new `MagicTypeId` from a prefix and suffix.
    ///
    /// # Arguments
    ///
    /// * `prefix` - A [`TypeIdPrefix`] representing the type of the ID.
    /// * `suffix` - A [`TypeIdSuffix`] containing the UUID information.
    ///
    /// [`TypeIdPrefix`]: typeid_prefix::TypeIdPrefix
    /// [`TypeIdSuffix`]: typeid_suffix::TypeIdSuffix
    ///
    /// # Returns
    ///
    /// A new `MagicTypeId` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use mti::prelude::*;
    /// use typeid_prefix::prelude::*;
    /// use typeid_suffix::prelude::*;
    ///
    /// let prefix = TypeIdPrefix::from_str("user").unwrap();
    /// let suffix = TypeIdSuffix::new::<V7>();
    /// let type_id = MagicTypeId::new(prefix, suffix);
    ///
    /// assert!(type_id.to_string().starts_with("user_"));
    /// ```
    #[must_use]
    pub fn new(prefix: TypeIdPrefix, suffix: TypeIdSuffix) -> Self {
        let string_repr = if prefix.is_empty() {
            suffix.to_string()
        } else {
            format!("{prefix}_{suffix}")
        };
        Self { prefix, suffix, string_repr }
    }

    /// Returns a reference to the prefix of the `MagicTypeId`.
    ///
    /// # Returns
    ///
    /// A reference to the [`TypeIdPrefix`].
    ///
    /// [`TypeIdPrefix`]: typeid_prefix::TypeIdPrefix
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    /// use std::str::FromStr;
    ///
    /// let type_id = MagicTypeId::from_str("user_01h455vb4pex5vsknk084sn02q").unwrap();
    /// assert_eq!(type_id.prefix().as_str(), "user");
    /// ```
    #[must_use]
    pub const fn prefix(&self) -> &TypeIdPrefix {
        &self.prefix
    }

    /// Returns a reference to the suffix of the `MagicTypeId`.
    ///
    /// # Returns
    ///
    /// A reference to the [`TypeIdSuffix`].
    ///
    /// [`TypeIdSuffix`]: typeid_suffix::TypeIdSuffix
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    /// use std::str::FromStr;
    ///
    /// let type_id = MagicTypeId::from_str("user_01h455vb4pex5vsknk084sn02q").unwrap();
    /// assert_eq!(type_id.suffix().to_string(), "01h455vb4pex5vsknk084sn02q");
    /// ```
    #[must_use]
    pub const fn suffix(&self) -> &TypeIdSuffix {
        &self.suffix
    }

    /// Returns the string representation of the `MagicTypeId`.
    ///
    /// # Returns
    ///
    /// A string slice containing the full `MagicTypeId`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    /// use std::str::FromStr;
    ///
    /// let type_id = MagicTypeId::from_str("user_01h455vb4pex5vsknk084sn02q").unwrap();
    /// assert_eq!(type_id.as_str(), "user_01h455vb4pex5vsknk084sn02q");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.string_repr
    }
}

impl Display for MagicTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string_repr)
    }
}

impl FromStr for MagicTypeId {
    type Err = MagicTypeIdError;

    /// Parses a string into a `MagicTypeId`.
    ///
    /// The string should be in the format "`prefix_suffix`" or just "suffix" if there's no prefix.
    ///
    /// # Errors
    ///
    /// Returns a `MagicTypeIdError` if:
    /// - The string is not in the correct format.
    /// - The prefix is invalid according to the [`TypeIdPrefix`] specification.
    /// - The suffix is not a valid base32-encoded UUID according to the [`TypeIdSuffix`] specification.
    ///
    /// [`TypeIdPrefix`]: TypeIdPrefix
    /// [`TypeIdSuffix`]: typeid_suffix::TypeIdSuffix
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    /// use std::str::FromStr;
    ///
    /// let type_id = MagicTypeId::from_str("user_01h455vb4pex5vsknk084sn02q").unwrap();
    /// assert_eq!(type_id.prefix().as_str(), "user");
    ///
    /// assert!(MagicTypeId::from_str("invalid!_01h455vb4pex5vsknk084sn02q").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((prefix_str, suffix_str)) = s.rsplit_once('_') {
            if prefix_str.is_empty() {
                return Err(MagicTypeIdError::Prefix(ValidationError::InvalidStartCharacter));
            }
            let prefix = TypeIdPrefix::from_str(prefix_str)?;
            let suffix = TypeIdSuffix::from_str(suffix_str)?;
            Ok(Self::new(prefix, suffix))
        } else {
            let suffix = TypeIdSuffix::from_str(s)?;
            Ok(Self::new(TypeIdPrefix::default(), suffix))
        }
    }
}

impl Deref for MagicTypeId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.string_repr
    }
}

impl AsRef<str> for MagicTypeId {
    fn as_ref(&self) -> &str {
        &self.string_repr
    }
}

impl Borrow<str> for MagicTypeId {
    fn borrow(&self) -> &str {
        &self.string_repr
    }
}

impl PartialEq<str> for MagicTypeId {
    fn eq(&self, other: &str) -> bool {
        self.string_repr == other
    }
}

impl PartialEq<MagicTypeId> for str {
    fn eq(&self, other: &MagicTypeId) -> bool {
        self == other.string_repr
    }
}

impl PartialEq<&str> for MagicTypeId {
    fn eq(&self, other: &&str) -> bool {
        &self.string_repr == other
    }
}

impl PartialEq<MagicTypeId> for &str {
    fn eq(&self, other: &MagicTypeId) -> bool {
        *self == other.string_repr
    }
}