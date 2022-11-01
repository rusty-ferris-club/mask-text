use mask_text::mask;

fn main() {
    println!("{}", mask::Kind::All("text to mask", "*").mask());
}
