Workspaces can help manage multiple related packages that are developed in tandem.
A workspace is a set of packages that share the same Cargo.lock and output directory.

All packages in a workspace share a single target directory.

Packages in a workspace aren't assumed to depend on each other. Dependencies must be explicitly defined in Cargo.toml for the package (for example, in adder/Cargo.toml)

Running `cargo build` will build all packages in the workspace. Similarly, `cargo test` will run tests for all packages.

To run a specific package, use the `-p` flag. For example:

```sh
cargo run -p adder
```

To run tests for a particular crate, use `-p`:

```sh
cargo test -p add-one
```

The workspace has only one Cargo.lock file at the top level of the workspace rather than having a Cargo.lock in each crate’s directory.
This ensures that all crates are using the same version of all dependencies.
If we add the rand package to the adder/Cargo.toml and add-one/Cargo.toml files, Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock.
Making all crates in the workspace use the same dependencies means the crates in the workspace will always be compatible with each other.

If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately.
The cargo publish command does not have an `--all` flag or a `-p` flag, so you must change to each crate’s directory and run cargo publish on each crate in the workspace to publish the crates.
