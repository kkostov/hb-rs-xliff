//! Parser for XLIFF 1.2 and XLIFF 2.0 files.
//!
//! ## Description
//!
//! The library allows the consumer to read and export translations stored in XLIFF format.
//!
//!
//! ## Specifications
//!
//! - XLIFF 1.2 http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html
//!
//! - XLIFF 2.0 http://docs.oasis-open.org/xliff/xliff-core/v2.0/xliff-core-v2.0.html
//! ```
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub mod store;
pub mod t;

//
//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
