#[cfg(test)]
pub mod tests {
    use day_three::{get_future_range, get_numbers, get_past_range, get_present_range, has_symbols};

    #[test]
    pub fn test_get_present_range() {
        let doc: String = std::fs::read_to_string("../day_three/src/input.txt").expect("Failed to read document");
        assert_eq!(get_present_range("114", doc.as_str()), Some((5, 7)));
        assert_eq!(get_present_range("631", doc.as_str()), None);
    }

    #[test]
    pub fn test_get_past_range() {
        let doc: String = std::fs::read_to_string("../day_three/src/input.txt").expect("Failed to read document");
        assert_eq!(get_past_range("114", doc.as_str()), None);
        assert_eq!(get_past_range("631", doc.as_str()), None);
        assert_eq!(get_past_range("35", doc.as_str()), Some((13, 14)));
    }

    #[test]
    pub fn test_get_future_range() {
        let doc: String = std::fs::read_to_string("../day_three/src/input.txt").expect("Failed to read document");
        assert_eq!(get_future_range("114", doc.as_str()), Some((16, 18)));
        assert_eq!(get_future_range("631", doc.as_str()), None);
        assert_eq!(get_future_range("598", doc.as_str()), None);
    }

    #[test]
    pub fn test_has_symbols() {
        let doc: String = std::fs::read_to_string("../day_three/src/input.txt").expect("Failed to read document");
        assert_eq!(has_symbols(doc.lines().nth(0).unwrap()), false);
        assert_eq!(has_symbols(doc.lines().nth(1).unwrap()), true);
    }

    #[test]
    pub fn test_get_numbers() {
        let doc: String = std::fs::read_to_string("../day_three/src/input.txt").expect("Failed to read document");
        assert_eq!(get_numbers(doc.lines().nth(0).unwrap()), ["467", "114"]);
        assert_eq!(get_numbers(doc.lines().nth(1).unwrap()), Vec::<&str>::new());
        assert_eq!(get_numbers(doc.lines().nth(2).unwrap()), ["35", "633"]);
    }
}
