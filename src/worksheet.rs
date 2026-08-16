use super::model::{Cell, CellValue};
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::Cursor;

pub struct Worksheet {
    pub cells: Vec<Cell>,
    pub rows: Vec<Vec<Cell>>,
}

impl Worksheet {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn parse(
        content: &[u8],
        shared_strings: &Option<crate::shared_strings::SharedStrings>,
        styles: &Option<crate::styles::Styles>,
    ) -> Result<Self, XlsxError> {
        let mut reader = Reader::from_reader(Cursor::new(content));
        reader.check_end_names(true);
        reader.expand_empty_elements(true);

        let mut buffer = Vec::new();
        let mut cells = Vec::new();
        let mut rows = Vec::new();
        let mut current_row = Vec::new();

        let mut in_cell = false;
        let mut current_address = String::new();
        let mut current_value = String::new();
        let mut current_type = String::new();
        let mut current_style_id = None;

        loop {
            let event = reader.read_event_into(&mut buffer)?;
            match event {
                Event::Start(e) => {
                    // Create a local name that lives longer
                    let local_name = e.local_name().as_ref().to_owned();
                    if local_name == b"row" {
                        current_row.clear();
                    } else if local_name == b"c" {
                        in_cell = true;
                        current_value.clear();
                        current_type.clear();
                        current_style_id = None;

                        // Parse cell attributes
                        for attr in e.attributes() {
                            let attr = attr?;
                            // Create a key that lives longer
                            let local_key = attr.key.local_name().as_ref().to_owned();
                            if local_key == b"r" {
                                current_address =
                                    String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            } else if local_key == b"t" {
                                current_type =
                                    String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            } else if local_key == b"s" {
                                let style_str =
                                    String::from_utf8_lossy(attr.value.as_ref()).to_string();
                                current_style_id = Some(style_str.parse::<usize>()?);
                            }
                        }
                    }
                }
                Event::Text(e) => {
                    if in_cell {
                        current_value.push_str(&e.unescape()?);
                    }
                }
                Event::End(e) => {
                    // Create a local name that lives longer
                    let local_name = e.local_name().as_ref().to_owned();
                    if local_name == b"c" {
                        in_cell = false;

                        // Create cell
                        let cell = Self::create_cell(
                            &current_address,
                            &current_value,
                            &current_type,
                            current_style_id,
                            shared_strings,
                            styles,
                        )?;

                        cells.push(cell.clone());
                        current_row.push(cell);
                    } else if local_name == b"row" {
                        if !current_row.is_empty() {
                            rows.push(current_row.clone());
                        }
                    }
                }
                Event::Eof => break,
                _ => (),
            }
            buffer.clear();
        }

        Ok(Self { cells, rows })
    }

    fn create_cell(
        address: &str,
        value: &str,
        cell_type: &str,
        style_id: Option<usize>,
        shared_strings: &Option<crate::shared_strings::SharedStrings>,
        styles: &Option<crate::styles::Styles>,
    ) -> Result<Cell, XlsxError> {
        let cell_style = if let Some(style_id) = style_id {
            if let Some(styles) = styles {
                styles.get_style(style_id).cloned()
            } else {
                None
            }
        } else {
            None
        };

        if value.is_empty() {
            return Ok(Cell {
                address: address.to_string(),
                value: CellValue::Empty,
                style: cell_style,
            });
        }

        let cell_value = match cell_type {
            "s" => {
                // Shared string
                if let Some(shared_strings) = shared_strings {
                    let index = value.parse::<usize>()?;
                    if let Some(s) = shared_strings.get(index) {
                        CellValue::String(s.to_string())
                    } else {
                        CellValue::String(value.to_string())
                    }
                } else {
                    CellValue::String(value.to_string())
                }
            }
            "b" => {
                // Boolean
                CellValue::Bool(value == "1")
            }
            "" | "n" => {
                // Number or date
                let num = value.parse::<f64>()?;
                CellValue::Number(num)
            }
            _ => {
                // Unknown type, treat as string
                CellValue::String(value.to_string())
            }
        };

        Ok(Cell {
            address: address.to_string(),
            value: cell_value,
            style: cell_style,
        })
    }
}
