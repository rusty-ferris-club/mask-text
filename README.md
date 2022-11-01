# mask-text
[![Crates.io](https://img.shields.io/crates/v/mask-text?style=flat-square)](https://crates.io/crates/mask-text)
[![CI](https://github.com/rusty-ferris-club/mask-text/actions/workflows/ci.yaml/badge.svg)](https://github.com/rusty-ferris-club/mask-text/actions/workflows/ci.yaml)


This is a library to mask text with multiple masking options

## Usage 
```toml
[dependencies]
mask-text = { version = "0.1.1" }
```

## Mask Options
* Percentage
* Regex
* Prefix
* All

## Examples
```rs

fn main() {
    // masking 80% of the given text. minimum masking chars should bigger then 3.
    let percentage_result = mask_text::Kind::Percentage("text to mask".to_string(), 80, 3, "*".to_string()).mask();

    // masking text by regex group.
    let re = Regex::new("([a-z].*) (mask) ([a-z].*)").unwrap();
    let regex_result = mask_text::Kind::Regex("text to mask on group".to_string(), re, 2, "*".to_string()).mask();
    
    // masking 3 first chars from the string.
    let prefix_result = mask_text::Kind::Prefix("text to mask".to_string(), 3, "*".to_string()).mask();

    // masking all chars.
    let all_result = mask_text::Kind::All("text to mask".to_string(), "*".to_string()).mask()
}
```

[All the examples here](./mask-text/examples/README.md)

## Thanks
To all [Contributors](https://github.com/rusty-ferris-club/mask-text/graphs/contributors) - you make this happen, thanks!

## Copyright
Copyright (c) 2022 [@kaplanelad](https://github.com/kaplanelad). See [LICENSE](LICENSE.txt) for further details.
