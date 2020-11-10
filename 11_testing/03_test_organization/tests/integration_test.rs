use test_organization;

mod common;

// We create a tests directory at the top level of our project directory, next
// to src. Cargo knows to look for integration test files in this directory. We
// can then make as many test files as we want to in this directory, and Cargo
// will compile each of the files as an individual crate.

// We don’t need to annotate any code in tests/integration_test.rs with
// #[cfg(test)]. Cargo treats the tests directory specially and compiles files
// in this directory only when we run cargo test.

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, test_organization::add_two(2));
}

// Treating each integration test file as its own crate is useful to create
// separate scopes that are more like the way end users will be using your
// crate. However, this means files in the tests directory don’t share the same
// behavior as files in src do

// Convention for utility functions that are used accross files is to have a
// tests/common/mod.rs. Through such naming convention, rust knows not to treat
// the common module as an integration test file.

// If our project is a binary crate that only contains a src/main.rs file and
// doesn’t have a src/lib.rs file, we can’t create integration tests in the
// tests directory and bring functions defined in the src/main.rs file into
// scope with a use statement. Only library crates expose functions that other
// crates can use; binary crates are meant to be run on their own.

// This is one of the reasons Rust projects that provide a binary have a
// straightforward src/main.rs file that calls logic that lives in the src/lib.
// rs file. Using that structure, integration tests can test the library crate
// with use to make the important functionality available. If the important
// functionality works, the small amount of code in the src/main.rs file will
// work as well, and that small amount of code doesn’t need to be tested.
