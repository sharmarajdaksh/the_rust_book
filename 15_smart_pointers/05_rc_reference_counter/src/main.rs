// There are cases when a single value might have multiple owners.
// For example, in graph data structures, multiple edges might point to the
// same node, and that node is conceptually owned by all of the edges that
// point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges
// pointing to it.

// The referent counting smart pointer Rc<T> tracks the number of references
// to a value which determines whether or not the value is still in use.
// The value is cleaned up if there are zero references to it.

// We use the Rc<T> type when we want to allocate some data on the heap for
// multiple parts of our program to read and we can’t determine at compile time
// which part will finish using the data last. If we knew which part would
// finish last, we could just make that part the data’s owner, and the normal
// ownership rules enforced at compile time would take effect.

// Note that Rc<T> is only for use in single-threaded scenarios.

use std::rc::Rc;

enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));

    // Won't compile with Box<T> since both b and share share ownership of a
    // Instead of taking ownership of a, we’ll clone the Rc<List> that a is
    // holding, thereby increasing the number of references from one to two and
    // letting a and b share ownership of the data in that Rc<List>. We’ll also
    // clone a when creating c, increasing the number of references from two to
    // three.
    // Every time we call Rc::clone, the reference count to the data within the
    // Rc<List> will increase, and the data won’t be cleaned up unless there
    // are zero references to it.

    // Using Rc::clone instead of a.clone() here is convention
    // The implementation of Rc::clone doesn’t make a deep copy of all the data
    // like most types’ implementations of clone do.
    // The call to Rc::clone only increments the reference count, which doesn’t
    // take much time. Deep copies of data can take a lot of time. By using
    // Rc::clone for reference counting, we can visually distinguish between
    // the deep-copy kinds of clones and the kinds of clones that increase the
    // reference count.
    // When looking for performance problems in the code, we only need to
    // consider the deep-copy clones and can disregard calls to Rc::clone.
    println!("count after creating a = {}", Rc::strong_count(&a));
    // strong_count prints the reference count
    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// Via immutable references, Rc<T> allows you to share data between multiple
// parts of your program for reading only.
