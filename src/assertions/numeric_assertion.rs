use super::Assertion;
use num_traits::{Float, Zero};
use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};
use std::ops::RangeBounds;

/// Specific assertions for numeric types
impl<T> Assertion<T>
where
    T: PartialOrd + Display + Zero + Copy,
{
    /// Asserts that the value is greater than or equal to the given value
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// 5.should().be_greater_than_or_equal_to(5);
    /// ```
    #[track_caller]
    pub fn be_greater_than_or_equal_to(self, other: T) -> Self {
        assert!(
            self.value >= other,
            "Expected value to be greater than or equal to {}, but got {}",
            other,
            self.value
        );
        self
    }

    /// Asserts that the value is greater than the given value
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// 5.should().be_greater_than(4);
    /// ```
    #[track_caller]
    pub fn be_greater_than(self, other: T) -> Self {
        assert!(
            self.value > other,
            "Expected value to be greater than {}, but got {}",
            other,
            self.value
        );
        self
    }

    /// Asserts that the value is less than or equal to the given value
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// 5.should().be_less_than_or_equal_to(5);
    /// ```
    #[track_caller]
    pub fn be_less_than_or_equal_to(self, other: T) -> Self {
        assert!(
            self.value <= other,
            "Expected value to be less than or equal to {}, but got {}",
            other,
            self.value
        );
        self
    }

    /// Asserts that the value is less than the given value
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// 5.should().be_less_than(6);
    /// ```
    #[track_caller]
    pub fn be_less_than(self, other: T) -> Self {
        assert!(
            self.value < other,
            "Expected value to be less than {}, but got {}",
            other,
            self.value
        );
        self
    }

    /// Asserts that the value is positive
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// 5.should().be_positive();
    /// ```
    #[track_caller]
    pub fn be_positive(self) -> Self {
        assert!(
            self.value > T::zero(),
            "Expected positive value, but found {}",
            self.value
        );
        self
    }

    /// Asserts that the value is negative
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// (-5).should().be_negative();
    /// ```
    #[track_caller]
    pub fn be_negative(self) -> Self {
        assert!(
            self.value < T::zero(),
            "Expected negative value, but found {}",
            self.value
        );
        self
    }

    /// Asserts that the value is in the given range
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// 5.should().be_in_range(1..=10);
    /// ```
    #[track_caller]
    pub fn be_in_range(self, range: impl RangeBounds<T> + Debug) -> Self {
        assert!(
            range.contains(&self.value),
            "Expected value {} to be in range {:?}, but it wasn't",
            self.value,
            range
        );
        self
    }

    /// Asserts that the value is not in the given range
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// 20.should().not_be_in_range(1..=10);
    /// ```
    #[track_caller]
    pub fn not_be_in_range(self, range: impl RangeBounds<T> + Debug) -> Self {
        assert!(
            !range.contains(&self.value),
            "Expected value {} to not be in range {:?}, but it was",
            self.value,
            range
        );
        self
    }
}

/// Specific assertions for floating-point types
impl<T> Assertion<T>
where
    T: Float + Display,
{
    /// Asserts that the value is within `tolerance` of `expected`
    ///
    /// The tolerance bound is inclusive: a difference exactly equal to
    /// `tolerance` still passes.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// // The difference is exactly 0.5, and the inclusive bound accepts it.
    /// 0.5_f64.should().be_close_to(1.0, 0.5);
    /// ```
    #[track_caller]
    pub fn be_close_to(self, expected: T, tolerance: T) -> Self {
        assert!(
            (self.value - expected).abs() <= tolerance,
            "Expected value {} to be close to {} (tolerance {}), but it wasn't",
            self.value,
            expected,
            tolerance
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::*;
    use rstest::*;

    #[rstest]
    #[case(43, 42)]
    #[case(1, 0)]
    fn should_be_greater_and_positive(#[case] input: isize, #[case] value: isize) {
        input.should().be_greater_than(value).be_positive();
    }

    #[rstest]
    #[case(42.0, 41.99)]
    #[case(1.0, 0.9)]
    fn should_be_greater_f64(#[case] input: f64, #[case] value: f64) {
        input.should().be_greater_than(value);
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

    #[test]
    fn should_be_in_inclusive_range() {
        5.should().be_in_range(1..=10);
    }

    #[test]
    fn should_be_in_exclusive_range() {
        5.should().be_in_range(1..10);
    }

    #[test]
    #[should_panic(expected = "to be in range")]
    fn be_in_range_panics_at_exclusive_end() {
        10.should().be_in_range(1..10);
    }

    #[test]
    fn should_not_be_in_range() {
        20.should().not_be_in_range(1..=10);
    }

    #[rstest]
    #[case(1.0f64, 1.0001, 0.001)]
    #[case(1.0f64, 0.9999, 0.001)]
    fn should_be_close_to_f64(#[case] input: f64, #[case] expected: f64, #[case] tolerance: f64) {
        input.should().be_close_to(expected, tolerance);
    }

    #[rstest]
    #[case(1.0f32, 1.05, 0.1)]
    fn should_be_close_to_f32(#[case] input: f32, #[case] expected: f32, #[case] tolerance: f32) {
        input.should().be_close_to(expected, tolerance);
    }

    #[test]
    #[should_panic(expected = "to be close to")]
    fn be_close_to_panics_when_outside_tolerance() {
        1.0f64.should().be_close_to(1.5, 0.1);
    }
}
