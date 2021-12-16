use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, FromRepr};

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.3)
#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UriSecuritySupportedKeyword {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "tls")]
    TLS,
}

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.2)
#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UriAuthenticationSupportedKeyword {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "requesting-user-name")]
    RequestingUserName,
    #[strum(serialize = "basic")]
    Basic,
    #[strum(serialize = "digest")]
    Digest,
    #[strum(serialize = "certificate")]
    Certificate,
}

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.12)
#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrinterStateReasonKeyword {
    #[strum(serialize = "none")]
    None,
}

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.32)
#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PdlOverrideSupportedKeyword {
    #[strum(serialize = "attempted")]
    Attempted,
    #[strum(serialize = "not-attempted")]
    NotAttempted,
}

/// [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.32)
#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompressionSupportedKeyword {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "deflate")]
    Deflate,
    #[strum(serialize = "gzip")]
    Gzip,
    #[strum(serialize = "compress")]
    Compress,
}
