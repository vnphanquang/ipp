use astro_dnssd::DNSServiceBuilder;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

mod printer;

use printer::IppPrinter;

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

    let printer = Arc::new(IppPrinter::new(&uri, NAME));

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
        .with_name(NAME)
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

    println!("============================");
    println!("Requested in {}, {}", req.method(), req.uri().path());
    println!(
        "IPP Printer - printer_uri_supported: {:?}\n",
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

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

// fn test_encoding<T: IppEncode + std::fmt::Debug>(raw: T) {
//     println!("raw: {:?}", raw);
//     let encoded = raw.to_ipp();
//     println!("encoded: {:?}", encoded);
//     let decoded = T::from_ipp(&encoded, 0);
//     println!("decoded: {:?}", decoded);
// }

// fn test() {
//     // i32
//     test_encoding(32 as i32);

//     // String
//     let text_wo_lang = String::from("Text Without Lang");
//     test_encoding(text_wo_lang);

//     // bool
//     test_encoding(true);
//     test_encoding(false);

//     // TextWithLang
//     let text_with_lang = TextWithLang {
//         lang: String::from("en"),
//         text: String::from("Text With Lang"),
//     };
//     test_encoding(text_with_lang);

//     // DateTime
//     let date = Utc::now();
//     test_encoding(date);o
// }
