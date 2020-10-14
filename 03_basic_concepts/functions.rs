// Rust does not care where functions are defined (before or after function calls)
// Further, the last expression in a Rust function gets returned even without a return
fn five() -> i32 {
    5 // Returns without explicit return
}
// Note that statements ARE NOT followed by a semicolon

fn plus_one(number: i32) -> i32 {
    number + 1 // If we added a semicolon, this wouldn't work as expected
}

fn factorial(number: i32) -> i32 {
    match number {
        0 => 1,
        _ => number * factorial(number - 1)
    }
}

fn main() {
    let f = five();
    println!("f is: {}", f);

    let f_plus_one = plus_one(f);
    println!("f_plus_one is: {}", f_plus_one);

    let f_factorial = factorial(f_plus_one);
    println!("f_factorial is {}", f_factorial);
}
