/// Converts a comma-separated string into a vector of strings.
///
/// This function splits a comma-separated string into individual elements,
/// trims whitespace from each element, and filters out empty strings.
/// Each resulting string is converted into an owned String type.
///
/// # Arguments
///
/// * `comma_separated_values` - A string slice containing comma-separated values.
///
/// # Returns
///
/// A Vec<String> containing the processed strings from the input.
///
/// # Example
///
/// ```rust
/// let input = "hello, world, , rust  ";
/// let result = byteutils::string::to_array(input);
/// assert_eq!(result, vec!["hello", "world", "rust"]);
/// ```
pub fn to_array(comma_separated_values: &str) -> Vec<String> {
    comma_separated_values
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

/// Escapes special characters in a SQL string by replacing backslashes with double backslashes
/// and single quotes with double single quotes.
///
/// # Arguments
///
/// * `input` - The string to escape
///
/// # Returns
///
/// A String with SQL special characters properly escaped
///
/// # Example
///
/// ```rust
/// let input = "O'Connor\\Path";
/// let escaped = byteutils::string::escape_sql(input);
/// assert_eq!(escaped, "O''Connor\\\\Path");
/// ```
pub fn escape_sql(input: &str) -> String {
    input.replace('\\', "\\\\").replace('\'', "''")
}
