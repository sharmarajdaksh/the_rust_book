# Useful documentation comments

Rust has a particular kind of comments for documentation, called `documentation comments`, that generate html documentation for the public API items.
These are meant to explain to developers how to _use_ your crate, as opposed to how the crate is _implemented_.

Documentation comments are made using `///` are support Markown.
These are expected to be placed right before the item they document.

````rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

We can generate the HTML documentation from this documentation comment by running `cargo doc`.
This command runs the `rustdoc` tool distributed with Rust and puts the generated HTML documentation in the `target/doc` directory.
For convenience, running `cargo doc --open` will build the HTML for your current crate’s documentation (as well as the documentation for all of your crate’s dependencies) and open the result in a web browser.

## Common documentation sections

- **Examples**: For examples
- **Panics**: Scenarios where the function panics.
- **Errors**: If the function returns a `Result`, describe the kind of errors that might occur and what conditions might cause those errors.
- **Safety**: If the function is `unsafe` to call, there should be a section explaining why the function is unsafe and covering invariants that the function expects callers to uphold.

## Documentation comments as tests

`cargo test` will also run code examples in your documentation as tests. For example, these comments in the above documentation for add_one will get executed by `cargo test`

````rust
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
````

## Commenting contained items

Another style of doc comment, `//!`, adds documentation to the item that contains the comments rather than adding documentation to the items following the comments.
We typically use these doc comments inside the crate root file (src/lib.rs by convention) or inside a module to document the crate or the module as a whole.

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```

These comments are assumed to describe the entire crate.

## Exporting a Convenient Public API with pub use

The structure of your public API is a major consideration when publishing a crate.
People who use your crate are less familiar with the structure than you are and might have difficulty finding the pieces they want to use if your crate has a large module hierarchy.

If the structure isn’t convenient for others to use from another library, you don’t have to rearrange your internal organization: instead, you can re-export items to make a public structure that’s different from your private structure by using pub use.
Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.

```rust
// Example re-exports for a package
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

In cases where there are many nested modules, re-exporting the types at the top level with pub use can make a significant difference in the experience of people who use the crate.

Choosing `pub use` gives you flexibility in how you structure your crate internally and decouples that internal structure from what you present to your users.

# Setting up a Crates.io account

Once you create an account on crates.io, you can retrieve your API key from your account settings. Using this API key, you can login using `cargo login <API KEY>`.

# Adding metadata to a new crate

Before publishing, you’ll need to add some metadata to your crate by adding it to the [package] section of Cargo.toml.

Your crate will need a unique name. While you’re working on a crate locally, you can name a crate whatever you’d like. However, crate names on crates.io are allocated on a first-come, first-served basis.

A description and license are required so people will know what your crate does and under what terms they can use it. Add a description that is just a sentence or two, because it will appear with your crate in search results. For the license field, you need to give a license identifier value. The Linux Foundation’s Software Package Data Exchange (SPDX) lists the identifiers you can use for this value.
If you want to use a license that doesn’t appear in the SPDX, you need to place the text of that license in a file, include the file in your project, and then use license-file to specify the name of that file instead of using the license key.
Many people in the Rust community license their projects in the same way as Rust by using a dual license of `MIT OR Apache-2.0`. This practice demonstrates that you can also specify multiple license identifiers separated by OR to have multiple licenses for your project.

A sample [package] section for a ready-to publish crate:

```rust
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

# Publishing to crates.io

Be careful when publishing a crate because a publish is permanent.
The version can never be overwritten, and the code cannot be deleted.
One major goal of crates.io is to act as a permanent archive of code so that builds of all projects that depend on crates from crates.io will continue to work.
Allowing version deletions would make fulfilling that goal impossible.
However, there is no limit to the number of crate versions you can publish.

Running `cargo publish` once you have logged in with `cargo login` will publish the package to crates.io.

## Publishing versions of an existing crate

When you’ve made changes to your crate and are ready to release a new version, you change the version value specified in your `Cargo.toml` file and republish.
Use the Semantic Versioning rules to decide what an appropriate next version number is based on the kinds of changes you’ve made.
Then run cargo publish to upload the new version.

## Removing versions from crates.io with `cargo yank`

Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new dependency.
This is useful when a crate version is broken for one reason or another.
In such situations, Cargo supports yanking a crate version.

Yanking a version prevents new projects from starting to depend on that version while allowing all existing projects that depend on it to continue to download and depend on that version.
Essentially, a yank means that all projects with a Cargo.lock will not break, and any future Cargo.lock files generated will not use the yanked version.

```bash
$ cargo yank --vers 1.0.1
```

Yanks can also be undone using --undo

```bash
$ cargo yank --vers 1.0.1 --undo
```

A yank does not delete any code, so mistakes like uploading secrets can't really be corrected using yank.
