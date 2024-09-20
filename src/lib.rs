#[rustfmt::skip]
#[cfg(test)]
#[path = "./test.rs"]
mod test;

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
///
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
