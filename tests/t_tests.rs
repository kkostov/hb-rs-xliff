extern crate xliff;

use std::path::PathBuf;
use xliff::t::T;

#[test]
fn test_t_get_translation() {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "simplev1_2.xliff"]
        .iter()
        .collect();
    let sut = T::load(&path);

    let result = sut.t(None, "fIC-hX-uRv.text");

    assert!(result.is_some());
    let translation = result.unwrap();

    assert_eq!(translation.id, "fIC-hX-uRv.text");
    assert_eq!(
        translation.source_text().unwrap(),
        "Pet projects are awesome"
    );
    assert_eq!(
        translation.target_text().unwrap(),
        "Странични проекти"
    );
    assert_eq!(
        translation.note.as_ref().unwrap().text,
        r#"Class = "UILabel"; text = "Pet projects are awesome"; ObjectID = "fIC-hX-uRv";"#
    );
}

#[test]
fn test_t_get_translation_twice() {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "simplev1_2.xliff"]
        .iter()
        .collect();
    let sut = T::load(&path);

    let result1 = sut.t(None, "fIC-hX-uRv.text");
    assert!(result1.is_some());

    let result2 = sut.t(None, "fIC-hX-uRv.text");
    assert!(result2.is_some());
}

#[test]
fn test_t_load_using_path() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/simplev1_2.xliff");

    let sut = T::load(&d.to_str().unwrap());

    let result1 = sut.t(None, "fIC-hX-uRv.text");
    assert!(result1.is_some());

    let result2 = sut.t(None, "fIC-hX-uRv.text");
    assert!(result2.is_some());
}

#[test]
fn test_t_get_translation_with_domain() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/simplev1_2.xliff");

    let sut = T::load(&d.to_str().unwrap());

    let result1 = sut.t(Some("SampleApp/en.lproj/InfoPlist.strings"), "More text");
    assert!(result1.is_some());

    let result2 = sut.t(Some("SampleApp/en.lproj/Localizable.strings"), "More text");
    assert!(result2.is_some());
}

#[test]
fn test_t_get_translation_with_source() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/simplev1_2.xliff");

    let sut = T::load(&d.to_str().unwrap());

    let result1 = sut.t_source(None, "Some text");
    assert!(result1.is_some());
    assert_eq!(result1.unwrap().source_text().unwrap(), "Some text");
}

#[test]
fn test_t_get_translation_with_source_and_domain() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/simplev1_2.xliff");

    let sut = T::load(&d.to_str().unwrap());

    let result1 = sut.t_source(Some("SampleApp/en.lproj/InfoPlist.strings"), "More text");
    assert!(result1.is_some());
    assert_eq!(result1.unwrap().source_text().unwrap(), "More text");

    let result2 = sut.t_source(Some("SampleApp/en.lproj/Localizable.strings"), "More text");
    assert!(result2.is_some());
    assert_eq!(result2.unwrap().source_text().unwrap(), "More text");
}
