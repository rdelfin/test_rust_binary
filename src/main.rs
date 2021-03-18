pub static TEST_TEXT: &str = include_str!("include/test.txt");

fn main() {
    println!("Test text:\n{}", TEST_TEXT);
}
