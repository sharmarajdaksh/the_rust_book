fn main() {
    // Rust variables are mutable at default
    let x = 5;
    // x = 6; // INVALID

    // Shadowing
    let x = x + 1;
    // Shadowing is also used for type conversions

    let mut x = 9; // You CAN shadow an immutable variable with a mutable variable
    x = 10; // VALID
    println!("x is now {}", x);

    // Consts, unlike variables, can never be mutable (duh)
    // Data type MUST be annotated
    const MAX_POINTS:u32 = 100_000;
    // Constants must be set to only a constant expression
    // They CANNOT be assigned the result of a function call or any other value that must be
    // computed during runtime
}