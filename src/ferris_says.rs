use std::io::{stdout, BufWriter};
use ferris_says::say;

pub fn ferris_says() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let writer = BufWriter::new(stdout.lock());
    say(&message, width, writer).unwrap();
}