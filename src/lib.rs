use curl::easy::Easy;
use serde_xml_rs::from_reader;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

mod goodreads_remote;
use goodreads_remote::*;
mod data_structures;
use data_structures::*;

impl RemoteSearch {
    fn to_search_result(&self) -> SearchResult {
        let books: Vec<Book> = self.results.work.iter().map(|work| work.to_book() ).collect();
        return SearchResult {
            start: self.results_start.body,
            end: self.results_end.body,
            total: self.total_results.body,
            source: self.source.body.clone(),
            books: books,
        }
    }
}

impl Work {
    fn to_book(&self) -> Book {
        return Book {
            id: self.id.body.clone(),
            title: self.best_book.title.body.clone(),
            author: Author { id: self.best_book.author.id.body.clone(), name: self.best_book.author.name.body.clone() },
            cover: CoverSet { small: self.best_book.small_image_url.body.clone(), normal: self.best_book.image_url.body.clone() },
            rating: self.average_rating.body.clone(),
        };
    }
}

pub struct Goodreads<'a> {
    key: &'a str
}

impl<'a> Goodreads<'a> {
    pub fn new(key: &str) -> Goodreads {
        return Goodreads{ key }
    }

    pub fn search<F>(&self, query: &str, page: i32, callback: F) where F: Fn(SearchResult) {
        let mut easy = Easy::new();
        let mut final_data = Vec::new();
        let encoded_query: String = utf8_percent_encode(query, NON_ALPHANUMERIC).collect();
        let url = format!("https://www.goodreads.com/search/index.xml?q={}&page={}&key={}", encoded_query.as_str(), page, self.key);

        easy.url(url.as_str()).unwrap();
        {
            let mut transfer = easy.transfer();
            transfer.write_function( |data| {
                final_data.extend_from_slice(data);
                return Ok(data.len());
            }).unwrap();
            transfer.perform().unwrap();
        }

        let response: GoodreadsResponse = from_reader(final_data.as_slice()).unwrap();
        let result: SearchResult = response.search.to_search_result();

        callback(result);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
