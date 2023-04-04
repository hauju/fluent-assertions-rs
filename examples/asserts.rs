
use fluent_assertions::{ShouldString, ShouldBool};

fn main() {
    let message = "hello world";
    message.should().start_with("hello");

    let is_verified = true;
    is_verified.should().be_false();
}