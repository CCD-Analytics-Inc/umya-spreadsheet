use super::driver::*;
use super::XlsxError;
use quick_xml::Writer;
use std::io;
use structs::Worksheet;

const SUB_DIR: &'static str = "xl/drawings";

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    drawing_id: &usize,
    vml_drawing_id: &usize,
    arv: &mut zip::ZipWriter<W>,
) -> Result<(), XlsxError> {
    if worksheet.has_legacy_drawing() == false {
        return Ok(());
    }

    let file_name = format!("vmlDrawing{}.vml", vml_drawing_id);

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // xml
    write_start_tag(
        &mut writer,
        "xml",
        vec![
            ("xmlns:v", "urn:schemas-microsoft-com:vml"),
            ("xmlns:o", "urn:schemas-microsoft-com:office:office"),
            ("xmlns:x", "urn:schemas-microsoft-com:office:excel"),
        ],
        false,
    );

    // o:shapelayout
    write_start_tag(&mut writer, "o:shapelayout", vec![("v:ext", "edit")], false);

    // o:idmap
    write_start_tag(
        &mut writer,
        "o:idmap",
        vec![("v:ext", "edit"), ("data", drawing_id.to_string().as_str())],
        true,
    );

    write_end_tag(&mut writer, "o:shapelayout");

    let mut id = (drawing_id * 1000) + 25;

    // ole_object
    if worksheet.has_ole_objects() {
        // v:shapetype
        write_start_tag(
            &mut writer,
            "v:shapetype",
            vec![
                ("id", "_x0000_t75"),
                ("coordsize", "21600,21600"),
                ("o:spt", "75"),
                ("o:preferrelative", "t"),
                ("path", "m@4@5l@4@11@9@11@9@5xe"),
                ("filled", "f"),
                ("stroked", "f"),
            ],
            false,
        );

        // v:stroke
        write_start_tag(&mut writer, "v:stroke", vec![("joinstyle", "miter")], true);

        // v:formulas
        write_start_tag(&mut writer, "v:formulas", vec![], false);
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "if lineDrawn pixelLineWidth 0")],
            true,
        );
        write_start_tag(&mut writer, "v:f", vec![("eqn", "sum @0 1 0")], true);
        write_start_tag(&mut writer, "v:f", vec![("eqn", "sum 0 0 @1")], true);
        write_start_tag(&mut writer, "v:f", vec![("eqn", "prod @2 1 2")], true);
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @3 21600 pixelWidth")],
            true,
        );
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @3 21600 pixelHeight")],
            true,
        );
        write_start_tag(&mut writer, "v:f", vec![("eqn", "sum @0 0 1")], true);
        write_start_tag(&mut writer, "v:f", vec![("eqn", "prod @6 1 2")], true);
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @7 21600 pixelWidth")],
            true,
        );
        write_start_tag(&mut writer, "v:f", vec![("eqn", "sum @8 21600 0")], true);
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @7 21600 pixelHeight")],
            true,
        );
        write_start_tag(&mut writer, "v:f", vec![("eqn", "sum @10 21600 0")], true);
        write_end_tag(&mut writer, "v:formulas");

        // v:path
        write_start_tag(
            &mut writer,
            "v:path",
            vec![
                ("o:extrusionok", "f"),
                ("gradientshapeok", "t"),
                ("o:connecttype", "rect"),
            ],
            true,
        );

        // o:lock
        write_start_tag(
            &mut writer,
            "o:lock",
            vec![("v:ext", "edit"), ("aspectratio", "t")],
            true,
        );

        write_end_tag(&mut writer, "v:shapetype");

        let mut r_id = 1;
        for ole_object in worksheet.get_ole_objects().get_ole_object() {
            // v:shape
            ole_object.get_shape().write_to(&mut writer, &id, &r_id);
            r_id += 1;
            id += 1;
        }
    }

    // comment
    if worksheet.has_comments() {
        // v:shapetype
        write_start_tag(
            &mut writer,
            "v:shapetype",
            vec![
                ("id", "_x0000_t202"),
                ("coordsize", "21600,21600"),
                ("o:spt", "202"),
                ("path", "m,l,21600r21600,l21600,xe"),
            ],
            false,
        );

        // v:stroke
        write_start_tag(&mut writer, "v:stroke", vec![("joinstyle", "miter")], true);

        // v:path
        write_start_tag(
            &mut writer,
            "v:path",
            vec![("gradientshapeok", "t"), ("o:connecttype", "rect")],
            true,
        );

        write_end_tag(&mut writer, "v:shapetype");

        for comment in worksheet.get_comments() {
            // v:shape
            comment.get_shape().write_to(&mut writer, &id, &0);
            id += 1;
        }
    }

    write_end_tag(&mut writer, "xml");

    let _ = make_file_from_writer(&file_name, arv, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}

//fn get_style_string_comment(comment: &Comment) -> String {
//    let style = format!(
//        "position:absolute;margin-left:{};margin-top:{};width:{};height:{};z-index:1;visibility:{}",
//        comment.get_margin_left(),
//        comment.get_margin_top(),
//        comment.get_width(),
//        comment.get_height(),
//        if comment.get_visible() == &true {"visible"} else {"hidden"}
//    );
//    style
//}
