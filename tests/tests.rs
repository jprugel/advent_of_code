#[cfg(test)]
pub mod tests {
    use day_three::{expand_range, get_future_range, get_numbers, get_past_range, get_present_range, has_symbols, remove_symbols};
    const TEST_PATH: &'static str = "../day_three/text/test.txt";
    pub fn get_doc() -> String {
        std::fs::read_to_string(TEST_PATH).expect("Failed to read document")
    }

    #[test]
    pub fn test_get_present_range() {
        let binding = get_doc();
        let doc: &str = binding.as_str();
        assert_eq!(get_present_range("114", &doc), Some((5, 7)));
        assert_eq!(get_present_range("631", &doc), None);
    }

    #[test]
    pub fn test_get_past_range() {
        let binding = get_doc();
        let doc: &str = binding.as_str();
        assert_eq!(get_past_range("114", doc), None);
        assert_eq!(get_past_range("631", doc), None);
        assert_eq!(get_past_range("35", doc), Some((13, 14)));
    }

    #[test]
    pub fn test_get_future_range() {
        let binding = get_doc();
        let doc: &str = binding.as_str();
        assert_eq!(get_future_range("114", doc), Some((16, 18)));
        assert_eq!(get_future_range("631", doc), None);
        assert_eq!(get_future_range("598", doc), None);
    }

    #[test]
    pub fn test_has_symbols() {
        let binding = get_doc();
        let doc: &str = binding.as_str();
        assert_eq!(has_symbols(doc.lines().nth(0).unwrap()), false);
        assert_eq!(has_symbols(doc.lines().nth(1).unwrap()), true);
    }

    #[test]
    pub fn test_get_numbers() {
        let binding = get_doc();
        let doc: &str = binding.as_str();
        assert_eq!(get_numbers(doc.lines().nth(0).unwrap().to_string()), ["467", "114"]);
        assert_eq!(get_numbers(doc.lines().nth(1).unwrap().to_string()), Vec::<&str>::new());
        assert_eq!(get_numbers(doc.lines().nth(2).unwrap().to_string()), ["35", "633"]);
    }

    #[test]
    pub fn test_expand_range() {
        assert_eq!(expand_range((3, 5), 9), (2, 6));
        assert_eq!(expand_range((0, 5), 9), (0, 6));
        assert_eq!(expand_range((3, 9), 9), (2, 9));
    }

    #[test]
    pub fn test_remove_symbols() {
        let binding = get_doc();
        let doc: &str = binding.as_str();
        assert_eq!(remove_symbols(doc.lines().nth(0).unwrap()), "467114");
    }
}
