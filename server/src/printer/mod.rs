use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::process::Command;
use std::str::FromStr;
use std::sync::Arc;

use astro_dnssd::DNSServiceBuilder;
use chrono::{DateTime, Utc};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};

use ipp_encoder::{
    encoder::{
        Attribute, AttributeGroup, AttributeName, AttributeValue, IppEncode, IppVersion, Operation,
        TextWithLang,
    },
    spec::{
        attribute::{OperationAttribute, PrinterAttribute},
        operation::{OperationID, PrinterState, StatusCode as IppStatusCode},
        tag::{DelimiterTag, ValueTag},
        value::{
            CompressionSupportedKeyword, PdlOverrideSupportedKeyword, PrinterStateReasonKeyword,
            UriAuthenticationSupportedKeyword, UriSecuritySupportedKeyword,
        },
    },
};

mod job;
use job::IppJob;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub struct IppPrinter {
    uri: String,
    port: u16,
    name: String,
    state: PrinterState,
    started_at: DateTime<Utc>,
    jobs: Vec<IppJob>,
}

impl IppPrinter {
    pub async fn start(uri: &str, port: u16, name: &str) {
        let printer = Arc::new(IppPrinter::new(&uri, port, name));

        let make_svc = make_service_fn(move |_| {
            let inner_printer = printer.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    let inner_printer = inner_printer.clone();
                    async move { inner_printer.http_handle(req).await }
                }))
            }
        });

        let address = SocketAddr::from(([127, 0, 0, 1], port));
        let server = Server::bind(&address).serve(make_svc);
        let graceful = server.with_graceful_shutdown(shutdown_signal());

        let dns_service = DNSServiceBuilder::new("_ipp._tcp", port)
            .with_name(&name)
            .register();

        match dns_service {
            Ok(dns) => {
                println!("DNS service registered: {:?}", dns);

                IppPrinter::system_add_printer(&name, port);

                if let Err(e) = graceful.await {
                    eprintln!("server error: {}", e);

                    IppPrinter::system_remove_printer(&name);
                } else {
                    println!("Dropping... {:?}", dns);
                    println!("gracefully shut down!");

                    IppPrinter::system_remove_printer(&name);
                }
            }
            Err(e) => {
                eprintln!("Error registering dns service: {:?}", e);
            }
        }
    }

    fn new(uri: &str, port: u16, name: &str) -> Self {
        Self {
            uri: String::from(uri),
            port,
            name: String::from(name),
            state: PrinterState::Idle,
            started_at: Utc::now(),
            jobs: Vec::new(),
        }
    }

    fn system_add_printer(name: &str, port: u16) {
        if cfg!(target_os = "macos") {
            Command::new("lpadmin")
                .args(["-p", &name.replace(" ", "_")])
                .args(["-D", name])
                .args(["-v", &format!("ipp://localhost:{}", port)])
                .args(["-P", "/System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/PrintCore.framework/Versions/A/Resources/Generic.ppd"])
                .args(["-o", "printer-is-shared=false", "-E"])
                .output()
                .expect("failed to auto-add printer to system with lpadmin");
        } else {
            // no support yet
        }
    }

    fn system_remove_printer(name: &str) {
        if cfg!(target_os = "macos") {
            Command::new("lpadmin")
                .args(["-x", &name.replace(" ", "_")])
                .output()
                .expect("failed to auto-remove printer to system with lpadmin");
        } else {
            // no support yet
        }
    }

    async fn http_handle(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let mut res = Response::new(Body::empty());

        println!("============================");
        println!("Requested in {}, {}", req.method(), req.uri().path());
        println!(
            "IPP Printer - printer_uri_supported: {:?}\n",
            self.printer_uri_supported()
        );

        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                *res.body_mut() = Body::from("IPP Server");
            }
            (&Method::POST, "/") => {
                let bytes = hyper::body::to_bytes(req.into_body())
                    .await
                    .unwrap()
                    .to_vec();

                let bytes = self.handle(&bytes);

                // let (_, operation) = Operation::from_ipp(&bytes, 0);
                // println!("\nResponse Operation Counter: {}", operation.to_json());

                *res.status_mut() = hyper::StatusCode::OK;
                *res.body_mut() = bytes.into();

                // println!("\nResponse Body: {:?}", *res.body());
                println!("============================");
            }
            _ => {
                *res.status_mut() = hyper::StatusCode::NOT_FOUND;
            }
        }

        Ok(res)
    }

    pub fn ps_to_pdf(ps_path: &str, pdf_path: &str) {
        Command::new("gs")
            .args([
                "-q",
                "-sPAPERSIZE=a4",
                "-dSAFER",
                "-dBATCH",
                "-dNOPAUSE",
                "-sDEVICE=pdfimage8",
                "-r600",
                "-dDownScaleFactor=3",
            ])
            .arg(format!("-sOutputFile={}", pdf_path))
            .args(["-c", "save", "pop"])
            .arg("-f")
            .arg(ps_path)
            .output()
            .expect("failed to execute gs command for ps-pdf conversion");
    }

    fn handle(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let (_, request) = Operation::from_ipp(&bytes, 0);

        println!("\nRequest: {}", request.to_json());
        println!("OperationID: {}\n", request.operation_id().unwrap() as i32);

        let mut response = Operation {
            version: IppVersion { major: 1, minor: 1 },
            request_id: request.request_id,
            operation_id_or_status_code: IppStatusCode::SuccessfulOk as u16,
            attribute_groups: HashMap::new(),
            data: Vec::new(),
        };

        let operation_attribute_group = self.request_operation_attributes();
        response
            .attribute_groups
            .insert(operation_attribute_group.tag, operation_attribute_group);

        if request.version.major != 1 {
            response.operation_id_or_status_code =
                IppStatusCode::ServerErrorVersionNotSupported as u16;
        } else {
            if !self
                .operation_supported()
                .values
                .contains(&AttributeValue::Number(
                    request.operation_id_or_status_code as i32,
                ))
            {
                response.operation_id_or_status_code =
                    IppStatusCode::ServerErrorOperationNotSupported as u16;
            } else {
                if let Some((supported, unsupported)) = self.request_printer_attributes(&request) {
                    // insert unsupported-attributes group
                    let mut unsupported_group = AttributeGroup {
                        tag: DelimiterTag::UnsupportedAttributes,
                        attributes: HashMap::new(),
                    };
                    for value in unsupported {
                        let attribute = Attribute {
                            tag: ValueTag::Unsupported,
                            name: AttributeName::Unsupported(value),
                            values: vec![AttributeValue::TextWithoutLang(String::from(
                                "unsupported",
                            ))],
                        };
                        unsupported_group
                            .attributes
                            .insert(attribute.name.clone(), attribute);
                    }
                    response
                        .attribute_groups
                        .insert(unsupported_group.tag, unsupported_group);

                    // insert printer-attributes group
                    let printer_attribute_group = AttributeGroup {
                        tag: DelimiterTag::PrinterAttributes,
                        attributes: supported
                            .into_iter()
                            .map(|attr| (attr.name.clone(), attr))
                            .collect(),
                    };
                    response
                        .attribute_groups
                        .insert(printer_attribute_group.tag, printer_attribute_group);
                }
                match request.operation_id().unwrap() {
                    OperationID::PrintJob => {
                        let path = "data.ps";
                        std::fs::write(path, &request.data).unwrap();
                        let pdf_path = "converted.pdf";
                        IppPrinter::ps_to_pdf(path, pdf_path);
                    }
                    OperationID::GetPrinterAttributes
                    | OperationID::ValidateJob
                    | OperationID::CancelJob
                    | OperationID::GetJobAttributes
                    | OperationID::GetJobs => {}
                    _ => {}
                }
            };
        }

        println!("\nResponse: {}\n", response.to_json());

        response.to_ipp()
    }
}

// operation attribute constructor
impl IppPrinter {
    fn printer_uri(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Uri,
            name: AttributeName::Operation(OperationAttribute::PrinterUri),
            values: vec![AttributeValue::TextWithoutLang(self.uri.clone())],
        }
    }

    fn attributes_charset(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Charset,
            name: AttributeName::Operation(OperationAttribute::AttributesCharset),
            values: vec![AttributeValue::TextWithoutLang(String::from("utf-8"))],
        }
    }

    fn attributes_natural_language(&self) -> Attribute {
        Attribute {
            tag: ValueTag::NaturalLanguage,
            name: AttributeName::Operation(OperationAttribute::AttributesNaturalLanguage),
            values: vec![AttributeValue::TextWithoutLang(String::from("en-US"))],
        }
    }

    fn request_operation_attributes(&self) -> AttributeGroup {
        let printer_uri = self.printer_uri();
        let attributes_charset = self.attributes_charset();
        let attributes_natural_language = self.attributes_natural_language();

        AttributeGroup {
            tag: DelimiterTag::OperationAttributes,
            attributes: HashMap::from([
                (printer_uri.name.clone(), printer_uri),
                (attributes_charset.name.clone(), attributes_charset),
                (
                    attributes_natural_language.name.clone(),
                    attributes_natural_language,
                ),
            ]),
        }
    }
}

// intrinsic printer attribute constructor
impl IppPrinter {
    pub fn ipp_printer_versions_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Keyword,
            name: AttributeName::Printer(PrinterAttribute::IppVersionsSupported),
            values: vec![AttributeValue::TextWithoutLang(String::from("1.1"))],
        }
    }

    pub fn printer_uri_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Uri,
            name: AttributeName::Printer(PrinterAttribute::PrinterUriSupported),
            values: vec![AttributeValue::TextWithoutLang(self.uri.clone())],
        }
    }

    pub fn uri_security_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Keyword,
            name: AttributeName::Printer(PrinterAttribute::UriSecuritySupported),
            values: vec![AttributeValue::TextWithoutLang(
                UriSecuritySupportedKeyword::None.to_string(),
            )],
        }
    }

    pub fn uri_authentication_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Keyword,
            name: AttributeName::Printer(PrinterAttribute::UriAuthenticationSupported),
            values: vec![AttributeValue::TextWithoutLang(
                UriAuthenticationSupportedKeyword::None.to_string(),
            )],
        }
    }

    pub fn printer_name(&self) -> Attribute {
        Attribute {
            tag: ValueTag::NameWithLanguage,
            name: AttributeName::Printer(PrinterAttribute::PrinterName),
            values: vec![AttributeValue::TextWithLang(TextWithLang {
                lang: String::from("en"),
                text: self.name.clone(),
            })],
        }
    }

    pub fn printer_state_reasons(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Keyword,
            name: AttributeName::Printer(PrinterAttribute::PrinterStateReasons),
            values: vec![AttributeValue::TextWithoutLang(
                PrinterStateReasonKeyword::None.to_string(),
            )],
        }
    }

    pub fn printer_state(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Enum,
            name: AttributeName::Printer(PrinterAttribute::PrinterState),
            values: vec![AttributeValue::Number(self.state as i32)],
        }
    }

    pub fn operation_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Enum,
            name: AttributeName::Printer(PrinterAttribute::OperationsSupported),
            values: vec![
                AttributeValue::Number(OperationID::PrintJob as i32),
                AttributeValue::Number(OperationID::ValidateJob as i32),
                AttributeValue::Number(OperationID::CancelJob as i32),
                AttributeValue::Number(OperationID::GetPrinterAttributes as i32),
                AttributeValue::Number(OperationID::GetJobAttributes as i32),
                AttributeValue::Number(OperationID::GetJobs as i32),
            ],
        }
    }

    pub fn charset_configured(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Charset,
            name: AttributeName::Printer(PrinterAttribute::CharsetConfigured),
            values: vec![AttributeValue::TextWithoutLang(String::from("utf-8"))],
        }
    }

    pub fn charset_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Charset,
            name: AttributeName::Printer(PrinterAttribute::CharsetSupported),
            values: vec![AttributeValue::TextWithoutLang(String::from("utf-8"))],
        }
    }

    pub fn natural_language_configured(&self) -> Attribute {
        Attribute {
            tag: ValueTag::NaturalLanguage,
            name: AttributeName::Printer(PrinterAttribute::NaturalLanguageConfigured),
            values: vec![AttributeValue::TextWithoutLang(String::from("en-US"))],
        }
    }

    pub fn generated_natural_language_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::NaturalLanguage,
            name: AttributeName::Printer(PrinterAttribute::GeneratedNaturalLanguageSupported),
            values: vec![AttributeValue::TextWithoutLang(String::from("en-US"))],
        }
    }

    pub fn document_format_default(&self) -> Attribute {
        Attribute {
            tag: ValueTag::MimeMediaType,
            name: AttributeName::Printer(PrinterAttribute::DocumentFormatDefault),
            values: vec![AttributeValue::TextWithoutLang(String::from(
                "application/postscript",
            ))],
        }
    }

    pub fn document_format_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::MimeMediaType,
            name: AttributeName::Printer(PrinterAttribute::DocumentFormatSupported),
            values: vec![
                AttributeValue::TextWithoutLang(String::from("text/html")),
                AttributeValue::TextWithoutLang(String::from("text/plain")),
                AttributeValue::TextWithoutLang(String::from("application/vnd.hp-PCL")),
                AttributeValue::TextWithoutLang(String::from("application/octet-stream")),
                AttributeValue::TextWithoutLang(String::from("application/pdf")),
                AttributeValue::TextWithoutLang(String::from("application/postscript")),
            ],
        }
    }

    pub fn printer_is_accepting_jobs(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Boolean,
            name: AttributeName::Printer(PrinterAttribute::PrinterIsAcceptingJobs),
            // FIXME: when is printer not accepting jobs?
            values: vec![AttributeValue::Boolean(true)],
        }
    }

    pub fn queued_job_count(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Integer,
            name: AttributeName::Printer(PrinterAttribute::QueuedJobCount),
            values: vec![AttributeValue::Number(self.jobs.len() as i32)],
        }
    }

    pub fn pdl_override_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Keyword,
            name: AttributeName::Printer(PrinterAttribute::PdlOverrideSupported),
            values: vec![AttributeValue::TextWithoutLang(
                PdlOverrideSupportedKeyword::NotAttempted.to_string(),
            )],
        }
    }

    pub fn printer_up_time(&self) -> Attribute {
        let now = Utc::now();
        let uptime = now - self.started_at;

        Attribute {
            tag: ValueTag::Integer,
            name: AttributeName::Printer(PrinterAttribute::PrinterUpTime),
            values: vec![AttributeValue::Number(uptime.num_seconds() as i32)],
        }
    }

    pub fn printer_current_time(&self) -> Attribute {
        Attribute {
            tag: ValueTag::DateTime,
            name: AttributeName::Printer(PrinterAttribute::PrinterCurrentTime),
            values: vec![AttributeValue::DateTime(Utc::now())],
        }
    }

    pub fn compression_supported(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Keyword,
            name: AttributeName::Printer(PrinterAttribute::CompressionSupported),
            values: vec![
                AttributeValue::TextWithoutLang(CompressionSupportedKeyword::Deflate.to_string()),
                AttributeValue::TextWithoutLang(CompressionSupportedKeyword::Gzip.to_string()),
            ],
        }
    }

    fn request_printer_attribute(&self, attribute_name: &str) -> Option<Attribute> {
        match PrinterAttribute::from_str(attribute_name) {
            Ok(printer_attr_name) => match printer_attr_name {
                PrinterAttribute::IppVersionsSupported => {
                    Some(self.ipp_printer_versions_supported())
                }
                PrinterAttribute::PrinterUriSupported => Some(self.printer_uri_supported()),
                PrinterAttribute::UriSecuritySupported => Some(self.uri_security_supported()),
                PrinterAttribute::UriAuthenticationSupported => {
                    Some(self.uri_authentication_supported())
                }
                PrinterAttribute::PrinterName => Some(self.printer_name()),
                PrinterAttribute::PrinterState => Some(self.printer_state()),
                PrinterAttribute::PrinterStateReasons => Some(self.printer_state_reasons()),
                PrinterAttribute::OperationsSupported => Some(self.operation_supported()),
                PrinterAttribute::CharsetConfigured => Some(self.charset_configured()),
                PrinterAttribute::CharsetSupported => Some(self.charset_supported()),
                PrinterAttribute::NaturalLanguageConfigured => {
                    Some(self.natural_language_configured())
                }
                PrinterAttribute::GeneratedNaturalLanguageSupported => {
                    Some(self.generated_natural_language_supported())
                }
                PrinterAttribute::DocumentFormatDefault => Some(self.document_format_default()),
                PrinterAttribute::DocumentFormatSupported => Some(self.document_format_supported()),
                PrinterAttribute::PrinterIsAcceptingJobs => Some(self.printer_is_accepting_jobs()),
                PrinterAttribute::QueuedJobCount => Some(self.queued_job_count()),
                PrinterAttribute::PdlOverrideSupported => Some(self.pdl_override_supported()),
                PrinterAttribute::PrinterUpTime => Some(self.printer_up_time()),
                PrinterAttribute::PrinterCurrentTime => Some(self.printer_current_time()),
                PrinterAttribute::CompressionSupported => Some(self.compression_supported()),
                _ => None,
            },
            Err(_) => None,
        }
    }

    fn request_printer_attributes(
        &self,
        request: &Operation,
    ) -> Option<(Vec<Attribute>, Vec<String>)> {
        match request
            .attribute_groups
            .get(&DelimiterTag::OperationAttributes)
        {
            Some(operation_attribute_group) => {
                match operation_attribute_group
                    .attributes
                    .get(&AttributeName::Operation(
                        OperationAttribute::RequestedAttributes,
                    )) {
                    Some(requested) => {
                        let mut supported = Vec::new();
                        let mut unsupported = Vec::new();

                        for value in &requested.values {
                            if let AttributeValue::TextWithoutLang(value_str) = value {
                                if let Some(attribute) = self.request_printer_attribute(value_str) {
                                    supported.push(attribute);
                                } else {
                                    unsupported.push(String::from(value_str));
                                }
                            }
                        }

                        Some((supported, unsupported))
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
}
