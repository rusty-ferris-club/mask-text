# mask-text
[![Crates.io](https://img.shields.io/crates/v/mask-text?style=flat-square)](https://crates.io/crates/mask-text)
[![CI](https://github.com/rusty-ferris-club/mask-text/actions/workflows/ci.yaml/badge.svg)](https://github.com/rusty-ferris-club/mask-text/actions/workflows/ci.yaml)


This is a library to mask text with multiple masking options

## Usage 
```toml
[dependencies]
mask-text = { version = "0.1.0" }
```

## Mask Options
* Percentage
* Regex
* Prefix
* All

## Examples
```rs
use mask_text::mask;

fn main() {
    // masking 80% of the given text. minimum masking chars should bigger then 3.
    let percentage_result = mask::Kind::Percentage("text to mask", 80, 3, "*").mask();

    // masking text by regex group.
    let re = Regex::new("([a-z].*) (mask) ([a-z].*)").unwrap();
    let regex_result = mask::Kind::Regex("text to mask on group", re, 2, "*").mask();
    
    // masking 3 first chars from the string.
    let prefix_result = mask::Kind::Prefix("text to mask", 3, "*").mask();

    // masking all chars.
    let all_result = mask::Kind::All("text to mask", "*").mask()
}
```

[All the examples here](./mask-text/examples/README.md)

## Thanks
To all [Contributors](https://github.com/rusty-ferris-club/mask-text/graphs/contributors) - you make this happen, thanks!

## Copyright
Copyright (c) 2022 [@kaplanelad](https://github.com/kaplanelad). See [LICENSE](LICENSE.txt) for further details.
