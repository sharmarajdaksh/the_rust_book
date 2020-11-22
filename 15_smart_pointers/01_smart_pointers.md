The most common type of pointer in Rust is a reference, indicated by `&`. References
borrow the value they point to. They have no overhead.

_Smart pointers_ are data structures that act like a pointer but also have additional metadata and capabilities.

- For example, the _reference counting_ smart pointer enables multiple owners of data by keeping track of the number of owners, and cleaning up data when no owners remain.

Another difference between references and smart pointers is that references are pointers that only borrow data;
smart pointers, in many cases, own the data they point to.

Two of the most common smart pointers are `String` and `Vec<T>`.
Both these count as smart pointers because they own some memory and allow manipulation. They also have metadata and extra capabilities or guarantees (like `String` ensuring that data will always be valid UTF-8).

Smart pointers are usually implemented using structs. The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the `Deref` and `Drop` traits.

- The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers.
- The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope

Some of the most common smart pointers in the standard library are:

- `Box<T>` for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

Another concept is _interior mutability_, a pattern where an immutable type exposes an API for mutation of interior value.
