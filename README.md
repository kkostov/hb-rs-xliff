XLIFF Parser
====================

[![Build Status](https://travis-ci.com/kkostov/hb-rs-xliff.svg?branch=master)](https://travis-ci.com/kkostov/hb-rs-xliff)

This is a library for reading and writing localized text stored in XLIFF format.

[Docs](https://docs.rs/xliff)

⚠ This is work in progress - check below for the current status of the implementation.

## Examples

### Reading XLIFF file

```rust no-run

let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "simplev1_2.xliff"]
        .iter()
        .collect();
let translations = T::load(&path);

let translation = translations.t(None, "Some text");

assert_eq!(
        translation.source_text().unwrap(),
        "Some text"
    );
assert_eq!(
    translation.target_text().unwrap(),
    "je précise quelque chose de très..."
);
```

### Reading XLIFF string

```rust no-run

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
</xliff>"#;

let translations = T::load_str(xliff_string);
let translation = translations.t(None, "CFBundleName");

```
## Changelog

[Version history](./CHANGELOG.md)

## Parse XLIFF 1.2

[Spec](http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html)


### Implementation status

.
- [ ] `<xliff>`  
    - [x] `<file>`
         - [x] `<header>`
            - [ ] `<skl>`
                - [ ] `<internal-file> | <external-file>`
            - [ ] `<phase-group>`
                - [ ] `<phase>`
                - [ ] `note`
            - [ ] `<glossary>`
                - [ ] `<internal-file> | <external-file>`
            - [ ] `<reference>`
                - [ ] `<internal-file> | <external-file>`
            - [ ] `<count-group>`
                - [ ] `<count>`
            - [x] `<tool>`
            - [ ] `<prop-group>`
                - [ ] `<prop>`
            - [x] `<note>`
         - [x] `<body>`
            - [ ] `<group>`
                - [ ] `<context-group>`
                    - [ ] `<context>`
                - [ ] `<count-group>`
                    - [ ] `<count>`
                - [ ] `<prop-group>`
                    - [ ] `<prop>`
                - [ ] `<note>`
            - [x] `<trans-unit>`
                - [x] `<source>`
                - [x] `<target>`
                - [ ] `<context-group>`
                    - [ ] `<context>`
                - [ ] `<count-group>`
                    - [ ] `<count>`
                - [ ] `<prop-group>`
                    - [ ] `<prop>`
                - [ ] `<seg-srouce>`
                - [x] `<note>`
                - [ ] `<alt-trans>`
            - [ ] `<bin-unit>`
                - [ ] `<bin-source>`
                - [ ] `<bin-target>`
                - [ ] `<context-group>`
                    - [ ] `<context>`
                - [ ] `<count-group>`
                    - [ ] `<count>`
                - [ ] `<prop-group>`
                    - [ ] `<prop>`
         - [ ] `<note>`
         - [ ] `<trans-unit>`
   

## Roadmap

- [x] Basic support for importing translations from [XLIFF 1.2](https://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html)

- [x] Provide a t("key") interface

- [ ] Complete parsing support for XLIFF 1.2

- [ ] Support loading files which contain multiple languages

- [ ] Export translations to XLIFF 1.2

- [ ] Import translations from [XLIFF 2.0](http://docs.oasis-open.org/xliff/xliff-core/v2.0/xliff-core-v2.0.html)

- [ ] Export translations to XLIFF 2.0


## Acknowledgements

* Depends on [quick-xml](https://crates.io/crates/quick-xml)


* The names "OASIS" and "XLIFF" are trademarks of [OASIS](https://www.oasis-open.org/), 
the owner and developer of the XLIFF specification.
