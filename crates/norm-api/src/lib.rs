//! High-level, filesystem-aware Rust API for the `.norm` format.
//!
//! The facade owns explicit filesystem orchestration and packaged release
//! assets. Format parsing and validation semantics remain implemented once in
//! [`norm_spec_core`].

#![forbid(unsafe_code)]

mod assets;
mod collect;
mod error;
mod paths;

pub use assets::{
    PROFILE_NAMES, SchemaLoadError, embedded_schema_bundle, profile_template,
    schema_bundle_from_dir,
};
pub use collect::{CollectRequest, collect};
pub use error::{ApiError, FailureClass};
pub use norm_spec_core::{CollectResponse, CollectedNorm, ErrorDetail, ParsedNorm};
