use std::io;
// By default, Rust brings only a few types into the scope of every program in the prelude.
// If a type you want to use isn’t in the prelude, you have to bring that type into scope
// explicitly with a use statement.
// Using the std::io library provides you with a number of useful features,
// including the ability to accept user input.

use std::cmp::Ordering; // An enum, like the Result type

use rand::Rng;
// The Rng trait defines methods that random number generators implement,
// and this trait must be in scope for us to use those methods.

fn main() {
    println!("#####################");
    println!("# Guess the number! #");
    println!("#####################");
    println!();

    // Rust defaults to i32 by default
    // One of the many integer types
    let secret_number = rand::thread_rng().gen_range(1, 101); // Immutable

    loop { // An infinite loop

        println!("Please input your guess.");

        // Variables in Rust are declared with `let`

        // Rust variables are immutable by default.
        // Using `mut` before a variable name explicitly marks it as mutable

        // `String::new()` returns a new instance of String
        // :: indicates that `new` is an associated function of the String type
        // Rust refers to static functions as `associated` functions

        let mut guess = String::new(); // A mutable variable

        // We can use `io` instead of `std::io` owing to the `use` declaration
        io::stdin() // std::io::stdin() returns an instance of std::io::Stdin

            // read_line() method on the Stdin object
            // Takes input on stdinput and places it into the
            // String `guess`
            // The string here must be mutable for obvious reasons
            // Reminiscent of Go writing to a ioWriter object
            .read_line(&mut guess) // The & indicates a reference
            // References, like variables, are immutable by default
            // Hence the need for the &mut
            // read_line returns the number of bytes written to passed string

            // read_line() returns an io::Result object
            // All Result objects are enums
            // Expect is an error handling call
            // If a Result object contains an Err, the program crashes with the specified message
            .expect("Failed to read line");
        // If expect is not called, the program would compile with warnings

        println!("You guessed: {}", guess.trim());

        // Rust allows you to `shadow` the previous value of `guess` with a new one
        // Often used for type conversion
        // Trim whitespace
        // Parse to u32 (inferred from type of `guess`
        // Error handing example in Rust:
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // The _ is a catchall value
        };

        // A `match` expression is made up of `arms`
        // Sort of like a switch statement
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
