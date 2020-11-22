# Cargo Install

`cargo install` allows you to install and use crates locally (only pakcages that have binary targets).
All binaries installed via `cargo install` live in the installation root's bin folder.
(For rustup, this is generally $HOME/.cargo/bin)

For example, `cargo run ripgrep` installs ripgrep, which you can check with `rg --help`.
