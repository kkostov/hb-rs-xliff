//! Parser for XLIFF 1.2 and XLIFF 2.0 files.
//!
//! ## Description
//!
//! The library allows to read and export translations in XLIFF format.
//! Check the README.md for the current status of the implementation.
//!
//!
//! ## Example
//!
//! Lookup translation by source text keyword:
//!```rust,no_run
//! use std::env;
//! use xliff::t::T;
//!
//! let translations = T::load("./en.xliff");
//!
//!    match translations.t_source(None, "Some text") {
//!        None => println!("translation not found"),
//!        Some(unit) => println!("> {}", unit.target_text().unwrap_or(&String::new())),
//!    }
//!```
//!
//!
//! Lookup translation by translation unit id:
//!```rust,no_run
//! use std::env;
//! use xliff::t::T;
//!
//! let translations = T::load("./en.xliff");
//!
//!    match translations.t(None, "fIC-hX-uRv.text") {
//!        None => println!("translation not found"),
//!        Some(unit) => println!("> {}", unit.target_text().unwrap_or(&String::new())),
//!    }
//!```
//!
//!
//! ## Specifications
//!
//! - XLIFF 1.2 http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html
//!
//! - XLIFF 2.0 http://docs.oasis-open.org/xliff/xliff-core/v2.0/xliff-core-v2.0.html
//!
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub mod writers;
pub mod store;
pub mod t;
