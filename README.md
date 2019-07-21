XLIFF Parser
====================

[![Build Status](https://travis-ci.com/kkostov/hb-rs-xliff.svg?branch=master)](https://travis-ci.com/kkostov/hb-rs-xliff)

This is a library for reading and writing localized text stored in XLIFF format.


## Example

```rust no-run

let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "simplev1_2.xliff"]
        .iter()
        .collect();
let translations = T::load(&path);

let translation = sut.t(None, "Some text");

assert_eq!(
        translation.source_text().unwrap(),
        "Some text"
    );
    assert_eq!(
        translation.target_text().unwrap(),
        "je précise quelque chose de très..."
    );
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
