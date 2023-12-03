use std::fs;
use advent_1::elf;

fn main() {
    let filename = "input.txt";
    println!("Calculating input from {filename}");
    let content = fs::read_to_string(filename).unwrap_or_default();
    let num = elf::parse_content(content);
    println!("Result: {num}");
}
