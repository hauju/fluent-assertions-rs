use super::Assertion;
use std::fmt::Debug;

impl<T: Debug + PartialEq> Assertion<Option<T>> {
    /// Asserts that the Option is Some and unwraps it into an `Assertion<T>`
    #[track_caller]
    pub fn be_some(self) -> Assertion<T> {
        match self.value {
            Some(value) => Assertion { value },
            None => panic!("Expected Some, but got None"),
        }
    }

    /// Asserts that the Option is Some and contains the expected value
    #[track_caller]
    pub fn contains(self, expected: &T) -> Self {
        assert_eq!(
            self.value.as_ref(),
            Some(expected),
            "Expected Some({:?}), but was {:?}",
            expected,
            self.value
        );
        self
    }

    /// Asserts that the Option is None
    #[track_caller]
    pub fn be_none(self) -> Self {
        assert!(
            self.value.is_none(),
            "Expected None, but got {:?}",
            self.value
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::*;
    use rstest::*;

    #[rstest]
    #[case(None)]
    fn should_be_none(#[case] input: Option<String>) {
        input.should().be_none();
    }

    #[rstest]
    #[case(42f64)]
    #[case(0.0)]
    fn should_be_some(#[case] expected: f64) {
        let input = Some(expected);
        input.should().be_some().be(expected);
    }

    #[rstest]
    #[case("hello")]
    fn should_contain(#[case] expected: &str) {
        let input = Some(expected);
        input.should().contains(&expected);
    }

    #[rstest]
    #[case(Some(String::from("hello")))]
    fn should_contain_string(#[case] input: Option<String>) {
        input.should().contains(&String::from("hello"));
    }

    #[test]
    #[should_panic(expected = "Expected None, but got Some(42)")]
    fn be_none_panics_with_actual_value() {
        Some(42).should().be_none();
    }
}
