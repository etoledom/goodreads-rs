use super::goodreads_remote::*;
use super::data_structures::*;

impl RemoteSearch {
    pub fn to_search_result(&self) -> SearchResult {
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
    pub fn to_book(&self) -> Book {
        return Book {
            id: self.id.body.clone(),
            title: self.best_book.title.body.clone(),
            author: Author { id: self.best_book.author.id.body.clone(), name: self.best_book.author.name.body.clone() },
            cover: CoverSet { small: self.best_book.small_image_url.body.clone(), normal: self.best_book.image_url.body.clone() },
            rating: self.average_rating.body.clone(),
        };
    }
}
