use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, FromRepr};

/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.5.1)
#[derive(
    strum_macros::Display,
    EnumString,
    Serialize,
    Deserialize,
    FromRepr,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
)]
pub enum DelimiterTag {
    OperationAttributes = 0x01,
    JobAttributes = 0x02,
    EndOfAttributes = 0x03,
    PrinterAttributes = 0x04,
    UnsupportedAttributes = 0x05,
}

/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.5.2)
#[derive(Serialize, Deserialize, FromRepr, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValueTag {
    // "out-of-band" values - "Out-of-Band Attribute Value Tags" registry
    Unsupported = 0x10,
    Unknown = 0x12,
    NoValue = 0x13,

    // integer values - "Attribute Syntaxes" registry
    Integer = 0x21,
    Boolean = 0x22,
    Enum = 0x23,

    // octetString values - "Attribute Syntaxes" registry
    OctetStringUnspecified = 0x30,
    DateTime = 0x31,
    Resolution = 0x32,
    RangeOfInteger = 0x33,
    BegCollection = 0x34,
    TextWithLanguage = 0x35,
    NameWithLanguage = 0x36,
    EndCollection = 0x37,

    // character-string values - "Attribute Syntaxes" registry
    TextWithoutLanguage = 0x41,
    NameWithoutLanguage = 0x42,
    Keyword = 0x44,
    Uri = 0x45,
    UriScheme = 0x46,
    Charset = 0x47,
    NaturalLanguage = 0x48,
    MimeMediaType = 0x49,
    MemberAttrName = 0x4a,
}
