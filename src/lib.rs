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
        println!("{}", url);
        return self.api_fetch.fetch(url.as_str());
    }
}

#[cfg(test)]
mod tests {
    use crate::api_fetch::ApiFetch;
    use crate::Goodreads;

    struct TestFetch<'a> {
        response: &'a [u8],
    }

    impl<'a> ApiFetch for TestFetch<'a> {
        fn fetch(&self, _url: &str) -> Vec<u8> {
            let mut response = Vec::new();
            response.extend_from_slice(self.response);
            return response;
        }
    }

    #[test]
    fn goodreads_fetch_test() {
        let api_fetch = TestFetch{ response: &[0] };
        let goodreads = Goodreads{
            key: "",
            api_fetch: Box::new(api_fetch),
        };

        let response = goodreads.fetch("");
        assert_eq!(response, [0]);
    }

    #[test]
    fn test_search() {
        let response_xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <GoodreadsResponse><search><results-start>1</results-start><results-end>20</results-end><total-results>15373</total-results><source>Goodreads</source><query-time-seconds>0.31</query-time-seconds>
        <results><work><id type="integer">4640799</id><books_count type="integer">710</books_count><ratings_count type="integer">6182429</ratings_count><text_reviews_count type="integer">98348</text_reviews_count><original_publication_year type="integer">1997</original_publication_year><original_publication_month type="integer">6</original_publication_month><original_publication_day type="integer">26</original_publication_day><average_rating>4.47</average_rating>
        <best_book type="Book"><id type="integer">3</id><title>Harry Potter and the Sorcerer's Stone (Harry Potter, #1)</title><author><id type="integer">1077326</id><name>J.K. Rowling</name></author><image_url>https://i.gr-assets.com/images/S/compressed.photo.goodreads.com/books/1474154022l/3._SX98_.jpg</image_url><small_image_url>https://i.gr-assets.com/images/S/compressed.photo.goodreads.com/books/1474154022l/3._SY75_.jpg</small_image_url></best_book></work></results></search></GoodreadsResponse>
        "#;
        let api_fetch = TestFetch{ response: response_xml.as_bytes() };
        let goodreads = Goodreads{
            key: "",
            api_fetch: Box::new(api_fetch),
        };
        goodreads.search("Harry Potter", 1, |search_result| {
           assert_eq!(search_result.books[0].title, "Harry Potter and the Sorcerer's Stone (Harry Potter, #1)");
        });
    }
}
