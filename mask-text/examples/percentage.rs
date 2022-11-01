use mask_text::mask;

fn main() {
    println!(
        "{}",
        mask::Kind::Percentage("text to mask", 80, 3, "*").mask()
    );
}
