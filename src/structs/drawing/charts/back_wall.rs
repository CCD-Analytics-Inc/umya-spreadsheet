// c:backWall
use super::ShapeProperties;
use super::Thickness;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct BackWall {
    thickness: Option<Thickness>,
    shape_properties: Option<ShapeProperties>,
}

impl BackWall {
    pub fn get_thickness(&self) -> Option<&Thickness> {
        self.thickness.as_ref()
    }

    pub fn get_thickness_mut(&mut self) -> Option<&mut Thickness> {
        self.thickness.as_mut()
    }

    pub fn set_thickness(&mut self, value: Thickness) -> &mut BackWall {
        self.thickness = Some(value);
        self
    }

    pub fn get_shape_properties(&self) -> Option<&ShapeProperties> {
        self.shape_properties.as_ref()
    }

    pub fn get_shape_properties_mut(&mut self) -> Option<&mut ShapeProperties> {
        self.shape_properties.as_mut()
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) -> &mut Self {
        self.shape_properties = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"c:thickness" {
                    let mut obj = Thickness::default();
                    obj.set_attributes(reader, e);
                    self.set_thickness(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"c:spPr" {
                    let mut obj = ShapeProperties::default();
                    obj.set_attributes(reader, e);
                    self.set_shape_properties(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:backWall" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:backWall")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:backWall
        write_start_tag(writer, "c:backWall", vec![], false);

        // c:thickness
        if let Some(v) = &self.thickness {
            v.write_to(writer);
        }

        // c:spPr
        if let Some(v) = &self.shape_properties {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:backWall");
    }
}
