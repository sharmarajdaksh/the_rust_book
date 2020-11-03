// The Result enum is defined as having two variants, Ok and Err, as follows:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");
    // Return type: Result<File, Error>
    // In the case where File::open succeeds, the value in the variable f will
    // be an instance of Ok that contains a file handle. In the case where it
    // fails, the value in f will be an instance of Err that contains more
    // information about the kind of error that happened.

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Matching for particular error kinds, such as FileNotFound:
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // Create file if NotFound
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // A better way to do the same as above, which I can't understand right now:
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect

    // If the Result value is the Ok variant, unwrap will return the value
    // inside the Ok. If the Result is the Err variant, unwrap will call the
    // panic! macro for us.

    // Will call panic if code errors
    let f = File::open("hello.txt").unwrap();

    // expect, which is similar to unwrap, lets us also choose the panic! error
    // message. Using expect instead of unwrap and providing good error
    // messages can convey your intent and make tracking down the source of a
    // panic easier.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Naive/Simple approach to propagating errors
fn read_username_from_file_basic() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Concise approach using the ? operator
// The ? placed after a Result value is defined to work in almost the same way
// as the match expressions we defined to handle the Result values
//
// There is a difference between what the match expression does and what the ?
// operator does: error values that have the ? operator called on them go
// through the from function, defined in the From trait in the standard
// library, which is used to convert errors from one type into another. When
// the ? operator calls the from function, the error type received is converted
// into the error type defined in the return type of the current function. This
// is useful when a function returns one error type to represent all the ways a
// function might fail, even if parts might fail for many different reasons. As
// long as each error type implements the from function to define how to
// convert itself to the returned error type, the ? operator takes care of the
// conversion automatically.
fn read_username_from_file_concise() -> Result<String, io::Error> {
    // Nice
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// The ? operator eliminates a lot of boilerplate and makes this function’s
// implementation simpler. We could even shorten this code further by chaining
// method calls immediately after the ?
fn read_username_from_file_chained() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    // Reading a file into a string is a fairly common operation, so Rust
    // provides the convenient fs::read_to_string function that opens the file,
    // creates a new String, reads the contents of the file, puts the contents
    // into that String, and returns it.
    fs::read_to_string("hello.txt")
}

// The ? operator can be used in functions that have a return type of Result

// we’re only allowed to use the ? operator in a function that returns Result
// or Option or another type that implements std::ops::Try. When you’re writing
// code in a function that doesn’t return one of these types, and you want to
// use ? when you call other functions that return Result<T, E>, you have two
// choices to fix this problem. One technique is to change the return type of
// your function to be Result<T, E> if you have no restrictions preventing
// that. The other technique is to use a match or one of the Result<T, E>
// methods to handle the Result<T, E> in whatever way is appropriate.

// The main function is special, and there are restrictions on what its return
// type must be. One valid return type for main is (), and conveniently,
// another valid return type is Result<T, E>, as shown here:
//
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//
//     Ok(())
// }
// Box<dyn Error> type is called a trait object.
// For now, read Box<dyn Error> as "any kind of error"
