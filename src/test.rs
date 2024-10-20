use crate::string::*;
use crate::vec::*;
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

#[test]
fn test_escape_sql_basic() {
    assert_eq!(escape_sql("normal text"), "normal text");
}

#[test]
fn test_escape_sql_with_single_quotes() {
    assert_eq!(escape_sql("O'Connor"), "O''Connor");
}

#[test]
fn test_escape_sql_with_backslashes() {
    assert_eq!(escape_sql("C:\\path"), "C:\\\\path");
}

#[test]
fn test_escape_sql_with_both() {
    assert_eq!(escape_sql("O'Connor\\path"), "O''Connor\\\\path");
}

#[test]
fn test_escape_sql_empty_string() {
    assert_eq!(escape_sql(""), "");
}

#[test]
fn test_enclose_quotes_basic() {
    assert_eq!(enclose_quotes("name"), "'name'");
}

#[test]
fn test_enclose_quotes_empty_string() {
    assert_eq!(enclose_quotes(""), "''");
}

#[test]
fn test_enclose_quotes_with_spaces() {
    assert_eq!(enclose_quotes("table name"), "'table name'");
}

#[test]
fn test_is_contain_word() {
    assert!(is_contain_word("Hello world", "world"));
    assert!(is_contain_word("The quick brown fox", "quick"));
    assert!(!is_contain_word("Hello world", "worl"));
    assert!(!is_contain_word("HelloWorld", "World"));
}

#[test]
fn test_has_contain_words() {
    let words = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];
    assert!(has_contain_words("I like apple pie", &words));
    assert!(has_contain_words("Banana split is delicious", &words));
    assert!(!has_contain_words("I love peaches and pears", &words));
}

#[test]
fn test_dedup_integers() {
    let mut numbers = vec![1, 2, 3, 2, 4, 1, 5];
    dedup(&mut numbers);
    assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_dedup_strings() {
    let mut words = vec!["apple", "banana", "apple", "cherry", "banana", "date"];
    dedup(&mut words);
    assert_eq!(words, vec!["apple", "banana", "cherry", "date"]);
}

#[test]
fn test_dedup_empty_vec() {
    let mut empty: Vec<i32> = vec![];
    dedup(&mut empty);
    assert_eq!(empty, vec![]);
}

#[test]
fn test_dedup_all_unique() {
    let mut unique = vec![1, 2, 3, 4, 5];
    dedup(&mut unique);
    assert_eq!(unique, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_dedup_all_same() {
    let mut same = vec![1, 1, 1, 1, 1];
    dedup(&mut same);
    assert_eq!(same, vec![1]);
}

#[test]
fn test_retain_even_numbers() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6];
    retain_if(&mut numbers, |&x| x % 2 == 0);
    assert_eq!(numbers, vec![2, 4, 6]);
}

#[test]
fn test_retain_odd_numbers() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6];
    retain_if(&mut numbers, |&x| x % 2 != 0);
    assert_eq!(numbers, vec![1, 3, 5]);
}

#[test]
fn test_retain_all() {
    let mut numbers = vec![1, 2, 3];
    retain_if(&mut numbers, |_| true);
    assert_eq!(numbers, vec![1, 2, 3]);
}

#[test]
fn test_retain_none() {
    let mut numbers = vec![1, 2, 3];
    retain_if(&mut numbers, |_| false);
    assert_eq!(numbers, Vec::<i32>::new());
}

#[test]
fn test_retain_greater_than_threshold() {
    let mut numbers = vec![1, 5, 2, 8, 3, 7];
    let threshold = 4;
    retain_if(&mut numbers, |&x| x > threshold);
    assert_eq!(numbers, vec![5, 8, 7]);
}

#[test]
fn test_retain_strings_by_length() {
    let mut strings = vec![
        String::from("a"),
        String::from("ab"),
        String::from("abc"),
        String::from("abcd"),
    ];
    retain_if(&mut strings, |s| s.len() > 2);
    assert_eq!(strings, vec![String::from("abc"), String::from("abcd")]);
}

#[test]
fn test_empty_vector() {
    let mut empty: Vec<i32> = Vec::new();
    retain_if(&mut empty, |&x| x > 0);
    assert_eq!(empty, Vec::<i32>::new());
}

#[test]
fn test_retain_custom_struct() {
    struct Person {
        name: String,
        age: u32,
    }

    let mut people = vec![
        Person {
            name: String::from("Alice"),
            age: 25,
        },
        Person {
            name: String::from("Bob"),
            age: 30,
        },
        Person {
            name: String::from("Charlie"),
            age: 35,
        },
    ];

    retain_if(&mut people, |p| p.age > 28);

    assert_eq!(people.len(), 2);
    assert_eq!(people[0].name, "Bob");
    assert_eq!(people[1].name, "Charlie");
}
