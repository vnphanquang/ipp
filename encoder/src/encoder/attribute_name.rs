use crate::spec::attribute::{
    JobAttribute, JobTemplateAttribute, OperationAttribute, PrinterAttribute,
};

use super::{error::AttributeNameParseError, IppEncode};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Generalized attribute name from different group (operation, printer, job, job-template)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeName {
    Operation(OperationAttribute),
    Printer(PrinterAttribute),
    JobTemplate(JobTemplateAttribute),
    Job(JobAttribute),
    Unsupported(String),
}

impl std::str::FromStr for AttributeName {
    type Err = AttributeNameParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = OperationAttribute::from_str(str) {
            Ok(Self::Operation(n))
        } else if let Ok(n) = PrinterAttribute::from_str(str) {
            Ok(Self::Printer(n))
        } else if let Ok(n) = JobTemplateAttribute::from_str(str) {
            Ok(Self::JobTemplate(n))
        } else if let Ok(n) = JobAttribute::from_str(str) {
            Ok(Self::Job(n))
        } else {
            Ok(Self::Unsupported(String::from(str)))
        }
    }
}

impl std::fmt::Display for AttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attr = match self {
            Self::Operation(attr) => attr.to_string(),
            Self::Printer(attr) => attr.to_string(),
            Self::JobTemplate(attr) => attr.to_string(),
            Self::Job(attr) => attr.to_string(),
            Self::Unsupported(attr) => String::from(attr),
        };
        write!(f, "{}", &attr)
    }
}

impl AttributeName {
    pub fn is_empty(&self) -> bool {
        if let Self::Unsupported(attr) = self {
            attr.is_empty()
        } else {
            false
        }
    }
}

impl IppEncode for AttributeName {
    fn from_ipp(bytes: &[u8], offset: usize) -> (usize, Self) {
        let (delta, raw_name) = String::from_ipp(bytes, offset);
        (delta, Self::from_str(&raw_name).unwrap())
    }

    fn to_ipp(&self) -> Vec<u8> {
        self.to_string().to_ipp()
    }

    fn ipp_len(&self) -> usize {
        self.to_string().ipp_len()
    }
}
