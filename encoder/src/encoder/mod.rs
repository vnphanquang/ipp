//! # ipp_encoder::encoder
//!
//! The `encoder` module provides the `IppEncode` trait and implements
//! encoder / decoder for IPP operations
//!

mod attribute;
mod attribute_group;
mod attribute_name;
mod attribute_value;
mod datetime;
mod error;
mod ipp_version;
mod operation;
mod primitives;
mod text_with_lang;
mod traits;

pub use attribute::Attribute;
pub use attribute_group::AttributeGroup;
pub use attribute_name::AttributeName;
pub use attribute_value::AttributeValue;
pub use ipp_version::IppVersion;
pub use operation::Operation;
pub use text_with_lang::TextWithLang;
pub use traits::IppEncode;
