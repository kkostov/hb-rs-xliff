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

    assert!(result.is_some(), true);
    let translation = result.unwrap();

    assert_eq!(translation.id, "fIC-hX-uRv.text");
    assert_eq!(translation.source_text().unwrap(), "Pet projects are awesome");
    assert_eq!(translation.target_text().unwrap(), "Странични проекти");
    assert_eq!(translation.note.as_ref().unwrap().text, r#"Class = "UILabel"; text = "Pet projects are awesome"; ObjectID = "fIC-hX-uRv";"#);
}

#[test]
fn test_t_get_translation_twice() {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "simplev1_2.xliff"]
        .iter()
        .collect();
    let sut = T::load(&path);

    let result1 = sut.t(None, "fIC-hX-uRv.text");
    assert!(result1.is_some(), true);

    let result2 = sut.t(None, "fIC-hX-uRv.text");
    assert!(result2.is_some(), true);
}