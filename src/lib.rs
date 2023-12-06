// Purpose: Parts are not valid if they do not have an adjacent symbol that is not a "."
pub fn get_sum_valid_parts(doc: &str) -> i32 {
    let number_slices = get_number_slices(doc);
    let ranges: Vec<(usize, usize)> = number_slices
        .iter()
        .map(|number| get_ranges_from_needle(doc, number))
        .collect();
    let long_ranges: Vec<(usize, usize)> = ranges.into_iter().map(expand_range).collect();
    let filtered_by_symbols: Vec<(usize, usize)> = long_ranges
        .into_iter()
        .filter(|&range| {
            (doc[range.0..range.1])
                .chars()
                .any(|c| !c.is_ascii_digit() && c != '.')
        })
        .collect();
    dbg!(filtered_by_symbols);
    doc.len() as i32
}

pub fn get_number_slices(input: &str) -> Vec<String> {
    let doc_parts: Vec<&str> = input.split('.').collect();
    let filtered_symbols: Vec<String> = doc_parts
        .iter()
        .map(|&part| part.chars().filter(|&c| c.is_ascii_digit()).collect())
        .collect();

    filtered_symbols
        .into_iter()
        .filter(|part| part.parse::<i32>().is_ok())
        .collect()
}

pub fn expand_range(input_range: (usize, usize)) -> (usize, usize) {
    let mut output_range: (usize, usize) = (0, 0);
    match input_range.0 {
        0 => output_range.0 = 0,
        _ => output_range.0 = input_range.0 - 1,
    };
    match input_range.1 % 12 {
        0 => output_range.1 = input_range.1,
        _ => output_range.1 = input_range.1 + 1,
    };
    output_range
}

pub fn get_ranges_from_needle(haystack: &str, needle: &str) -> (usize, usize) {
    if let Some(start) = haystack.find(needle) {
        (start, start + needle.len())
    } else {
        panic!("Number not found: {}", needle);
    }
}
