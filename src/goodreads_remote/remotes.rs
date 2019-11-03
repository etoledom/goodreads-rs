use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GoodreadsResponse {
    pub search: RemoteSearch,
}

#[derive(Deserialize, Debug)]
pub struct RemoteSearch {
    pub results: Results,
    #[serde(rename = "results-start")]
    pub results_start: XMLInt,
    #[serde(rename = "results-end")]
    pub results_end: XMLInt,
    #[serde(rename = "total-results")]
    pub total_results: XMLInt,
    pub source: XMLString,
}

#[derive(Deserialize, Debug)]
pub struct Results {
    pub work: Vec<Work>,
}

#[derive(Deserialize, Debug)]
pub struct Work {
    pub id: XMLInt,
    pub average_rating: XMLFloat,
    pub best_book: RemoteBook,
}

#[derive(Deserialize, Debug)]
pub struct RemoteBook {
    pub title: XMLString,
    pub id: XMLInt,
    pub author: RemoteAuthor,
    pub image_url: XMLString,
    pub small_image_url: XMLString,
}

#[derive(Deserialize, Debug)]
pub struct RemoteAuthor {
    pub id: XMLInt,
    pub name: XMLString,
}

#[derive(Deserialize, Debug)]
pub struct XMLString {
    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct XMLInt {
    #[serde(rename = "$value")]
    pub body: i32,
}

#[derive(Deserialize, Debug)]
pub struct XMLFloat {
    #[serde(rename = "$value")]
    pub body: f32,
}