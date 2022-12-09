# Bytenum

Bytenum is a rust derive macro that creates a try_from<u8> implementation for an enum with unit variants. 

## Usage

Add this to your `Cargo.toml`:

```toml
bytenum = "0.1.3"
```

Example:

```rust
use bytenum::Bytenum;

#[derive(Bytenum, Debug, PartialEq)]
#[bytenum(repr = "u16")] // u8, u16, u32 are supported. default is u8
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn convert_variants() -> Result<(), Box<dyn std::error::Error + 'static>> {
    [Color::Red, Color::Green, Color::Blue]
        .into_iter()
        .enumerate()
        .try_for_each(|(value, color)| {
            assert_eq!(color, Color::try_from(value as u16)?);
            Ok(())
        })
}
```