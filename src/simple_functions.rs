//! This module is for the simplest forms of functions.

/// Adds two 32 bit integers and returns the result.
///
/// Does not check for overflows.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Increments the first element of an array by 1.
///
/// Takes a mutable reference to and array and returns a reference to the array.
pub fn inc_array(array: &mut [i32]) -> &mut [i32] {
    // How does this function guarentee that the passed array will contain data at 0?
    array[1] += 1;
    array
}

pub fn print_array(array: &[i32]) -> &[i32] {
    for element in array.iter() {
        println!("{} ", element);
    }
    array
}


#[cfg(test)]
mod tests {
    // This bring the functions defined outside of the `tests` module into scope.
    use super::*;

    #[test]
    fn test_add() {
        // This works because the i32 type implements the `Copy` trait.
        // When type that implements the `Copy` trait is moved, the entire data is copied.
        let (a, b) = (8, 42);
        assert_eq!(add(a, b), 50);
        assert_eq!(add(a, b), 50);
    }

    #[test]
    fn test_inc_array() {
        // The array must be mutable in order to be able to pass a mutable reference.
        let mut array = [0, 1, 2, 3, 4];

        // This line will break it because array_2 would be a mutable reference to the same object
        // as array. So once array_2 is assigned to that data, array can no longer reference it.
        // let mut array_2 = inc_array(&mut array);
        assert_eq!(inc_array(&mut array), [0, 2, 2, 3, 4]);
        assert_eq!(inc_array(&mut array), [0, 3, 2, 3, 4]);
    }

    #[test]
    #[should_panic]
    fn test_inc_array_empty() {
        // This comples, but panics at runtime due to an index out of bounds.
        // Its possible that this will become a compile time error in a future major version.
        let mut array = [0];
        assert_eq!(inc_array(&mut array), [0]);
    }

    #[test]
    fn test_print_array() {
        // This test passes where `test_inc_array` would fail if it included its commented out line
        // because an immutable variable may have multiple references to it.
        let array = [0, 1, 2];
        let array_2 = print_array(&array);
        assert_eq!(print_array(&array), [0, 1, 2]);
        assert_eq!(print_array(&array_2), [0, 1, 2]);
    }
}
