#[test]
fn varint_standard_values() -> Result<(), super::Error> {
    use super::VarInt;
    // Create the list of standard values
    let val_0 = VarInt::from_value(0)?;
    let val_1 = VarInt::from_value(1)?;
    let val_largest_num = VarInt::from_value(2147483647)?;
    let val_minus_one = VarInt::from_value(-1)?;
    let val_smallest_num = VarInt::from_value(-2147483648)?;

    // Check that the values are still the same
    assert_eq!(val_0.value(), 0);
    assert_eq!(val_1.value(), 1);
    assert_eq!(val_largest_num.value(), 2147483647);
    assert_eq!(val_minus_one.value(), -1);
    assert_eq!(val_smallest_num.value(), -2147483648);

    // Check that encoding works properly
    assert_eq!(val_0.to_bytes()?, [0x00]);
    assert_eq!(val_1.to_bytes()?, [0x01]);
    assert_eq!(val_largest_num.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0x07]);
    assert_eq!(val_minus_one.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0x0f]);
    assert_eq!(val_smallest_num.to_bytes()?, [0x80, 0x80, 0x80, 0x80, 0x08]);

    // Check that decoding works properly
    assert_eq!(val_0.value(), VarInt::from_bytes(&[0x00])?.0.value());
    assert_eq!(val_1.value(), VarInt::from_bytes(&[0x01])?.0.value());
    assert_eq!(val_largest_num.value(), VarInt::from_bytes(&[0xff, 0xff, 0xff, 0xff, 0x07])?.0.value());
    assert_eq!(val_minus_one.value(), VarInt::from_bytes(&[0xff, 0xff, 0xff, 0xff, 0x0f])?.0.value());
    assert_eq!(val_smallest_num.value(), VarInt::from_bytes(&[0x80, 0x80, 0x80, 0x80, 0x08])?.0.value());
    return Ok(());
}

#[test]
fn varlong_standard_values() -> Result<(), super::Error> {
    use super::VarLong;
    // Create the list of standard values
    let val_0 = VarLong::from_value(0)?;
    let val_1 = VarLong::from_value(1)?;
    let val_largest_num = VarLong::from_value(9223372036854775807)?;
    let val_minus_one = VarLong::from_value(-1)?;
    let val_smallest_num = VarLong::from_value(-9223372036854775808)?;

    // Check that the values are still the same
    assert_eq!(val_0.value(), 0);
    assert_eq!(val_1.value(), 1);
    assert_eq!(val_largest_num.value(), 9223372036854775807);
    assert_eq!(val_minus_one.value(), -1);
    assert_eq!(val_smallest_num.value(), -9223372036854775808);

    // Check that encoding works properly
    assert_eq!(val_0.to_bytes()?, [0x00]);
    assert_eq!(val_1.to_bytes()?, [0x01]);
    assert_eq!(val_largest_num.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);
    assert_eq!(val_minus_one.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);
    assert_eq!(val_smallest_num.to_bytes()?, [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
    return Ok(());
}

#[test]
fn position_standard_values() -> Result<(), super::Error> {
    use super::Position;
    // Create the list of standard values
    let zeroed = Position::from_values(0, 0, 0);
    let max_value = Position::from_values(i32::MAX, i16::MAX, i32::MAX);
    let min_value = Position::from_values(i32::MIN, i16::MIN, i32::MIN);

    // Check that the values are still the same
    assert_eq!(zeroed.get_x(), 0);
    assert_eq!(zeroed.get_y(), 0);
    assert_eq!(zeroed.get_z(), 0);
    assert_eq!(max_value.get_x(), i32::MAX);
    assert_eq!(max_value.get_y(), i16::MAX);
    assert_eq!(max_value.get_z(), i32::MAX);
    assert_eq!(min_value.get_x(), i32::MIN);
    assert_eq!(min_value.get_y(), i16::MIN);
    assert_eq!(min_value.get_z(), i32::MIN);

    // Check that encoding works properly
    assert_eq!(zeroed.to_bytes()?, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(max_value.to_bytes()?, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!(min_value.to_bytes()?, [0x00, 0x00, 0x06, 0x00, 0x00, 0x01, 0x80, 0x0E]);
    return Ok(());
}

#[test]
fn username_api() -> Result<(), super::Error> {
    use super::UUID;
    // Create a UUID from a username
    let uuid = UUID::from_username(String::from("thisjaiden"))?;
    // Test username -> UUID
    assert_eq!(uuid.clone().to_value()?, 0x09773765901b4da1a1243467f482b8b3);
    // Test UUID -> username
    assert_eq!(uuid.to_username()?, String::from("thisjaiden"));
    return Ok(());
}
