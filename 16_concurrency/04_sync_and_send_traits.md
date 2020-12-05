# Extensible concurrency with Sync and Send traits

Rust has _very_ few concurrency features. Most features are part of the standard library
rather than the language itself. However, for handling concurrency, you can write your
own concurrency features or use libraries.
Two concepts, however, are embedded in the language: the `std::marker` traits `Sync` and `Send`.

## Allowing transference of ownership between threads with Send

The `Send` marker trait indicates that ownership of the type implementing Send can be transferred between threads.
Almost all Rust types are `Send`, with `Rc<T>` being one of the exceptions.
`Rc<T>`, when cloned, might have the reference count updated by two threads
at once, which could potentially lead to problems.
Any type composed entirely of `Send` types is automatically marked as `Send` as well.
Almost all primitive types are `Send`, aside from raw pointers.

## Allowing access from multiple threads with Sync

The `Sync` marker incidcates that a type can be referenced from multiple threads.
Thus, any type `T` is `Sync` if `&T` is Send, meaning that the reference can
be sent safely to another thread.
Like `Send`, primitive types are also `Sync` and all types composed exclusively of
types implementing `Sync` are also `Sync`.
`Rc<T>` is also not `Sync` for the same reason that it is not `Send`.
The `Refcell<T>` type and the family of related `Cell<T>` types are not `Sync`.
The implementation of borrow checkin gdone by `Refcell<T>` at runtime is
not thread-safe.
`Mutex<T>` is `Sync` and can be used to share access with multiple threads.

## Implementing Send and Sync manually is unsafe

Because types made up of `Send` and `Sync` are automatically also `Send`
and `Sync`, these don't need to be implemented manually.
As marker traits, these don't even have any methods to implement.
They're only useful for enforcing invariants related to concurrency.

Manually implementing these involves implementing unsafe Rust code.
It required careful thought to uphold safety guarantees.
