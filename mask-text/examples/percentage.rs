fn main() {
    println!(
        "{}",
        mask_text::Kind::Percentage("text to mask".to_string(), 80, 3, "*".to_string()).mask()
    );
}
