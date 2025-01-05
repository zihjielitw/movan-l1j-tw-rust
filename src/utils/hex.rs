
pub const fn hex_to_decimal(hex_string: &str) -> Option<i64> {
    match i64::from_str_radix(hex_string, 16) {
        Ok(parsed_int) => Some(parsed_int),
        Err(_) => None,
    }
}