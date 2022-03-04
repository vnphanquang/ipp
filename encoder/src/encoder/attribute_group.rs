use crate::spec::tag::DelimiterTag;

use super::{Attribute, AttributeName, IppEncode};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

/// An "attribute-group" field contains zero or more "attribute" fields.
///
/// ```
/// -----------------------------------------------
/// |           begin-attribute-group-tag         |  1 byte
/// ----------------------------------------------------------
/// |                   attribute                 |  p bytes |- 0 or more
/// ----------------------------------------------------------
/// ```
///
/// ----------------------------------------------------------
///
/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.1.2)
///
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeGroup {
    pub tag: DelimiterTag,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub attributes: HashMap<AttributeName, Attribute>,
}

impl IppEncode for HashMap<DelimiterTag, AttributeGroup> {
    fn from_ipp(bytes: &[u8], offset: usize) -> (usize, Self) {
        let mut decoded: Self = HashMap::new();

        let mut shifting_offset = offset;

        let read_tag = |bytes: &[u8], offset: usize| -> (usize, Option<DelimiterTag>) {
            let slice: [u8; 1] = bytes[offset..offset + 1].try_into().unwrap();
            let raw_int = u8::from_be_bytes(slice);
            (1, DelimiterTag::from_repr(raw_int as usize))
        };

        let (delta, mut tag_opt) = read_tag(bytes, shifting_offset);
        shifting_offset += delta;

        let mut attributes: HashMap<AttributeName, Attribute> = HashMap::new();

        while shifting_offset < bytes.len() {
            if let Some(tag) = tag_opt {
                if tag == DelimiterTag::EndOfAttributes {
                    break;
                }

                // read attributes in group
                let (mut delta, mut attribute_opt) = Attribute::from_ipp(bytes, shifting_offset);
                loop {
                    if shifting_offset > bytes.len() {
                        break;
                    }

                    if let Some(attribute) = attribute_opt {
                        attributes.insert(attribute.name.clone(), attribute);
                        shifting_offset += delta;
                        let next = Attribute::from_ipp(bytes, shifting_offset);
                        delta = next.0;
                        attribute_opt = next.1;
                    } else {
                        break;
                    }
                }

                decoded.insert(tag, AttributeGroup { tag, attributes });

                attributes = HashMap::new();
                let next_tag = read_tag(bytes, shifting_offset);
                shifting_offset += next_tag.0;
                tag_opt = next_tag.1;
            } else {
                break;
            }
        }

        (shifting_offset - offset, decoded)
    }

    fn to_ipp(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::with_capacity(self.ipp_len());

        let mut groups: Vec<&AttributeGroup> = Vec::new();
        if let Some(group) = self.get(&DelimiterTag::OperationAttributes) {
            groups.push(group);
        }
        if let Some(group) = self.get(&DelimiterTag::UnsupportedAttributes) {
            groups.push(group);
        }
        if let Some(group) = self.get(&DelimiterTag::PrinterAttributes) {
            groups.push(group);
        }
        if let Some(group) = self.get(&DelimiterTag::JobAttributes) {
            groups.push(group);
        }

        for group in groups {
            // write delimiter tag
            vec.append(&mut (group.tag as u8).to_be_bytes().to_vec());

            for attribute in group.attributes.values() {
                // write attribute
                vec.append(&mut attribute.to_ipp());
            }
        }

        // end-of-attributes tag
        vec.append(&mut (DelimiterTag::EndOfAttributes as u8).to_be_bytes().to_vec());

        vec
    }

    fn ipp_len(&self) -> usize {
        let mut len: usize = 0;

        for group in self.values() {
            len += 1; // delimiter tag
            for attribute in group.attributes.values() {
                len += attribute.ipp_len();
            }
        }

        len += 1; // end-of-attributes tag

        len
    }
}
