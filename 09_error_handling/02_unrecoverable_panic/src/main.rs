// When the panic! macro executes, your program will print a failure message,
// unwind and clean up the stack, and then quit.

// By default, when a panic occurs, the program starts unwinding, which means
// Rust walks back up the stack and cleans up the data from each function it
// encounters. But this walking back and cleanup is a lot of work. The
// alternative is to immediately abort, which ends the program without cleaning
// up. Memory that the program was using will then need to be cleaned up by the
// operating system. If in your project you need to make the resulting binary
// as small as possible, you can switch from unwinding to aborting upon a panic
// by adding panic = 'abort' to the appropriate [profile] sections in your
// Cargo.toml file. For example, if you want to abort on panic in release mode, // add this:
//
// [profile.release]
// panic = 'abort'

fn main() {
    panic!("crash and burn");
    // By default, only prints the actual line where panic was called
    // That is, it does not print the backtrace by default

    // To see complete backtrace (for example, if the panic is occurring in
    // some external library), set the RUST_BACKTRACE env variable to 1
    //
    // RUST_BACKTRACE=1 cargo run
}
