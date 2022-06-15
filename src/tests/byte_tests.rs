use crate::lib::ops::bytes::Byte;

#[test]
fn create_byte() {
    let byte = Byte::from(0u8);

    assert!(!byte.is_signed());
    assert_eq!(byte.get_value(), 0u8);

    let mut signed_byte = Byte::from(0i8);

    assert!(signed_byte.is_signed());
    assert_eq!(signed_byte.get_value(), 0u8);

    signed_byte = Byte::from(-1i8);

    assert!(signed_byte.is_signed());
    assert_eq!(signed_byte.get_value(), 128u8);
}

#[test]
fn add() {
    let mut byte1 = Byte::from(1u8);
    let mut byte2 = Byte::from(2u8);

    assert_eq!((byte1 + 1u8).get_value(), 2u8);
    assert_eq!((byte2 + 1i8).get_value(), 3u8);
    assert_eq!((byte2 + -1i8).get_value(), 1u8);

    byte1 += 1u8;
    assert_eq!(byte1.get_value(), 2u8);

    byte2 += 1i8;
    assert_eq!(byte2.get_value(), 3u8);

    byte2 += -1i8;
    assert_eq!(byte2.get_value(), 2u8);

    byte1 += Byte::from(1u8);
    assert_eq!(byte1.get_value(), 3u8);

    byte2 += Byte::from(1i8);
    assert_eq!(byte2.get_value(), 3u8);

    byte2 += Byte::from(-1i8);
    assert_eq!(byte2.get_value(), 2u8);
}

#[test]
fn to_signed() {
    let mut byte = Byte::from(128u8);

    assert_eq!(byte.to_signed(), -1i8);

    byte = Byte::from(129u8);

    assert_eq!(byte.to_signed(), -2i8);
}
