

pub struct OptionAssertion<T> {
    value: Option<T>,
}

impl <T> OptionAssertion<T> {
    pub fn new(value: Option<T>) -> Self {
        OptionAssertion { value }
    }

    pub fn be_some(&self) -> &Self {
        assert!(self.value.is_some(), "Expected Some, but got None");
        self
    }

    pub fn be_none(&self) -> &Self {
        assert!(self.value.is_none(), "Expected None, but got Some");
        self
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use crate::assertions::ShouldOption;

    #[rstest]
    #[case(None)]
    fn should_be_none(#[case] input: Option<String>) {
        input.should()
            .be_none();
    }

    #[rstest]
    #[case(Some(42f64))]
    #[case(Some(0.0))]
    fn should_be_some(#[case] input: Option<f64>) {
        input.should()
            .be_some();
    }
}