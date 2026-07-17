use super::Assertion;
use std::fmt::Debug;

impl<T: Debug + PartialEq> Assertion<Option<T>> {
    /// Asserts that the Option is Some and unwraps it into an `Assertion<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// Some(5).should().be_some().be(5);
    /// ```
    #[track_caller]
    pub fn be_some(self) -> Assertion<T> {
        match self.value {
            Some(value) => Assertion { value },
            None => panic!("Expected Some, but got None"),
        }
    }

    /// Asserts that the Option is None
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// None::<i32>.should().be_none();
    /// ```
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

/// Containment assertion for `Option`.
///
/// This lives on a trait rather than being an inherent method for the same
/// coherence reason as [`CollectionAssertion`]: an inherent `contain` on
/// `Assertion<Option<T>>` would clash with the inherent `contain` from the
/// `impl<T: AsRef<str>> Assertion<T>` string assertions, because coherence
/// cannot rule out a future `AsRef<str>` impl for `Option<_>`.
pub trait OptionAssertion<T> {
    /// Asserts that the Option is Some and contains the expected value
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// Some(5).should().contain(&5);
    /// ```
    fn contain(self, expected: &T) -> Self;
}

impl<T: Debug + PartialEq> OptionAssertion<T> for Assertion<Option<T>> {
    #[track_caller]
    fn contain(self, expected: &T) -> Self {
        assert_eq!(
            self.value.as_ref(),
            Some(expected),
            "Expected Some({:?}), but was {:?}",
            expected,
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
        input.should().contain(&expected);
    }

    #[rstest]
    #[case(Some(String::from("hello")))]
    fn should_contain_string(#[case] input: Option<String>) {
        input.should().contain(&String::from("hello"));
    }

    #[test]
    #[should_panic(expected = "Expected None, but got Some(42)")]
    fn be_none_panics_with_actual_value() {
        Some(42).should().be_none();
    }
}
