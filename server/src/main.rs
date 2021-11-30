use astro_dnssd::DNSServiceBuilder;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use chrono::{DateTime, Utc};

use ipp_encoder::{
    Attribute, AttributeGroup, AttributeName, AttributeValue, CompressionSupportedKeyword,
    DelimiterTag, IppEncode, Operation, OperationID, PdlOverrideSupportedKeyword, PrinterAttribute,
    PrinterState, PrinterStateReasonKeyword, TextWithLang, UriAuthenticationSupportedKeyword,
    UriSecuritySupportedKeyword, ValueTag,
};

fn test_encoding<T: IppEncode + std::fmt::Debug>(raw: T) {
    println!("raw: {:?}", raw);
    let encoded = raw.to_ipp();
    println!("encoded: {:?}", encoded);
    let decoded = T::from_ipp(&encoded, 0);
    println!("decoded: {:?}", decoded);
}

fn test() {
    // // i32
    // test_encoding(32 as i32);

    // // String
    // let text_wo_lang = String::from("Text Without Lang");
    // test_encoding(text_wo_lang);

    // // bool
    // test_encoding(true);
    // test_encoding(false);

    // // TextWithLang
    // let text_with_lang = TextWithLang {
    //     lang: String::from("en"),
    //     text: String::from("Text With Lang"),
    // };
    // test_encoding(text_with_lang);

    // // DateTime
    // let date = Utc::now();
    // test_encoding(date);o
}

#[tokio::main]
async fn main() {
    const PORT: u16 = 6363;

    let address = SocketAddr::from(([127, 0, 0, 1], PORT));

    let hostname = gethostname::gethostname()
        .to_str()
        .unwrap_or("127.0.0.1")
        .to_string();
    let uri = format!("ipp//{}:{}/", hostname, PORT);

    const NAME: &str = "Rust IPP Printer";

    let printer = Arc::new(IppPrinter::new(&uri, &NAME));

    let make_svc = make_service_fn(move |_| {
        let inner_printer = printer.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let inner_printer = inner_printer.clone();
                async move { http_handler(req, inner_printer).await }
            }))
        }
    });

    let server = Server::bind(&address).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    let dns_service = DNSServiceBuilder::new("_ipp._tcp", 6363)
        .with_name(&NAME)
        .register();

    match dns_service {
        Ok(dns) => {
            println!("DNS service registered: {:?}", dns);

            if let Err(e) = graceful.await {
                eprintln!("server error: {}", e);
            } else {
                println!("Dropping... {:?}", dns);
                println!("gracefully shut down!");
            }
        }
        Err(e) => {
            eprintln!("Error registering dns service: {:?}", e);
        }
    }
}

async fn http_handler(
    req: Request<Body>,
    printer: Arc<IppPrinter>,
) -> Result<Response<Body>, Infallible> {
    let mut res = Response::new(Body::empty());

    println!("Requested in {}, {}", req.method(), req.uri().path());
    println!(
        "IPP Printer - printer_uri_supported: {:?}",
        printer.printer_uri_supported()
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

            let bytes = printer.handle(&bytes);

            *res.status_mut() = hyper::StatusCode::OK;
            *res.body_mut() = bytes.into();
        }
        _ => {
            *res.status_mut() = hyper::StatusCode::NOT_FOUND;
        }
    }

    Ok(res)
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

struct IppJob;

pub struct IppPrinter {
    uri: String,
    name: String,
    state: PrinterState,
    started_at: DateTime<Utc>,
    jobs: Vec<IppJob>,
}

impl IppPrinter {
    pub fn new(uri: &str, name: &str) -> Self {
        Self {
            uri: String::from(uri),
            name: String::from(name),
            state: PrinterState::Stopped,
            started_at: Utc::now(),
            jobs: Vec::new(),
        }
    }

    pub fn handle(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let (_, request) = Operation::from_ipp(&bytes, 0);

        println!("Request: {:?}", request);

        match request.operation_id().unwrap() {
            OperationID::ValidateJob => {
                let response = Operation {
                    version: request.version,
                    request_id: request.request_id,
                    operation_id_or_status_code: ipp_encoder::StatusCode::SuccessfulOk as u16,
                    attribute_groups: HashMap::new(),
                };

                println!("Response: {:?}", response);

                response.to_ipp()
            }
            _ => Vec::new(),
        }
    }

    fn intrinsic_attributes(&self) -> HashMap<DelimiterTag, AttributeGroup> {
        let printer_uri_supported = self.printer_uri_supported();
        let uri_security_supported = self.uri_security_supported();
        let uri_authentication_supported = self.uri_authentication_supported();
        let printer_name = self.printer_name();
        let printer_state_reasons = self.printer_state_reasons();
        let printer_state = self.printer_state();
        let operation_supported = self.operation_supported();
        let charset_configured = self.charset_configured();
        let charset_supported = self.charset_supported();
        let natural_language_configured = self.natural_language_configured();
        let generated_natural_language_supported = self.generated_natural_language_supported();
        let document_format_default = self.document_format_default();
        let document_format_supported = self.document_format_supported();
        let printer_is_accepting_jobs = self.printer_is_accepting_jobs();
        let queue_job_count = self.queue_job_count();
        let pdl_override_supported = self.pdl_override_supported();
        let printer_up_time = self.printer_up_time();
        let printer_current_time = self.printer_current_time();
        let compression_supported = self.compression_supported();

        let printer_attribute_group = AttributeGroup {
            tag: DelimiterTag::PrinterAttributes,
            attributes: HashMap::from([
                (printer_uri_supported.name.clone(), printer_uri_supported),
                (uri_security_supported.name.clone(), uri_security_supported),
                (
                    uri_authentication_supported.name.clone(),
                    uri_authentication_supported,
                ),
                (printer_name.name.clone(), printer_name),
                (printer_state.name.clone(), printer_state),
                (printer_state_reasons.name.clone(), printer_state_reasons),
                (operation_supported.name.clone(), operation_supported),
                (charset_configured.name.clone(), charset_configured),
                (charset_supported.name.clone(), charset_supported),
                (
                    natural_language_configured.name.clone(),
                    natural_language_configured,
                ),
                (
                    generated_natural_language_supported.name.clone(),
                    generated_natural_language_supported,
                ),
                (
                    document_format_default.name.clone(),
                    document_format_default,
                ),
                (
                    printer_is_accepting_jobs.name.clone(),
                    printer_is_accepting_jobs,
                ),
                (
                    document_format_supported.name.clone(),
                    document_format_supported,
                ),
                (queue_job_count.name.clone(), queue_job_count),
                (pdl_override_supported.name.clone(), pdl_override_supported),
                (printer_up_time.name.clone(), printer_up_time),
                (printer_current_time.name.clone(), printer_current_time),
                (compression_supported.name.clone(), compression_supported),
            ]),
        };

        HashMap::from([(printer_attribute_group.tag, printer_attribute_group)])
    }
}

// intrinsic printer attribute constructor
impl IppPrinter {
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
                // AttributeValue::Number(OperationID::PrintJob as i32),
                AttributeValue::Number(OperationID::ValidateJob as i32),
                // AttributeValue::Number(OperationID::CancelJob as i32),
                // AttributeValue::Number(OperationID::GetPrinterAttributes as i32),
                // AttributeValue::Number(OperationID::GetJobAttributes as i32),
                // AttributeValue::Number(OperationID::GetJobs as i32),
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
                AttributeValue::TextWithoutLang(String::from("application?octet-stream")),
                AttributeValue::TextWithoutLang(String::from("application/pdf")),
                AttributeValue::TextWithoutLang(String::from("application/postscript")),
            ],
        }
    }

    pub fn printer_is_accepting_jobs(&self) -> Attribute {
        Attribute {
            tag: ValueTag::Boolean,
            name: AttributeName::Printer(PrinterAttribute::PrinterIsAcceptingJobs),
            values: vec![AttributeValue::Boolean(true)],
        }
    }

    pub fn queue_job_count(&self) -> Attribute {
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
}
