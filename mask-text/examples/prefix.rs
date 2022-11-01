use mask_text::mask;

fn main() {
    println!(
        "{}",
        mask::Kind::Prefix("text to mask".to_string(), 3, "*".to_string()).mask()
    );
}
