use super::Assertion;
use std::fmt::Debug;

/// Specific assertions for Result types
impl<T, E> Assertion<Result<T, E>>
where
    T: Debug,
    E: Debug,
{
    /// Asserts that the Result is Ok
    pub fn be_ok(self) -> Assertion<T> {
        match self.value {
            Ok(value) => Assertion { value },
            Err(e) => panic!("Expected Ok, but got Err({:?})", e),
        }
    }

    /// Asserts that the Result is Err
    pub fn be_err(self) -> Assertion<E> {
        match self.value {
            Ok(v) => panic!("Expected Err, but got Ok({:?})", v),
            Err(error) => Assertion { value: error },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::*;
    use rstest::*;

    #[rstest]
    #[case(Ok(42))]
    #[case(Ok(0))]
    fn should_be_ok(#[case] input: Result<i32, String>) {
        input.should().be_ok();
    }

    #[rstest]
    fn should_be_err() {
        let error = "Error message".to_string();
        let result: Result<i32, String> = Err(error);
        result.should().be_err();
    }
}
