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
    let max_index = doc_length - line_length - 1;
    let (present_first, present_last) = get_present_range(&needle, &haystack)?;
    match present_first.cmp(&max_index) {
        Ordering::Greater => None,
        _ => Some((present_first + line_length + 1, present_last + line_length + 1)),
    }
}

pub fn has_symbols(target: &str) -> bool {
    target.contains(|c: char| !(c.is_digit(10) || c == '.'))
}

pub fn get_numbers(target: &str) -> Vec<&str> {
    target
        .split('.')
        .filter(|&substring| !substring.is_empty() && !has_symbols(substring))
        .collect()
}