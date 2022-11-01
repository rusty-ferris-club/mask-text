use mask_text::mask;

fn main() {
    println!(
        "{}",
        mask::Kind::Percentage("text to mask".to_string(), 80, 3, "*".to_string()).mask()
    );
}
