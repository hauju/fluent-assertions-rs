use std::fmt::Display;

pub mod bool_assertion;
pub mod collection_assertion;
pub mod error_assertion;
pub mod numeric_assertion;
pub mod option_assertion;
pub mod result_assertion;
pub mod string_assertion;

pub use collection_assertion::CollectionAssertion;

// General Should trait for all types
pub trait Should: Sized {
    fn should(self) -> Assertion<Self>;
}

// Implement Should for all types
impl<T> Should for T {
    fn should(self) -> Assertion<Self> {
        Assertion { value: self }
    }
}

// General Assertion struct
pub struct Assertion<T> {
    value: T,
}

/// General assertions for all types
impl<T> Assertion<T>
where
    T: PartialEq + Display,
{
    #[track_caller]
    pub fn be(self, other: T) -> Self {
        assert!(
            self.value == other,
            "Expected value to be {}, but got {}",
            other,
            self.value
        );
        self
    }

    #[track_caller]
    pub fn not_be(self, other: T) -> Self {
        assert!(
            self.value != other,
            "Expected value to not be {}, but got {}",
            other,
            self.value
        );
        self
    }
}
