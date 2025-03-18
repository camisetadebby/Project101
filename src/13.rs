// Rust Code for Project 101
use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Something went wrong reading file");
    println!("{}", contents);
}
