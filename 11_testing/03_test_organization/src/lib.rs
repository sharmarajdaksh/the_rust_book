// The Rust community thinks about tests in terms of two main categories: unit
// tests and integration tests.
// Unit tests are small and more focused, testing one module in isolation at a
// time, and can test private interfaces.
// Integration tests are entirely external to your library and use your code in
// the same way any other external code would, using only the public interface
// and potentially exercising multiple modules per test.

// The purpose of unit tests is to test each unit of code in isolation from the
// rest of the code to quickly pinpoint where code is and isn’t working as
// expected. You’ll put unit tests in the src directory in each file with the
// code that they’re testing. The convention is to create a module named tests
// in each file to contain the test functions and to annotate the module with
// #[cfg(test)]
// It tells Rust to compile and run the test code only when you run cargo test,
// not when you run cargo build. This saves compile time when you only want to
// build the library and saves space in the resulting compiled artifact because
// the tests are not included.
// The attribute cfg stands for configuration and tells Rust that the following
// item should only be included given a certain configuration option. In this
// case, the configuration option is test, which is provided by Rust for
// compiling and running tests. By using the cfg attribute, Cargo compiles our
// test code only if we actively run the tests with cargo test. This includes
// any helper functions that might be within this module, in addition to the
// functions annotated with #[test].

// In Rust, integration tests are entirely external to your library. They use
// your library in the same way any other code would, which means they can only
// call functions that are part of your library’s public API. Their purpose is
// to test whether many parts of your library work together correctly. Units of
// code that work correctly on their own could have problems when integrated,
// so test coverage of the integrated code is important as well. To create
// integration tests, you first need a tests directory.

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
