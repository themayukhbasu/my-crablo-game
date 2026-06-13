use std::io::{stdout, BufWriter};
use ferris_says::say;

// hello world

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn hello() {
    println!("Hello World from Rust");

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}