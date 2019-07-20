extern crate xliff;

use quick_xml::events::Event::{Decl, Eof, Start};
use quick_xml::Reader;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use xliff::store::*;

#[test]
fn test_sample_as_xml_count_all_tags() {
    let src: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut buf = Vec::new();
    let mut r = Reader::from_reader(src);
    let mut count = 0;
    loop {
        match r.read_event(&mut buf).unwrap() {
            Start(_) => count += 1,
            Decl(e) => println!("{:?}", e.version()),
            Eof => break,
            _ => (),
        }
        buf.clear();
    }

    assert_eq!(count, 35);
}

#[test]
fn test_sample_reads_file_tags() {
    let src: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut sut: xliff::store::Store = Store::new();
    sut.load(src);

    assert_eq!(sut.groups.len(), 3);
}

#[test]
fn test_sample_reads_file_attributes() {
    let src: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut sut: xliff::store::Store = Store::new();
    sut.load(src);

    assert_eq!(
        sut.groups[0].address,
        "SampleApp/Base.lproj/Main.storyboard"
    );

    assert_eq!(sut.groups[0].source_locale.is_some(), true);
    match &sut.groups[0].source_locale {
        None => {
            assert!(false, "source_locale must be set");
        }
        Some(locale) => {
            assert_eq!(locale.identifier, "en");
        }
    }

    assert_eq!(sut.groups[0].target_locale.is_some(), true);
    match &sut.groups[0].target_locale {
        None => {
            assert!(false, "target_locale must be set");
        }
        Some(locale) => {
            assert_eq!(locale.identifier, "bg");
        }
    }
}

#[test]
fn test_sample_reads_translation_units() {
    let src: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut sut: xliff::store::Store = Store::new();
    sut.load(src);

    assert_eq!(sut.groups[0].units.len(), 1);
    assert_eq!(sut.groups[0].units[0].id, "fIC-hX-uRv.text");
    assert_eq!(sut.groups[0].units[0].translate, true);

    assert_eq!(sut.groups[1].units.len(), 2);
    assert_eq!(sut.groups[1].units[0].id, "CFBundleName");
    assert_eq!(sut.groups[1].units[0].translate, true);
    assert_eq!(sut.groups[1].units[1].id, "2");
    assert_eq!(sut.groups[1].units[1].translate, false);
}

#[test]
fn test_sample_reads_translation_source() {
    let src: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut sut: xliff::store::Store = Store::new();
    sut.load(src);

    assert_eq!(
        sut.groups[0].units[0].source.clone().unwrap().text,
        "Pet projects are awesome"
    );
    assert_eq!(
        sut.groups[1].units[0].source.clone().unwrap().text,
        "SampleApp"
    );
    assert_eq!(
        sut.groups[1].units[1].source.clone().unwrap().text,
        "Do not translate this"
    );
}

#[test]
fn test_sample_reads_translation_target() {
    let src: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut sut: xliff::store::Store = Store::new();
    sut.load(src);

    assert_eq!(
        sut.groups[0].units[0].target.clone().unwrap().text,
        "Странични проекти"
    );
    assert_eq!(sut.groups[1].units[0].target.is_none(), true);
    assert_eq!(sut.groups[1].units[1].target.is_none(), true);
}

#[test]
fn test_sample_reads_translation_note() {
    let src: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut sut: xliff::store::Store = Store::new();
    sut.load(src);

    assert_eq!(
        sut.groups[0].units[0].note.clone().unwrap().text,
        r#"Class = "UILabel"; text = "Pet projects are awesome"; ObjectID = "fIC-hX-uRv";"#
    );
    assert_eq!(
        sut.groups[1].units[0].note.clone().unwrap().text,
        r#"Bundle name"#
    );
    assert_eq!(
        sut.groups[1].units[1].note.clone().unwrap().text,
        r#"A note from the author"#
    );
    assert_eq!(
        sut.groups[2].units[0].note.clone().unwrap().text,
        r#"No comment provided by engineer."#
    );
}

#[test]
fn test_sample_source_target_retrievers() {
    let src: &[u8] = include_bytes!("simplev1_2.xliff");
    let mut sut: xliff::store::Store = Store::new();
    sut.load(src);

    assert_eq!(
        sut.groups[0].units[0].source_text().unwrap(),
        "Pet projects are awesome"
    );
    assert_eq!(
        sut.groups[0].units[0].target_text().unwrap(),
        "Странични проекти"
    );
    assert_eq!(
        sut.groups[1].units[0].source_text().unwrap(),
        "SampleApp"
    );
    assert_eq!(
        sut.groups[1].units[1].source_text().unwrap(),
        "Do not translate this"
    );
}

#[test]
fn test_file_using_file_reader() {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "simplev1_2.xliff"]
        .iter()
        .collect();
    let mut file = File::open(path).expect("Failed to open the file");

    let mut buffer: Vec<u8> = Default::default();
    file.read_to_end(&mut buffer).expect("failed to read file");

    let mut sut: xliff::store::Store = Store::new();
    sut.load(buffer.iter().as_slice());

    assert_eq!(
        sut.groups[0].units[0].source.clone().unwrap().text,
        "Pet projects are awesome"
    );
}
