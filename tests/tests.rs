#[cfg(test)]
pub mod tests {
    use day_two::{get_minimum_product, get_valid_id};
    const LAW: [(&str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];
    const TESTS: [&'static str; 5] = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn part_one() {
        assert_eq!(get_valid_id(TESTS[0], LAW), 1);
        assert_eq!(get_valid_id(TESTS[1], LAW), 2);
        assert_eq!(get_valid_id(TESTS[2], LAW), 0);
        assert_eq!(get_valid_id(TESTS[3], LAW), 0);
        assert_eq!(get_valid_id(TESTS[4], LAW), 5);
    }
    #[test]
    fn part_two() {
        assert_eq!(get_minimum_product(TESTS[0]), 48);
        assert_eq!(get_minimum_product(TESTS[1]), 12);
        assert_eq!(get_minimum_product(TESTS[2]), 1560);
        assert_eq!(get_minimum_product(TESTS[3]), 630);
        assert_eq!(get_minimum_product(TESTS[4]), 36);
    }
}
