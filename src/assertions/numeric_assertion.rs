use std::cmp::PartialOrd;
use std::fmt::Display;
use num_traits::Zero;

pub trait ShouldNumeric {
    type Assertion;
    fn should(self) -> Self::Assertion;
}

impl<T> ShouldNumeric for T
where
    T: PartialOrd + Display + Copy + num_traits::Zero,
{
    type Assertion = NumericAssertion<T>;

    fn should(self) -> NumericAssertion<T> {
        NumericAssertion::new(self)
    }
}

pub struct NumericAssertion<T: PartialOrd + Display> {
    value: T,
}

impl<T> NumericAssertion<T>
where
T: PartialOrd + Display + Zero + Copy,
{
    pub fn new(value: T) -> Self {
        NumericAssertion { value }
    }

    pub fn be_greater_than_or_equal_to(&self, other: T) -> &Self {
        assert!(
            self.value >= other,
            "Expected value to be greater than or equal to {}, but got {}",
            other,
            self.value
        );
        self
    }

    pub fn be_greater_than(&self, other: T) -> &Self {
        assert!(self.value > other, "Expected value to be greater than {}, but got {}", other, self.value);
        self
    }

    pub fn be_less_than_or_equal_to(&self, other: T) -> &Self {
        assert!(self.value <= other, "Expected value to be less than or equal to {}, but got {}", other, self.value);
        self
    }

    pub fn be_less_than(&self, other: T) -> &Self {
        assert!(self.value < other, "Expected value to be less than {}, but got {}", other, self.value);
        self
    }

    pub fn be_positive(&self) -> &Self {
        assert!(self.value > T::zero(), "Expected positive value, but found {}", self.value);
        self
    }

    pub fn be_negative(&self) -> &Self {
        assert!(self.value < T::zero(), "Expected negative value, but found {}", self.value);
        self
    }

    pub fn be(&self, other: T) -> &Self {
        assert!(self.value == other, "Expected value to be {}, but got {}", other, self.value);
        self
    }

    pub fn not_be(&self, other: T) -> &Self {
        assert!(self.value != other, "Expected value to not be {}, but got {}", other, self.value);
        self
    }

    pub fn be_in_range(&self, start: T, end: T) -> &Self {
        assert!(self.value >= start && self.value <= end, "Expected value to be in range {}-{}, but got {}", start, end, self.value);
        self
    }

    pub fn not_be_in_range(&self, start: T, end: T) -> &Self {
        assert!(self.value < start || self.value > end, "Expected value to not be in range {}-{}, but got {}", start, end, self.value);
        self
    }

}

#[cfg(test)]
mod tests {
    use crate::numeric_assertion::ShouldNumeric;

    #[test]
    fn test_num_assertions() {
        let actual = 42_u8;
        actual
            .should()
            .be_greater_than(42)
            .be_positive();
    }

    #[test]
    fn test_negativ_num_assertions() {
        let actual = -42_i8;
        actual
            .should()
            .be_greater_than(-43)
            .be_negative()
            .not_be(32)
            .be(-42);
    }

    #[test]
    fn test_int_assertions() {
        let actual = 42_i32;
        actual
            .should()
            .be_greater_than(42);
    }

    #[test]
    fn test_float_assertions() {
        let actual = 42.0;
        actual
            .should()
            .be_greater_than(41.99);
    }
}