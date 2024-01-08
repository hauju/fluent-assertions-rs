use crate::Should;

pub struct BoolAssertion {
    value: bool,
}

impl Should for bool {
    type Assertion = BoolAssertion;

    fn should(self) -> BoolAssertion {
        BoolAssertion::new(self)
    }
}

impl BoolAssertion {
    pub fn new(value: bool) -> Self {
        BoolAssertion { value }
    }

    pub fn be_true(&self) -> &Self {
        assert!(self.value, "Expected true, but got false");
        self
    }

    pub fn be_false(&self) -> &Self {
        assert!(!self.value, "Expected false, but got true");
        self
    }

    pub fn be(&self, other: bool) -> &Self {
        assert!(
            self.value == other,
            "Expected value to be {}, but got {}",
            other,
            self.value
        );
        self
    }

    pub fn not_be(&self, other: bool) -> &Self {
        assert!(
            self.value != other,
            "Expected value to not be {}, but got {}",
            other,
            self.value
        );
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
        
        let other_boolean = true;
        the_boolean.should().be(other_boolean);
        the_boolean.should().not_be(false);
    }
}
