//! Error types for the `MagicTypeId` crate.
//!
//! This module defines the error types that can occur when working with `MagicTypeIds`.
//! It includes errors related to both the prefix and suffix components of a `MagicTypeId`.

use std::fmt;

use typeid_prefix::prelude::*;
use typeid_suffix::prelude::*;

#[cfg(feature = "instrument")]
use tracing::{error, instrument};

/// Represents errors that can occur when working with `MagicTypeIds`.
///
/// This enum encapsulates errors from both the prefix and suffix components
/// of a `MagicTypeId`, allowing for more specific error handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MagicTypeIdError {
    /// Errors related to the `TypeID` prefix.
    ///
    /// These errors occur when there's an issue with the prefix part of a `MagicTypeId`,
    /// such as invalid characters or length.
    Prefix(ValidationError),

    /// Errors related to the `TypeID` suffix.
    ///
    /// These errors occur when there's an issue with the suffix part of a `MagicTypeId`,
    /// such as invalid encoding or an incorrect UUID format.
    Suffix(DecodeError),
}

impl fmt::Display for MagicTypeIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prefix(err) => write!(f, "Prefix error: {err}"),
            Self::Suffix(err) => write!(f, "Suffix error: {err}"),
        }
    }
}

impl std::error::Error for MagicTypeIdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Prefix(err) => Some(err),
            Self::Suffix(err) => Some(err),
        }
    }
}

impl From<ValidationError> for MagicTypeIdError {
    #[cfg_attr(feature = "instrument", instrument(level = "error", fields(error = %err)))]
    fn from(err: ValidationError) -> Self {
        #[cfg(feature = "instrument")]
        error!("Converting ValidationError to MagicTypeIdError: {}", err);
        Self::Prefix(err)
    }
}

impl From<DecodeError> for MagicTypeIdError {
    #[cfg_attr(feature = "instrument", instrument(level = "error", fields(error = %err)))]
    fn from(err: DecodeError) -> Self {
        #[cfg(feature = "instrument")]
        error!("Converting DecodeError to MagicTypeIdError: {}", err);
        Self::Suffix(err)
    }
}
