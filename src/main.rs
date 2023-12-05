use day_two::{get_minimum_product, get_valid_id};
use std::fs::read_to_string;
use std::str::Lines;

const LAW: [(&str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn main() {
    let doc: String = read_to_string("./day_two/src/input.txt").expect("Failed to read doc");
    let lines: Lines = doc.lines();
    let games: Vec<i32> = lines.map(|line| get_valid_id(line, LAW)).collect();
    println!("Total of game Id's: {:?}", games.iter().sum::<i32>());
    let lines: Lines = doc.lines();
    let products: Vec<i32> = lines.map(get_minimum_product).collect();
    println!("Total of game products: {:?}", products.iter().sum::<i32>());
}
