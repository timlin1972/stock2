pub fn convert_date_to_fugle_format(date: &str) -> String {
    // Convert date from "YYYYMMDD" to "YYYY-MM-DD"
    format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
}

fn format_commas(value: u64) -> String {
    let s = value.to_string();
    let bytes = s.as_bytes();
    let mut result = String::new();
    let len = bytes.len();
    for (i, &b) in bytes.iter().enumerate() {
        result.push(b as char);
        if (len - i - 1).is_multiple_of(3) && i != len - 1 {
            result.push(',');
        }
    }
    result
}

pub fn str_volume(volume: u64) -> String {
    format_commas((volume as f64 / 1000.0) as u64)
}
