#[derive(Debug)]
pub struct  Author {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct CoverSet {
    pub small: String,
    pub normal: String,
}

#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: Author,
    pub cover: CoverSet,
    pub rating: f32,
}

#[derive(Debug)]
pub struct SearchResult {
    pub start: i32,
    pub end: i32,
    pub total: i32,
    pub source: String,
    pub books: Vec<Book>,
}
