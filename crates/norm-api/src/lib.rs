//! High-level, filesystem-aware Rust API for the `.norm` format.
//!
//! The facade owns explicit filesystem orchestration and packaged release
//! assets. Format parsing and validation semantics remain implemented once in
//! [`norm_spec_core`].

#![forbid(unsafe_code)]

mod assets;

pub use assets::{
    PROFILE_NAMES, SchemaLoadError, embedded_schema_bundle, profile_template,
    schema_bundle_from_dir,
};
