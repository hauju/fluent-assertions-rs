use super::Assertion;
use std::fmt::Debug;

impl<T: Debug + PartialEq> Assertion<Option<T>> {
    /// Asserts that the Option is Some
    pub fn be_some(&self) -> &Self {
        assert!(self.value.is_some(), "Expected Some, but got None");
        self
    }

    /// Asserts that the Option is Some and contains the expected value
    pub fn contains(&self, expected: &T) -> &Self {
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
    pub fn be_none(&self) -> &Self {
        assert!(self.value.is_none(), "Expected None, but got Some");
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
        input.should().be_some().contains(&expected);
    }

    #[rstest]
    #[case("hello")]
    fn should_contain(#[case] expected: &str) {
        let input = Some(expected);
        input.should().be_some().contains(&expected);
    }

    #[rstest]
    #[case(Some(String::from("hello")))]
    fn should_contain_string(#[case] input: Option<String>) {
        input.should().be_some().contains(&String::from("hello"));
    }
}
