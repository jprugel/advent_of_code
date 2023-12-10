fn main() {
    let doc: String = std::fs::read_to_string("./day_three/text/test.txt").expect("Failed to read document");
    let doc_without_newline = doc.replace('\n', "");
    dbg!(doc_without_newline.chars().nth(13).unwrap());
}
