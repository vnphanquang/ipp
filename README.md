# ipp-encoder

Internet Printing Protocol (IPP) encoder and decoder implementation in Rust, following [RFC 8010][rfc-8010] and [RFC 8011][rfc-8011]. Inspired by [watson's ipp-encoder implementation in javascript][waston/ipp-encoder]

Note: the `ipp-encoder` crate only implements a core encoder/decoder. To see example usage, look into [printer folder](./printer/src/main.rs).

```toml
[dependencies]

ipp_encoder = { version = "1.0" }
```

## Todo

- [ ] cargo feature flags:
  - [ ] `std`: implement `IppEncode` trait for primitives
  - [ ] `serde`: implement `serde` traits for serialization & deserialization
- [ ] unit testing
- [ ] printer wrapper package

[rfc-8010]: https://datatracker.ietf.org/doc/html/rfc8010
[rfc-8011]: https://datatracker.ietf.org/doc/html/rfc8011
[waston/ipp-encoder]: https://github.com/watson/ipp-encoder
