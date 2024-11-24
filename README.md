# byteutils

# byteutils - A Rust Utility Library for Common Byte and String and Vec Operations

`byteutils` is a small Rust library that provides a collection of frequently used utility functions for working with bytes, strings, and vectors. It includes common tasks such as converting between strings and byte arrays, manipulating strings, and performing common operations on vectors.

## Features

- **Byte Manipulation**: Convert between byte arrays and hex strings, string and byte conversions.
- **String Operations**: String splitting, escaping SQL characters, enclosing in quotes, and more.
- **Vector Utilities**: Operations for deduplication, filtering, reversing, splitting, and more.

## Installation

Add `byteutils` to your `Cargo.toml`:

```toml
[dependencies]
byteutils = "0.1.0"
```

## Example Usage

### Byte Utilities

```rust
// Convert bytes to hex string
let bytes = &[15, 255, 0, 128];
let hex_string = byteutils::bytes_to_hex(bytes);
assert_eq!(byteutils::bytes_to_hex(bytes), "0fff0080");

// Convert hex string to bytes
let bytes = byteutils::hex_to_bytes("0fff0080").unwrap();
assert_eq!(byteutils::hex_to_bytes("0fff0080").unwrap(), vec![15, 255, 0, 128]);

// Convert bytes to string
let bytes = "Hello".as_bytes();
let string = byteutils::bytes_to_string(bytes).unwrap();
assert_eq!(string, "Hello");

// Convert string to bytes
let string = "Hello";
let bytes = byteutils::string_to_bytes(string);
assert_eq!(bytes, vec![72, 101, 108, 108, 111]);

// Convert string to hex
let string = "Hello";
let hex = byteutils::string_to_hex(string);
assert_eq!(hex, "48656c6c6f");

// Convert hex to string
let hex = "48656c6c6f";
let string = byteutils::hex_to_string(hex).unwrap();
assert_eq!(string, "Hello");
```

### String Utilities

```rust
// Convert a comma-separated string to a vector of words
let input = "hello, world, , rust  ";
let result = byteutils::string::to_array(input);
assert_eq!(result, vec!["hello", "world", "rust"]);

// Escape SQL characters in a string
let input = "O'Connor\\Path";
let escaped = byteutils::string::escape_sql(input);
assert_eq!(escaped, "O''Connor\\\\Path");

// Enclose a string in single quotes
let name = "table_name";
let quoted = byteutils::string::enclose_quotes(name);
assert_eq!(quoted, "'table_name'");

// Check if a word exists in a string
let source = "Hello world! Welcome to Rust.";
assert!(byteutils::string::is_contain_word(source, "world"));
assert!(!byteutils::string::is_contain_word(source, "Rust!"));

// Check if multiple words exist in a string
let source = "The quick brown fox jumps over the lazy dog.";
let words = vec!["quick".to_string(), "slow".to_string(), "fox".to_string()];
assert!(byteutils::string::has_contain_words(source, &words));

let no_match_words = vec!["cat".to_string(), "elephant".to_string()];
assert!(!byteutils::string::has_contain_words(source, &no_match_words));

// Replace placeholders in a string
let template = "Hello {{name}}! Welcome to {{place}}.";
let result = byteutils::string::replace_placeholder(template, "name", "John");
assert_eq!(result, "Hello John! Welcome to {{place}}.");

// Replace multiple placeholders using a HashMap
use std::collections::HashMap;
let mut replacements = HashMap::new();
replacements.insert("name".to_string(), "John".to_string());
replacements.insert("age".to_string(), "30".to_string());

let template = "Hello {{name}}! You are {{age}} years old.";
let result = byteutils::string::replace_multiple_placeholders(template, &replacements);
assert_eq!(result, "Hello John! You are 30 years old.");
```

### Vector Utilities

```rust
// Deduplicate a vector
let mut numbers = vec![1, 2, 3, 2, 4, 1, 5];
byteutils::vec::dedup(&mut numbers);
assert_eq!(numbers, vec![1, 2, 3, 4, 5]);

// Retain elements based on a condition
let mut numbers = vec![1, 2, 3, 4, 5, 6];
byteutils::vec::retain_if(&mut numbers, |&x| x % 2 == 0);
assert_eq!(numbers, vec![2, 4, 6]);

// Reverse a vector in place
let mut vec = vec![1, 2, 3, 4, 5];
byteutils::vec::reverse_in_place(&mut vec);
assert_eq!(vec, vec![5, 4, 3, 2, 1]);

// Split a vector at a given index
let mut vec = vec![1, 2, 3, 4, 5];
let (left, right) = byteutils::vec::split_at_vec(&mut vec, 3);
assert_eq!(left, vec![1, 2, 3]);
assert_eq!(right, vec![4, 5]);

// Get unique elements from a vector
let numbers = vec![1, 2, 2, 3, 1, 4];
let unique = byteutils::vec::get_unique(&numbers);
assert_eq!(unique, vec![1, 2, 3, 4]);

// Get unique elements from a vector of strings
let words = vec!["apple", "banana", "apple", "cherry"];
let unique = byteutils::vec::get_unique(&words);
assert_eq!(unique, vec!["apple", "banana", "cherry"]);

// Handle empty vectors
let empty: Vec<i32> = vec![];
let unique = byteutils::vec::get_unique(&empty);
assert_eq!(unique, Vec::<i32>::new());
```

## Available Functions

### byteutils

- **bytes_to_hex**: Converts a byte slice into a hex string.
- **hex_to_bytes**: Converts a hex string into a byte vector.
- **bytes_to_string**: Converts a byte slice into a UTF-8 string.
- **string_to_bytes**: Converts a string into a byte vector.
- **string_to_hex**: Converts a string into a hex string.
- **hex_to_string**: Converts a hex string into a UTF-8 string.

### byteutils::string

- **to_array**: Splits a string by commas and trims whitespace.
- **escape_sql**: Escapes special characters in a string for SQL queries.
- **enclose_quotes**: Encloses a string in single quotes.
- **is_contain_word**: Checks if a string contains a specific word.
- **has_contain_words**: Checks if a string contains all the specified words.
- **replace_placeholder**: Replaces a single placeholder in a string with a value.
- **replace_multiple_placeholders**: Replaces multiple placeholders in a string using a HashMap.

### byteutils::vec

- **dedup**: Removes duplicate elements from a vector.
- **retain_if**: Retains elements of a vector that satisfy a given condition.
- **reverse_in_place**: Reverses the order of elements in a vector.
- **split_at_vec**: Splits a vector into two at a specified index.
- **get_unique**: Returns a new vector with unique elements.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
