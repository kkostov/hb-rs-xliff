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
    assert_eq!(translation.target_text().unwrap(), "Странични проекти");
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

#[test]
fn test_t_load_translation_from_string() {
    let xliff_string = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff xmlns="urn:oasis:names:tc:xliff:document:1.2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" version="1.2" xsi:schemaLocation="urn:oasis:names:tc:xliff:document:1.2 http://docs.oasis-open.org/xliff/v1.2/os/xliff-core-1.2-strict.xsd">
  <file original="HelloWidgets/en.lproj/InfoPlist.strings" source-language="en" target-language="en" datatype="plaintext">
    <header>
      <tool tool-id="com.apple.dt.xcode" tool-name="Xcode" tool-version="12.0" build-num="12A6159"/>
    </header>
    <body>
      <trans-unit id="CFBundleName" xml:space="preserve">
        <source>HelloWidgets</source>
        <target>HelloWidgets Translated</target>
        <note>Bundle name</note>
      </trans-unit>
    </body>
  </file>
  <file original="HelloWidgets/en.lproj/Localizable.strings" source-language="en" target-language="en" datatype="plaintext">
    <header>
      <tool tool-id="com.apple.dt.xcode" tool-name="Xcode" tool-version="12.0" build-num="12A6159"/>
    </header>
    <body>
      <trans-unit id="Hello, world!" xml:space="preserve">
        <source>Hello, world!</source>
        <target>Hello, world! Translated</target>
        <note>No comment provided by engineer.</note>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

    let sut = T::load_str(xliff_string);

    let result = sut.t(None, "CFBundleName");

    assert!(result.is_some());
    let translation = result.unwrap();

    assert_eq!(translation.id, "CFBundleName");
    assert_eq!(translation.source_text().unwrap(), "HelloWidgets");
    assert_eq!(
        translation.target_text().unwrap(),
        "HelloWidgets Translated"
    );
    assert_eq!(translation.note.as_ref().unwrap().text, r#"Bundle name"#);
}
