use bytenum::Bytenum;

#[derive(Bytenum, Debug, PartialEq, Copy, Clone)]
#[repr(u16)]
enum Color {
    Red = 0x04,
    Green = 0x15,
    Blue = 0x34,
}

#[test]
fn convert_variants() -> Result<(), Box<dyn std::error::Error + 'static>> {
    assert_eq!(Color::Red, Color::try_from(0x04)?);
    assert_eq!(Color::Green, Color::try_from(0x15)?);
    assert_eq!(Color::Blue, Color::try_from(0x34)?);
    Ok(())
}
