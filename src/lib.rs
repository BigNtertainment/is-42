//! # Is 42
//! A crate you can use to check if a number is equal to 42.

/// Checks if a number is 42.
/// ```
/// let number = 42;
/// 
/// assert!(is_42::is_42(number));
/// ```
pub fn is_42(num: i32) -> bool {
    num == 42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checking_with_42() {
        assert!(is_42(42));
    }

    #[test]
    fn checking_with_not_42() {
        assert!(!is_42(10));
        assert!(!is_42(20));
        assert!(!is_42(55));
        assert!(!is_42(-1));
        assert!(!is_42(24));
    }
}
