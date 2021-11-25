use astro_dnssd::DNSServiceBuilder;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

use chrono::Utc;

use ipp_encoder::{IppEncode, Operation, TextWithLang};

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
    let address = SocketAddr::from(([127, 0, 0, 1], 6363));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(http_handler)) });

    let server = Server::bind(&address).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    let dns_service = DNSServiceBuilder::new("_ipp._tcp", 6363)
        .with_name("Rust IPP Printer")
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

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut res = Response::new(Body::empty());

    println!("Requested in {}, {}", req.method(), req.uri().path());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *res.body_mut() = Body::from("IPP Server");
        }
        (&Method::POST, "/") => {
            let bytes = hyper::body::to_bytes(req.into_body())
                .await
                .unwrap()
                .to_vec();

            let (_, operation) = Operation::from_ipp(&bytes, 0);

            println!("bytes {}", operation.to_json());
            println!("operation-id: {:?}", operation.operation_id());
        }
        _ => {
            *res.status_mut() = StatusCode::NOT_FOUND;
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
