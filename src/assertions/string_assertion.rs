use crate::Assertion;

/// Specific assertions for strings
impl<T: AsRef<str>> Assertion<T> {
    /// Asserts that the string is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// "".should().be_empty();
    /// ```
    #[track_caller]
    pub fn be_empty(self) -> Self {
        assert!(
            self.value.as_ref().is_empty(),
            "Expected string to be empty, but got '{}'",
            self.value.as_ref()
        );
        self
    }
    /// Asserts that the string is not empty
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// "hello".should().not_be_empty();
    /// ```
    #[track_caller]
    pub fn not_be_empty(self) -> Self {
        assert!(
            !self.value.as_ref().is_empty(),
            "Expected string to not be empty, but got empty string"
        );
        self
    }
    /// Asserts that the string starts with a given prefix
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// "hello world".should().start_with("hello");
    /// ```
    #[track_caller]
    pub fn start_with(self, prefix: &str) -> Self {
        assert!(
            self.value.as_ref().starts_with(prefix),
            "Expected string to start with '{}', but got '{}'",
            prefix,
            self.value.as_ref()
        );
        self
    }
    /// Asserts that the string ends with a given suffix
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// "hello world".should().end_with("world");
    /// ```
    #[track_caller]
    pub fn end_with(self, suffix: &str) -> Self {
        assert!(
            self.value.as_ref().ends_with(suffix),
            "Expected string to end with '{}', but got '{}'",
            suffix,
            self.value.as_ref()
        );
        self
    }

    /// Asserts that the string contains a given substring
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// "hello world".should().contain("lo wo");
    /// ```
    #[track_caller]
    pub fn contain(self, substring: &str) -> Self {
        assert!(
            self.value.as_ref().contains(substring),
            "Expected string to contain '{}', but it didn't",
            substring
        );
        self
    }

    /// Asserts that the string has a given length
    ///
    /// The length is measured with [`str::len`], i.e. the number of bytes,
    /// not the number of characters. For strings containing multi-byte
    /// UTF-8 characters these two counts differ.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// "hello".should().have_length(5);
    /// // Bytes, not chars: 'é' is a two-byte UTF-8 sequence.
    /// "é".should().have_length(2);
    /// ```
    #[track_caller]
    pub fn have_length(self, length: usize) -> Self {
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

    #[test]
    #[should_panic(expected = "Expected string to start with 'hello'")]
    fn start_with_panics_when_actual_shorter_than_prefix() {
        "hi".should().start_with("hello");
    }

    #[test]
    #[should_panic(expected = "Expected string to end with 'world'")]
    fn end_with_panics_when_actual_shorter_than_suffix() {
        "hi".should().end_with("world");
    }

    #[test]
    #[should_panic(expected = "Expected string to start with")]
    fn start_with_panics_on_multibyte_actual() {
        "é".should().start_with("prefix");
    }
}
