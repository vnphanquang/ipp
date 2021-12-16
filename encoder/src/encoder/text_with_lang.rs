use super::IppEncode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
