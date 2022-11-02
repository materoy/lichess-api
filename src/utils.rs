use std::str::Utf8Error;

pub fn json_deserialize<T: for<'a> serde::Deserialize<'a>>(
    json: &str,
) -> Option<Result<T, serde_json::Error>> {
    // Filters for nextline
    if json == "\n" {
        return None;
    }
    Some(serde_json::from_str::<T>(json))
}

pub(crate) fn string_from_bytes(bytes: &[u8]) -> Result<&str, Utf8Error> {
    std::str::from_utf8(bytes)
}
