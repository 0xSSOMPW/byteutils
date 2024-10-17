use regex::Regex;

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

/// Encloses a string in single quotes for SQL string literals.
///
/// # Arguments
///
/// * `name` - The string to enclose in quotes
///
/// # Returns
///
/// A String wrapped in single quotes
///
/// # Example
///
/// ```rust
/// let name = "table_name";
/// let quoted = byteutils::string::enclose_quotes(name);
/// assert_eq!(quoted, "'table_name'");
/// ```
pub fn enclose_quotes(name: &str) -> String {
    format!("'{}'", name)
}

/// Checks if a given word is present in the source string as a whole word, ignoring case.
///
/// This function creates a case-insensitive regular expression pattern that matches the given word
/// as a whole word (bounded by word boundaries) and checks if this pattern
/// is found in the source string.
///
/// # Arguments
///
/// * `src` - A string slice that holds the text to search in.
/// * `word` - A string slice that holds the word to search for.
///
/// # Returns
///
/// Returns `true` if the word is found as a whole word in the source string (ignoring case),
/// `false` otherwise.
///
/// # Examples
///
/// ```rust
/// let source = "Hello world! Welcome to Rust.";
/// assert!(byteutils::string::is_contain_word(source, "world"));
/// assert!(!byteutils::string::is_contain_word(source, "Rust!"));
/// ```
pub fn is_contain_word(src: &str, word: &str) -> bool {
    let pattern = format!(r"(?i)\b{}\b", regex::escape(word));
    let re = Regex::new(&pattern).unwrap();
    re.is_match(src)
}

/// Checks if any of the given words are present in the source string as whole words.
///
/// This function iterates through the given list of words and checks if any
/// of them are present in the source string as whole words using the
/// `is_contain_word` function.
///
/// # Arguments
///
/// * `src` - A string slice that holds the text to search in.
/// * `words` - A slice of String values representing the words to search for.
///
/// # Returns
///
/// Returns `true` if any of the words in the list are found as whole words
/// in the source string, `false` otherwise.
///
/// # Examples
///
/// ```rust
/// let source = "The quick brown fox jumps over the lazy dog.";
/// let words = vec!["quick".to_string(), "slow".to_string(), "fox".to_string()];
/// assert!(byteutils::string::has_contain_words(source, &words));
///
/// let no_match_words = vec!["cat".to_string(), "elephant".to_string()];
/// assert!(!byteutils::string::has_contain_words(source, &no_match_words));
/// ```
pub fn has_contain_words(src: &str, words: &[String]) -> bool {
    words.iter().any(|word| is_contain_word(src, word))
}
