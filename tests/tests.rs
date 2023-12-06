#[cfg(test)]
pub mod tests {
    use day_three::get_sum_valid_parts;

    const DOC: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_one() {
        assert_eq!(get_sum_valid_parts(DOC), 4361);
    }
}
