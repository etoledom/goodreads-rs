use goodreads_rs::make_get_request;

fn main() {
    let key = std::env::var("GOODREADS_KEY").unwrap();
    make_get_request(key, Box::new(|result| {
        println!("THIS WORKS!");
        println!("Results: {:?}", result);
    }));
}