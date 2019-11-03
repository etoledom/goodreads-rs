use goodreads_rs::Goodreads;

fn main() {
    let key = std::env::var("GOODREADS_KEY").unwrap();
    let goodreads = Goodreads::new(key.as_str());

    goodreads.search("Sharp Objects", 1, |result| {
        println!("{:?}", result);
    });
}
