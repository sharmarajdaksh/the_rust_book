// At its simplest, a test in Rust is a function that’s annotated with the test
// attribute. `Attributes` are metadata about pieces of code, such as the derive
// attribute used with structs

// #[test] specifies a function as a test function
// tests are run using `cargo test`

#[cfg(test)]
mod tests {
    use super::*; // Allows using super module functions/structs

    #[test] // Tells the test runner that this is a test
    fn exploration() {
        assert_eq!(2 + 2, 4); // macro to assert equal
    }

    // #[test]
    // fn another() {
    // Tests faile when something in the test function panics
    // Each test runs in a new thread. A dead test thread causes the
    // corresponding test to be marked as failed
    // panic!("Make this test fail"); // This will make the test fail
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller)); // Assert expects a boolean argument
                                            // Calls panic! if boolean is false
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2)); // Uses == internally
        assert_ne!(5, add_two(2)); // Uses != internally

        // When the assertions fail, these macros print their arguments using
        // debug formatting, which means the values being compared must
        // implement the PartialEq and Debug traits. All the primitive types
        // and most of the standard library types implement these traits. For
        // structs and enums that you define, you’ll need to implement
        // PartialEq to assert that values of those types are equal or not
        // equal. You’ll need to implement Debug to print the values when the
        // assertion fails.
        // Generally easily derivable using `#[derive(PartialEq, Debug)]`
    }

    // Custom failure messages as possible
    // All arguments passed to assert, assert_eq, and assert_ne, besides the
    // required arguments are passed to format!
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // In addition to checking that our code returns the correct values we
    // expect, it’s also important to check that our code handles error
    // conditions as we expect.
    // We do this by adding another attribute, should_panic, to our test
    // function. This attribute makes a test pass if the code inside the
    // function panics; the test will fail if the code inside the function
    // doesn’t panic.
    // To make should_panic tests more precise, we can add an optional expected
    // parameter to the should_panic attribute. The test harness will make sure
    // that the failure message contains the provided text.
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // We can also write tests that use Result<T, E>
    #[test]
    fn it_works() -> Result<(), String> {
        // The it_works function now has a return type, Result<(), String>. In
        // the body of the function, rather than calling the assert_eq! macro,
        // we return Ok(()) when the test passes and an Err with a String
        // inside when the test fails.
        // enables you to use the question mark operator
        // Can't use #[should_panic] on such tests. Instead, Err should be
        // directly returned in case that the test fails

        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
