extern crate xliff;

use xliff::store::Store;
use xliff::writers::xliff12::*;


#[test]
fn test_print_empty_store() {
    let store = Store::new();
    let expected = r#"<?xml version="1.0" encoding="UTF-8"?><xliff xmlns="urn:oasis:names:tc:xliff:document:1.2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" version="1.2" xsi:schemaLocation="urn:oasis:names:tc:xliff:document:1.2 http://docs.oasis-open.org/xliff/v1.2/os/xliff-core-1.2-strict.xsd"></xliff>"#;

    let result = WriterXliff12::write(&store);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(result.unwrap()).unwrap(), expected.to_string());
}