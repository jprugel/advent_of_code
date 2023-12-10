use std::cmp::Ordering;

pub fn get_present_range(needle: &str, haystack: &str) -> Option<(usize, usize)> {
    let first = haystack.find(needle)?;
    let last = first + needle.len() - 1;
    Some((first, last))
}

pub fn get_past_range(needle: &str, haystack: &str) -> Option<(usize, usize)> {
    let line_length = haystack.lines().nth(0)?.len();
    let (present_first, present_last) = get_present_range(&needle, &haystack)?;
    match present_first.cmp(&line_length) {
        Ordering::Less => None,
        _ => Some((present_first - line_length - 1, present_last - line_length - 1)),
    }
}

pub fn get_future_range(needle: &str, haystack: &str) -> Option<(usize, usize)> {
    let line_length = haystack.lines().nth(0)?.len();
    let doc_length = haystack.len();
    let max_index = doc_length - line_length;
    let (present_first, present_last) = get_present_range(&needle, &haystack)?;
    match present_first.cmp(&max_index) {
        Ordering::Greater => None,
        _ => Some((present_first + line_length + 1, present_last + line_length + 1)),
    }
}

pub fn has_symbols(target: &str) -> bool {
    target.contains(|c: char| !(c.is_digit(10) || c == '.'))
}

pub fn get_numbers(target: String) -> Vec<String> {
    let removed_dot: Vec<&str> = target
        .split('.')
        .collect();
    let removed_symbols: Vec<String> = removed_dot
        .iter()
        .map(|&substring| remove_symbols(substring))
        .filter(|substring| !substring.is_empty())
        .collect();
    removed_symbols
}

pub fn remove_symbols(substring: &str) -> String {
    substring
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect()
}

pub fn is_valid(context: &str, past: Option<(usize, usize)>, present: Option<(usize, usize)>, future: Option<(usize, usize)>) -> bool {
    let line_length = context.lines().nth(0).unwrap().len();
    let doc_length = context.len();
    let max_index = doc_length - line_length;
    let past: (usize, usize) = expand_range(past.unwrap_or((0, 0)), max_index);
    let present: (usize, usize) = expand_range(present.unwrap_or((0, 0)), max_index);
    let future: (usize, usize) = expand_range(future.unwrap_or((0, 0)), max_index);
    println!("Past {:?} || Present {:?} || Future {:?}", &context[past.0..past.1], &context[present.0..present.1], &context[future.0..future.1]);
    has_symbols(&context[past.0..past.1]) || has_symbols(&context[present.0..present.1]) || has_symbols(&context[future.0..future.1])
}

pub fn expand_range(range: (usize, usize), max: usize) -> (usize, usize) {
    let mut new_range: (usize, usize) = (0, 0);
    match range.0.cmp(&0usize) {
        Ordering::Greater => new_range.0 = range.0 - 1,
        _ => new_range.0 = range.0,
    };
    match range.1.cmp(&max) {
        Ordering::Less => new_range.1 = range.1 + 1,
        _ => new_range.1 = range.1,
    };
    new_range
}

