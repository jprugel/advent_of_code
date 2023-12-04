use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Lines;

const LAW: [(&str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn main() {
    let doc: String = read_to_string("./day_two/src/input.txt").expect("Failed to read doc");
    let lines: Lines = doc.lines();
    // We need to figure out the game number, the max number of red, green and blue in each line
    let games: Vec<i32> = lines
        .map(|line| {
            let game_number: i32 = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim_matches(':')
                .parse::<i32>()
                .unwrap();
            let groups: Vec<&str> = line.split(": ").nth(1).unwrap().split("; ").collect();
            let mut merger: HashMap<&str, i32> = HashMap::default();
            groups.iter().for_each(|group| {
                let zipped: HashMap<&str, i32> = get_pairs(group);
                zipped.into_iter().for_each(|(color, count)| {
                    let law: HashMap<&str, i32> = LAW.into_iter().collect();
                    if count > law[color] {
                        merger.insert(color, count);
                    }
                });
            });
            match merger.len() {
                0 => game_number,
                _ => 0,
            }
        })
        .collect();
    println!("Total of game Id's: {:?}", games.iter().sum::<i32>());
}

fn get_pairs(input: &str) -> HashMap<&str, i32> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let color: Vec<&str> = parts
        .into_iter()
        .filter(|&part| part.parse::<i32>().is_err())
        .map(|part| part.trim_matches(','))
        .collect();
    let parts: Vec<&str> = input.split_whitespace().collect();
    let count: Vec<i32> = parts
        .into_iter()
        .filter_map(|part| part.parse::<i32>().ok())
        .collect();
    color.into_iter().zip(count.into_iter()).collect()
}
