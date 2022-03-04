use super::IppEncode;

impl IppEncode for i32 {
    fn ipp_bytes() -> usize {
        4
    }
    fn from_ipp(bytes: &[u8], offset: usize) -> (usize, Self) {
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
    fn from_ipp(bytes: &[u8], offset: usize) -> (usize, Self) {
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

    fn from_ipp(bytes: &[u8], offset: usize) -> (usize, Self) {
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
