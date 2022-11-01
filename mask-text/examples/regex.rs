use mask_text::mask;
use regex::Regex;

fn main() {
    let re = Regex::new("([a-z].*) (mask) ([a-z].*)").unwrap();
    println!(
        "{}",
        mask::Kind::Regex("text to mask on group", re, 2, "*").mask()
    );
}
