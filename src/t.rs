//! Defines helper functions which can be used to retrieve translations

use crate::store::{Store, Unit};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Translation helper
pub struct T {
    /// The currently loaded translations grouped by context.
    pub store: Store,
}

impl T {
    /// Reads and interprets the contents of the specified string.
    ///
    /// # Errors
    /// This function will return an error if reading or interpreting the XLIFF string is unsuccessfull.
    pub fn load_str(string: &str) -> Self {
        let mut store: Store = Store::new();
        store.load(string.as_bytes());

        return T { store };
    }

    /// Reads and interprets the contents of the specified file.
    ///
    /// # Errors
    /// This function will return an error if reading the file is not successful.
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let file_path = path.as_ref();
        let mut file = File::open(file_path).unwrap();

        let mut buffer: Vec<u8> = Default::default();
        file.read_to_end(&mut buffer).expect("failed to read file");

        let mut store: Store = Store::new();
        store.load(buffer.iter().as_slice());

        return T { store };
    }

    /// Returns the first translation matching the provided `unit_id`.
    ///
    /// The value of `unit_id` is used to match against the `id` attribute of each `<trans-unit>`
    /// element.
    /// The `id` attribute values are determined by the tool that created the extracted the xliff
    /// document, they may or may not be the same as the translation source value.
    ///
    /// The specificity of the match can be increased by providing a value for `domain`
    /// which is used to match against the `address` attribute of `<file>` elements.
    ///
    /// # Example
    ///
    /// The following example will retrieve the first translation unit with id `fIC-hX-uRv.text`:
    ///
    /// ```rust,no_run
    /// use std::env;
    /// use xliff::t::T;
    ///
    /// let translations = T::load("./en.xliff");
    ///
    ///    match translations.t(None, "fIC-hX-uRv.text") {
    ///        None => println!("translation not found"),
    ///        Some(unit) => println!("> {}", unit.target_text().unwrap_or(&String::new())),
    ///    }
    /// ```
    ///
    /// Explicitly specify the file in which to lookup the translation unit:
    ///
    /// ```rust,no_run
    /// use std::env;
    /// use xliff::t::T;
    ///
    /// let translations = T::load("./en.xliff");
    ///
    ///    match translations.t(Some("SampleApp/en.lproj/Localizable.strings"), "fIC-hX-uRv.text") {
    ///        None => println!("translation not found"),
    ///        Some(unit) => println!("> {}", unit.target_text().unwrap_or(&String::new())),
    ///    }
    /// ```
    pub fn t(&self, domain: Option<&str>, unit_id: &str) -> Option<&Unit> {
        match domain {
            None => {
                for group in self.store.groups.iter() {
                    match group.units.iter().find(|u| {
                        return u.id == String::from(unit_id);
                    }) {
                        None => (),
                        Some(result) => return Some(result),
                    }
                }
            }
            Some(address) => {
                match self.store.groups.iter().find(|g| {
                    return g.address == String::from(address);
                }) {
                    None => (),
                    Some(group) => {
                        match group.units.iter().find(|u| {
                            return u.id == String::from(unit_id);
                        }) {
                            None => (),
                            Some(result) => return Some(result),
                        }
                    }
                }
            }
        }

        return None;
    }

    /// Returns the first translation matching the provided `source_text`.
    ///
    /// The value of `source_text` is used to a matching `<trans-unit>` based on the value of its
    /// `<source>` element.
    ///
    /// The specificity of the match can be increased by providing a value for `domain`
    /// which is used to match against the `address` attribute of `<file>` elements.
    ///
    /// # Example
    ///
    /// The following example will retrieve the first translation unit with source `Some text`:
    ///
    /// ```rust,no_run
    /// use std::env;
    /// use xliff::t::T;
    ///
    /// let translations = T::load("./en.xliff");
    ///
    ///    match translations.t_source(None, "Some text") {
    ///        None => println!("translation not found"),
    ///        Some(unit) => println!("> {}", unit.target_text().unwrap_or(&String::new())),
    ///    }
    /// ```
    ///
    /// Explicitly specify the file in which to lookup the translation unit:
    ///
    /// ```rust,no_run
    /// use std::env;
    /// use xliff::t::T;
    ///
    /// let translations = T::load("./en.xliff");
    ///
    ///    match translations.t_source(Some("SampleApp/en.lproj/Localizable.strings"), "Some text") {
    ///        None => println!("translation not found"),
    ///        Some(unit) => println!("> {}", unit.target_text().unwrap_or(&String::new())),
    ///    }
    /// ```
    pub fn t_source(&self, domain: Option<&str>, source_text: &str) -> Option<&Unit> {
        match domain {
            None => {
                for group in self.store.groups.iter() {
                    match group.units.iter().find(|u| {
                        return u.source_text() == Some(&String::from(source_text));
                    }) {
                        None => (),
                        Some(result) => return Some(result),
                    }
                }
            }
            Some(address) => {
                match self.store.groups.iter().find(|g| {
                    return g.address == String::from(address);
                }) {
                    None => (),
                    Some(group) => {
                        match group.units.iter().find(|u| {
                            return u.source_text() == Some(&String::from(source_text));
                        }) {
                            None => (),
                            Some(result) => return Some(result),
                        }
                    }
                }
            }
        }

        return None;
    }
}
