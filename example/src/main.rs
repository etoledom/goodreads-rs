use goodreads_rs::make_get_request;

fn main() {
    make_get_request(Box::new(|| {
        println!("THIS WORKS!")
    }));
    println!("Hello world");
}