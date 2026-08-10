//! Canonical semantic engine for the `.norm` format.
//!
//! The bootstrap exposes only protocol identity. Parser and validator modules
//! are added after the A1 behavior contract is complete.

#![forbid(unsafe_code)]

/// Identifier for the initial `.norm` format contract.
pub const FORMAT_ID: &str = "norm-spec/a1";

/// Machine API identifier for parse responses.
pub const PARSE_API_VERSION: &str = "norm-spec/parse/v1";

/// Machine API identifier for collect responses.
pub const COLLECT_API_VERSION: &str = "norm-spec/collect/v1";

/// Return the compiled crate version.
#[must_use]
pub const fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::{COLLECT_API_VERSION, FORMAT_ID, PARSE_API_VERSION};

    #[test]
    fn protocol_identifiers_are_explicitly_versioned() {
        assert!(PARSE_API_VERSION.ends_with("/v1"));
        assert!(COLLECT_API_VERSION.ends_with("/v1"));
        assert_eq!(FORMAT_ID, "norm-spec/a1");
    }
}
