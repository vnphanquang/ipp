mod printer;

use printer::IppPrinter;

#[tokio::main]
async fn main() {
    const PORT: u16 = 6363;
    const NAME: &str = "Rust_IPP_Printer";
    let uri = format!("http://{}:{}/", "localhost", PORT);

    IppPrinter::start(&uri, PORT, NAME).await;
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
