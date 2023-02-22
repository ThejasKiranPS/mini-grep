# MiniGrep
Minimal command-line utility to search for the occurence of a string of characters in a file.

This was developed in the process of learning Rust.

## Building
```cargo build --release```

build will be saved in `./target/release`

## Usage
`minigrep {query} {filename}`

## Example
`./target/release/minigrep frog poem.txt`