use mask_text::mask;
use regex::Regex;

fn main() {
    let re = Regex::new("([a-z].*) (mask) ([a-z].*)").unwrap();
    println!(
        "{}",
        mask::Kind::Regex("text to mask on group".to_string(), re, 2, "*".to_string()).mask()
    );
}
