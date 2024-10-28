use std::collections::HashSet;
use std::hash::Hash;

/// Removes duplicate elements from a vector in-place.
///
/// This function efficiently removes duplicate elements from the input vector
/// while preserving the order of first occurrence for each unique element.
/// It uses a HashSet for fast lookup and the `retain` method for in-place filtering.
///
/// # Type Parameters
///
/// * `T`: The type of elements in the vector. It must implement `Eq`, `Hash`, and `Copy` traits.
///
/// # Arguments
///
/// * `v` - A mutable reference to the vector to be deduplicated.
///
/// # Example
///
/// ```rust
/// let mut numbers = vec![1, 2, 3, 2, 4, 1, 5];
/// byteutils::vec::dedup(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
/// ```
///
/// # Note
///
/// This function requires the `Copy` trait because it needs to copy elements
/// into the HashSet. For types that don't implement `Copy`, consider using
/// references or implementing a different deduplication strategy.
pub fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

/// Retains only the elements specified by the predicate.
///
/// In-place variant of `Vec::retain()`. This function will remove all elements
/// for which the predicate returns `false`, while keeping all elements for which
/// the predicate returns `true`.
///
/// # Arguments
///
/// * `v` - A mutable reference to the vector to be filtered
/// * `predicate` - A closure that takes a reference to an element and returns a boolean
///
/// # Examples
///
/// ```
/// let mut numbers = vec![1, 2, 3, 4, 5, 6];
/// byteutils::vec::retain_if(&mut numbers, |&x| x % 2 == 0);
/// assert_eq!(numbers, vec![2, 4, 6]);
/// ```
pub fn retain_if<T>(v: &mut Vec<T>, predicate: impl Fn(&T) -> bool) {
    v.retain(predicate);
}

/// Reverses the order of elements in the vector in place.
///
/// This function modifies the original vector, reversing the order of its elements
/// without allocating a new vector.
///
/// # Arguments
///
/// * `v` - A mutable reference to the vector to be reversed
///
/// # Examples
///
/// ```
/// let mut vec = vec![1, 2, 3, 4, 5];
/// byteutils::vec::reverse_in_place(&mut vec);
/// assert_eq!(vec, vec![5, 4, 3, 2, 1]);
/// ```
///
/// # Note
///
/// This function has a time complexity of O(n/2) where n is the length of the vector.
/// It performs in-place swapping, which is memory-efficient for large vectors.
pub fn reverse_in_place<T>(v: &mut Vec<T>) {
    let len = v.len();
    for i in 0..len / 2 {
        v.swap(i, len - 1 - i);
    }
}

/// Splits a vector into two at the given index, creating two new vectors.
///
/// This function takes a mutable reference to a vector and an index, and returns
/// two new vectors. The first vector contains cloned elements from the original vector
/// up to (but not including) the given index, and the second vector contains
/// cloned elements of the remaining items.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the vector, which must implement the `Clone` trait
///
/// # Arguments
///
/// * `v` - A mutable reference to the vector to be split
/// * `at` - The index at which to split the vector
///
/// # Returns
///
/// A tuple containing two new vectors: (left, right)
///
/// # Panics
///
/// This function will panic if `at` is greater than the length of the vector.
///
/// # Examples
///
/// ```
/// let mut vec = vec![1, 2, 3, 4, 5];
/// let (left, right) = byteutils::vec::split_at_vec(&mut vec, 3);
/// assert_eq!(left, vec![1, 2, 3]);
/// assert_eq!(right, vec![4, 5]);
/// ```
///
/// # Note
///
/// - This function creates new vectors, which means it allocates new memory and clones elements.
/// - If you only need to work with the split parts without creating new vectors or cloning elements,
///   consider using the standard library's `split_at` or `split_at_mut` methods instead.
/// - The original vector `v` is not modified by this operation.
pub fn split_at_vec<T: Clone>(v: &mut Vec<T>, at: usize) -> (Vec<T>, Vec<T>) {
    if at > v.len() {
        panic!("index out of bounds");
    }
    let (left, right) = v.split_at_mut(at);
    (left.to_vec(), right.to_vec())
}

/// Returns a new vector containing only unique elements from the input slice,
/// preserving the order of their first occurrence.
///
/// This function takes a slice of elements and returns a new vector containing
/// only the unique elements, maintaining the order in which they first appeared
/// in the original slice.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the slice, which must implement `Clone`, `Eq`, and `Hash` traits.
///
/// # Arguments
///
/// * `input` - A slice of elements to be processed.
///
/// # Returns
///
/// A new `Vec<T>` containing only the unique elements from the input slice.
///
/// # Performance
///
/// - Time complexity: O(n), where n is the length of the input slice.
/// - Space complexity: O(n) for storing both the result and the hash set.
///
/// # Examples
///
/// Basic usage with integers:
/// ```
/// let numbers = vec![1, 2, 2, 3, 1, 4];
/// let unique = byteutils::vec::get_unique(&numbers);
/// assert_eq!(unique, vec![1, 2, 3, 4]);
/// ```
///
/// Using with strings:
/// ```
/// let words = vec!["apple", "banana", "apple", "cherry"];
/// let unique = byteutils::vec::get_unique(&words);
/// assert_eq!(unique, vec!["apple", "banana", "cherry"]);
/// ```
///
/// Empty input:
/// ```
/// let empty: Vec<i32> = vec![];
/// let unique = byteutils::vec::get_unique(&empty);
/// assert_eq!(unique, Vec::<i32>::new());
/// ```
///
/// # Note
///
/// - This function creates a new vector and clones elements, which means it allocates new memory.
/// - The implementation uses a HashSet internally for O(1) lookups while preserving order.
/// - The original slice is not modified by this operation.
/// - Compared to the O(n²) version using Vec::contains, this implementation is much more efficient
///   for large inputs, though it requires elements to implement the Hash trait.
pub fn get_unique<T: Clone + Eq + std::hash::Hash>(input: &[T]) -> Vec<T> {
    let mut seen = HashSet::with_capacity(input.len());
    let mut result = Vec::with_capacity(input.len());

    for item in input {
        if seen.insert(item) {
            result.push(item.clone());
        }
    }

    result
}
