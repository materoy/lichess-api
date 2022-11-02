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

pub(crate) fn read_token() -> Option<String> {
    match std::fs::read_to_string(".env") {
        Ok(data) => {
            let key_val = data.split(" ");
            match key_val.last() {
                Some(token) => Some(strip_trailing_newline(token).to_string()),
                None => None,
            }
        }
        Err(err) => {
            eprintln!("Error while reading file .env: {}", err);
            None
        }
    }
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_read() {
        let token = read_token();
        assert!(token.is_some());
        assert!(token.unwrap().starts_with("lip"));
    }
}
