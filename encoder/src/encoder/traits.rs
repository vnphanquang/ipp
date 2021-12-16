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
