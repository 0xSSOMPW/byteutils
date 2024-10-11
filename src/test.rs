use crate::string::*;
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

#[test]
fn test_to_array_basic() {
    let result = to_array("a,b,c");
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_to_array_with_spaces() {
    let result = to_array(" a , b , c ");
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_to_array_with_empty_elements() {
    let result = to_array("a,,b,,c");
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_to_array_with_empty_string() {
    let result = to_array("");
    assert_eq!(result, Vec::<String>::new());
}

#[test]
fn test_to_array_with_only_commas() {
    let result = to_array(",,,");
    assert_eq!(result, Vec::<String>::new());
}

#[test]
fn test_to_array_with_whitespace_only_elements() {
    let result = to_array("a, , ,b,  ,c");
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_to_array_with_mixed_whitespace() {
    let result = to_array("a,\tb\n,  c  ");
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_to_array_single_element() {
    let result = to_array("hello");
    assert_eq!(result, vec!["hello"]);
}

#[test]
fn test_to_array_preserves_inner_spaces() {
    let result = to_array("hello world,foo bar,baz");
    assert_eq!(result, vec!["hello world", "foo bar", "baz"]);
}

#[test]
fn test_to_array_with_special_characters() {
    let result = to_array("hello!,@world,#rust$");
    assert_eq!(result, vec!["hello!", "@world", "#rust$"]);
}
