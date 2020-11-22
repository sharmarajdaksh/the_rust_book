// Implementing the `Deref` trait allows customizing the behavior of the
// dereference operator *

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// The reason the deref method returns a reference to a value, and that the
// plain dereference outside the parentheses in *(y.deref()) is still
// necessary, is the ownership system.
// If the deref method returned the value directly instead of a reference to
// the value, the value would be moved out of self. We don’t want to take
// ownership of the inner value inside MyBox<T> in this case or in most cases
// where we use the dereference operator.

fn main() {
    let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x); // true
    assert_eq!(5, *y); // true
                       // Interpreted as: *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    // Defer coercion due to the Deref implementation on String
    hello(&(*m)[..]); // Same as
    hello(&m); // This
}

// Deref coercion is a convenience that Rust performs on arguments to functions
// and methods.
// Deref coercion works only on types that implement the Deref trait.
// Deref coercion converts such a type into a reference to another type.
// For example, deref coercion can convert &String to &str because String
// implements the Deref trait such that it returns str.
// Deref coercion happens automatically when we pass a reference to a
// particular type’s value as an argument to a function or method that doesn’t
// match the parameter type in the function or method definition. A sequence of
// calls to the deref method converts the type we provided into the type the
// parameter needs.

// Deref coercion was added to Rust so that programmers writing function and
// method calls don’t need to add as many explicit references and dereferences ]// with & and *.

// When the Deref trait is defined for the types involved, Rust will analyze
// the types and use Deref::deref as many times as necessary to get a reference
// to match the parameter’s type. The number of times that Deref::deref needs
// to be inserted is resolved at compile time, so there is no runtime penalty
// for taking advantage of deref coercion!

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Deref Coercion and Mutability
//
// Similar to Deref for immutable references, DerefMut overrides the * operator
// for mutable references

// Rust does deref coercion when it finds types and trait implementations in
// the following three cases:
// - From &T to &U where T: Deref<Target=U>
// - From &mut T to &mut U when T: DerefMut<Target=U>
// - From &mut T to &U when T: Deref<Target=U>
//
// Because of the borrowing rules, if you have a mutable reference, that
// mutable reference must be the only reference to that data (otherwise, the
// program wouldn’t compile). Converting one mutable reference to one immutable
// reference will never break the borrowing rules
