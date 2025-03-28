use crate::xml_read_loop;

use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Spreadsheet;
use crate::structs::Stylesheet;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

pub fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(arv.by_name(PKG_STYLES)?);
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(true);

    let theme = spreadsheet.get_theme().clone();

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"styleSheet" {
                let mut obj = Stylesheet::default();
                obj.set_attributes(&mut reader, e);
                obj.make_style();
                spreadsheet.set_stylesheet(obj);
            }
        },
        Event::Eof => break
    );

    Ok(())
}
