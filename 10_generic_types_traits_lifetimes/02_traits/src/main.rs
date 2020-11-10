use std::fmt::Debug;
use std::fmt::Display;

// A trait tells the Rust compiler about functionality a particular type has
// and can share with other types. We can use traits to define shared behavior
// in an abstract way. We can use trait bounds to specify that a generic can be
// any type that has certain behavior.
// Traits are Rust's version of interfaces, with some differences

// A type’s behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the same methods on
// all of those types. Trait definitions are a way to group method signatures
// together to define a set of behaviors necessary to accomplish some purpose.

pub trait Summary {
    // method signatures for types that implement this trait
    fn summarize(&self) -> String;

    // Default implementations can also be specified
    fn readmore(&self) -> String {
        String::from("Read more...")
        // Default implementations can call other methods in the same trait,
        // even if those other methods don’t have a default implementation. In
        // this way, a trait can provide a lot of useful functionality and only
        // require implementors to specify a small part of it
        //
        // Note that it isn’t possible to call the default implementation from
        // an overriding implementation of that same method. (No super() calls)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement the Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    // Since readmore is not defined, the default implementations gets used
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Traits as parameters
// Instead of a concrete type, use the impl keyword and the trait name
pub fn notify_1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The impl Trait syntax works for straightforward cases but is actually syntax
// sugar for a longer form, which is called a trait bound;
// More convenient for more complex signatures
//it looks like this:
pub fn notify_2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// We can also specify more than one trait bound.
//
// pub fn notify(item: &(impl Summary + Display)) {
// OR
// pub fn notify<T: Summary + Display>(item: &T) {

// Rust has alternate syntax for specifying trait bounds inside a where clause
// after the function signature
// Allows for less cluttered signatures
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Something
}

// Return types that implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// However, you can only use impl Trait if you’re returning a single type. For
// example, this code that returns either a NewsArticle or a Tweet with the
// return type specified as impl Summary wouldn’t work:

//This code does not compile!
// Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions
// around how the impl Trait syntax is implemented in the compiler
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// One restriction to note with trait implementations is that we can implement
// a trait on a type only if either the trait or the type is local to our
// crate. For example, we can implement standard library traits like Display on
// a custom type like Tweet as part of our crate functionality,
// because the type Tweet is local to our crate. We can also
// implement Summary on Vec<T> in our crate, because the trait
// Summary is local to our crate.
//
// But we can’t implement external traits on external types. For example, we
// can’t implement the Display trait on Vec<T> within our crate,
// because Display and Vec<T> are defined in the standard library and aren’t
// local to our crate. This restriction is part of a property of
// programs called `coherence`, and more specifically the `orphan rule`, so
// named because the parent type is not present. This rule ensures that other
// people’s code can’t break your code and vice versa. Without the rule, two
// crates could implement the same trait for the same type, and Rust wouldn’t
// know which implementation to use.

// A function that finds the largest char OR int in an array
// Implemented using generics and traits
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // Passed arrays must be of types that implement
    // PartialOrd to support > comparisons
    // Copy to support the assignment `let mut largest = list[0];`

    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    &largest
}

// By using a trait bound with an impl block that uses generic type parameters,
// we can implement methods conditionally for types that implement the
// specified traits
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only for pairs with types that implement both Display and PartialOrd Traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements
// another trait. Implementations of a trait on any type that satisfies the
// trait bounds are called `blanket implementations` and are extensively used in
// the Rust standard library. For example, the standard library implements the
// ToString trait on any type that implements the Display trait. The impl block
// in the standard library looks similar to this code:
// impl<T: Display> ToString for T {
//     // --snip--
// }

// Traits and trait bounds let us write code that uses generic type parameters
// to reduce duplication but also specify to the compiler that we want the
// generic type to have particular behavior. The compiler can then use the
// trait bound information to check that all the concrete types used with our
// code provide the correct behavior. In dynamically typed languages, we would
// get an error at runtime if we called a method on a type which didn’t define
// the method. But Rust moves these errors to compile time so we’re forced to
// fix the problems before our code is even able to run. Additionally, we don’t
// have to write code that checks for behavior at runtime because we’ve already
// checked at compile time. Doing so improves performance without having to
// give up the flexibility of generics.

// Another kind of generic is called lifetimes.
// Rather than ensuring that a type has the behavior we want, lifetimes ensure
// that references are valid as long as we need them to be.
