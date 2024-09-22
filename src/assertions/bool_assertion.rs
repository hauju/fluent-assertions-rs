use super::Assertion;

/// Specific assertions for booleans
impl Assertion<bool> {
    /// Asserts that the value is true
    pub fn be_true(self) -> Self {
        assert!(self.value, "Expected true, but got false");
        self
    }

    /// Asserts that the value is false
    pub fn be_false(self) -> Self {
        assert!(!self.value, "Expected false, but got true");
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::*;

    #[test]
    fn test_bool_assertions() {
        let the_boolean = false;
        the_boolean.should().be_false();

        let the_boolean = true;
        the_boolean.should().be_true();

        let the_boolean = false;
        the_boolean.should().be(false);
    }
}
