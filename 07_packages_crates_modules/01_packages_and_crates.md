A crate is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate. 
A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

Several rules determine what a package can contain. 
- A package must contain zero or one library crates, and no more. 
- It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. 
Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. 
Cargo passes the crate root files to rustc to build the library or binary.

If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package. 
A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
