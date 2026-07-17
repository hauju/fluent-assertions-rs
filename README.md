# fluent-assertions

[![crates.io](https://img.shields.io/crates/v/fluent-assertions.svg)](https://crates.io/crates/fluent-assertions)
[![docs.rs](https://docs.rs/fluent-assertions/badge.svg)](https://docs.rs/fluent-assertions)
[![CI](https://github.com/hauju/fluent-assertions-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/hauju/fluent-assertions-rs/actions/workflows/ci.yml)

## Introduction

A fluent assertions framework for Rust, inspired by the renowned Fluent Assertions framework in .NET. This powerful library aims to make your Rust test assertions more expressive, readable, and maintainable by enabling a natural language-like syntax for your test cases.

fluent-assertions provides an extensive set of assertion methods that are both easy to use and understand. These methods are designed to work seamlessly with Rust's core testing functionality, allowing you to write tests that are not only more expressive but also more enjoyable to read and write.

With fluent-assertions, you can chain assertions with ease, allowing for complex validations without sacrificing readability. This framework also comes with informative error messages, making it easier to debug and pinpoint the source of failures in your test suite.

Whether you're new to Rust or an experienced developer, fluent-assertions is designed to help you write cleaner, more readable tests that effectively communicate their intent. With this powerful library at your disposal, you'll be able to focus on what matters most: building reliable and efficient Rust applications.

## Usage

Add fluent-assertions as a dev-dependency:

```bash
cargo add --dev fluent-assertions
```

Bring the prelude into scope and call `.should()` on any supported value to start a chain of assertions:

```rust
use fluent_assertions::*;

// Strings — chained checks (have_length counts bytes, not chars)
"hello world".should().start_with("hello").contain("world").have_length(11);

// Cross-type equality: a String compared against a &str
"foo".to_string().should().be("foo");

// Numbers, ranges and floats (be_close_to tolerance is inclusive)
42.should().be_greater_than(40).be_positive();
5.should().be_in_range(1..=10);
3.14.should().be_close_to(3.1, 0.05);

// Booleans
true.should().be_true();

// Collections: Vec, arrays and slices
[1, 2, 3].should().contain(&2).have_length(3);

// Options and Results: be_some()/be_ok() unwrap, into_inner() returns the value
let five = Some(5).should().be_some().into_inner();
five.should().be(5);
Ok::<i32, String>(42).should().be_ok().into_inner().should().be(42);
None::<i32>.should().be_none();

// Errors: be_err() unwraps the error so you can keep asserting on it
let err: Result<(), std::io::Error> = Err(std::io::Error::other("boom"));
err.should().be_err().contain_message("boom");
```

Assertions panic on failure, just like the standard `assert!` macros, so a failing assertion fails the test.

## Examples

```bash
cargo run --example asserts
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.