use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// ref: [rfc8011](https://datatracker.ietf.org/doc/html/rfc8011#section-5.4)
#[derive(
    Serialize,
    Deserialize,
    EnumString,
    strum_macros::Display,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
)]
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

#[derive(
    Serialize,
    Deserialize,
    EnumString,
    strum_macros::Display,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
)]
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

#[derive(
    Serialize,
    Deserialize,
    EnumString,
    strum_macros::Display,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
)]
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

#[derive(
    Serialize,
    Deserialize,
    EnumString,
    strum_macros::Display,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
)]
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
