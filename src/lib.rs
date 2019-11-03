use serde_xml_rs::from_reader;

mod api_fetch;
use api_fetch::*;
mod goodreads_remote;
use goodreads_remote::*;
mod data_structures;
use data_structures::*;
mod utils;

pub struct Goodreads<'a> {
    key: &'a str,
    api_fetch: Box<dyn ApiFetch>,
}

impl<'a> Goodreads<'a> {
    pub fn new(key: &str) -> Goodreads {
        let api_fetch = Box::new(CurlApiFetch{});
        return Goodreads{ key, api_fetch }
    }

    pub fn search<F>(&self, query: &str, page: i32, callback: F) where F: Fn(SearchResult) {
        let path = format!("search/index.xml?q={}&page={}&key={}", query, page, self.key);
        let response_data = self.fetch(path.as_str());
        let response: GoodreadsResponse = from_reader(response_data.as_slice()).unwrap();
        let result: SearchResult = response.search.to_search_result();

        callback(result);
    }

    fn fetch<'b>(&self, path: &'b str) -> Vec<u8> {
        let url = URL::with(path);
        return self.api_fetch.fetch(url.as_str());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
