#[cfg(test)]
use crate::*;

#[test]
fn test_bytes_to_hex() {
    assert_eq!(bytes_to_hex(&[0xDE, 0xAD, 0xBE, 0xEF]), "deadbeef");
    assert_eq!(bytes_to_hex(&[0x00, 0xFF]), "00ff");
    assert_eq!(bytes_to_hex(&[15, 255, 0, 128]), "0fff0080");
}

#[test]
fn test_hex_to_bytes() {
    assert_eq!(
        hex_to_bytes("deadbeef").unwrap(),
        vec![0xDE, 0xAD, 0xBE, 0xEF]
    );
    assert_eq!(hex_to_bytes("00FF").unwrap(), vec![0x00, 0xFF]);
    assert_eq!(hex_to_bytes("0fff0080").unwrap(), vec![15, 255, 0, 128]);
    assert!(hex_to_bytes("invalid").is_err());
    assert!(hex_to_bytes("0").is_err());
}
