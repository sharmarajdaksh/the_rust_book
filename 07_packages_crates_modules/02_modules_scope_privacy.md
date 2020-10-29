Modules let us organize code within a crate into groups for readability and easy reuse. 
Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

You could create a new library using `cargo new --lib restaurant`, generating a `src/lib.rs`.
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

`src/main.rs` and `src/lib.rs` are called crate roots. 
The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.

Example module tree for a project having the above `restaurant` module:
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```


