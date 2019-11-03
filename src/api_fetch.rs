use curl::easy::Easy;

pub trait ApiFetch {
    fn fetch(&self, url: &str) -> Vec<u8>;
}

pub struct CurlApiFetch;
impl ApiFetch for CurlApiFetch {
    fn fetch(&self, url: &str) -> Vec<u8> {
        let mut easy = Easy::new();
        let mut final_data = Vec::new();

        easy.url(url).unwrap();
        {
            let mut transfer = easy.transfer();
            transfer.write_function( |data| {
                final_data.extend_from_slice(data);
                return Ok(data.len());
            }).unwrap();
            transfer.perform().unwrap();
        }
        return final_data;
    }
}
