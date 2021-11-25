use chrono::{DateTime, Datelike, Offset, TimeZone, Timelike, Utc};
use std::{collections::HashMap, str::FromStr};
use strum_macros::{EnumString, FromRepr};

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.11)
#[derive(FromRepr, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrinterState {
    Idle = 3,
    Processing = 4,
    Stopped = 5,
}

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.3.7)
#[derive(FromRepr, Debug, PartialEq, Eq, Clone, Copy)]
pub enum JobState {
    Pending = 3,
    PendingHeld = 4,
    Processing = 5,
    ProcessingStopped = 6,
    Canceled = 7,
    Aborted = 8,
    Completed = 9,
}

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.15)
#[derive(FromRepr, Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperationID {
    PrintJob = 0x0002,
    PrintUri = 0x0003,
    ValidateJob = 0x0004,
    CreateJob = 0x0005,
    SendDocument = 0x0006,
    SendUri = 0x0007,
    CancelJob = 0x0008,
    GetJobAttributes = 0x0009,
    GetJobs = 0x000A,
    GetPrinterAttributes = 0x000B,
    HoldJob = 0x000C,
    ReleaseJob = 0x000D,
    RestartJob = 0x000E,
    PausePrinter = 0x0010,
    ResumePrinter = 0x0011,
    PurgeJobs = 0x0012,
}

/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.5.1)
#[derive(FromRepr, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DelimiterTag {
    OperationAttributes = 0x01,
    JobAttributes = 0x02,
    EndOfAttributes = 0x03,
    PrinterAttributes = 0x04,
    UnsupportedAttributes = 0x05,
}

/// ref: [rfc8010](https://datatracker.ietf.org/doc/html/rfc8010#section-3.5.2)
#[derive(FromRepr, Debug, PartialEq, Eq, Clone, Copy)]
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

pub enum BooleanValue {
    False = 0x00,
    True = 0x01,
}

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

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#appendix-B.1.2.1)
#[derive(FromRepr, Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatusCode {
    SuccessfulOk = 0x0000,
    SuccessfulOkIgnoredOrSubstitutedAttributes = 0x0001,
    SuccessfulOkConflictingAttributes = 0x0002,
    ClientErrorBadRequest = 0x0400,
    ClientErrorForbidden = 0x0401,
    ClientErrorNotAuthenticated = 0x0402,
    ClientErrorNotAuthorized = 0x0403,
    ClientErrorNotPossible = 0x0404,
    ClientErrorTimeout = 0x0405,
    ClientErrorNotFound = 0x0406,
    ClientErrorGone = 0x0407,
    ClientErrorRequestEntityTooLarge = 0x0408,
    ClientErrorRequestValueTooLong = 0x0409,
    ClientErrorDocumentFormatNotSupported = 0x040A,
    ClientErrorAttributesOrValuesNotSupported = 0x040B,
    ClientErrorUriSchemeNotSupported = 0x040C,
    ClientErrorCharsetNotSupported = 0x040D,
    ClientErrorConflictingAttributes = 0x040E,
    ClientErrorCompressionNotSupported = 0x040F,
    ClientErrorCompressionError = 0x0410,
    ClientErrorDocumentFormatError = 0x0411,
    ClientErrorDocumentAccessError = 0x0412,
    ServerErrorInternalError = 0x0500,
    ServerErrorOperationNotSupported = 0x0501,
    ServerErrorServiceUnavailable = 0x0502,
    ServerErrorVersionNotSupported = 0x0503,
    ServerErrorDeviceError = 0x0504,
    ServerErrorTemporaryError = 0x0505,
    ServerErrorNotAcceptingJobs = 0x0506,
    ServerErrorBusy = 0x0507,
    ServerErrorJobCanceled = 0x0508,
    ServerErrorMultipleDocumentJobsNotSupported = 0x0509,
    UnknownStatusCode = 0xffff,
}

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4)
#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PrinterAttribute {
    #[strum(serialize = "printer-uri-supported")]
    PrinterUriSupported,
    #[strum(serialize = "uri-security-supported")]
    UriSecuritySupported,
    #[strum(serialize = "uri-authentication-supported")]
    UriAuthenticationSupported,
    #[strum(serialize = "printer-name")]
    PrinterName,
    #[strum(serialize = "printer-location")]
    PrinterLocation,
    #[strum(serialize = "printer-info")]
    PrinterInfo,
    #[strum(serialize = "printer-more-info")]
    PrinterMoreInfo,
    #[strum(serialize = "printer-driver-installer")]
    PrinterDriverInstaller,
    #[strum(serialize = "printer-make-and-model")]
    PrinterMakeAndModel,
    #[strum(serialize = "printer-more-info-manufacturer")]
    PrinterMoreInfoManufacturer,
    #[strum(serialize = "printer-state")]
    PrinterState,
    #[strum(serialize = "printer-state-reasons")]
    PrinterStateReasons,
    #[strum(serialize = "printer-state-message")]
    PrinterStateMessage,
    #[strum(serialize = "ipp-versions-supported")]
    IppVersionsSupported,
    #[strum(serialize = "operations-supported")]
    OperationsSupported,
    #[strum(serialize = "multiple-document-jobs-supported")]
    MultipleDocumentJobsSupported,
    #[strum(serialize = "charset-configured")]
    CharsetConfigured,
    #[strum(serialize = "charset-supported")]
    CharsetSupported,
    #[strum(serialize = "natural-language-configured")]
    NaturalLanguageConfigured,
    #[strum(serialize = "generated-natural-language-supported")]
    GeneratedNaturalLanguageSupported,
    #[strum(serialize = "document-format-default")]
    DocumentFormatDefault,
    #[strum(serialize = "document-format-supported")]
    DocumentFormatSupported,
    #[strum(serialize = "printer-is-accepting-jobs")]
    PrinterIsAcceptingJobs,
    #[strum(serialize = "queued-job-count")]
    QueuedJobCount,
    #[strum(serialize = "printer-message-from-operator")]
    PrinterMessageFromOperator,
    #[strum(serialize = "color-supported")]
    ColorSupported,
    #[strum(serialize = "reference-uri-schemes-supported")]
    ReferenceUriSchemesSupported,
    #[strum(serialize = "pdl-override-supported")]
    PdlOverrideSupported,
    #[strum(serialize = "printer-up-time")]
    PrinterUpTime,
    #[strum(serialize = "printer-current-time")]
    PrinterCurrentTime,
    #[strum(serialize = "multiple-operation-time-out")]
    MultipleOperationTimeOut,
    #[strum(serialize = "compression-supported")]
    CompressionSupported,
    #[strum(serialize = "job-k-octets-supported")]
    JobKOctetsSupported,
    #[strum(serialize = "job-impressions-supported")]
    JobImpressionsSupported,
    #[strum(serialize = "job-media-sheets-supported")]
    JobMediaSheetsSupported,
    #[strum(serialize = "pages-per-minute")]
    PagesPerMinute,
    #[strum(serialize = "pages-per-minute-color")]
    PagesPerMinuteColor,
}

#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum JobTemplateAttribute {
    #[strum(serialize = "job-priority")]
    JobPriority,
    #[strum(serialize = "job-hold-until")]
    JobHoldUntil,
    #[strum(serialize = "job-sheets")]
    JobSheets,
    #[strum(serialize = "multiple-document-handling")]
    MultipleDocumentHandling,
    #[strum(serialize = "copies")]
    Copies,
    #[strum(serialize = "finishings")]
    Finishings,
    #[strum(serialize = "page-ranges")]
    PageRanges,
    #[strum(serialize = "sides")]
    Sides,
    #[strum(serialize = "number-up")]
    NumberUp,
    #[strum(serialize = "orientation-requested")]
    OrientationRequested,
    #[strum(serialize = "media")]
    Media,
    #[strum(serialize = "printer-resolution")]
    PrinterResolution,
    #[strum(serialize = "print-quality")]
    PrintQuality,
}

#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum JobAttribute {
    #[strum(serialize = "job-uri")]
    JobUri,
    #[strum(serialize = "job-id")]
    JobId,
    #[strum(serialize = "job-printer-uri")]
    JobPrinterUri,
    #[strum(serialize = "job-more-info")]
    JobMoreInfo,
    #[strum(serialize = "job-name")]
    JobName,
    #[strum(serialize = "job-originating-user-name")]
    JobOriginatingUserName,
    #[strum(serialize = "job-state")]
    JobState,
    #[strum(serialize = "job-state-reasons")]
    JobStateReasons,
    #[strum(serialize = "job-state-message")]
    JobStateMessage,
    #[strum(serialize = "job-detailed-status-messages")]
    JobDetailedStatusMessages,
    #[strum(serialize = "job-document-access-errors")]
    JobDocumentAccessErrors,
    #[strum(serialize = "number-of-documents")]
    NumberOfDocuments,
    #[strum(serialize = "output-device-assigned")]
    OutputDeviceAssigned,
    #[strum(serialize = "time-at-creation")]
    TimeAtCreation,
    #[strum(serialize = "time-at-processing")]
    TimeAtProcessing,
    #[strum(serialize = "time-at-completed")]
    TimeAtCompleted,
    #[strum(serialize = "job-printer-up-time")]
    JobPrinterUpTime,
    #[strum(serialize = "date-time-at-creation")]
    DateTimeAtCreation,
    #[strum(serialize = "date-time-at-processing")]
    DateTimeAtProcessing,
    #[strum(serialize = "date-time-at-completed")]
    DateTimeAtCompleted,
    #[strum(serialize = "number-of-intervening-jobs")]
    NumberOfInterveningJobs,
    #[strum(serialize = "job-message-from-operator")]
    JobMessageFromOperator,
    #[strum(serialize = "job-k-octets")]
    JobKOctets,
    #[strum(serialize = "job-impressions")]
    JobImpressions,
    #[strum(serialize = "job-media-sheets")]
    JobMediaSheets,
    #[strum(serialize = "job-k-octets-processed")]
    JobKOctetsProcessed,
    #[strum(serialize = "job-impressions-completed")]
    JobImpressionsCompleted,
    #[strum(serialize = "job-media-sheets-completed")]
    JobMediaSheetsCompleted,
}

#[derive(EnumString, strum_macros::Display, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OperationAttribute {
    #[strum(serialize = "requested-attributes")]
    RequestedAttributes,
    #[strum(serialize = "printer-uri")]
    PrinterUri,
    #[strum(serialize = "attributes-charset")]
    AttributesCharset,
    #[strum(serialize = "attributes-natural-language")]
    AttributesNaturalLanguage,
}

pub trait IppEncode {
    fn ipp_value_length_bytes() -> usize {
        2
    }
    fn ipp_bytes() -> usize {
        panic!("No implementation for ipp_bytes is provided for this type");
    }
    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self);
    fn to_ipp(&self) -> Vec<u8>;
    fn ipp_len(&self) -> usize {
        Self::ipp_bytes() + Self::ipp_value_length_bytes()
    }
}

impl IppEncode for i32 {
    fn ipp_bytes() -> usize {
        4
    }
    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {
        let value_offset_start = offset + Self::ipp_value_length_bytes();
        let value_offset_end = value_offset_start + Self::ipp_bytes();

        let slice: [u8; 4] = bytes[value_offset_start..value_offset_end]
            .try_into()
            .unwrap();
        let value = i32::from_be_bytes(slice);

        (value.ipp_len(), value)
    }

    fn to_ipp(&self) -> Vec<u8> {
        let value_bytes = self.to_be_bytes().to_vec();

        let value_length = value_bytes.len() as u16;
        let value_length_bytes = value_length.to_be_bytes().to_vec();

        [value_length_bytes, value_bytes].concat()
    }
}

impl IppEncode for String {
    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {
        let len_slice: [u8; 2] = bytes[offset..(offset + Self::ipp_value_length_bytes())]
            .try_into()
            .unwrap();
        let len = u16::from_be_bytes(len_slice);

        let value_offset_start = offset + Self::ipp_value_length_bytes();
        let value_offset_end = value_offset_start + len as usize;
        let value_slice: Vec<u8> = bytes[value_offset_start..value_offset_end].to_vec();
        let value = String::from_utf8(value_slice).unwrap();

        (value.ipp_len(), value)
    }

    fn to_ipp(&self) -> Vec<u8> {
        let value_bytes = self.as_bytes().to_vec();

        let value_length = value_bytes.len() as u16;
        let value_length_bytes = value_length.to_be_bytes().to_vec();

        [value_length_bytes, value_bytes].concat()
    }

    fn ipp_len(&self) -> usize {
        self.as_bytes().len() + Self::ipp_value_length_bytes()
    }
}

impl IppEncode for bool {
    fn ipp_bytes() -> usize {
        1
    }

    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {
        let value_offset_start = offset + Self::ipp_value_length_bytes();
        let value_offset_end = value_offset_start + Self::ipp_bytes();

        let slice: [u8; 1] = bytes[value_offset_start..value_offset_end]
            .try_into()
            .unwrap();
        let value = match i8::from_be_bytes(slice) {
            0x00 => false,
            0x01 => true,
            _ => unreachable!(),
        };

        (value.ipp_len(), value)
    }

    fn to_ipp(&self) -> Vec<u8> {
        let value_bytes = (*self as i8).to_be_bytes().to_vec();

        let value_length = value_bytes.len() as u16;
        let value_length_bytes = value_length.to_be_bytes().to_vec();

        [value_length_bytes, value_bytes].concat()
    }
}

#[derive(Debug)]
pub struct TextWithLang {
    pub lang: String,
    pub text: String,
}

impl IppEncode for TextWithLang {
    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {
        let lang_offset = offset + Self::ipp_value_length_bytes();
        let (lang_len, lang) = String::from_ipp(bytes, lang_offset);

        let text_offset = lang_offset + lang_len;
        let (text_len, text) = String::from_ipp(bytes, text_offset);

        (
            text_len + lang_len + Self::ipp_value_length_bytes(),
            Self { lang, text },
        )
    }

    fn to_ipp(&self) -> Vec<u8> {
        let lang_bytes = self.lang.to_ipp();
        let text_bytes = self.text.to_ipp();

        let total_len = lang_bytes.len() as u16 + text_bytes.len() as u16;
        let total_len_bytes = total_len.to_be_bytes().to_vec();

        [total_len_bytes, lang_bytes, text_bytes].concat()
    }

    fn ipp_len(&self) -> usize {
        Self::ipp_value_length_bytes() + self.lang.ipp_len() + self.text.ipp_len()
    }
}

impl IppEncode for DateTime<Utc> {
    fn ipp_bytes() -> usize {
        11
    }

    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {
        let start = offset + Self::ipp_value_length_bytes();

        let slice_offset = start + 8;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let direction = u8::from_be_bytes(slice) as char;

        let slice_offset = start + 9;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let hour_from_utc = u8::from_be_bytes(slice);

        let slice_offset = start + 10;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let minutes_from_utc = u8::from_be_bytes(slice);

        let mut drift = (hour_from_utc * 60 - minutes_from_utc) as i8;
        if direction == '-' {
            drift *= -1;
        }

        let slice_offset = start;
        let slice: [u8; 2] = bytes[slice_offset..slice_offset + 2].try_into().unwrap();
        let year = u16::from_be_bytes(slice);

        let slice_offset = start + 2;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let month = u8::from_be_bytes(slice);

        let slice_offset = start + 3;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let day = u8::from_be_bytes(slice);

        let slice_offset = start + 4;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let hour = u8::from_be_bytes(slice);

        let slice_offset = start + 5;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let minutes = (i8::from_be_bytes(slice) + drift) as u8;

        let slice_offset = start + 6;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let seconds = u8::from_be_bytes(slice);

        let slice_offset = start + 7;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let deciseconds = u8::from_be_bytes(slice);

        let value = Utc
            .ymd(year as i32, month as u32, day as u32)
            .and_hms_micro(
                hour as u32,
                minutes as u32,
                seconds as u32,
                deciseconds as u32 * 100,
            );

        (value.ipp_len(), value)
    }

    fn to_ipp(&self) -> Vec<u8> {
        let value_length = self.ipp_len() as u16;
        let value_length_bytes = value_length.to_be_bytes().to_vec();

        let year = self.year() as u16;
        let year_bytes = year.to_be_bytes().to_vec();

        let month = self.month() as u8;
        let month_bytes = month.to_be_bytes().to_vec();

        let day = self.day() as u8;
        let day_bytes = day.to_be_bytes().to_vec();

        let hour = self.hour() as u8;
        let hour_bytes = hour.to_be_bytes().to_vec();

        let minutes = self.minute() as u8;
        let minutes_bytes = minutes.to_be_bytes().to_vec();

        let seconds = self.second() as u8;
        let seconds_bytes = seconds.to_be_bytes().to_vec();

        let deciseconds = 0 as u8;
        let deciseconds_bytes = deciseconds.to_be_bytes().to_vec();

        let local_minus_utc = self.timezone().fix().local_minus_utc() / 60;

        let mut direction = '+';
        if local_minus_utc < 0 {
            direction = '-';
        }
        let direction_bytes = (direction as u8).to_be_bytes().to_vec();

        let hour_from_utc = (local_minus_utc / 60) as u8;
        let hour_from_utc_bytes = hour_from_utc.to_be_bytes().to_vec();

        let minutes_from_utc = (local_minus_utc % 60) as u8;
        let minutes_from_utc_bytes = minutes_from_utc.to_be_bytes().to_vec();

        [
            value_length_bytes,
            year_bytes,
            month_bytes,
            day_bytes,
            hour_bytes,
            minutes_bytes,
            deciseconds_bytes,
            seconds_bytes,
            direction_bytes,
            hour_from_utc_bytes,
            minutes_from_utc_bytes,
        ]
        .concat()
    }
}

pub enum AttributeValue {
    TextWithoutLang(String),
    Number(i32),
    Boolean(bool),
    TextWithLang(TextWithLang),
    DateTime(DateTime<Utc>),
}

impl AttributeValue {
    fn from_ipp(bytes: &Vec<u8>, offset: usize, value_tag: ValueTag) -> (usize, Self) {
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

    fn to_ipp(&self) -> Vec<u8> {
        match self {
            Self::Boolean(raw_value) => raw_value.to_ipp(),
            Self::Number(raw_value) => raw_value.to_ipp(),
            Self::DateTime(raw_value) => raw_value.to_ipp(),
            Self::TextWithLang(raw_value) => raw_value.to_ipp(),
            Self::TextWithoutLang(raw_value) => raw_value.to_ipp(),
        }
    }

    fn ipp_len(&self) -> usize {
        match self {
            Self::Boolean(raw_value) => raw_value.ipp_len(),
            Self::Number(raw_value) => raw_value.ipp_len(),
            Self::DateTime(raw_value) => raw_value.ipp_len(),
            Self::TextWithLang(raw_value) => raw_value.ipp_len(),
            Self::TextWithoutLang(raw_value) => raw_value.ipp_len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeName {
    Operation(OperationAttribute),
    Printer(PrinterAttribute),
    JobTemplate(JobTemplateAttribute),
    Job(JobAttribute),
    Unsupported(String),
}

impl AttributeName {
    pub fn from_str(str: &str) -> Self {
        if let Ok(n) = OperationAttribute::from_str(str) {
            Self::Operation(n)
        } else if let Ok(n) = PrinterAttribute::from_str(str) {
            Self::Printer(n)
        } else if let Ok(n) = JobTemplateAttribute::from_str(str) {
            Self::JobTemplate(n)
        } else if let Ok(n) = JobAttribute::from_str(str) {
            Self::Job(n)
        } else {
            Self::Unsupported(String::from(str))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Operation(attr) => attr.to_string(),
            Self::Printer(attr) => attr.to_string(),
            Self::JobTemplate(attr) => attr.to_string(),
            Self::Job(attr) => attr.to_string(),
            Self::Unsupported(attr) => String::from(attr),
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Self::Unsupported(attr) = self {
            attr == ""
        } else {
            false
        }
    }
}

impl IppEncode for AttributeName {
    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {
        let (delta, raw_name) = String::from_ipp(bytes, offset);
        (delta, Self::from_str(&raw_name))
    }

    fn to_ipp(&self) -> Vec<u8> {
        self.to_string().to_ipp()
    }

    fn ipp_len(&self) -> usize {
        self.to_string().ipp_len()
    }
}

pub struct Attribute {
    pub tag: ValueTag,
    pub name: AttributeName,
    pub values: Vec<AttributeValue>,
}

impl Attribute {
    fn decode_one(bytes: &Vec<u8>, offset: usize) -> (usize, bool, Option<Self>) {
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

    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Option<Self>) {
        let (mut first_offset, _, first_attribute_opt) = Self::decode_one(bytes, offset);

        let next_offset = offset + first_offset;

        if let Some(mut first_attribute) = first_attribute_opt {
            if next_offset > bytes.len() {
                (0, None)
            } else {
                let (mut next_offset, mut has_name, mut next_attribute_opt) =
                    Self::decode_one(bytes, next_offset);

                loop {
                    if let Some(mut next_attribute) = next_attribute_opt {
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
                    } else {
                        break;
                    }
                }

                (first_offset, Some(first_attribute))
            }
        } else {
            (0, None)
        }
    }

    fn to_ipp(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.ipp_len());
        if self.values.len() > 0 {
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

    fn ipp_len(&self) -> usize {
        if self.values.len() == 0 {
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

pub struct AttributeGroup {
    pub tag: DelimiterTag,
    pub attributes: HashMap<AttributeName, Attribute>,
}

impl IppEncode for HashMap<DelimiterTag, AttributeGroup> {
    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {
        let mut decoded: Self = HashMap::new();

        let mut shifting_offset = offset;

        let read_tag = |bytes: &Vec<u8>, offset: usize| -> (usize, Option<DelimiterTag>) {
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

        for (_, group) in self {
            // write delimiter tag
            vec.append(&mut (group.tag as u8).to_be_bytes().to_vec());

            for (_, attribute) in &group.attributes {
                // write attribute
                vec.append(&mut attribute.to_ipp());
            }
        }

        vec
    }

    fn ipp_len(&self) -> usize {
        let mut len: usize = 0;

        for (_, group) in self {
            len += 1; // delimiter tag
            for (_, attribute) in &group.attributes {
                len += attribute.ipp_len();
            }
        }

        len += 1; // end-of-attributes tag

        len
    }
}

pub struct IppVersion {
    pub major: u8,
    pub minor: u8,
}

// pub struct OperationBase {
//     pub version: IppVersion,
//     pub request_id: u8,
//     pub attribute_groups: HashMap<DelimiterTag, AttributeGroup>,
// }

// pub struct OperationRequest {
//     pub base: OperationBase,
//     pub operation_id: OperationID,
// }

// pub struct OperationResponse {
//     pub base: OperationBase,
//     pub status_code: StatusCode,
// }

pub struct Operation {
    pub version: IppVersion,
    pub operation_id_or_status_code: u16,
    pub request_id: u32,
    pub attribute_groups: HashMap<DelimiterTag, AttributeGroup>,
}

impl IppEncode for Operation {
    fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {
        let mut shifting_offset = offset;

        // read version.major
        let slice: [u8; 1] = bytes[offset..offset + 1].try_into().unwrap();
        let major = u8::from_be_bytes(slice);
        shifting_offset += slice.len();

        // read version.minor
        let slice: [u8; 1] = bytes[offset..offset + 1].try_into().unwrap();
        let minor = u8::from_be_bytes(slice);
        shifting_offset += slice.len();

        // read operation-id or status-code
        let slice: [u8; 2] = bytes[offset..offset + 1].try_into().unwrap();
        let operation_id_or_status_code = u16::from_be_bytes(slice);
        shifting_offset += slice.len();

        // read request-id
        let slice: [u8; 4] = bytes[offset..offset + 1].try_into().unwrap();
        let request_id = u32::from_be_bytes(slice);
        shifting_offset += slice.len();

        // read attribute groups
        let (delta, attribute_groups): (usize, HashMap<DelimiterTag, AttributeGroup>) =
            HashMap::from_ipp(bytes, shifting_offset);
        shifting_offset += delta;

        (
            shifting_offset - offset,
            Self {
                version: IppVersion { major, minor },
                request_id,
                operation_id_or_status_code,
                attribute_groups,
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
        ]
        .concat()
    }

    fn ipp_len(&self) -> usize {
        self.version.major.to_be_bytes().len()
            + self.version.minor.to_be_bytes().len()
            + self.operation_id_or_status_code.to_be_bytes().len()
            + self.request_id.to_be_bytes().len()
            + self.attribute_groups.ipp_len()
    }
}

impl Operation {
    pub fn operation_id(&self) -> Option<OperationID> {
        OperationID::from_repr(self.operation_id_or_status_code as usize)
    }
    pub fn status_code(&self) -> Option<StatusCode> {
        StatusCode::from_repr(self.operation_id_or_status_code as usize)
    }
}
