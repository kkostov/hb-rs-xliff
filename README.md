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
```

## Roadmap

- [x] Read XLIFF 1.2 (https://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html)

- [x] Provide a t("key") interface

- [ ] Write XLIFF 1.2

- [ ] Read XLIFF 2.0 (http://docs.oasis-open.org/xliff/xliff-core/v2.0/xliff-core-v2.0.html)

- [ ] Write XLIFF 2.0

- [ ] Read TMX (https://www.gala-global.org/tmx-14b)

- [ ] Write TMX
