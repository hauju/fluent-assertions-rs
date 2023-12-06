
use self::string_assertion::StringAssertion;
use self::{bool_assertion::BoolAssertion, numeric_assertion::NumericAssertion};

use std::cmp::PartialOrd;
use std::fmt::Display;

pub mod bool_assertion;
pub mod string_assertion;
pub mod numeric_assertion;
pub mod option_assertion;
pub mod result_assertion;


pub trait ShouldBool {
    type Assertion;
    fn should(self) -> Self::Assertion;
}

impl ShouldBool for bool {
    type Assertion = BoolAssertion;

    fn should(self) -> BoolAssertion {
        BoolAssertion::new(self)
    }
}

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

pub trait ShouldOption<T> {
    type Assertion;
    fn should(self) -> Self::Assertion;
}

impl<T> ShouldOption<T> for Option<T> {
    type Assertion = option_assertion::OptionAssertion<T>;

    fn should(self) -> option_assertion::OptionAssertion<T> {
        option_assertion::OptionAssertion::new(self)
    }
}

pub trait ShouldResult<T, E> {
    type Assertion;
    fn should(self) -> Self::Assertion;
}

impl<T, E> ShouldResult<T, E> for Result<T, E> {
    type Assertion = result_assertion::ResultAssertion<T, E>;

    fn should(self) -> result_assertion::ResultAssertion<T, E> {
        result_assertion::ResultAssertion::new(self)
    }
}

pub trait ShouldString {
    type Assertion;
    fn should(self) -> Self::Assertion;
}

impl<T: AsRef<str>> ShouldString for T {
    type Assertion = StringAssertion<T>;

    fn should(self) -> StringAssertion<T>{
        StringAssertion::new(self)
    }
}


