mod remotes;
pub use remotes::*;

const BASE_URL: &str = "https://www.goodreads.com/";

#[allow(non_snake_case)]
pub mod URL {
    use super::BASE_URL;
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC, AsciiSet};

    const URL_SET: &AsciiSet  = &NON_ALPHANUMERIC
        .remove(b'/')
        .remove(b'.')
        .remove(b'=')
        .remove(b'?')
        .remove(b'&');

    pub fn with(path: &str) -> String {
        let encoded_path = encode(path);
        return format!("{}{}", BASE_URL, encoded_path);
    }

    fn encode(string: &str) -> String {
        let encoded_path: String = utf8_percent_encode(string, URL_SET).collect();
        return encoded_path;
    }
}
