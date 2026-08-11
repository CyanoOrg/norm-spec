use std::{error::Error, fmt};

use norm_spec_core::ErrorDetail;

/// Stable caller-facing classification for a high-level API failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureClass {
    /// The request is invalid or names unavailable configuration.
    Usage,
    /// Filesystem or input processing prevented the requested operation.
    Operation,
}

/// A handled high-level API failure with stable machine-readable detail.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiError {
    class: FailureClass,
    detail: ErrorDetail,
}

impl ApiError {
    pub(crate) fn usage(detail: ErrorDetail) -> Self {
        Self {
            class: FailureClass::Usage,
            detail,
        }
    }

    pub(crate) fn operation(detail: ErrorDetail) -> Self {
        Self {
            class: FailureClass::Operation,
            detail,
        }
    }

    /// Return whether the caller or the attempted operation caused the failure.
    #[must_use]
    pub const fn class(&self) -> FailureClass {
        self.class
    }

    /// Return the stable error code, message, and optional path or field.
    #[must_use]
    pub const fn detail(&self) -> &ErrorDetail {
        &self.detail
    }

    /// Consume the error and return its stable machine-readable detail.
    #[must_use]
    pub fn into_detail(self) -> ErrorDetail {
        self.detail
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.detail.code, self.detail.message)
    }
}

impl Error for ApiError {}
