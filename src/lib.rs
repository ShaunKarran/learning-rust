//! This module is just to play around and get familiar with the rust language.

/// Adds two 32 bit integers and returns the result.
///
/// Does not check for overflows.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // This bring the functions defined outside of the `tests` module into scope.
    use super::*;

    #[test]
    fn test_add_two_positives() {
        assert_eq!(add(8, 42), 50);
    }

    #[test]
    fn test_access_vars_after_moving() {
        // This works because the i32 type implements the `Copy` trait.
        // When type that implements the `Copy` trait is moved, the entire data is copied.
        let (a, b) = (8, 42);
        assert_eq!(add(a, b), 50);
        assert_eq!(add(a, b), 50);
    }
}
