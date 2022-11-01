use mask_text::mask;

fn main() {
    println!("{}", mask::Kind::Prefix("text to mask", 3, "*").mask());
}
