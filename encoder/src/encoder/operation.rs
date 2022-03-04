use crate::spec::{
    operation::{OperationID, StatusCode},
    tag::DelimiterTag,
};

use super::{AttributeGroup, IppEncode, IppVersion};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

///
/// Operation request or response
///
/// ```
/// -----------------------------------------------
/// |                  version-number             |   2 bytes  - required
/// -----------------------------------------------
/// |               operation-id (request)        |
/// |                      or                     |   2 bytes  - required
/// |               status-code (response)        |
/// -----------------------------------------------
/// |                   request-id                |   4 bytes  - required
/// -----------------------------------------------
/// |                 attribute-group             |   n bytes - 0 or more
/// -----------------------------------------------
/// |              end-of-attributes-tag          |   1 byte   - required
/// -----------------------------------------------
/// |                     data                    |   y bytes  - optional
/// -----------------------------------------------
/// ```
///
/// ----------------------------------------------------------
///
/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.1)
///
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub version: IppVersion,
    pub operation_id_or_status_code: u16,
    pub request_id: u32,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub attribute_groups: HashMap<DelimiterTag, AttributeGroup>,
    #[serde(skip)]
    /// additional data in trailing bytes
    pub data: Vec<u8>,
}

impl IppEncode for Operation {
    fn from_ipp(bytes: &[u8], offset: usize) -> (usize, Self) {
        let mut shifting_offset = offset;

        // read version.major
        let slice: [u8; 1] = bytes[shifting_offset..shifting_offset + 1]
            .try_into()
            .unwrap();
        let major = u8::from_be_bytes(slice);
        shifting_offset += slice.len();

        // read version.minor
        let slice: [u8; 1] = bytes[shifting_offset..shifting_offset + 1]
            .try_into()
            .unwrap();
        let minor = u8::from_be_bytes(slice);
        shifting_offset += slice.len();

        // read operation-id or status-code
        let slice: [u8; 2] = bytes[shifting_offset..shifting_offset + 2]
            .try_into()
            .unwrap();
        let operation_id_or_status_code = u16::from_be_bytes(slice);
        shifting_offset += slice.len();

        // read request-id
        let slice: [u8; 4] = bytes[shifting_offset..shifting_offset + 4]
            .try_into()
            .unwrap();
        let request_id = u32::from_be_bytes(slice);
        shifting_offset += slice.len();

        // read attribute groups
        let (delta, attribute_groups): (usize, HashMap<DelimiterTag, AttributeGroup>) =
            HashMap::from_ipp(bytes, shifting_offset);
        shifting_offset += delta;

        // read additional data (trailing bytes)
        let data = (&bytes[shifting_offset..]).to_vec();

        (
            shifting_offset - offset,
            Self {
                version: IppVersion { major, minor },
                request_id,
                operation_id_or_status_code,
                attribute_groups,
                data,
            },
        )
    }

    fn to_ipp(&self) -> Vec<u8> {
        // write version major
        let major_bytes = self.version.major.to_be_bytes().to_vec();

        // write version minor
        let minor_bytes = self.version.minor.to_be_bytes().to_vec();

        // write operation-id or status-code
        let operation_or_status_bytes = self.operation_id_or_status_code.to_be_bytes().to_vec();

        // write request-id
        let request_id_bytes = self.request_id.to_be_bytes().to_vec();

        // write attribute groups
        let attribute_groups_bytes = self.attribute_groups.to_ipp();

        [
            major_bytes,
            minor_bytes,
            operation_or_status_bytes,
            request_id_bytes,
            attribute_groups_bytes,
            self.data.to_vec(),
        ]
        .concat()
    }

    fn ipp_len(&self) -> usize {
        self.version.major.to_be_bytes().len()
            + self.version.minor.to_be_bytes().len()
            + self.operation_id_or_status_code.to_be_bytes().len()
            + self.request_id.to_be_bytes().len()
            + self.attribute_groups.ipp_len()
            + self.data.len()
    }
}

impl Operation {
    pub fn operation_id(&self) -> Option<OperationID> {
        OperationID::from_repr(self.operation_id_or_status_code as usize)
    }
    pub fn status_code(&self) -> Option<StatusCode> {
        StatusCode::from_repr(self.operation_id_or_status_code as usize)
    }

    pub fn to_json(&self) -> String {
        // FIXME: handle error gracefully
        serde_json::to_string(self).unwrap()
    }
}
