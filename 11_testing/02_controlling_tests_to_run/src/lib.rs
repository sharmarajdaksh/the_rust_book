// By default `cargo test` runs all tests in parallel, and does not print all
// test output to keep the output tidy

// Some command line options go to cargo test, and some go to the resulting
// test binary. To separate these two types of arguments, you list the
// arguments that go to cargo test followed by the separator -- and then the
// ones that go to the test binary. Running `cargo test --help` displays the
// options you can use with cargo test, and running `cargo test -- --help`
// displays the options you can use after the separator --.

// To prevent tests from running in parallel, or to control the amount of
// parallel threads, you can use something like:
// $ cargo test -- --test-threads=1

// By default, if a test passes, Rust does not print anything (such as calls to
// println!). Only in case of failure will these tests get printed.
// To show all output, you can use
// $ cargo test -- --show-output

// To run a single test, simply pass the name of the test
// $ cargo test one_hundred

// To run multiple tests that contain a string, you can run
// $ cargo test add
// This will run both add_two_and_two and add_three_and_two

// Sometimes a few specific tests can be very time-consuming to execute, so you
// might want to exclude them during most runs of cargo test. Rather than
// listing as arguments all tests you do want to run, you can instead annotate
// the time-consuming tests using the ignore attribute to exclude them
//
// #[test]
// #[ignore]
// fn expensive_test() {
//     // code that takes an hour to run
// }
//
// Now, `cargo test` will not run this test
// To run only the ignored tests, run:
// $ cargo test -- --ignored
//
// To run all tests, including ignored tests:
// $ cargo test -- --include-ignored
//

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
