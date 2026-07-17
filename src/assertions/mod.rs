use std::fmt::Debug;

pub mod bool_assertion;
pub mod collection_assertion;
pub mod error_assertion;
pub mod numeric_assertion;
pub mod option_assertion;
pub mod result_assertion;
pub mod string_assertion;

pub use collection_assertion::CollectionAssertion;
pub use option_assertion::OptionAssertion;

// General Should trait for all types
pub trait Should: Sized {
    #[must_use = "this creates an assertion but checks nothing — call an assertion method like `.be(...)`"]
    fn should(self) -> Assertion<Self>;
}

// Implement Should for all types
// (`#[must_use]` lives on the trait declaration above; the compiler rejects it
// on trait methods inside impl blocks and it propagates from the declaration.)
impl<T> Should for T {
    fn should(self) -> Assertion<Self> {
        Assertion { value: self }
    }
}

// General Assertion struct
pub struct Assertion<T> {
    value: T,
}

impl<T> Assertion<T> {
    /// Consumes the assertion and returns the inner value.
    ///
    /// This enables assert-and-continue patterns, unwrapping an inner value
    /// after asserting on it:
    ///
    /// ```
    /// use fluent_assertions::*;
    /// let v = Some(5).should().be_some().into_inner();
    /// assert_eq!(v, 5);
    /// ```
    pub fn into_inner(self) -> T {
        self.value
    }
}

/// General assertions for all types
impl<T> Assertion<T>
where
    T: Debug,
{
    /// Asserts that the value equals `other`, allowing cross-type comparison.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// "foo".to_string().should().be("foo");
    /// ```
    #[track_caller]
    pub fn be<U: Debug>(self, other: U) -> Self
    where
        T: PartialEq<U>,
    {
        assert!(
            self.value == other,
            "Expected value to be {:?}, but got {:?}",
            other,
            self.value
        );
        self
    }

    /// Asserts that the value does not equal `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_assertions::*;
    /// 42.should().not_be(0);
    /// ```
    #[track_caller]
    pub fn not_be<U: Debug>(self, other: U) -> Self
    where
        T: PartialEq<U>,
    {
        assert!(
            self.value != other,
            "Expected value to not be {:?}, but got {:?}",
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
    fn be_allows_cross_type_comparison() {
        "foo".to_string().should().be("foo");
    }

    #[test]
    fn be_allows_debug_only_types() {
        Some(5).should().be(Some(5));
    }

    #[test]
    fn into_inner_returns_wrapped_value() {
        let v = Some(5).should().be_some().into_inner();
        assert_eq!(v, 5);
    }
}
