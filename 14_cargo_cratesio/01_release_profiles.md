Release profiles are predefined and customizable profiles with different configurations
 Profiles control how the code is compiled.
Each profile is configured independently of the others

Cargo has two main profiles:
- `dev`, used when `cargo build` is run
- `release`, when `cargo build --release` is run

Cargo has default settings for each of the profiles that apply when there 
aren’t any `[profile.*]` sections in the project’s Cargo.toml file. By adding 
`[profile.*]` sections for any profile you want to customize, you can override 
any subset of the default settings. 
For example, here are the default values for the opt-level setting for the dev 
and release profiles:
```rust
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
The opt-level setting controls the number of optimizations Rust will apply to
the code, ranging from 0 to 3.

