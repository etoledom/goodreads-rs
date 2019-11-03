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

    fn encode<'a>(string: &'a str) -> String {
        return utf8_percent_encode(string, URL_SET).collect();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn encode_test() {
            let to_encode = "O'Higgins Dzień";
            let expected = "O%27Higgins%20Dzie%C5%84";

            let encoded = encode(to_encode);

            assert_eq!(encoded, expected);
        }

        #[test]
        fn with_path_test() {
            let path = "search/index.xml?jurassic=ninja space";
            let expected = "https://www.goodreads.com/search/index.xml?jurassic=ninja%20space";

            let full_url = with(path);

            assert_eq!(full_url, expected);
        }
    }
}
