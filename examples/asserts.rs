
use fluent_assertions::*;

fn main() {
    let message = "hello world";
    message.should().start_with("hello");

    let message = "hello world";
    message.should().be("hello world");

    let is_verified = true;
    is_verified.should().be_true();

    let anwser = 42;
    anwser.should().be_greater_than(40);
}