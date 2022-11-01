fn main() {
    println!(
        "{}",
        mask_text::Kind::Prefix("text to mask".to_string(), 3, "*".to_string()).mask()
    );
}
