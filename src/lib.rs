
pub mod assertions;
pub use assertions::*;


#[cfg(test)]
mod tests {
    use crate::assertions::{ShouldString, ShouldBool};

    #[test]
    fn test_str_assertions() {
        let actual = "ABCDEFGHI";
        actual
            .should()
            .start_with("AB")
            .end_with("HI")
            .contain("EF")
            .have_length(9);
    }

    #[test]
    fn test_string_assertions() {
        let actual_string = "ABCDEFGHI".to_string();
        actual_string
            .should()
            .start_with("AB")
            .end_with("HI")
            .contain("EF")
            .have_length(9);
    }

    #[test]
    fn test_bool_assertions() {
        let the_boolean = false;
        the_boolean.should().be_false();
    }
}
