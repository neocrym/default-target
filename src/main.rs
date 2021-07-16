//! A command line tool to print the current default Clang/LLVM target triple.
//!
//! The LLVM ecosystem names compiler targets as [**target triples**][1],
//! made up of three to five parameters describing the target system.
//!
//! This crate is a small binary program that returns the *default*
//! target triple for your current Rust installation. This program
//! is useful if you want to write shell scripts or other simple
//! programs that are aware of your Rust installation's default target.
//!
//! You can find default-target's crate metadata on [crates.io][2],
//! documentation on [docs.rs][3], and source code on [GitHub][4].
//!
//! # Installation
//! Run this command in your shell to install default-target.
//! ```
//! cargo install default-target
//! ```
//! # Usage
//! Once default-target has been installed in your path, you can run:
//! ```
//! default-target
//! ```
//! and it should return your target. Something like: `x86_64-apple-darwin`
//! or `x86_64-unknown-linux-gnu`.
//!
//! [1]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
//! [2]: https://crates.io/crates/default-target
//! [3]: https://docs.rs/default-target
//! [4]: https://github.com/neocrym/default-target
use std::fmt::{Display, Formatter};
use std::process::{Command, exit};
use std::str;

/// The [`rustc`][1] output field name that shows the target.
///
/// [1]: https://doc.rust-lang.org/rustc/what-is-rustc.html
const TARGET_FIELD: &str = "host: ";

/// The custom error type for this crate.
#[derive(Clone, Debug)]
struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Calls rustc in a subprocess and returns the default Clang target triple.
///
/// This method is based on [this StackOverflow answer][2].
///
/// # Arguments:
///  - `rustc_path`: The path to a [`rustc`][1] executable. We call this
///     rustc executable in a subprocess to discover its default
///     target triple.
///
/// [1]: https://doc.rust-lang.org/rustc/what-is-rustc.html
/// [2]: https://stackoverflow.com/a/66955420
fn get_default_target(rustc_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Query rustc for defaults.
    let output = Command::new(rustc_path)
        .arg("-vV")
        .output()
        .map_err(Box::new)?;

    // Decode stdout.
    let stdout = std::str::from_utf8(&output.stdout).map_err(Box::new)?;

    // Parse the default target from stdout.
    let host = stdout
        .lines()
        .find(|l| l.starts_with(TARGET_FIELD))
        .map(|l| &l[TARGET_FIELD.len()..])
        .ok_or_else(|| Box::new(Error::new("Failed to parse target from rustc output.")))?
        .to_owned();
    Ok(host)
}

#[doc(hidden)]
fn main() {
    match get_default_target("rustc") {
        Ok(target) => println!("{}", target),
        Err(msg) => {
            eprintln!("Error: {}", msg);
            exit(1);
        }
    }
}
