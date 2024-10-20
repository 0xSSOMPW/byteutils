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
