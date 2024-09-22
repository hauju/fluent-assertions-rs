use crate::Assertion;

/// Specific assertions for strings
impl<T: AsRef<str>> Assertion<T> {
    /// Asserts that the string is empty
    pub fn be_empty(&self) -> &Self {
        assert!(
            self.value.as_ref().is_empty(),
            "Expected string to be empty, but got '{}'",
            self.value.as_ref()
        );
        self
    }
    /// Asserts that the string is not empty
    pub fn not_be_empty(&self) -> &Self {
        assert!(
            !self.value.as_ref().is_empty(),
            "Expected string to not be empty, but got empty string"
        );
        self
    }
    /// Asserts that the string starts with a given prefix
    pub fn start_with(&self, prefix: &str) -> &Self {
        assert!(
            self.value.as_ref().starts_with(prefix),
            "Expected string to start with '{}', but it started with '{}'",
            prefix,
            &self.value.as_ref()[0..prefix.len()]
        );
        self
    }
    /// Asserts that the string ends with a given suffix
    pub fn end_with(&self, suffix: &str) -> &Self {
        assert!(
            self.value.as_ref().ends_with(suffix),
            "Expected string to end with '{}', but it ended with '{}'",
            suffix,
            &self.value.as_ref()[self.value.as_ref().len() - suffix.len()..]
        );
        self
    }

    /// Asserts that the string contains a given substring
    pub fn contain(&self, substring: &str) -> &Self {
        assert!(
            self.value.as_ref().contains(substring),
            "Expected string to contain '{}', but it didn't",
            substring
        );
        self
    }

    /// Asserts that the string has a given length
    pub fn have_length(&self, length: usize) -> &Self {
        assert!(
            self.value.as_ref().len() == length,
            "Expected string to have length {}, but it had length {}",
            length,
            self.value.as_ref().len()
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::*;
    use rstest::*;

    #[test]
    fn test_str_assertions() {
        let actual = "ABCDEFGHI";
        actual
            .should()
            .start_with("AB")
            .end_with("HI")
            .contain("EF")
            .have_length(9);
    }

    #[test]
    fn test_string_assertions() {
        let actual_string = "ABCDEFGHI".to_string();
        actual_string
            .should()
            .start_with("AB")
            .end_with("HI")
            .contain("EF")
            .have_length(9);
    }

    #[rstest]
    #[case(String::default())]
    #[case(String::from(""))]
    #[case("".to_string())]
    fn should_be_empty(#[case] input: String) {
        input.should().be_empty();
    }

    #[rstest]
    #[case(String::from("hello"))]
    #[case("42".to_string())]
    fn should_not_be_empty(#[case] input: String) {
        input.should().not_be_empty();
    }

    #[rstest]
    #[case("hello")]
    #[case("42")]
    fn should_be(#[case] input: &str) {
        input.should().be(input);
    }
}
