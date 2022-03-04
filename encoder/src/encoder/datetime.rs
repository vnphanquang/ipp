use super::IppEncode;
use chrono::{DateTime, Datelike, Offset, TimeZone, Timelike, Utc};

impl IppEncode for DateTime<Utc> {
    fn ipp_bytes() -> usize {
        11
    }

    fn from_ipp(bytes: &[u8], offset: usize) -> (usize, Self) {
        let start = offset + Self::ipp_value_length_bytes();

        let slice_offset = start + 8;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let direction = u8::from_be_bytes(slice) as char;

        let slice_offset = start + 9;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let hour_from_utc = u8::from_be_bytes(slice);

        let slice_offset = start + 10;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let minutes_from_utc = u8::from_be_bytes(slice);

        let mut drift = (hour_from_utc * 60 - minutes_from_utc) as i8;
        if direction == '-' {
            drift *= -1;
        }

        let slice_offset = start;
        let slice: [u8; 2] = bytes[slice_offset..slice_offset + 2].try_into().unwrap();
        let year = u16::from_be_bytes(slice);

        let slice_offset = start + 2;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let month = u8::from_be_bytes(slice);

        let slice_offset = start + 3;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let day = u8::from_be_bytes(slice);

        let slice_offset = start + 4;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let hour = u8::from_be_bytes(slice);

        let slice_offset = start + 5;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let minutes = (i8::from_be_bytes(slice) + drift) as u8;

        let slice_offset = start + 6;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let seconds = u8::from_be_bytes(slice);

        let slice_offset = start + 7;
        let slice: [u8; 1] = bytes[slice_offset..slice_offset + 1].try_into().unwrap();
        let deciseconds = u8::from_be_bytes(slice);

        let value = Utc
            .ymd(year as i32, month as u32, day as u32)
            .and_hms_micro(
                hour as u32,
                minutes as u32,
                seconds as u32,
                deciseconds as u32 * 100,
            );

        (value.ipp_len(), value)
    }

    fn to_ipp(&self) -> Vec<u8> {
        let value_length = self.ipp_len() as u16;
        let value_length_bytes = value_length.to_be_bytes().to_vec();

        let year = self.year() as u16;
        let year_bytes = year.to_be_bytes().to_vec();

        let month = self.month() as u8;
        let month_bytes = month.to_be_bytes().to_vec();

        let day = self.day() as u8;
        let day_bytes = day.to_be_bytes().to_vec();

        let hour = self.hour() as u8;
        let hour_bytes = hour.to_be_bytes().to_vec();

        let minutes = self.minute() as u8;
        let minutes_bytes = minutes.to_be_bytes().to_vec();

        let seconds = self.second() as u8;
        let seconds_bytes = seconds.to_be_bytes().to_vec();

        let deciseconds = 0_u8;
        let deciseconds_bytes = deciseconds.to_be_bytes().to_vec();

        let local_minus_utc = self.timezone().fix().local_minus_utc() / 60;

        let mut direction = '+';
        if local_minus_utc < 0 {
            direction = '-';
        }
        let direction_bytes = (direction as u8).to_be_bytes().to_vec();

        let hour_from_utc = (local_minus_utc / 60) as u8;
        let hour_from_utc_bytes = hour_from_utc.to_be_bytes().to_vec();

        let minutes_from_utc = (local_minus_utc % 60) as u8;
        let minutes_from_utc_bytes = minutes_from_utc.to_be_bytes().to_vec();

        [
            value_length_bytes,
            year_bytes,
            month_bytes,
            day_bytes,
            hour_bytes,
            minutes_bytes,
            deciseconds_bytes,
            seconds_bytes,
            direction_bytes,
            hour_from_utc_bytes,
            minutes_from_utc_bytes,
        ]
        .concat()
    }
}
