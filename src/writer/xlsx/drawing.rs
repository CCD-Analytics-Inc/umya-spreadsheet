use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io;

use ::structs::Worksheet;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/drawings";

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    drawing_id: &usize,
    arv: &mut zip::ZipWriter<W>
) -> Result<(), XlsxError> 
{
    if worksheet.has_drawing_object() == false {
        return Ok(());
    }

    let file_name = format!("drawing{}.xml", drawing_id);

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    worksheet.get_worksheet_drawing().write_to(&mut writer);
    
    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), arv, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}
