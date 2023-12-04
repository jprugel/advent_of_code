use std::collections::HashMap;

pub fn get_game_number(input: &str) -> i32 {
    input
        .split_whitespace()
        .nth(1)
        .unwrap()
        .trim_matches(':')
        .parse::<i32>()
        .unwrap()
}

pub fn get_groups(input: &str) -> Vec<&str> {
    input.split(": ").nth(1).unwrap().split("; ").collect()
}

pub fn create_map(input: &str) -> HashMap<&str, i32> {
    let tokens: Vec<&str> = input
        .split_whitespace()
        .map(|token| token.trim_matches(','))
        .collect();
    let keys: Vec<&str> = tokens
        .iter()
        .filter(|&&token| token.parse::<i32>().is_err())
        .copied()
        .collect();
    let values: Vec<i32> = tokens
        .iter()
        .filter_map(|&token| token.parse::<i32>().ok())
        .collect();
    keys.into_iter().zip(values.into_iter()).collect()
}

pub fn merge_greatest_map(vec: Vec<HashMap<&str, i32>>) -> HashMap<&str, i32> {
    let result: HashMap<&str, i32> = vec.into_iter().flat_map(|map| map.into_iter()).fold(
        HashMap::new(),
        |mut acc, (key, value)| {
            *acc.entry(key).or_insert(value) = (*acc.get(&key).unwrap_or(&0)).max(value);
            acc
        },
    );
    result
}

pub fn get_valid_id(input: &str, validator: [(&str, i32); 3]) -> i32 {
    let game_number: i32 = get_game_number(input);
    let groups: Vec<&str> = get_groups(input);
    let maps: Vec<HashMap<&str, i32>> = groups.iter().map(|group| create_map(group)).collect();
    let merged: HashMap<&str, i32> = merge_greatest_map(maps);
    let law: HashMap<&str, i32> = validator.into_iter().collect();
    let check: bool = merged.iter().all(|(&key, &value)| value <= law[key]);
    match check {
        true => game_number,
        false => 0,
    }
}

pub fn get_product_minimum(input: HashMap<&str, i32>) -> i32 {
    input.values().product()
}

pub fn get_minimums_from_maps(input: Vec<HashMap<&str, i32>>) -> HashMap<&str, i32> {
    let mut result: HashMap<&str, i32> = HashMap::new();
    input.iter().for_each(|map| {
        map.iter().for_each(|(&key, &value)| {
            if value >= *result.get(&key).unwrap_or(&0) {
                result.insert(key, value);
            }
        })
    });
    result
}

pub fn get_minimum_product(input: &str) -> i32 {
    let groups: Vec<&str> = get_groups(input);
    let maps: Vec<HashMap<&str, i32>> = groups.iter().map(|group| create_map(group)).collect();
    let minimums: HashMap<&str, i32> = get_minimums_from_maps(maps);
    let product: i32 = get_product_minimum(minimums);
    product
}
