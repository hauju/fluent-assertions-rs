
use fluent_assertions::*;

fn main() {
    let message = "hello world";
    message.should().start_with("hello");

    let message = "hello world";
    message.should().be("hello world");

    let is_verified = true;
    is_verified.should().be_true();

    let anwser: i32 = 42;
    anwser.should().be_greater_than(40);

    let anwser: f64 = 42.0;
    anwser.should().be_greater_than(40.0);

    let result: Result<f64, String> = Ok(42f64);
    result.should().be_ok();
}