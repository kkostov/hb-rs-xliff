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

## Roadmap

- [x] Basic support for importing translations from [XLIFF 1.2](https://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html)

- [x] Provide a t("key") interface

- [ ] Support for `<header>` elements

- [ ] Support for `<group>` elements

- [ ] Support for `<context>` elements

- [ ] Support for `<alt-trans>` elements

- [ ] Support for `xml:lang` attributes

- [ ] Support loading files which contain multiple languages

- [ ] Export translations to XLIFF 1.2

- [ ] Import translations from [XLIFF 2.0](http://docs.oasis-open.org/xliff/xliff-core/v2.0/xliff-core-v2.0.html)

- [ ] Export translations to XLIFF 2.0


## Acknowledgements

* Depends on [quick-xml](https://crates.io/crates/quick-xml)


* The names "OASIS" and "XLIFF" are trademarks of [OASIS](https://www.oasis-open.org/), 
the owner and developer of the XLIFF specification.
