use serde::{Deserialize, Serialize};

/// 2 bytes of IPP version
/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.4.1)
///
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct IppVersion {
    pub major: u8,
    pub minor: u8,
}
