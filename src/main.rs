use day_one::convert;
use std::str::Lines;

pub fn main() {
    let doc = std::fs::read_to_string("./day_one/src/input.txt").expect("Failed to read doc");
    let lines: Lines = doc.lines();
    let numbers: Vec<i32> = lines.map(convert).collect();
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}
