# Bytenum

Bytenum is a rust derive macro that creates a try_from<u8> implementation for an enum with unit variants. 

## Usage

Add this to your `Cargo.toml`:

```toml
bytenum = "0.1.0"
```

And then add then add a derive `Bytenum` attribute on an enum
that has less than `256` variants (the amount that can be represented by a `u8`).

```rust
#[derive(Bytenum, Debug, PartialEq)]
enum TestEnum {
    Red,
    Green,
    Blue,
}

fn check_enum() -> Result<(), Box<dyn std::error::Error + 'static>> {
  [Color::Red, Color::Green, Color::Blue]
        .into_iter()
        .enumerate()
        .try_for_each(|(index, color)| {
            assert_eq!(color, Color::try_from(index as u8)?);
            Ok(())
        })
}

```