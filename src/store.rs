//! Defines a translation store implementation which can be used to interact with XLIFF files

use quick_xml::events::BytesStart;
use quick_xml::events::Event::{End, Eof, Start, Text};
use quick_xml::Reader;
use std::io::BufRead;

/// The content of a translation unit or a note
pub struct UnitValue {
    /// Plain text value of the node.
    pub text: String,
}

impl Clone for UnitValue {
    fn clone(&self) -> Self {
        UnitValue {
            text: self.text.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.text = source.text.clone();
    }
}

/// A unit of translatable data.
/// Translation unit - The <trans-unit> elements contains a <source>, <target> and associated elements.
pub struct Unit {
    /// Identifier - uniquely identify the <trans-unit> within all
    /// <trans-unit> and <bin-unit> elements within the same <file>.
    pub id: String,
    /// Indicates whether the <trans-unit> is to be translated.
    /// http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html#translate
    pub translate: bool,
    /// Source translation. The <source> element is used to delimit a unit of text
    /// that could be a paragraph, a title, a menu item, a caption, etc.
    pub source: Option<UnitValue>,
    /// Target translation. The <target> element contains the translation of the content
    /// of the sibling <source> element.
    pub target: Option<UnitValue>,
    /// Source language - The language for the <source> elements in the given <file> element.
    pub source_locale: Option<Locale>,
    /// Target language - The language for the <target> elements in the given <file> element.
    pub target_locale: Option<Locale>,
    /// Note - The <note> element is used to add localization-related
    /// comments to the XLIFF document. The content of <note> may be instructions
    /// from developers about how to handle the <source>, comments from the translator
    /// about the translation, or any comment from anyone involved in processing the XLIFF file.
    pub note: Option<UnitValue>,
}

impl Unit {
    /// New translation unit instance
    pub fn new() -> Unit {
        Unit {
            id: String::new(),
            translate: true,
            source: None,
            target: None,
            source_locale: None,
            target_locale: None,
            note: None,
        }
    }

    /// Get a reference to the value of the <source> element in this translation `Unit`.
    pub fn source_text(&self) -> Option<&String> {
        match self.source.as_ref() {
            None => None,
            Some(t) => {
                Some(&t.text)
            },
        }
    }

    /// Get a reference to the value of the <target> element in this translation `Unit`.
    pub fn target_text(&self) -> Option<&String> {
        match self.target.as_ref() {
            None => None,
            Some(t) => {
                Some(&t.text)
            },
        }
    }
}

/// Language definition
pub struct Locale {
    /// A language code as described in the [RFC 4646], the successor to [RFC 3066].
    /// The values for this attribute follow the same rules as the values for xml:lang.
    /// Unlike the other XLIFF attributes, the values for xml:lang are not case-sensitive.
    pub identifier: String,
}

impl Locale {
    fn new(identifier: String) -> Locale {
        Locale {
            identifier: identifier.to_lowercase(),
        }
    }
}

/// File - The <file> element corresponds to a single extracted original document.
/// http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html#file
pub struct TranslationFile {
    /// Original file - The original attribute specifies the name of
    /// the original file from which the contents of a <file> element has been extracted.
    pub address: String,
    /// Source language - The language for the <source> elements
    /// in the given <file> element.
    pub source_locale: Option<Locale>,
    /// Target language - The language for the <target> elements
    /// in the given <file> element.
    pub target_locale: Option<Locale>,
    /// Translation units - A collection of translation units for the given file
    pub units: Vec<Unit>,
    /// Data type - The datatype attribute specifies the kind of text contained in the element.
    pub data_type: String,
}

impl TranslationFile {
    fn new(address: &str) -> TranslationFile {
        TranslationFile {
            address: String::from(address),
            source_locale: None,
            target_locale: None,
            units: vec![],
            data_type: String::new(),
        }
    }
}

/// A helper class which can be used to parse XLIFF
pub struct Store {
    /// A collection of file groups
    pub groups: Vec<TranslationFile>,
}

impl Store {
    /// Returns an empty translation store instance
    pub fn new() -> Store {
        Store { groups: vec![] }
    }

    /// Configures the store with the provided translation contents
    ///
    /// # Example
    /// ```no-run
    /// let mut file = File::open("translation.xliff").expect("Failed to open the file");
    ///
    /// let mut buffer: Vec<u8> = Default::default();
    /// file.read_to_end(&mut buffer).expect("failed to read file");
    ///
    /// let mut sut: xliff::store::Store = Store::new();
    /// sut.load(buffer.iter().as_slice());
    /// ```
    pub fn load<R: BufRead>(&mut self, r: R) {
        let mut buf = Vec::new();
        let mut r = Reader::from_reader(r);

        let mut tu_context: bool = false;
        let mut tag_context: Context = Context::Unknown;

        loop {
            match r.read_event(&mut buf).unwrap() {
                Start(ref e) => match e.name() {
                    b"file" => self.handle_file(e),
                    b"trans-unit" => {
                        tu_context = true;
                        self.handle_trans_unit(e);
                    }
                    b"source" => {
                        tag_context = Context::Source;
                    }
                    b"target" => {
                        tag_context = Context::Target;
                    }
                    b"note" => {
                        tag_context = Context::Note;
                    }
                    _ => (),
                },
                End(ref e) => {
                    match e.name() {
                        b"trans-unit" => tu_context = false,
                        _ => (),
                    }
                    tag_context = Context::Unknown;
                }
                Text(e) => match tag_context {
                    Context::Source => match self.groups.last_mut().unwrap().units.last_mut() {
                        None => panic!("found a source tag without a parent <trans-unit>"),
                        Some(unit) => {
                            unit.source = Some(UnitValue {
                                text: e.unescape_and_decode(&r).unwrap(),
                            })
                        }
                    },
                    Context::Target => match self.groups.last_mut().unwrap().units.last_mut() {
                        None => panic!("found a target tag without a parent <trans-unit>"),
                        Some(unit) => {
                            unit.target = Some(UnitValue {
                                text: e.unescape_and_decode(&r).unwrap(),
                            })
                        }
                    },
                    Context::Note => {
                        if tu_context {
                            match self.groups.last_mut().unwrap().units.last_mut() {
                                None => panic!("found a note tag without a parent <trans-unit>"),
                                Some(unit) => {
                                    unit.note = Some(UnitValue {
                                        text: e.unescape_and_decode(&r).unwrap(),
                                    })
                                }
                            }
                        }
                    }
                    _ => {}
                },
                Eof => break,
                _ => (),
            }
            buf.clear();
        }
    }

    fn handle_trans_unit(&mut self, e: &BytesStart) {
        let mut unit = Unit::new();

        e.attributes().for_each(|a| {
            let attr = a.unwrap();
            match attr.key {
                b"id" => {
                    unit.id = String::from_utf8(attr.value.into_owned()).unwrap();
                }
                b"translate" => {
                    unit.translate =
                        String::from_utf8(attr.value.into_owned()).unwrap() != false.to_string();
                }
                _ => (),
            }
        });
        self.groups.last_mut().unwrap().units.push(unit);
    }

    fn handle_file(&mut self, e: &BytesStart) {
        let mut file = TranslationFile::new("");

        e.attributes().for_each(|a| {
            let attr = a.unwrap();
            match attr.key {
                b"original" => {
                    file.address = String::from_utf8(attr.value.into_owned()).unwrap();
                }
                b"source-language" => {
                    file.source_locale = Some(Locale::new(
                        String::from_utf8(attr.value.into_owned()).unwrap(),
                    ))
                }
                b"target-language" => {
                    file.target_locale = Some(Locale::new(
                        String::from_utf8(attr.value.into_owned()).unwrap(),
                    ))
                }
                _ => (),
            }
        });
        self.groups.push(file);
    }
}

enum Context {
    Unknown,
    Source,
    Target,
    Note,
}
