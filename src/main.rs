extern crate rustylox;

fn main() {
    if let Err(e) = rustylox::run() {
        println!("Rustylox error: {}", e);
    }
}
