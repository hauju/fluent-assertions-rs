use fluent_assertions::*;

#[test]
fn readme_usage_example() {
    // Strings
    "hello world"
        .should()
        .start_with("hello")
        .contain("world")
        .have_length(11);

    // Numbers
    42.should().be_greater_than(40).be_positive();

    // Booleans
    true.should().be_true();

    // Options
    Some(5).should().be_some();
    None::<i32>.should().be_none();

    // Results
    let ok: Result<i32, String> = Ok(42);
    ok.should().be_ok();

    let err: Result<(), std::io::Error> = Err(std::io::Error::other("boom"));
    err.should().be_err().contain_message("boom");
}
