use std::collections::HashMap;

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

/// Replaces placeholders in a string with specified replacement values.
///
/// This function takes a string containing placeholders in the format `{{placeholder}}` and
/// replaces them with the specified replacement value. It uses regex pattern matching to
/// find and replace all occurrences of the placeholder.
///
/// # Arguments
///
/// * `input` - A string slice that contains the text with placeholders
/// * `placeholder` - A string slice representing the placeholder name (without braces)
/// * `replacement` - A string slice containing the value to replace the placeholder with
///
/// # Returns
///
/// Returns a new String with all occurrences of the placeholder replaced with the replacement value.
///
/// # Examples
///
/// ```rust
/// let template = "Hello {{name}}! Welcome to {{place}}.";
/// let result = byteutils::string::replace_placeholder(template, "name", "John");
/// assert_eq!(result, "Hello John! Welcome to {{place}}.");
/// ```
///
/// # Panics
///
/// This function will panic if the regex pattern creation fails, which should only happen
/// if the placeholder contains characters that make an invalid regex pattern.
pub fn replace_placeholder(input: &str, placeholder: &str, replacement: &str) -> String {
    // Create a regex pattern that matches {{placeholder}} exactly
    let pattern = format!(r"\{{\{{{}}}\}}", regex::escape(placeholder));

    // Compile the regex pattern - using unwrap is safe here because we control the pattern format
    let re = Regex::new(&pattern).expect("Failed to create regex pattern");

    // Replace all occurrences and return the result
    re.replace_all(input, replacement).into_owned()
}

/// Replaces multiple placeholders in a string using a map of placeholder-value pairs.
///
/// This function processes a template string containing multiple placeholders in the format
/// `{{placeholder}}` and replaces each one with its corresponding value from the provided map.
/// If a placeholder in the template doesn't have a corresponding entry in the map, it remains
/// unchanged in the output string.
///
/// # Arguments
///
/// * `template` - A string slice containing the template text with placeholders
/// * `replacements` - A HashMap where keys are placeholder names and values are their replacements
///
/// # Returns
///
/// Returns a new String with all matched placeholders replaced with their corresponding values.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// let mut replacements = HashMap::new();
/// replacements.insert("name".to_string(), "John".to_string());
/// replacements.insert("age".to_string(), "30".to_string());
///
/// let template = "Hello {{name}}! You are {{age}} years old.";
/// let result = byteutils::string::replace_multiple_placeholders(template, &replacements);
/// assert_eq!(result, "Hello John! You are 30 years old.");
/// ```
pub fn replace_multiple_placeholders(
    template: &str,
    replacements: &HashMap<String, String>,
) -> String {
    let mut result = template.to_string();

    // Iterate through each placeholder-value pair and apply replacements
    for (placeholder, value) in replacements {
        result = replace_placeholder(&result, placeholder, value);
    }

    result
}
