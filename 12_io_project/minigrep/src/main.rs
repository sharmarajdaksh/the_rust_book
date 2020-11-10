use std::env;
use std::process;

use minigrep::Config;

// Nice documentation on project structuring for binary projects:
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects

fn main() {
    // std::env::args() returns an iterator
    // collect turns the iterator to a vector
    let args: Vec<String> = env::args().collect();

    // Using unwrap_or_else allows us to define some custom, non-panic! error
    // handling. If the Result is an Ok value, this method’s behavior is
    // similar to unwrap: it returns the inner value Ok is wrapping. However,
    // if the value is an Err value, this method calls the code in the closure,
    // which is an anonymous function we define and pass as an argument to
    // unwrap_or_else
    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintln writes to stderr
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
