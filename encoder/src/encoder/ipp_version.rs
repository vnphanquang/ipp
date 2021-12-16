use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct IppVersion {
    pub major: u8,
    pub minor: u8,
}
