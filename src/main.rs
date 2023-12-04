use std::fs::read_to_string;
use std::str::Lines;
use day_two::{get_valid_id};

const LAW: [(&str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn main() {
    let doc: String = read_to_string("./day_two/src/input.txt").expect("Failed to read doc");
    let lines: Lines = doc.lines();
    let games: Vec<i32> = lines
        .map(|line| get_valid_id(line, LAW))
        .collect();
    println!("Total of game Id's: {:?}", games.iter().sum::<i32>());
}


