# ipp_encoder

[![MIT][license.badge]][license] [![docs.rs][docs.rs.badge]][docs.rs] [![crates.io][crates.io.badge]][crates.io]

Internet Printing Protocol (IPP) encoder and decoder implementation in Rust, following [RFC 8010][rfc-8010] and [RFC 8011][rfc-8011]. Inspired by [watson's ipp-encoder implementation in javascript][waston/ipp-encoder]

Note: the `ipp-encoder` crate only implements a core encoder/decoder. To see example usage, look into [printer folder](./printer/src/main.rs).

```toml
[dependencies]

ipp_encoder = { version = "1.0" }
```

[Documentation][docs.rs]

## Todo

- [ ] cargo feature flags:
  - [ ] `std`: implement `IppEncode` trait for primitives
  - [ ] `serde`: implement `serde` traits for serialization & deserialization
- [ ] unit testing
- [ ] printer wrapper package

[rfc-8010]: https://datatracker.ietf.org/doc/html/rfc8010
[rfc-8011]: https://datatracker.ietf.org/doc/html/rfc8011
[waston/ipp-encoder]: https://github.com/watson/ipp-encoder

[crates.io.badge]: https://img.shields.io/crates/v/ipp_encoder.svg
[crates.io]: https://crates.io/crates/ipp_encoder

[license.badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license]: ./LICENSE

[docs.rs.badge]: https://docs.rs/ipp_encoder/badge.svg
[docs.rs]: https://docs.rs/ipp_encoder
