use super::XlsxError;
use super::model::CellStyle;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Cursor;

// Helper structs to store parsed style information
struct Font {
    bold: bool,
    italic: bool,
    underline: bool,
    font_size: Option<f64>,
    font_family: Option<String>,
    color: Option<String>,
}

struct Fill {
    bg_color: Option<String>,
    #[allow(dead_code)]
    fg_color: Option<String>,
}

struct CellXf {
    font_id: usize,
    fill_id: usize,
    // We'll add more properties as needed
}

pub struct Styles {
    #[allow(dead_code)]
    fonts: Vec<Font>,
    #[allow(dead_code)]
    fills: Vec<Fill>,
    #[allow(dead_code)]
    cell_xfs: Vec<CellXf>,
    pub cell_styles: Vec<CellStyle>,
}

impl Styles {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            fonts: Vec::new(),
            fills: Vec::new(),
            cell_xfs: Vec::new(),
            cell_styles: Vec::new(),
        }
    }

    pub fn parse(content: &[u8]) -> Result<Self, XlsxError> {
        let mut reader = Reader::from_reader(Cursor::new(content));
        reader.check_end_names(true);
        reader.expand_empty_elements(true);

        let mut buffer = Vec::new();
        let mut fonts = Vec::new();
        let mut fills = Vec::new();
        let mut cell_xfs = Vec::new();

        let mut in_fonts = false;
        let mut in_fills = false;
        let mut in_cell_xfs = false;

        loop {
            let event = reader.read_event_into(&mut buffer)?;
            match event {
                Event::Start(e) => {
                    // Create a local name that lives longer
                    let local_name = e.local_name().as_ref().to_owned();

                    if local_name == b"fonts" {
                        in_fonts = true;
                    } else if local_name == b"fills" {
                        in_fills = true;
                    } else if local_name == b"cellXfs" {
                        in_cell_xfs = true;
                    } else if in_fonts && local_name == b"font" {
                        // Parse font
                        let font = Self::parse_font(&mut reader, &mut buffer)?;
                        fonts.push(font);
                    } else if in_fills && local_name == b"fill" {
                        // Parse fill
                        let fill = Self::parse_fill(&mut reader, &mut buffer)?;
                        fills.push(fill);
                    } else if in_cell_xfs && local_name == b"xf" {
                        // Parse cellXf
                        let cell_xf = Self::parse_cell_xf(&e)?;
                        cell_xfs.push(cell_xf);
                    }
                }
                Event::End(e) => {
                    // Create a local name that lives longer
                    let local_name = e.local_name().as_ref().to_owned();

                    if local_name == b"fonts" {
                        in_fonts = false;
                    } else if local_name == b"fills" {
                        in_fills = false;
                    } else if local_name == b"cellXfs" {
                        in_cell_xfs = false;
                    }
                }
                Event::Eof => break,
                _ => (),
            }
            buffer.clear();
        }

        // Create cell styles from parsed data
        let cell_styles = Self::create_cell_styles(&fonts, &fills, &cell_xfs);

        Ok(Self {
            fonts,
            fills,
            cell_xfs,
            cell_styles,
        })
    }

    fn parse_font(
        reader: &mut Reader<Cursor<&[u8]>>,
        buffer: &mut Vec<u8>,
    ) -> Result<Font, XlsxError> {
        let mut bold = false;
        let mut italic = false;
        let mut underline = false;
        let mut font_size = None;
        let font_family = None;
        let mut color = None;

        loop {
            let event = reader.read_event_into(buffer)?;
            match event {
                Event::Start(e) => {
                    let name = e.local_name().as_ref().to_owned();

                    if name == b"b" {
                        bold = true;
                    } else if name == b"i" {
                        italic = true;
                    } else if name == b"u" {
                        underline = true;
                    } else if name == b"sz" {
                        // Parse font size
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.local_name().as_ref() == b"val" {
                                let size_str =
                                    String::from_utf8_lossy(attr.value.as_ref()).to_string();
                                font_size = Some(size_str.parse::<f64>()?);
                                break;
                            }
                        }
                    } else if name == b"name" {
                        // Parse font family
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.local_name().as_ref() == b"val" {
                                let _font_family =
                                    Some(String::from_utf8_lossy(attr.value.as_ref()).to_string());
                                break;
                            }
                        }
                    } else if name == b"color" {
                        // Parse font color
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.local_name().as_ref() == b"rgb" {
                                let rgb = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                                color = Some(format!("#{}", rgb));
                                break;
                            }
                        }
                    }
                }
                Event::End(e) if e.local_name().as_ref() == b"font" => {
                    break;
                }
                Event::Eof => break,
                _ => (),
            }
            buffer.clear();
        }

        Ok(Font {
            bold,
            italic,
            underline,
            font_size,
            font_family,
            color,
        })
    }

    fn parse_fill(
        reader: &mut Reader<Cursor<&[u8]>>,
        buffer: &mut Vec<u8>,
    ) -> Result<Fill, XlsxError> {
        let mut bg_color = None;
        let mut fg_color = None;

        loop {
            let event = reader.read_event_into(buffer)?;
            match event {
                Event::Start(e) => {
                    let name = e.local_name().as_ref().to_owned();

                    if name == b"patternFill" {
                        // Parse pattern fill
                        loop {
                            let inner_event = reader.read_event_into(buffer)?;
                            match inner_event {
                                Event::Start(inner_e) => {
                                    // Create a longer-lived sub_name
                                    let sub_name = inner_e.local_name().as_ref().to_owned();
                                    if sub_name == b"fgColor" {
                                        // Get rgb attribute
                                        for attr in inner_e.attributes() {
                                            let attr = attr?;
                                            if attr.key.local_name().as_ref() == b"rgb" {
                                                let rgb =
                                                    String::from_utf8_lossy(attr.value.as_ref())
                                                        .to_string();
                                                fg_color = Some(format!("#{}", rgb));
                                                break;
                                            }
                                        }
                                    } else if sub_name == b"bgColor" {
                                        // Get rgb attribute
                                        for attr in inner_e.attributes() {
                                            let attr = attr?;
                                            if attr.key.local_name().as_ref() == b"rgb" {
                                                let rgb =
                                                    String::from_utf8_lossy(attr.value.as_ref())
                                                        .to_string();
                                                bg_color = Some(format!("#{}", rgb));
                                                break;
                                            }
                                        }
                                    }
                                }
                                Event::End(inner_e)
                                    if inner_e.local_name().as_ref() == b"patternFill" =>
                                {
                                    break;
                                }
                                Event::Eof => break,
                                _ => (),
                            }
                            buffer.clear();
                        }
                    }
                }
                Event::End(e) if e.local_name().as_ref() == b"fill" => {
                    break;
                }
                Event::Eof => break,
                _ => (),
            }
            buffer.clear();
        }

        Ok(Fill { bg_color, fg_color })
    }

    fn parse_cell_xf(e: &quick_xml::events::BytesStart) -> Result<CellXf, XlsxError> {
        let mut font_id = 0;
        let mut fill_id = 0;

        for attr in e.attributes() {
            let attr = attr?;
            // Get local name as a owned slice
            let local_name = attr.key.local_name().as_ref().to_owned();
            if local_name == b"fontId" {
                font_id = String::from_utf8_lossy(attr.value.as_ref()).parse::<usize>()?;
            } else if local_name == b"fillId" {
                fill_id = String::from_utf8_lossy(attr.value.as_ref()).parse::<usize>()?;
            }
        }

        Ok(CellXf { font_id, fill_id })
    }

    fn create_cell_styles(fonts: &[Font], fills: &[Fill], cell_xfs: &[CellXf]) -> Vec<CellStyle> {
        let mut cell_styles = Vec::new();

        // Add default style
        cell_styles.push(CellStyle::default());

        // Create styles from cell_xfs
        for cell_xf in cell_xfs {
            let font = fonts.get(cell_xf.font_id).unwrap_or(&Font {
                bold: false,
                italic: false,
                underline: false,
                font_size: None,
                font_family: None,
                color: None,
            });

            let fill = fills.get(cell_xf.fill_id).unwrap_or(&Fill {
                bg_color: None,
                fg_color: None,
            });

            let cell_style = CellStyle {
                bg_color: fill.bg_color.clone(),
                fg_color: font.color.clone(),
                font_family: font.font_family.clone(),
                bold: font.bold,
                italic: font.italic,
                underline: font.underline,
                font_size: font.font_size,
            };

            cell_styles.push(cell_style);
        }

        cell_styles
    }

    pub fn get_style(&self, index: usize) -> Option<&CellStyle> {
        self.cell_styles.get(index)
    }
}
