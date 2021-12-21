//!
//! # ipp_encoder
//!
//! IPP encoder & decoder. This crate include two primary modules:
//!
//! - [`spec`](spec/index.html): RFC specification type mapping
//! - [`encoder`](encoder/index.html): core implementation for encoding & decoding IPP operation
//!
//! ## Examples
//!
//! See [ipp/server](https://github.com/vnphanquang/ipp/blob/main/server/src/main.rs) for full IPP server example
//!
//! ```rust
//! use ipp_encoder::encoder::Operation;
//!
//! let request: Vec<u8> = Vec::new();
//!
//! // ... get raw bytes from ipp server
//! // request = ...
//!
//! let (_, request) = Operation::from(&request, 0);
//!
//! println!("Request: {}", request.to_json()); // operation can be serialized
//!
//! // from spec same byte can be operation_id (request) or status_code (response)
//! println!"OperationID: {}", request.operation_id().unwrap() as i32);
//!
//! for (_, attribute_group) in request.attribute_groups {
//!     for (_, attribute) in attribute_group.attributes {
//!         // do something
//!     }
//! }
//!
//! // request.data contain trailing bytes (for example: postscript file)
//!
//! // later ...
//!
//! let mut response = Operation {
//! version: IppVersion { major: 1, minor: 1 },
//! request_id: request.request_id,
//! operation_id_or_status_code: IppStatusCode::SuccessfulOk as u16,
//! attribute_groups: HashMap::new(),
//! data: Vec::new(),
//! };
//!
//! println!("Response: {}", response.to_json()) // operation can be deserialized
//!
//! // response.to_ipp() for sending back response with IPP server
//! ```
//!
//!

pub mod encoder;
pub mod spec;
