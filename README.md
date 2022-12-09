# Bytenum

Bytenum is a rust derive macro that creates a `try_from<T>` implementation for an enum with only unit variants. 
`T` must be an unsigned numeric type such as `u8`, `u16`, or `u32`.

## Usage

Add this to your `Cargo.toml`:

```toml
bytenum = "0.1.6"
```

Example:

```rust
use bytenum::Bytenum;

#[derive(Bytenum, Debug, PartialEq, Copy, Clone)]
#[bytenum(repr = "u16")] // u8, u16, u32 are supported. default is u8 if this line is omitted
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