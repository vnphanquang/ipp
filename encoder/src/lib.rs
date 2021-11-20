use chrono::{DateTime, Datelike, Offset, TimeZone, Timelike, Utc};
use std::collections::HashMap;
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
#[derive(FromRepr, Debug, PartialEq, Eq, Clone, Copy)]
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
#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UriSecuritySupportedKeyword {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "tls")]
    TLS,
}

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.2)
#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
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
#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PdlOverrideSupportedKeyword {
    #[strum(serialize = "attempted")]
    Attempted,
    #[strum(serialize = "not-attempted")]
    NotAttempted,
}

/// [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4.32)
#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
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
#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
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
pub struct AttributeGroup {
    pub tag: DelimiterTag,
    pub attributes: HashMap<AttributeName, Attribute>,
}

pub struct OperationVersion {
    pub major: u8,
    pub minor: u8,
}

pub struct OperationBase {
    pub version: OperationVersion,
    pub request_id: u8,
    pub attribute_groups: HashMap<DelimiterTag, AttributeGroup>,
}

pub struct OperationRequest {
    pub base: OperationBase,
    pub operation_id: OperationID,
}

pub struct OperationResponse {
    pub base: OperationBase,
    pub status_code: StatusCode,
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
    String(String),
    Number(i32),
    Boolean(bool),
    TextWithLang(TextWithLang),
    DateTime(DateTime<Utc>),
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeName {
    Operation(OperationAttribute),
    Printer(PrinterAttribute),
    JobTemplate(JobTemplateAttribute),
    Job(JobAttribute),
}

// pub struct Attribute {
//     pub tag: ValueTag,
//     pub name: AttributeName,
//     pub values: Vec<AttributeValue>,
// }

// impl IppEncode for Attribute {
//     fn from_ipp(bytes: &Vec<u8>, offset: usize) -> (usize, Self) {}

//     fn to_ipp(&self) -> Vec<u8> {}

//     fn ipp_len(&self) -> usize {}
// }
