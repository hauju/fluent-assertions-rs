use super::Assertion;
use std::fmt::Debug;

/// Specific assertions for list-like collections (`Vec` and slices).
///
/// These live on a trait rather than being inherent methods because the string
/// assertions are implemented for the blanket `impl<T: AsRef<str>> Assertion<T>`.
/// Coherence conservatively assumes `Vec<_>` / `&[_]` might implement
/// `AsRef<str>` in the future, so inherent methods sharing the same names
/// (`be_empty`, `contain`, ...) would clash with the string impl. A trait sidesteps
/// that: it is implemented only for the concrete `Assertion<Vec<T>>` and
/// `Assertion<&[T]>`, never for `&str` / `String`.
pub trait CollectionAssertion<T> {
    /// Asserts that the collection is empty
    fn be_empty(self) -> Self;
    /// Asserts that the collection is not empty
    fn not_be_empty(self) -> Self;
    /// Asserts that the collection has a given length
    fn have_length(self, length: usize) -> Self;
    /// Asserts that the collection contains a given element
    fn contain(self, expected: &T) -> Self
    where
        T: PartialEq;
}

impl<T: Debug> CollectionAssertion<T> for Assertion<Vec<T>> {
    #[track_caller]
    fn be_empty(self) -> Self {
        assert!(
            self.value.is_empty(),
            "Expected collection to be empty, but got {:?}",
            self.value
        );
        self
    }

    #[track_caller]
    fn not_be_empty(self) -> Self {
        assert!(
            !self.value.is_empty(),
            "Expected collection to not be empty, but got empty collection"
        );
        self
    }

    #[track_caller]
    fn have_length(self, length: usize) -> Self {
        assert!(
            self.value.len() == length,
            "Expected collection to have length {}, but it had length {}",
            length,
            self.value.len()
        );
        self
    }

    #[track_caller]
    fn contain(self, expected: &T) -> Self
    where
        T: PartialEq,
    {
        assert!(
            self.value.contains(expected),
            "Expected collection to contain {:?}, but it didn't",
            expected
        );
        self
    }
}

impl<T: Debug> CollectionAssertion<T> for Assertion<&[T]> {
    #[track_caller]
    fn be_empty(self) -> Self {
        assert!(
            self.value.is_empty(),
            "Expected collection to be empty, but got {:?}",
            self.value
        );
        self
    }

    #[track_caller]
    fn not_be_empty(self) -> Self {
        assert!(
            !self.value.is_empty(),
            "Expected collection to not be empty, but got empty collection"
        );
        self
    }

    #[track_caller]
    fn have_length(self, length: usize) -> Self {
        assert!(
            self.value.len() == length,
            "Expected collection to have length {}, but it had length {}",
            length,
            self.value.len()
        );
        self
    }

    #[track_caller]
    fn contain(self, expected: &T) -> Self
    where
        T: PartialEq,
    {
        assert!(
            self.value.contains(expected),
            "Expected collection to contain {:?}, but it didn't",
            expected
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::*;
    use rstest::*;

    #[test]
    fn test_vec_assertions() {
        let numbers = vec![1, 2, 3];
        numbers.should().not_be_empty().have_length(3).contain(&2);
    }

    #[test]
    fn test_slice_assertions() {
        let numbers = [1, 2, 3];
        numbers
            .as_slice()
            .should()
            .not_be_empty()
            .have_length(3)
            .contain(&2);
    }

    #[rstest]
    #[case(Vec::<i32>::new())]
    #[case(vec![])]
    fn should_be_empty(#[case] input: Vec<i32>) {
        input.should().be_empty();
    }

    #[test]
    fn empty_slice_should_be_empty() {
        let empty: [i32; 0] = [];
        empty.as_slice().should().be_empty();
    }

    #[rstest]
    #[case(vec!["hello".to_string()])]
    fn should_contain_string(#[case] input: Vec<String>) {
        input.should().contain(&String::from("hello"));
    }

    #[test]
    #[should_panic(expected = "Expected collection to contain 4")]
    fn contain_panics_when_element_missing() {
        vec![1, 2, 3].should().contain(&4);
    }

    #[test]
    #[should_panic(expected = "Expected collection to be empty")]
    fn be_empty_panics_when_not_empty() {
        vec![1].should().be_empty();
    }

    #[test]
    #[should_panic(expected = "Expected collection to have length 5, but it had length 3")]
    fn have_length_panics_on_mismatch() {
        vec![1, 2, 3].should().have_length(5);
    }
}
