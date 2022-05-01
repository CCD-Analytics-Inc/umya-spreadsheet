// a:font
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SupplementalFont {
    script: StringValue,
    typeface: StringValue,
}
impl SupplementalFont {
    pub fn get_script(&self) -> &str {
        self.script.get_value()
    }

    pub fn set_script<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.script.set_value(value);
        self
    }

    pub fn get_typeface(&self) -> &str {
        self.typeface.get_value()
    }

    pub fn set_typeface<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.typeface.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"script") {
            Some(v) => {
                self.script.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"typeface") {
            Some(v) => {
                self.typeface.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:font
        write_start_tag(
            writer,
            "a:font",
            vec![
                ("script", self.script.get_value_string()),
                ("typeface", self.typeface.get_value_string()),
            ],
            true,
        );
    }
}
