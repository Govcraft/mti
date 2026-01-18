use crate::ValidationError;

/// A marker trait for types that can be validated as a `TypeID` prefix.
///
/// This trait is automatically implemented for any type that implements
/// `AsRef<str>` and can be converted to a `TypeIdPrefix` using `TryFrom`.
pub trait Validate {}

impl<T> Validate for T where T: AsRef<str> + TryFrom<T, Error = ValidationError> {}
