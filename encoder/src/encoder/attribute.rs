use serde::{Deserialize, Serialize};

use crate::spec::tag::{DelimiterTag, ValueTag};

use super::{AttributeName, AttributeValue, IppEncode};

///
/// Wrapper for IPP attribute
///
/// ```
/// -----------------------------------------------
/// |          attribute-with-one-value           |  q bytes
/// ----------------------------------------------------------
/// |             additional-value                |  r bytes |- 0 or more
/// ----------------------------------------------------------
/// ```
///
/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.1.3)
///
/// -----------------------------------------------
///
/// ### Single-valued or multi-valued with one value
///
/// Encoded with just an "attribute-with-one-value" field
///
/// ```
/// -----------------------------------------------
/// |                   value-tag                 |   1 byte
/// -----------------------------------------------
/// |               name-length  (value is u)     |   2 bytes
/// -----------------------------------------------
/// |                     name                    |   u bytes
/// -----------------------------------------------
/// |              value-length  (value is v)     |   2 bytes
/// -----------------------------------------------
/// |                     value                   |   v bytes
/// -----------------------------------------------
/// ```
///
/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.1.4)
///
/// -----------------------------------------------
///
/// ### Multi-valued with n values
///
/// Encoded with an "attribute-with-one-value" field followed by n-1 "additional-value" fields
///
/// ```
/// -----------------------------------------------
/// |                   value-tag                 |   1 byte
/// -----------------------------------------------
/// |            name-length  (value is 0x0000)   |   2 bytes
/// -----------------------------------------------
/// |              value-length (value is w)      |   2 bytes
/// -----------------------------------------------
/// |                     value                   |   w bytes
/// -----------------------------------------------
/// ```
///
/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.1.5)
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub tag: ValueTag,
    pub name: AttributeName,
    pub values: Vec<AttributeValue>,
}

impl Attribute {
    fn decode_one(bytes: &[u8], offset: usize) -> (usize, bool, Option<Self>) {
        let mut shifting_offset = offset;

        let slice: [u8; 1] = bytes[shifting_offset..shifting_offset + 1]
            .try_into()
            .unwrap();
        let raw_int = u8::from_be_bytes(slice);
        shifting_offset += 1;

        let decoded: Option<Self>;

        let mut has_name = false;

        if DelimiterTag::from_repr(raw_int as usize).is_some() {
            // if reach any other delimiter tag, return
            // (either a new attribute group or end-of-attributes)
            decoded = None;
            shifting_offset = offset;
        } else {
            // decode attribute-name
            let (delta, name) = AttributeName::from_ipp(bytes, shifting_offset);
            shifting_offset += delta;
            has_name = !name.is_empty();

            // decode actual value
            let value_tag = ValueTag::from_repr(raw_int as usize).unwrap();
            let (delta, value) = AttributeValue::from_ipp(bytes, shifting_offset, value_tag);
            shifting_offset += delta;

            decoded = Some(Attribute {
                tag: value_tag,
                name,
                values: vec![value],
            });
        }

        (shifting_offset - offset, has_name, decoded)
    }

    pub fn from_ipp(bytes: &[u8], offset: usize) -> (usize, Option<Self>) {
        let (mut first_offset, _, first_attribute_opt) = Self::decode_one(bytes, offset);

        let next_offset = offset + first_offset;

        if let Some(mut first_attribute) = first_attribute_opt {
            if next_offset > bytes.len() {
                (0, None)
            } else {
                let (mut next_offset, mut has_name, mut next_attribute_opt) =
                    Self::decode_one(bytes, next_offset);

                while let Some(mut next_attribute) = next_attribute_opt {
                    if has_name || (offset + first_offset + next_offset >= bytes.len()) {
                        break;
                    }
                    // add additional_value
                    first_attribute.values.append(&mut next_attribute.values);

                    // add to offset
                    first_offset += next_offset;

                    let next = Self::decode_one(bytes, offset + first_offset);
                    next_offset = next.0;
                    has_name = next.1;
                    next_attribute_opt = next.2;
                }

                (first_offset, Some(first_attribute))
            }
        } else {
            (0, None)
        }
    }

    pub fn to_ipp(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.ipp_len());
        if !self.values.is_empty() {
            for i in 0..self.values.len() {
                // write tag
                bytes.append(&mut (self.tag as u8).to_be_bytes().to_vec());

                // write name
                if i == 0 {
                    // first attribute write name-length and name
                    bytes.append(&mut self.name.to_ipp());
                } else {
                    // next attributes only write 2 bytes of name-length (0x00)
                    bytes.append(&mut String::from("").to_ipp());
                }

                // write value
                let value = &self.values[i];
                bytes.append(&mut value.to_ipp());
            }
        }
        bytes
    }

    pub fn ipp_len(&self) -> usize {
        if self.values.is_empty() {
            0
        } else {
            // each value has a 1 byte value-tag
            let tag_len = self.values.len();

            let name_len = self.name.to_string().ipp_len() +    // first value has name-length and name
            (self.values.len() - 1) * 2; // next values only has 2 bytes of name-length (0x00)

            let mut value_len: usize = 0;
            for value in &self.values {
                value_len += value.ipp_len();
            }

            tag_len + name_len + value_len
        }
    }
}
