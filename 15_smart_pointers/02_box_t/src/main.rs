// Boxes (Box<T>) allow data storage on the heap. That is basically all they do
// Common situations for using Box<T>
// - When you have a type whose size is unknown at compile time and you want
//   to use the value in a context requiring an exact size
// - When you want to transfer ownership of a large amount of data without
//   having to copy data
// - When you want to copy a value and care only that it's a type implementing
//   a particular trait rather than having a specific type

// At compile time, Rust needs to know how much space a type takes up.
// One type whose size can’t be known at compile time is a recursive type,
// where a value can have as part of itself another value of the same type.
// Because this nesting of values could theoretically continue infinitely, Rust
// doesn’t know how much space a value of a recursive type needs.
// However, boxes have a known size, so by inserting a box in a recursive type
// definition, you can have recursive types.

// A cons list is a data structure
// Each item in a cons list contains two elements: the value of the current
// item and the next item. The last item contains only Nil, without a next item.
// It is constructed by recursively calling the cons function.

// This type has, essentially, an infinite size
enum List {
    // Using Box<T> makes the size known: it's the size of a pointer
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // Store data on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    // Using a cons type
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

// Boxes provide indirection and heap allocation, but don't have any other
// special capabilities.
// Neither do they have any performance overhead
// Box<T> implements the `Deref` trait, which allows Box<T> values to be
// treated like references. When a Box<T> foes out of scope, the heap data
// it points to gets cleaned up due to the `Drop` trait implementation.
