

pub struct ResultAssertion<T, E> {
    value: Result<T, E>,
}

impl <T, E> ResultAssertion<T, E> {
    pub fn new(value: Result<T, E>) -> Self {
        ResultAssertion { value }
    }

    pub fn be_ok(&self) -> &Self {
        assert!(self.value.is_ok(), "Expected Ok, but got Err");
        self
    }

    pub fn be_err(&self) -> &Self {
        assert!(self.value.is_err(), "Expected Err, but got Ok");
        self
    }

}



#[cfg(test)]
mod tests {
    use rstest::*;
    use crate::assertions::ShouldResult;

    #[rstest]
    #[case(Ok(42))]
    #[case(Ok(0))]
    fn should_be_ok(#[case] input: Result<i32, String>) {
        input.should()
            .be_ok();
    }
    
}