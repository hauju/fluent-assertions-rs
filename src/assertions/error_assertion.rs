use crate::Assertion;
use std::error::Error;
use std::fmt::Debug;

// Specific assertions for Error types
impl<E: Error> Assertion<E> {
    /// Asserts that the error is equal to the given error
    pub fn match_error<T: Error + Debug>(self, error: T) -> Self {
        assert!(
            self.value.to_string() == error.to_string(),
            "Expected error to be {:?}, but got {:?}",
            error,
            self.value
        );
        self
    }

    /// Asserts that the error message contains the given message
    pub fn contain_message(self, expected_message: &str) -> Self {
        let error_message = self.value.to_string();
        assert!(
            error_message.contains(expected_message),
            "Expected error message to contain '{}', but was '{}'",
            expected_message,
            error_message
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::*;

    // Example custom error type for testing
    #[derive(Debug)]
    struct MyError(String);

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyError: {}", self.0)
        }
    }

    impl Error for MyError {}

    #[test]
    fn should_match_error() {
        let error = MyError("test error".to_string());
        error
            .should()
            .match_error(MyError("test error".to_string()))
            .contain_message("test error");
    }

    #[test]
    fn should_contain_error_message() {
        let error = MyError("test error".to_string());
        error.should().contain_message("test error");
    }

    #[test]
    #[should_panic(expected = "Expected error message to contain 'wrong message'")]
    fn should_be_wrong_message() {
        let error = MyError("test error".to_string());
        error.should().contain_message("wrong message");
    }
}
