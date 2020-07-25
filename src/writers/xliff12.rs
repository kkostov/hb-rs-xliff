//! Defines XLIFF 1.2 compatible output writer

use std::error::Error;
use std::io::Cursor;

use quick_xml::Writer;

pub use super::traits::XliffWriter;

use crate::store::{Store, TagCtx};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use std::collections::HashMap;

type WriterResult = Result<(), Box<dyn Error>>;

/// XLIFF 1.2 compatible output writer
pub struct WriterXliff12;

impl XliffWriter for WriterXliff12 {
    fn write(store: &Store) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        WriterXliff12::print_envelope(&mut writer)?;

        for file in &store.groups {
            Self::open_tag(&mut writer, TagCtx::File.to_str(), Some(file.attributes()));

            if let Some(file_header) = &file.header {
                Self::open_tag(&mut writer, TagCtx::Header.to_str(), None);
                for tool in &file_header.tools {
                    Self::open_tag(&mut writer, TagCtx::Tool.to_str(), Some(tool.attributes()));
                    Self::close_tag(&mut writer, TagCtx::Tool.to_str());
                }
                Self::close_tag(&mut writer, TagCtx::Header.to_str());
            }

            Self::close_tag(&mut writer, TagCtx::File.to_str());
        }

        WriterXliff12::print_envelope_end(&mut writer)?;

        return Ok(writer.into_inner().into_inner());
    }
}

impl WriterXliff12 {
    fn print_envelope(writer: &mut Writer<Cursor<Vec<u8>>>) -> WriterResult {
        // header <?xml version="1.0" encoding="UTF-8"?>
        writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
        Self::open_tag(writer, TagCtx::Xliff.to_str(), Some(vec![
            ("xmlns", "urn:oasis:names:tc:xliff:document:1.2"),
            ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
            ("version", "1.2"),
            ("xsi:schemaLocation", "urn:oasis:names:tc:xliff:document:1.2 http://docs.oasis-open.org/xliff/v1.2/os/xliff-core-1.2-strict.xsd")
        ]))
    }

    fn print_envelope_end(writer: &mut Writer<Cursor<Vec<u8>>>) -> WriterResult {
        Self::close_tag(writer, TagCtx::Xliff.to_str())
    }
}

impl WriterXliff12 {
    fn open_tag(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        tag: &str,
        attributes: Option<Vec<(&str, &str)>>,
    ) -> WriterResult {
        let mut elem = BytesStart::owned(tag.as_bytes(), tag.len());
        if let Some(attributes) = attributes {
            for attribute in attributes.into_iter() {
                elem.push_attribute(attribute)
            }
        }
        writer.write_event(Event::Start(elem))?;
        Ok(())
    }

    fn close_tag(writer: &mut Writer<Cursor<Vec<u8>>>, tag: &str) -> WriterResult {
        let elem = BytesEnd::owned(tag.as_bytes().to_vec());
        writer.write_event(Event::End(elem))?;
        writer.write_event(Event::Eof)?;
        Ok(())
    }
}
