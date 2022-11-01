use mask_text::mask;

fn main() {
    println!(
        "{}",
        mask::Kind::All("text to mask".to_string(), "*".to_string()).mask()
    );
}
