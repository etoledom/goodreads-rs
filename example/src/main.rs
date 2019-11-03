use goodreads_rs::Goodreads;

fn main() {
    let key = std::env::var("GOODREADS_KEY").unwrap();
    let goodreads = Goodreads::new(key.as_str());
    goodreads.search("Bernardo O'higgins", 1, |result| {
        println!("THIS WORKS!");
        println!("{:?}", result);
    });
}