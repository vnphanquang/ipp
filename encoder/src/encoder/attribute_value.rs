use super::{IppEncode, TextWithLang};
use crate::spec::tag::ValueTag;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// generalized attribute value of different types
///
/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8010#section-3.9)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    TextWithoutLang(String),
    Number(i32),
    Boolean(bool),
    TextWithLang(TextWithLang),
    DateTime(DateTime<Utc>),
}

impl AttributeValue {
    pub fn from_ipp(bytes: &Vec<u8>, offset: usize, value_tag: ValueTag) -> (usize, Self) {
        let mut len: usize = 0;
        let value: Self;
        match value_tag {
            ValueTag::Integer | ValueTag::Enum => {
                let (delta, raw_value) = i32::from_ipp(bytes, offset);
                len = delta;
                value = Self::Number(raw_value);
            }
            ValueTag::Boolean => {
                let (delta, raw_value) = bool::from_ipp(bytes, offset);
                len = delta;
                value = Self::Boolean(raw_value);
            }
            ValueTag::TextWithLanguage => {
                let (delta, raw_value) = TextWithLang::from_ipp(bytes, offset);
                len = delta;
                value = Self::TextWithLang(raw_value);
            }
            ValueTag::DateTime => {
                let (delta, raw_value) = DateTime::from_ipp(bytes, offset);
                len = delta;
                value = Self::DateTime(raw_value);
            }
            _ => {
                let (delta, raw_value) = String::from_ipp(bytes, offset);
                len = delta;
                value = Self::TextWithoutLang(raw_value);
            }
        }

        (len, value)
    }

    pub fn to_ipp(&self) -> Vec<u8> {
        match self {
            Self::Boolean(raw_value) => raw_value.to_ipp(),
            Self::Number(raw_value) => raw_value.to_ipp(),
            Self::DateTime(raw_value) => raw_value.to_ipp(),
            Self::TextWithLang(raw_value) => raw_value.to_ipp(),
            Self::TextWithoutLang(raw_value) => raw_value.to_ipp(),
        }
    }

    pub fn ipp_len(&self) -> usize {
        match self {
            Self::Boolean(raw_value) => raw_value.ipp_len(),
            Self::Number(raw_value) => raw_value.ipp_len(),
            Self::DateTime(raw_value) => raw_value.ipp_len(),
            Self::TextWithLang(raw_value) => raw_value.ipp_len(),
            Self::TextWithoutLang(raw_value) => raw_value.ipp_len(),
        }
    }
}
