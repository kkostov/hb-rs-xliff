extern crate xliff;

use xliff::store::Store;
use xliff::t::T;
use xliff::writers::xliff12::*;

#[test]
fn test_print_empty_store() {
    let store = Store::new();
    let expected = r#"<?xml version="1.0" encoding="UTF-8"?><xliff xmlns="urn:oasis:names:tc:xliff:document:1.2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" version="1.2" xsi:schemaLocation="urn:oasis:names:tc:xliff:document:1.2 http://docs.oasis-open.org/xliff/v1.2/os/xliff-core-1.2-strict.xsd"></xliff>"#;

    let result = WriterXliff12::write(&store);
    assert!(result.is_ok());
    assert_eq!(
        String::from_utf8(result.unwrap()).unwrap(),
        expected.to_string()
    );
}

#[test]
fn test_writes_file_for_each_group() {
    // load the sample xliff
    let sample_file: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut store: xliff::store::Store = Store::new();
    store.load(sample_file);

    // export the contents to a string
    let result = WriterXliff12::write(&store);
    let result_string = String::from_utf8(result.unwrap()).unwrap();

    // load the string to a new store, so we can assert
    let mut t = T::load_str(result_string.as_str());

    assert_eq!(t.store.groups.len(), 4);
}

#[test]
fn test_writes_file_attributes_from_group() {
    // load the sample xliff
    let sample_file: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut store: xliff::store::Store = Store::new();
    store.load(sample_file);

    // export the contents to a string
    let result = WriterXliff12::write(&store);
    let result_string = String::from_utf8(result.unwrap()).unwrap();

    // parse the string
    let mut t = T::load_str(result_string.as_str());

    for group in store.groups {
        assert!(t.store.groups.iter().any(|g| g.address == group.address
            && g.source_locale == group.source_locale
            && g.target_locale == group.target_locale
            && g.data_type == group.data_type))
    }
    assert_eq!(t.store.groups.len(), 4);
}
