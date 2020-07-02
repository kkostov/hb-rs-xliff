//! Defines XLIFF 1.2 compatible output writer

use std::error::Error;
use std::io::{Cursor};

use quick_xml::Writer;

pub use super::traits::XliffWriter;

use crate::store::Store;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};


type WriterResult = Result<(), Box<dyn Error>>;

/// XLIFF 1.2 compatible output writer
pub struct WriterXliff12;

impl XliffWriter for WriterXliff12 {
    fn write(store: &Store) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        WriterXliff12::print_envelope(&mut writer)?;
        WriterXliff12::print_envelope_end(&mut writer)?;

        return Ok(writer.into_inner().into_inner());
    }
}

impl WriterXliff12 {
    fn print_envelope(writer: &mut Writer<Cursor<Vec<u8>>>) -> WriterResult {
        // header <?xml version="1.0" encoding="UTF-8"?>
        writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;

        // opening <xliff tag>
        let mut elem = BytesStart::owned(b"xliff".to_vec(), "xliff".len());

        elem.push_attribute(("xmlns", "urn:oasis:names:tc:xliff:document:1.2"));
        elem.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
        elem.push_attribute(("version", "1.2"));
        elem.push_attribute(("xsi:schemaLocation", "urn:oasis:names:tc:xliff:document:1.2 http://docs.oasis-open.org/xliff/v1.2/os/xliff-core-1.2-strict.xsd"));

        writer.write_event(Event::Start(elem))?;

        return Ok(());
    }

    fn print_envelope_end(writer: &mut Writer<Cursor<Vec<u8>>>) -> WriterResult {
        let elem = BytesEnd::owned(b"xliff".to_vec());
        writer.write_event(Event::End(elem))?;
        writer.write_event(Event::Eof)?;
        Ok(())
    }
}
