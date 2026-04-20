# dmarcguard-sdk-rust

[![crates.io](https://img.shields.io/crates/v/dmarcguard-sdk.svg)](https://crates.io/crates/dmarcguard-sdk)
[![docs.rs](https://img.shields.io/docsrs/dmarcguard-sdk)](https://docs.rs/dmarcguard-sdk)
[![CI](https://github.com/dmarcguardhq/dmarcguard-sdk-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/dmarcguardhq/dmarcguard-sdk-rust/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

Official Rust SDK for the [DMARCguard](https://dmarcguard.io) API.

[DMARCguard](https://dmarcguard.io) is a hosted DMARC, SPF, and DKIM
monitoring and enforcement platform. This crate provides idiomatic Rust
bindings for the DMARCguard REST API so you can integrate domain
authentication workflows directly from your Rust applications.

## Installation

```toml
[dependencies]
dmarcguard-sdk = "0"
```

Or with `cargo`:

```sh
cargo add dmarcguard-sdk
```

## Usage

```rust
use dmarcguard_sdk::{DEFAULT_API_BASE_URL, VERSION};

fn main() {
    println!("dmarcguard-sdk v{VERSION} -> {DEFAULT_API_BASE_URL}");
}
```

Full API surface lands in upcoming releases. Follow progress on
[GitHub](https://github.com/dmarcguardhq/dmarcguard-sdk-rust) or on
[dmarcguard.io](https://dmarcguard.io).

## Links

- Website: <https://dmarcguard.io>
- API docs: <https://docs.dmarcguard.io>
- Crate: <https://crates.io/crates/dmarcguard-sdk>
- Rust docs: <https://docs.rs/dmarcguard-sdk>
- Source: <https://github.com/dmarcguardhq/dmarcguard-sdk-rust>

## Release process

Versioning and publishing are fully automated:

- [release-please](https://github.com/googleapis/release-please-action)
  opens a release PR based on
  [Conventional Commits](https://www.conventionalcommits.org/) landed on
  `main`. Merging that PR tags the release.
- CI then publishes to [crates.io](https://crates.io) using
  [trusted publishing](https://crates.io/docs/trusted-publishing) via
  [`rust-lang/crates-io-auth-action`](https://github.com/rust-lang/crates-io-auth-action)
  (OIDC, no long-lived token).

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
