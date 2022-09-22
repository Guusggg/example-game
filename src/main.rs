#[macro_use]
extern crate lazy_static;

mod client;

fn main() {
    println!("Hello world!");

    unsafe {
        client::init();

        for _ in 0..5 {
            println!("Calling loop!");
            client::step();
        }
    }
}