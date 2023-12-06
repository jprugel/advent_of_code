use day_three::get_sum_valid_parts;

fn main() {
    let doc = std::fs::read_to_string("./day_three/src/input.txt").expect("Failed to read doc");
    let total = get_sum_valid_parts(&doc);
    println!("Total: {}", total);
}
