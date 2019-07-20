//! Defines helper functions which can be used to retrieve translations

use std::path::PathBuf;
use crate::store::{Store, Unit};
use std::fs::File;
use std::io::Read;

/// Translation helper
pub struct T {
    store: Store
}

impl T {
    /// Initializes a repository for the specified translation format.
    pub fn load(path: &PathBuf) -> Self {

        let mut file = File::open(path).expect("Failed to open the file");

        let mut buffer: Vec<u8> = Default::default();
        file.read_to_end(&mut buffer).expect("failed to read file");

        let mut store: Store = Store::new();
        store.load(buffer.iter().as_slice());


        return T{store};
    }

    /// Returns the first translation matching the provided key.
    /// Optionally a domain value may be used to specify the xliff file address.
    pub fn t(&self, domain: Option<&str>, key: &str) -> Option<&Unit> {
        match domain {
            None => {
                for group in self.store.groups.iter() {
                    match group.units.iter().find(|u| {
                        return u.id == String::from(key);
                    }) {
                        None => (),
                        Some(result) => {
                            return Some(result)
                        },
                    }
                }
            },
            Some(address) => {
                match self.store.groups.iter().find(|g| {
                    return g.address == String::from(address);
                }) {
                    None => (),
                    Some(group) => {
                        match group.units.iter().find(|u| {
                            return u.id == String::from(key);
                        }) {
                            None => (),
                            Some(result) => {
                                return Some(result)
                            },
                        }
                    },
                }
            },
        }

        return None;
    }
}