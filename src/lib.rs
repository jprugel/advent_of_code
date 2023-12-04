use std::collections::HashMap;

const PATTERNS: [(&'static str, i32); 19] = [
    ("0", 0),
    ("1", 1),
    ("one", 1),
    ("2", 2),
    ("two", 2),
    ("3", 3),
    ("three", 3),
    ("4", 4),
    ("four", 4),
    ("5", 5),
    ("five", 5),
    ("6", 6),
    ("six", 6),
    ("7", 7),
    ("seven", 7),
    ("8", 8),
    ("eight", 8),
    ("9", 9),
    ("nine", 9),
];

pub fn convert(line: &str) -> i32 {
    let mut test: HashMap<usize, i32> = HashMap::default();
    line.char_indices().for_each(|(index, _)| {
        let pattern_map: HashMap<&str, i32> = PATTERNS.into_iter().collect();
        pattern_map.keys().for_each(|substring| {
            if (line[index..]).starts_with(substring) {
                test.insert(index, pattern_map[substring]);
            }
        });
    });
    let first = test.iter().min_by_key(|key| *key.0).unwrap().1;
    let last = test.iter().max_by_key(|key| *key.0).unwrap().1;
    let vec = vec![first, last];
    vec.iter().fold(0, |acc, x| acc * 10 + **x)
}
