// We can define a vector that takes a trait object.
// A trait object points to both an instance of a type implementing our trait
// as well as a table used to look up trait methods on that type at runtime
//
// Trait objects are created by specifying some sort of pointer, such as a &
// reference, or a Box<T> smart pointer and then the dyn keyword, and then
// specifying the relevant trait
// Trait objects can be used in place of generic or concrete types.
//
// Trait objects are a lot like objects, but they can't have data
//

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
// This works differently from defining a struct that uses a generic type
// parameter with trait bounds. A generic type parameter can only be
// substituted with one concrete type at a time, whereas trait objects allow
// for multiple concrete types to fill in for the trait object at runtime.

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button");
    }
}

// Trait objects perform dynamic dispatch
//
// When we use trait bounds on generics, the compiler generates nongeneric
// implementations of functions and methods for each concrete type that we
// use in place of a generic type parameter.
// The code that results from monomorphization is doing static dispatch, which
// is when the compiler knows what method you’re calling at compile time.
// This is opposed to dynamic dispatch, which is when the compiler can’t tell
// at compile time which method you’re calling.
// In dynamic dispatch cases, the compiler emits code that at runtime will
// figure out which method to call.
// When we use trait objects, Rust must use dynamic dispatch.
// The compiler doesn’t know all the types that might be used with the code
// that is using trait objects, so it doesn’t know which method implemented on
// which type to call.
// Instead, at runtime, Rust uses the pointers inside the trait object
// to know which method to call.
// There is a runtime cost when this lookup happens that doesn’t occur with
// static dispatch. Dynamic dispatch also prevents the compiler from choosing
// to inline a method’s code, which in turn prevents some optimizations.
//

//
// Object safety is required for Trait objects
//
// Only object-safe traits can be made into trait objects. A trait is object-
// safe if all methods defined in the trait have the two properties:
// - Return type isn't `self`
// - There are no generic type parameters
// Trait objects must be object safe because once you’ve used a trait object,
// Rust no longer knows the concrete type that’s implementing that trait.
// If a trait method returns the concrete Self type, but a trait object forgets
// the exact type that Self is, there is no way the method can use the original
// concrete type.
// The same is true of generic type parameters that are filled in with concrete
// type parameters when the trait is used: the concrete types become part of
// the type that implements the trait.
// When the type is forgotten through the use of a trait object, there is
// no way to know what types to fill in the generic type parameters with.
// The Clone trait, for example, is not object safe.
// pub trait Clone {
//    fn clone(&self) -> Self;
// }
