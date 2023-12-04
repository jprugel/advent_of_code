#[cfg(test)]
pub mod tests {
    use day_one::convert;

    #[test]
    fn day_one() {
        const TEST_ONE: &str = "two1nine";
        const TEST_TWO: &str = "eightwothree";
        const TEST_THREE: &str = "abcone2threexyz";
        const TEST_FOUR: &str = "xtwone3four";
        const TEST_FIVE: &str = "4nineeightseven2";
        const TEST_SIX: &str = "zoneight234";
        const TEST_SEVEN: &str = "7pqrstsixteen";
        assert_eq!(convert(TEST_ONE), 29);
        assert_eq!(convert(TEST_TWO), 83);
        assert_eq!(convert(TEST_THREE), 13);
        assert_eq!(convert(TEST_FOUR), 24);
        assert_eq!(convert(TEST_FIVE), 42);
        assert_eq!(convert(TEST_SIX), 14);
        assert_eq!(convert(TEST_SEVEN), 76);
    }
}
