use curl::easy::Easy;
use serde_xml_rs::from_reader;
use serde::Deserialize;
use std::thread;
use std::io::BufReader;
use xml::{EventReader, reader::XmlEvent};

#[derive(Deserialize, Debug)]
struct GoodreadsResponse {
    search: Search,
}

#[derive(Deserialize, Debug)]
struct Search {
    results: Results,
}

#[derive(Deserialize, Debug)]
struct Results {
    work: Vec<Work>,
}

#[derive(Deserialize, Debug)]
struct Work {
    id: XMLInt,
    average_rating: XMLFloat,
    best_book: RemoteBook,
}

#[derive(Deserialize, Debug)]
struct RemoteBook {
    title: XMLString,
    id: XMLInt,
    author: RemoteAuthor,
    image_url: XMLString,
    small_image_url: XMLString,
}

#[derive(Deserialize, Debug)]
struct RemoteAuthor {
    id: XMLInt,
    name: XMLString,
}

#[derive(Deserialize, Debug)]
struct XMLString {
    #[serde(rename = "$value")]
    body: String,
}

#[derive(Deserialize, Debug)]
struct XMLInt {
    #[serde(rename = "$value")]
    body: i32,
}

#[derive(Deserialize, Debug)]
struct XMLFloat {
    #[serde(rename = "$value")]
    body: f32,
}

pub fn make_get_request<F>(key: String, f: Box<F>) where F: Fn() + 'static, F: std::marker::Sync, F: std::marker::Send {
    let mut easy = Easy::new();
    let mut final_data = Vec::new();
    let url = format!("https://www.goodreads.com/search/index.xml?q=potter&key={}", key);
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
    println!("{:#?}", response);

}

#[cfg(test)]
mod tests {
    use crate::make_get_request;

    #[test]
    fn it_works() {

    }
}
