use std::cmp::PartialOrd;
use std::fmt::Display;
use num_traits::Zero;

use crate::Should;

pub struct NumericAssertion<T: PartialOrd + Display> {
    value: T,
}

impl Should for f64 {
    type Assertion = NumericAssertion<f64>;

    fn should(self) -> NumericAssertion<f64> {
        NumericAssertion::new(self)
    }
}

impl Should for f32 {
    type Assertion = NumericAssertion<f32>;

    fn should(self) -> NumericAssertion<f32> {
        NumericAssertion::new(self)
    }
}

impl Should for usize {
    type Assertion = NumericAssertion<usize>;

    fn should(self) -> NumericAssertion<usize> {
        NumericAssertion::new(self)
    }
}

impl Should for u64 {
    type Assertion = NumericAssertion<u64>;

    fn should(self) -> NumericAssertion<u64> {
        NumericAssertion::new(self)
    }
}

impl Should for u32 {
    type Assertion = NumericAssertion<u32>;

    fn should(self) -> NumericAssertion<u32> {
        NumericAssertion::new(self)
    }
}

impl Should for u16 {
    type Assertion = NumericAssertion<u16>;

    fn should(self) -> NumericAssertion<u16> {
        NumericAssertion::new(self)
    }
}

impl Should for u8 {
    type Assertion = NumericAssertion<u8>;

    fn should(self) -> NumericAssertion<u8> {
        NumericAssertion::new(self)
    }
}

impl Should for isize {
    type Assertion = NumericAssertion<isize>;

    fn should(self) -> NumericAssertion<isize> {
        NumericAssertion::new(self)
    }
}

impl Should for i64 {
    type Assertion = NumericAssertion<i64>;

    fn should(self) -> NumericAssertion<i64> {
        NumericAssertion::new(self)
    }
}

impl Should for i32 {
    type Assertion = NumericAssertion<i32>;

    fn should(self) -> NumericAssertion<i32> {
        NumericAssertion::new(self)
    }
}

impl Should for i16 {
    type Assertion = NumericAssertion<i16>;

    fn should(self) -> NumericAssertion<i16> {
        NumericAssertion::new(self)
    } 
}

impl Should for i8 {
    type Assertion = NumericAssertion<i8>;

    fn should(self) -> NumericAssertion<i8> {
        NumericAssertion::new(self)
    }
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
    use rstest::*;
    use crate::assertions::*;

    #[rstest]
    #[case(43, 42)]
    #[case(1, 0)]
    fn should_be_greater_and_positive(#[case] input: isize, #[case] value: isize) {
        input.should()
            .be_greater_than(value)
            .be_positive();
    }

    #[rstest]
    #[case(42.0, 41.99)]
    #[case(1.0, 0.9)]
    fn should_be_greater_f64(#[case] input: f64, #[case] value: f64) {
        input
            .should()
            .be_greater_than(value);
    }

    #[rstest]
    #[case(-42)]
    #[case(-3)]
    fn should_be_negative_i8(#[case] input: i8) {
        input
            .should()
            .be_greater_than(-43)
            .be_negative()
            .not_be(32)
            .be(input);
    }

    
}