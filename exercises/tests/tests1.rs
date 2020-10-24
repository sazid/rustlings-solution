// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true);
    }

    #[test]
    fn you_can_compare() {
        assert_eq!(42, 40 + 2);
    }

    #[test]
    fn you_can_compare_not_equals() {
        assert_ne!(2, 3);
    }
}

