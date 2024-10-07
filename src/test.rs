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

#[test]
fn test_bytes_to_string() {
    // Test valid UTF-8 sequences
    assert_eq!(bytes_to_string(&[72, 101, 108, 108, 111]).unwrap(), "Hello");
    assert_eq!(bytes_to_string("🦀 Rust".as_bytes()).unwrap(), "🦀 Rust");

    // Test empty string
    assert_eq!(bytes_to_string(&[]).unwrap(), "");

    // Test invalid UTF-8 sequences
    assert!(bytes_to_string(&[0xFF, 0xFF]).is_err());
}

#[test]
fn test_string_to_bytes() {
    // Test ASCII string
    assert_eq!(string_to_bytes("Hello"), vec![72, 101, 108, 108, 111]);

    // Test empty string
    assert_eq!(string_to_bytes(""), vec![]);

    // Test Unicode string
    assert_eq!(string_to_bytes("🦀 Rust"), "🦀 Rust".as_bytes().to_vec());
}

#[test]
fn test_string_to_hex() {
    // Test ASCII string
    assert_eq!(string_to_hex("Hello"), "48656c6c6f");

    // Test empty string
    assert_eq!(string_to_hex(""), "");

    // Test Unicode string
    assert_eq!(
        string_to_hex("🦀"),
        "f09fa680" // UTF-8 representation of the crab emoji
    );
}

#[test]
fn test_hex_to_string() {
    // Test ASCII string
    assert_eq!(hex_to_string("48656c6c6f").unwrap(), "Hello");

    // Test empty string
    assert_eq!(hex_to_string("").unwrap(), "");

    // Test Unicode string
    assert_eq!(
        hex_to_string("f09fa680").unwrap(), // UTF-8 representation of the crab emoji
        "🦀"
    );

    // Test invalid hex string
    assert!(hex_to_string("invalid").is_err());

    // Test invalid UTF-8 sequence
    assert!(hex_to_string("ff").is_err());
}

#[test]
fn test_roundtrip_conversions() {
    // Test string -> hex -> string
    let original = "Hello, 🦀 Rust!";
    let hex = string_to_hex(original);
    assert_eq!(hex_to_string(&hex).unwrap(), original);

    // Test string -> bytes -> string
    let bytes = string_to_bytes(original);
    assert_eq!(bytes_to_string(&bytes).unwrap(), original);

    // Test bytes -> hex -> bytes
    let original_bytes = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let hex = bytes_to_hex(&original_bytes);
    assert_eq!(hex_to_bytes(&hex).unwrap(), original_bytes);
}
