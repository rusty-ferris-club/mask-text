//! Mask implementation
//!
//!
//! # Example:
//! ```
#![doc = include_str!("../examples/percentage.rs")]
//! ```
use regex::Regex;
pub enum Kind {
    Percentage(String, u8, usize, String),
    Regex(String, Regex, usize, String),
    Prefix(String, usize, String),
    All(String, String),
}

impl Kind {
    #[must_use]
    pub fn mask(self) -> String {
        match self {
            Self::Percentage(mask_str, percentage, min_chars, mask_char) => {
                with_percentage(mask_str, percentage, min_chars, mask_char)
            }
            Self::Regex(mask_str, re, group, mask_char) => {
                with_regex(mask_str, re, group, mask_char)
            }
            Self::Prefix(mask_str, until, mask_char) => with_prefix(mask_str, until, mask_char),
            Self::All(mask_str, mask_char) => all(mask_str, mask_char),
        }
    }
}

/// Mask percentage text.
///
/// # Example
///
/// ```rust
/// use mask_text::mask;
/// let masked_text = mask::Kind::Percentage("text to mask".to_string(), 80, 3, "*".to_string()).mask();
/// ```
///
/// Arguments:
/// * `text` - Text to mask.
/// * `percentage` - Percentage number. 100% masking all the text.
/// * `min_chars` - The minimum number of the text to apply percentage logic. if
///   the text length is lower from the given text all the text will mask.
/// * `mask_char` - Mask char.
#[must_use]
fn with_percentage(text: String, percentage: u8, min_chars: usize, mask_char: String) -> String {
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    let mask_from = if text.len() > min_chars {
        text.len() - ((f32::from(percentage) * text.len() as f32) / 100.0).floor() as usize
    } else {
        // mask all text
        0
    };
    mask(text, mask_from, mask_char)
}

/// Mask string by regex. if the mask group is not found, we go to the safe side
/// and mask all the text
///
/// # Example
///
/// ```rust
/// use mask_text::mask;
/// use regex::Regex;
/// let re = Regex::new("([a-z].*) (mask) ([a-z].*)").unwrap();
/// let masked_text = mask::Kind::Regex("text to mask on group".to_string(), re, 2, "*".to_string()).mask();
/// ```
///
/// Arguments:
/// * `text` - Text to mask.
/// * `re` - Regex to capture.
/// * `group` - Mask regex group.
/// * `mask_char` - Mask char.
#[must_use]
fn with_regex(text: String, re: Regex, group: usize, mask_char: String) -> String {
    let cap_text = re
        .captures(&text)
        .and_then(|f| f.get(group))
        .map_or(text.as_str(), |m| m.as_str());

    text.replace(cap_text, &mask(cap_text.to_string(), 0, mask_char))
}

/// Mask string from prefix.
///
/// # Example
///
/// ```rust
/// use mask_text::mask;
/// let masked_text = mask::Kind::Prefix("text to mask".to_string(), 3, "*".to_string()).mask();
/// ```
///
/// Arguments:
/// * `text` - Text to mask.
/// * `until` - Mask chats from the start until the given number.
/// * `mask_char` - Mask char.
#[must_use]
fn with_prefix(text: String, until: usize, mask_char: String) -> String {
    let until = if until >= text.len() { 0 } else { until };
    mask(text, until, mask_char)
}

/// Mask all string
///
/// # Example
///
/// ```rust
/// use mask_text::mask;
/// let masked_text = mask::Kind::All("text to mask".to_string(), "*".to_string()).mask();
/// ```
///
/// Arguments:
/// * `text` - Text to mask.
/// * `mask_char` - Mask char.
#[must_use]
fn all(text: String, mask_char: String) -> String {
    mask(text, 0, mask_char)
}

fn mask(str: String, from: usize, mask_char: String) -> String {
    str.chars()
        .enumerate()
        .map(|(i, c)| {
            if c as u8 == 0x0d {
                "\r".to_string()
            } else if c as u8 == 0x0a {
                "\n".to_string()
            } else if i >= from {
                mask_char.to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod test_mask {

    use insta::assert_debug_snapshot;
    use rstest::rstest;

    use super::*;

    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_prepend_module_to_snapshot(false);
            settings.set_snapshot_suffix(format!($($expr,)*));
            let _guard = settings.bind_to_scope();
        }
    }

    #[rstest]
    #[case(20, 3, "*")]
    #[case(80, 3, "*")]
    #[case(100, 3, "*")]
    #[case(80, 20, "*")]
    #[case(80, 3, ".")]
    fn cat_mask_with_percentage(
        #[case] percentage: u8,
        #[case] min_chars: usize,
        #[case] mask_char: String,
    ) {
        let text = "text to mask".to_string();

        set_snapshot_suffix!(
            "[{}]-[{}]-[{}]",
            percentage,
            min_chars,
            mask_char.replace('*', "asterisk")
        );

        assert_debug_snapshot!(Kind::Percentage(text, percentage, min_chars, mask_char,).mask());
    }

    #[rstest]
    #[case("([a-z].*) (mask) ([a-z].*)", 2, "*")]
    #[case("([a-z].*) (mask) ([a-z].*)", 3, "*")]
    #[case("([a-z].*) (mask) ([a-z].*)", 1, ".")]
    fn cat_mask_with_regex(#[case] re: &str, #[case] group: usize, #[case] mask_char: String) {
        let text = "text to mask on group".to_string();

        set_snapshot_suffix!(
            "[{}]-[{}]-[{}]",
            re.replace('*', "asterisk"),
            group,
            mask_char.replace('*', "asterisk")
        );

        let re = Regex::new(re).unwrap();
        assert_debug_snapshot!(Kind::Regex(text, re, group, mask_char).mask());
    }

    #[rstest]
    #[case(3, "*")]
    #[case(200, "*")]
    fn cat_mask_with_prefix(#[case] until: usize, #[case] mask_char: String) {
        let text = "text to mask".to_string();

        set_snapshot_suffix!("[{}]-[{}]", until, mask_char.replace('*', "asterisk"));

        assert_debug_snapshot!(Kind::Prefix(text, until, mask_char).mask());
    }
}
