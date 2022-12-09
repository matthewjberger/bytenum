use bytenum::Bytenum;

#[derive(Bytenum, Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn test_bytenum() -> Result<(), Box<dyn std::error::Error + 'static>> {
    [Color::Red, Color::Green, Color::Blue]
        .into_iter()
        .enumerate()
        .try_for_each(|(value, color)| {
            assert_eq!(color, Color::try_from(value as u8)?);
            Ok(())
        })
}
