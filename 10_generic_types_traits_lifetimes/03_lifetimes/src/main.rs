// Every reference in Rust has a `lifetime`, which is the scope for
// which the reference is valid.
// Like types, lifetimes are inferred most of the time but sometimes they
// might need to be annotated when the lifetimes of references could be related
// to in multiple ways.
// Rust requires us to annotate relationships using generic lifetime parameters
// to ensure the actual reference useed at runtime will definitely be valid

// The main aim of lifetimes is to prevent dangling references, which cause a
// program to reference data other than the data it’s intended to reference.

// The Rust compiler has a borrow checker that compares scopes to determine
// whether all borrows are valid.

// This code won't compile
// {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+
// At compile time, Rust compares the size of the two lifetimes and sees that r
// has a lifetime of 'a but that it refers to memory with a lifetime of 'b. The
// program is rejected because 'b is shorter than 'a: the subject of the
// reference doesn’t live as long as the reference.

// Declaring references
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Lifetime annotations don’t change how long any of the references live.
// Just as functions can accept any type when the signature specifies a generic
// type parameter, functions can accept references with any lifetime by
// specifying a generic lifetime parameter.
// Lifetime annotations describe the relationships of the lifetimes of multiple
// references to each other without affecting the lifetimes.

// When we’re defining this function, we don’t know the concrete values that
// will be passed into this function, so we don’t know whether the if case or
// the else case will execute. We also don’t know the concrete lifetimes of the
// references that will be passed in, so we can’t look at the scopes to
// determine whether the returned reference will always be valid
// The borrow checker can’t determine this either, because it doesn’t know how
// the lifetimes of x and y relate to the lifetime of the return value. To fix
// this error, we’ll add generic lifetime parameters that define the
// relationship between the references so the borrow checker can perform its
// analysis.
// This function is invalid
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// It’s possible for structs to hold references, but in that case we would need
// to add a lifetime annotation on every reference in the struct’s definition.
struct ImportantExcerpt<'a> {
    // an instance of ImportantExcerpt can’t outlive the reference it holds in
    // its part field
    part: &'a str,
}

// When we implement methods on a struct with lifetimes, we use the same syntax
// as that of generic type parameters
// Lifetime names for struct fields always need to be declared after the impl
// keyword and then used after the struct’s name, because those lifetimes are
// part of the struct’s type.

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        // lifetime annotations not needed due to elision
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Some functions can compile even without lifetime annotations
// This is because certain patterns are programmed into Rust's compiler code
// so the borrow checker could infer these without explicit lifetime annotations
// These patterns are called lifetime elisions
// Still, elision does not provide full inference and in case of any ambiguity
// the compiler will raise an error

// The compiler uses three rules to figure out what lifetimes references have
// when there aren’t explicit annotations. The first rule applies to input
// lifetimes, and the second and third rules apply to output lifetimes. If the
// compiler gets to the end of the three rules and there are still references
// for which it can’t figure out lifetimes, the compiler will stop with an
// error. These rules apply to fn definitions as well as impl blocks.

// The first rule is that each parameter that is a reference gets its own
// lifetime parameter. In other words, a function with one parameter gets one
// lifetime parameter:
// fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters:
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32; and so on.

// The second rule is if there is exactly one input lifetime parameter, that
// lifetime is assigned to all output lifetime parameters:
// fn foo<'a>(x: &'a i32) -> &'a i32.

// The third rule is if there are multiple input lifetime parameters, but one
// of them is &self or &mut self because this is a method, the lifetime of self
// is assigned to all output lifetime parameters. This third rule makes methods
// much nicer to read and write because fewer symbols are necessary.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// input lifetimes: lifetimes on function or method parameters
// output lifetimes: lifetimes on return values

fn main() {
    // Valid call to longest
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Invalid call to longest
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // One special lifetime is 'static, which means that this reference can
    // live for the entire duration of the program. All string literals have
    // the 'static lifetime, which we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the program’s binary,
    // which is always available. Therefore, the lifetime of all string
    // literals is 'static.
}

// Using generic type parameters, trait bounds, and lifetimes together:

use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
