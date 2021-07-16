# default-target

## A command line tool to print the current default Clang/LLVM target triple.

The LLVM ecosystem names compiler targets as [**target triples**][1],
made up of three to five parameters describing the target system.

This crate is a small binary program that returns the *default*
target triple for your current Rust installation. This program
is useful if you want to write shell scripts or other simple
programs that are aware of your Rust installation's default target.

You can find default-target's crate metadata on [crates.io][2],
documentation on [docs.rs][3], and source code on [GitHub][4].

## Installation
Run this command in your shell to install default-target.
```
cargo install default-target
```

## Usage
Once default-target has been installed in your path, you can run:
```
default-target
```
and it should return your target. Something like: `x86_64-apple-darwin`
or `x86_64-unknown-linux-gnu`.

[1]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
[2]: https://crates.io/crates/default-target
[3]: https://docs.rs/default-target
[4]: https://github.com/neocrym/default-target
