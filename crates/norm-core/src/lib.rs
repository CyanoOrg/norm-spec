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

/// Machine API identifier for validate responses.
pub const VALIDATE_API_VERSION: &str = "norm-spec/validate/v1";

/// Machine API identifier for init responses.
pub const INIT_API_VERSION: &str = "norm-spec/init/v1";

/// Machine API identifier for scan responses.
pub const SCAN_API_VERSION: &str = "norm-spec/scan/v1";

/// Machine API identifier for handled CLI failures in machine mode.
pub const ERROR_API_VERSION: &str = "norm-spec/error/v1";

/// Return the compiled crate version.
#[must_use]
pub const fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::{
        COLLECT_API_VERSION, ERROR_API_VERSION, FORMAT_ID, INIT_API_VERSION, PARSE_API_VERSION,
        SCAN_API_VERSION, VALIDATE_API_VERSION,
    };

    #[test]
    fn protocol_identifiers_are_explicitly_versioned() {
        assert_eq!(PARSE_API_VERSION, "norm-spec/parse/v1");
        assert_eq!(COLLECT_API_VERSION, "norm-spec/collect/v1");
        assert_eq!(VALIDATE_API_VERSION, "norm-spec/validate/v1");
        assert_eq!(INIT_API_VERSION, "norm-spec/init/v1");
        assert_eq!(SCAN_API_VERSION, "norm-spec/scan/v1");
        assert_eq!(ERROR_API_VERSION, "norm-spec/error/v1");
        assert_eq!(FORMAT_ID, "norm-spec/a1");
    }
}
