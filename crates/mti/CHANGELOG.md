# Changelog

## 1.1.2

- Fix examples that omitted the UUID version, treated fallible string accessors as strings, or referenced the nonexistent `MagicTypeIdError::Validation` variant (issue #3).
- Demonstrate a newtype error that separates invalid TypeID syntax from an unexpected entity prefix.
- Correct UUID sorting and prefix sanitization documentation.
- Compile and run README examples as part of the doctest suite.
- Include both license texts in the published package.

This patch changes documentation and test coverage. Public API signatures,
features, dependency requirements, and runtime behavior are unchanged.
