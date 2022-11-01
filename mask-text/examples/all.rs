fn main() {
    println!(
        "{}",
        mask_text::Kind::All("text to mask".to_string(), "*".to_string()).mask()
    );
}
