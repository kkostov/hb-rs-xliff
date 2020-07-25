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
fn test_writes_file_for_each_group_in_order() {
    // load the sample xliff
    let sample_file: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut store: xliff::store::Store = Store::new();
    store.load(sample_file);

    // export the contents to a string
    let result = WriterXliff12::write(&store);
    let result_string = String::from_utf8(result.unwrap()).unwrap();

    // load the string to a new store, so we can assert
    let t = T::load_str(result_string.as_str());

    assert_eq!(t.store.groups.len(), 4);

    for (ix, group) in store.groups.iter().enumerate() {
        let written = t.store.groups.get(ix);
        assert!(written.is_some());
        assert!(written.unwrap().address == group.address);
        assert!(written.unwrap().source_locale == group.source_locale);
        assert!(written.unwrap().target_locale == group.target_locale);
        assert!(written.unwrap().data_type == group.data_type);
        assert!(written.unwrap().header.is_some() == group.header.is_some());
    }
}

#[test]
fn test_writes_header_tools() {
    // load the sample xliff
    let sample_file: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut store: xliff::store::Store = Store::new();
    store.load(sample_file);

    // export the contents to a string
    let result = WriterXliff12::write(&store);
    let result_string = String::from_utf8(result.unwrap()).unwrap();

    // load the string to a new store, so we can assert
    let t = T::load_str(result_string.as_str());

    assert_eq!(t.store.groups.len(), 4);

    for (ix, group) in store.groups.iter().enumerate() {
        let written = t.store.groups.get(ix);
        assert!(&written.is_some());

        if let Some(header) = &group.header {
            if !header.tools.is_empty() {
                let written_tools = &written.unwrap().header.as_ref().unwrap().tools;
                assert!(
                    !written_tools.is_empty(),
                    "Missing tool in header for file {}",
                    group.address
                );

                // the <tool> header depends on the current writer
                for (ix, _tool) in header.tools.iter().enumerate() {
                    assert_eq!(
                        written_tools.get(ix).unwrap().id,
                        "eu.headbright.hb-rs-xliff"
                    );
                    assert_eq!(written_tools.get(ix).unwrap().name, "Rust Xliff Crate");
                }
            }
        }
    }
}

#[test]
fn test_writes_header_notes() {
    // load the sample xliff
    let sample_file: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut store: xliff::store::Store = Store::new();
    store.load(sample_file);

    // export the contents to a string
    let result = WriterXliff12::write(&store);
    let result_string = String::from_utf8(result.unwrap()).unwrap();

    // load the string to a new store, so we can assert
    let t = T::load_str(result_string.as_str());

    assert_eq!(t.store.groups.len(), 4);
    let mut at_least_one_note = false;

    for (ix, group) in store.groups.iter().enumerate() {
        let written = t.store.groups.get(ix);
        assert!(&written.is_some());

        if let Some(header) = &group.header {
            if !header.notes.is_empty() {
                let written_notes = &written.unwrap().header.as_ref().unwrap().notes;
                assert_eq!(written_notes.len(), header.notes.len());

                for (ix, note) in header.notes.iter().enumerate() {
                    at_least_one_note = true;
                    assert_eq!(written_notes.get(ix).unwrap().text, note.text);
                }
            }
        }
    }

    assert!(
        at_least_one_note,
        "Test data is missing at least one header with a note."
    );
}
