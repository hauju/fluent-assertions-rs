use self::bool_assertion::BoolAssertion;
use crate::assertions::string_assertion::StringAssertion;


pub mod bool_assertion;
pub mod string_assertion;
pub mod numeric_assertion;


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

