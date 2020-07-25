//! Defines XLIFF 1.2 compatible output writer

use std::error::Error;
use std::io::Cursor;

use quick_xml::Writer;

pub use super::traits::XliffWriter;

use crate::store::{Store, TagCtx, TranslationFile, Unit};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};

type WriterResult = Result<(), Box<dyn Error>>;

/// XLIFF 1.2 compatible output writer
pub struct WriterXliff12;

impl XliffWriter for WriterXliff12 {
    fn write(store: &Store) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        WriterXliff12::print_envelope(&mut writer)?;

        for file in &store.groups {
            Self::open_tag(&mut writer, TagCtx::File.to_str(), Some(file.attributes()))?;

            Self::write_header(&mut writer, &file)?;
            Self::write_body(&mut writer, &file)?;

            Self::close_tag(&mut writer, TagCtx::File.to_str())?;
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

    fn write_text(writer: &mut Writer<Cursor<Vec<u8>>>, text: &str) -> WriterResult {
        let elem = BytesText::from_plain_str(text);
        writer.write_event(Event::Text(elem))?;
        Ok(())
    }

    fn close_tag(writer: &mut Writer<Cursor<Vec<u8>>>, tag: &str) -> WriterResult {
        let elem = BytesEnd::owned(tag.as_bytes().to_vec());
        writer.write_event(Event::End(elem))?;
        writer.write_event(Event::Eof)?;
        Ok(())
    }
}

impl WriterXliff12 {
    fn write_body(writer: &mut Writer<Cursor<Vec<u8>>>, file: &TranslationFile) -> WriterResult {
        Self::open_tag(writer, TagCtx::Body.to_str(), None)?;

        for unit in &file.units {
            match &unit.source {
                None => (),
                Some(unit_source) => {
                    Self::open_tag(
                        writer,
                        TagCtx::Unit.to_str(),
                        Some(Self::unit_attributes(unit)),
                    )?;

                    Self::open_tag(writer, TagCtx::Source.to_str(), None)?;
                    Self::write_text(writer, unit_source.text.as_str())?;
                    Self::close_tag(writer, TagCtx::Source.to_str())?;

                    match &unit.target {
                        None => (),
                        Some(unit_target) => {
                            Self::open_tag(writer, TagCtx::Target.to_str(), None)?;
                            Self::write_text(writer, unit_target.text.as_str())?;
                            Self::close_tag(writer, TagCtx::Target.to_str())?;
                        }
                    }

                    match &unit.note {
                        None => (),
                        Some(unit_note) => {
                            Self::open_tag(writer, TagCtx::Note.to_str(), None)?;
                            Self::write_text(writer, unit_note.text.as_str())?;
                            Self::close_tag(writer, TagCtx::Note.to_str())?;
                        }
                    }

                    Self::close_tag(writer, TagCtx::Unit.to_str())?;
                }
            }
        }

        Self::close_tag(writer, TagCtx::Body.to_str())?;
        Ok(())
    }

    fn unit_attributes(unit: &Unit) -> Vec<(&str, &str)> {
        vec![
            ("id", unit.id.as_str()),
            ("translate", Self::unit_translate_value(unit)),
            ("xml:space" = "preserve"),
        ]
    }

    fn unit_translate_value(unit: &Unit) -> &str {
        match unit.translate {
            true => "true",
            false => "false",
        }
    }
}

impl WriterXliff12 {
    fn write_header(
        mut writer: &mut Writer<Cursor<Vec<u8>>>,
        file: &TranslationFile,
    ) -> WriterResult {
        if let Some(file_header) = &file.header {
            Self::open_tag(&mut writer, TagCtx::Header.to_str(), None)?;
            for tool in &file_header.tools {
                Self::open_tag(&mut writer, TagCtx::Tool.to_str(), Some(tool.attributes()))?;
                Self::close_tag(&mut writer, TagCtx::Tool.to_str())?;
            }
            for note in &file_header.notes {
                Self::open_tag(&mut writer, TagCtx::Note.to_str(), None)?;
                Self::write_text(&mut writer, note.text.as_str())?;
                Self::close_tag(&mut writer, TagCtx::Note.to_str())?;
            }
            Self::close_tag(&mut writer, TagCtx::Header.to_str())?;
        }
        Ok(())
    }
}
