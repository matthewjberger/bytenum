use bytenum_derive::Bytenum;

#[derive(Debug, Bytenum)]
enum TestEnum {
    A,
    B,
    C,
    D,
}

#[test]
fn test_bytenum() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("{:#?}", TestEnum::try_from(0_u8)?);
    Ok(())
}
