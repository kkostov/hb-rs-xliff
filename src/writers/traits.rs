//! Defines the traits required to implement xliff writers.

use crate::store::Store;
use std::error::Error;

/// Xliff writer able to convert a translation `Store` object into a compatible output file.
pub trait XliffWriter {
    /// Creates a bytes array from the contents of the provided `Store` instance.
    fn write(store: &Store) -> Result<Vec<u8>, Box<dyn Error>>;
}
