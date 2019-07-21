//! Defines a translation store implementation which can be used to interact with XLIFF files

use quick_xml::events::BytesStart;
use quick_xml::events::Event::{Empty, End, Eof, Start, Text};
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
/// Translation unit - The `<trans-unit>` elements contains a `<source>, `<target>` and associated elements.
pub struct Unit {
    /// Identifier - uniquely identify the `<trans-unit>` within all
    /// `<trans-unit>` and `<bin-unit>` elements within the same `<file>.
    pub id: String,
    /// Indicates whether the `<trans-unit>` is to be translated.
    /// http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html#translate
    pub translate: bool,
    /// Source translation. The `<source>` element is used to delimit a unit of text
    /// that could be a paragraph, a title, a menu item, a caption, etc.
    pub source: Option<UnitValue>,
    /// Target translation. The `<target>` element contains the translation of the content
    /// of the sibling `<source>` element.
    pub target: Option<UnitValue>,
    /// Source language - The language for the `<source>` elements in the given `<file>` element.
    pub source_locale: Option<Locale>,
    /// Target language - The language for the `<target>` elements in the given `<file>` element.
    pub target_locale: Option<Locale>,
    /// Note - The `<note>` element is used to add localization-related
    /// comments to the XLIFF document. The content of `<note>` may be instructions
    /// from developers about how to handle the `<source>, comments from the translator
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

    /// Get a reference to the value of the `<source>` element in this translation `Unit`.
    pub fn source_text(&self) -> Option<&String> {
        match self.source.as_ref() {
            None => None,
            Some(t) => Some(&t.text),
        }
    }

    /// Get a reference to the value of the `<target>` element in this translation `Unit`.
    pub fn target_text(&self) -> Option<&String> {
        match self.target.as_ref() {
            None => None,
            Some(t) => Some(&t.text),
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

/// File - The `<file>` element corresponds to a single extracted original document.
/// http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html#file
pub struct TranslationFile {
    /// Original file - The original attribute specifies the name of
    /// the original file from which the contents of a `<file>` element has been extracted.
    pub address: String,
    /// Source language - The language for the `<source>` elements
    /// in the given `<file>` element.
    pub source_locale: Option<Locale>,
    /// Target language - The language for the `<target>` elements
    /// in the given `<file>` element.
    pub target_locale: Option<Locale>,
    /// Translation units - A collection of translation units for the given file
    pub units: Vec<Unit>,
    /// Data type - The datatype attribute specifies the kind of text contained in the element.
    pub data_type: String,

    /// File header
    pub header: Option<Header>,
}

impl TranslationFile {
    fn new(address: &str) -> TranslationFile {
        TranslationFile {
            address: String::from(address),
            source_locale: None,
            target_locale: None,
            units: vec![],
            data_type: String::new(),
            header: None,
        }
    }
}

/// Tool - The `<tool>` element describes the tool that has been used
/// to execute a given task in the document.
/// http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html#tool_elem
pub struct Tool {
    /// Tool identifier - The tool-id attribute allows unique identification of a `<tool>` element.
    /// It is also used in other elements in the file to refer to the given `<tool>` element.
    pub id: String,
    /// Tool name - The tool-name attribute specifies the name of a given tool.
    pub name: String,
    /// Tool version - The tool-version attribute specifies the version of a given tool.
    pub version: Option<String>,
    /// Tool company - The tool-company attribute specifies the company from which a tool originates.
    pub company: Option<String>,
}

impl Tool {
    fn new(id: String, name: String) -> Self {
        Tool {
            id,
            name,
            version: None,
            company: None,
        }
    }
}

/// File header - The `<header>` element contains metadata relating to the `<file>` element.
/// http://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html#header
pub struct Header {
    /// Tools used within this document
    pub tools: Vec<Tool>,
    ///Localization-related comments to the XLIFF document
    pub notes: Vec<UnitValue>,
}

impl Header {
    /// Returns an empty header instance
    fn new() -> Self {
        Header {
            tools: vec![],
            notes: vec![],
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

        let mut tags: Vec<TagCtx> = vec![];

        loop {
            match r.read_event(&mut buf).unwrap() {
                Start(ref e) => {
                    if let Some(tag) = TagCtx::from(e.name()) {
                        Store::open_tag(&mut tags, tag);
                        match tag {
                            TagCtx::File => self.handle_file(e),
                            TagCtx::Unit => self.handle_trans_unit(e),
                            TagCtx::Header => self.handle_file_header(e),
                            _ => (),
                        }
                    }
                }
                Empty(ref e) => {
                    if let Some(tag) = TagCtx::from(e.name()) {
                        Store::open_tag(&mut tags, tag);
                        match tag {
                            TagCtx::Tool => self.handle_header_tool(e),
                            _ => (),
                        }
                        Store::close_tag(&mut tags, tag);
                    }
                }
                End(ref e) => {
                    if let Some(tag) = TagCtx::from(e.name()) {
                        Store::close_tag(&mut tags, tag);
                    }
                }
                Text(e) => match tags.last() {
                    None => (),
                    Some(tag) => match tag {
                        TagCtx::Source => self.add_unit_source(e.unescape_and_decode(&r).unwrap()),
                        TagCtx::Target => self.add_unit_target(e.unescape_and_decode(&r).unwrap()),
                        TagCtx::Note => {
                            let count = tags.len();
                            if count >= 2 {
                                match &tags[count - 2] {
                                    TagCtx::Header => {
                                        self.add_header_note(e.unescape_and_decode(&r).unwrap())
                                    }
                                    TagCtx::Unit => {
                                        self.add_unit_note(e.unescape_and_decode(&r).unwrap())
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    },
                },
                Eof => break,
                _ => (),
            }
            buf.clear();
        }
    }

    fn add_unit_source(&mut self, text: String) {
        match self.groups.last_mut().unwrap().units.last_mut() {
            None => (),
            Some(unit) => unit.source = Some(UnitValue { text }),
        }
    }

    fn add_unit_target(&mut self, text: String) {
        match self.groups.last_mut().unwrap().units.last_mut() {
            None => (),
            Some(unit) => unit.target = Some(UnitValue { text }),
        }
    }

    fn add_unit_note(&mut self, text: String) {
        match self.groups.last_mut().unwrap().units.last_mut() {
            None => (),
            Some(unit) => unit.note = Some(UnitValue { text }),
        }
    }

    fn add_header_note(&mut self, text: String) {
        match self.groups.last_mut().unwrap().header.as_mut() {
            None => (),
            Some(header) => header.notes.push(UnitValue { text }),
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

    fn handle_file(&mut self, e: &BytesStart) -> () {
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

    fn handle_header_tool(&mut self, e: &BytesStart) -> () {
        match self.groups.last_mut().unwrap().header.as_mut() {
            None => (),
            Some(header) => {
                let mut id = String::new();
                let mut name = String::new();

                e.attributes().for_each(|a| {
                    let attr = a.unwrap();
                    match attr.key {
                        b"tool-id" => {
                            id = String::from_utf8(attr.value.into_owned()).unwrap();
                        }
                        b"tool-name" => {
                            name = String::from_utf8(attr.value.into_owned()).unwrap();
                        }
                        _ => (),
                    }
                });

                let mut tool = Tool::new(id, name);

                e.attributes().for_each(|a| {
                    let attr = a.unwrap();
                    match attr.key {
                        b"tool-version" => {
                            tool.version =
                                Some(String::from_utf8(attr.value.into_owned()).unwrap());
                        }
                        b"tool-company" => {
                            tool.company =
                                Some(String::from_utf8(attr.value.into_owned()).unwrap());
                        }
                        _ => (),
                    }
                });

                header.tools.push(tool);
            }
        }
    }

    fn handle_file_header(&mut self, _e: &BytesStart) -> () {
        match self.groups.last_mut() {
            None => (),
            Some(file) => file.header = Some(Header::new()),
        }
    }

    fn open_tag(tags: &mut Vec<TagCtx>, open_tag: TagCtx) -> () {
        tags.push(open_tag)
    }

    fn close_tag(tags: &mut Vec<TagCtx>, close_tag: TagCtx) -> () {
        let mut tag_closed = false;
        while !tag_closed {
            match tags.pop() {
                None => {
                    tag_closed = true;
                }
                Some(tag) => {
                    if tag == close_tag {
                        tag_closed = true;
                    }
                }
            }
        }
    }
}

/// The XML tag in which the current operation is taking place
#[derive(PartialEq, Copy, Clone)]
enum TagCtx {
    File,
    Header,
    Tool,
    Body,
    Source,
    Target,
    Note,
    Unit,
}

impl TagCtx {
    fn from(name: &[u8]) -> Option<Self> {
        match name {
            b"file" => Some(TagCtx::File),
            b"header" => Some(TagCtx::Header),
            b"tool" => Some(TagCtx::Tool),
            b"body" => Some(TagCtx::Body),
            b"source" => Some(TagCtx::Source),
            b"target" => Some(TagCtx::Target),
            b"note" => Some(TagCtx::Note),
            b"trans-unit" => Some(TagCtx::Unit),
            _ => None,
        }
    }
}
