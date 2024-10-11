#[rustfmt::skip]
#[cfg(test)]
#[path = "./test.rs"]
mod test;

pub mod string;

/// Converts a byte slice to its hexadecimal string representation.
///
/// This function takes a slice of bytes and returns a String containing the
/// hexadecimal representation of those bytes. Each byte is converted to a
/// two-character hexadecimal string.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to convert to hexadecimal.
///
/// # Returns
///
/// A String containing the hexadecimal representation of the input bytes.
///
/// # Example
/// ```rust
/// let bytes = &[15, 255, 0, 128];
/// let hex_string = byteutils::bytes_to_hex(bytes);
/// assert_eq!(byteutils::bytes_to_hex(bytes), "0fff0080");
/// ````
///
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

/// Converts a hexadecimal string to its byte representation.
///
/// This function takes a string slice containing a hexadecimal representation
/// and returns a Vec<u8> containing the corresponding bytes. The input string
/// must have an even number of characters and contain only valid hexadecimal
/// digits (0-9, a-f, A-F).
///
/// # Arguments
///
/// * `hex` - A string slice containing the hexadecimal representation to convert.
///
/// # Returns
///
/// A Result containing either:
/// - Ok(Vec<u8>): The byte representation of the input hexadecimal string.
/// - Err(String): An error message if the input is invalid.
///
/// # Example
/// ```rust
/// let bytes = byteutils::hex_to_bytes("0fff0080").unwrap();
/// assert_eq!(byteutils::hex_to_bytes("0fff0080").unwrap(), vec![15, 255, 0, 128]);
/// ````
///
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even number of characters".to_string());
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for chunk in hex.as_bytes().chunks(2) {
        let byte = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16)
            .map_err(|e| format!("Invalid hex string: {}", e))?;
        bytes.push(byte);
    }
    Ok(bytes)
}

/// Converts a byte slice to a UTF-8 string.
///
/// This function takes a slice of bytes and attempts to convert it to a UTF-8 string.
/// If the bytes do not represent valid UTF-8 data, an error is returned.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to convert to a string.
///
/// # Returns
///
/// A Result containing either:
/// - Ok(String): The UTF-8 string representation of the input bytes.
/// - Err(String): An error message if the bytes are not valid UTF-8.
///
/// # Example
/// ```rust
/// let bytes = "Hello".as_bytes();
/// let string = byteutils::bytes_to_string(bytes).unwrap();
/// assert_eq!(string, "Hello");
/// ```
///
pub fn bytes_to_string(bytes: &[u8]) -> Result<String, String> {
    String::from_utf8(bytes.to_vec()).map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
}

/// Converts a string to its byte representation.
///
/// This function takes a string slice and returns a vector of bytes
/// containing its UTF-8 representation.
///
/// # Arguments
///
/// * `s` - A string slice to convert to bytes.
///
/// # Returns
///
/// A Vec<u8> containing the UTF-8 byte representation of the input string.
///
/// # Example
/// ```rust
/// let string = "Hello";
/// let bytes = byteutils::string_to_bytes(string);
/// assert_eq!(bytes, vec![72, 101, 108, 108, 111]);
/// ```
///
pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// Converts a string to its hexadecimal representation.
///
/// This function takes a string slice and returns a hexadecimal string
/// representing the UTF-8 bytes of the input string.
///
/// # Arguments
///
/// * `s` - A string slice to convert to hexadecimal.
///
/// # Returns
///
/// A String containing the hexadecimal representation of the input string's UTF-8 bytes.
///
/// # Example
/// ```rust
/// let string = "Hello";
/// let hex = byteutils::string_to_hex(string);
/// assert_eq!(hex, "48656c6c6f");
/// ```
///
pub fn string_to_hex(s: &str) -> String {
    bytes_to_hex(s.as_bytes())
}

/// Converts a hexadecimal string to a UTF-8 string.
///
/// This function takes a hexadecimal string and attempts to convert it to a UTF-8 string.
/// The input hex string must have an even number of characters and contain only valid
/// hexadecimal digits. The resulting bytes must form a valid UTF-8 sequence.
///
/// # Arguments
///
/// * `hex` - A string slice containing the hexadecimal representation to convert.
///
/// # Returns
///
/// A Result containing either:
/// - Ok(String): The UTF-8 string representation of the input hexadecimal.
/// - Err(String): An error message if the input is invalid or the bytes are not valid UTF-8.
///
/// # Example
/// ```rust
/// let hex = "48656c6c6f";
/// let string = byteutils::hex_to_string(hex).unwrap();
/// assert_eq!(string, "Hello");
/// ```
///
pub fn hex_to_string(hex: &str) -> Result<String, String> {
    let bytes = hex_to_bytes(hex)?;
    bytes_to_string(&bytes)
}
