//! # dmarcguard-sdk
//!
//! Official Rust SDK for the [DMARCguard](https://dmarcguard.io) API.
//!
//! DMARCguard helps you monitor and enforce DMARC, SPF, and DKIM on your
//! sending domains. This crate provides idiomatic Rust bindings to the
//! DMARCguard REST API.
//!
//! Learn more at <https://dmarcguard.io>.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Crate version, sourced from `Cargo.toml` at build time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default base URL for the DMARCguard public API.
pub const DEFAULT_API_BASE_URL: &str = "https://api.dmarcguard.io";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn default_base_url_points_to_dmarcguard() {
        assert!(DEFAULT_API_BASE_URL.contains("dmarcguard.io"));
    }
}
