
pub mod bool_assertion;
pub mod string_assertion;
pub mod numeric_assertion;
pub mod option_assertion;
pub mod result_assertion;

pub trait Should {
    type Assertion;
    fn should(self) -> Self::Assertion;
}


