:warning:
> This crate is deprecated, please use [enum2repr](https://github.com/matthewjberger/enum2repr) instead :rocket:

# Bytenum

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/bytenum-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/bytenum)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bytenum.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bytenum)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bytenum-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/bytenum)

Bytenum is a rust derive macro that creates a `try_from<T>` implementation for an enum with only unit variants. 
All types supported by `#[repr(T)]` are supported by bytenum.

## Usage

Add this to your `Cargo.toml`:

```toml
bytenum = "0.1.9"
```

Example:

```rust
use bytenum::Bytenum;

#[derive(Bytenum, Debug, PartialEq, Copy, Clone)]
#[repr(u16)]
enum Color {
    Red = 0x04,
    Green = 0x15,
    Blue = 0x34,
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    assert_eq!(Color::Red, Color::try_from(0x04)?);
    assert_eq!(Color::Green, Color::try_from(0x15)?);
    assert_eq!(Color::Blue, Color::try_from(0x34)?);
    Ok(())
}
```
