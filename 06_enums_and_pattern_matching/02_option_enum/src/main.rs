// The Option type is used in many places because it encodes the very common
// scenario in which a value could be something or it could be nothing.
// Expressing this concept in terms of the type system means the compiler can
// check whether you’ve handled all the cases you should be handling;
// this functionality can prevent bugs that are extremely common in other
// programming languages.

// Rust doesn’t have the null feature that many other languages have.
// Null is a value that means there is no value there. In languages with
// null, variables can always be in one of two states: null or not-null.

// As such, Rust does not have nulls, but it does have an enum that can encode
// the concept of a value being present or absent. This enum is Option<T>
// enum Option<T> {
//     Some(T),
//     None,
// }
// Part of the prelude; Does not have to be imported
// In addition, so are its variants: you can use Some and None directly without
// the Option:: prefix. The Option<T> enum is still just a regular enum, and
//Some(T) and None are still variants of type Option<T>.
// <T> here is a generic type

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // Using None mandates a necessary
                                           // type specification for Option<T>

    // Note that Option<i32> is a different type and won't be compatible with
    // numeric types

    // In general, in order to use an Option<T> value, you want to have code
    // that will handle each variant. You want some code that will run only
    // when you have a Some(T) value, and this code is allowed to use the inner
    // T. You want some other code to run if you have a None value, and that
    // code doesn’t have a T value available. The match expression is a control
    // flow construct that does just this when used with enums: it will run
    // different code depending on which variant of the enum it has, and that
    // code can use the data inside the matching value.
}
