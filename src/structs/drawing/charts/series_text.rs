// c:tx
use crate::reader::driver::*;
use crate::structs::StringValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct SeriesText {
    value: StringValue,
}

impl SeriesText {
    pub fn get_value(&self) -> &str {
        self.value.get_value_str()
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.set_value(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:tx" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:tx")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:v
        write_start_tag(writer, "c:v", vec![], false);
        write_text_node(writer, self.value.get_value_str());
        write_end_tag(writer, "c:v");

        write_end_tag(writer, "c:tx");
    }
}
