use super::XlsxError;
use super::model::{Sheet, Workbook};
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;

pub struct WorkbookParser {
    pub sheets: Vec<SheetInfo>,
}

#[derive(Debug, Clone)]
pub struct SheetInfo {
    pub name: String,
    pub path: String,
}

impl WorkbookParser {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { sheets: Vec::new() }
    }

    pub fn parse(content: &[u8]) -> Result<Self, XlsxError> {
        let mut reader = Reader::from_reader(Cursor::new(content));
        reader.check_end_names(true);
        reader.expand_empty_elements(true);

        let mut buffer = Vec::new();
        let mut sheets = Vec::new();

        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Start(e)) => {
                    if e.local_name().as_ref() == b"sheet" {
                        let mut sheet_name = String::new();
                        let mut sheet_id = String::new();

                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.local_name().as_ref() == b"name" {
                                sheet_name =
                                    String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            } else if attr.key.local_name().as_ref() == b"r:id" {
                                sheet_id = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            }
                        }

                        // For now, we'll assume the sheet path is xl/worksheets/sheetN.xml
                        // This will be updated when we parse relationships
                        let sheet_num = sheets.len() + 1;
                        let sheet_path = format!("xl/worksheets/sheet{}.xml", sheet_num);

                        sheets.push(SheetInfo {
                            name: sheet_name,
                            path: sheet_path,
                        });
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(XlsxError::XmlParsing(e)),
                _ => (),
            }
            buffer.clear();
        }

        Ok(Self { sheets })
    }

    // Parse workbook relationships to get sheet paths
    pub fn parse_relationships(content: &[u8]) -> Result<HashMap<String, String>, XlsxError> {
        let mut reader = Reader::from_reader(Cursor::new(content));
        reader.check_end_names(true);
        reader.expand_empty_elements(true);

        let mut buffer = Vec::new();
        let mut relationships = HashMap::new();

        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Start(e)) => {
                    if e.local_name().as_ref() == b"Relationship" {
                        let mut id = String::new();
                        let mut target = String::new();

                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.local_name().as_ref() == b"Id" {
                                id = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            } else if attr.key.local_name().as_ref() == b"Target" {
                                target = String::from_utf8_lossy(attr.value.as_ref()).to_string();
                            }
                        }

                        if !id.is_empty() && !target.is_empty() {
                            relationships.insert(id, format!("xl/{}", target));
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(XlsxError::XmlParsing(e)),
                _ => (),
            }
            buffer.clear();
        }

        Ok(relationships)
    }
}

pub fn parse_xlsx_to_workbook<P: AsRef<Path>>(path: P) -> Result<Workbook, XlsxError> {
    let mut archive = crate::zip_reader::XlsxArchive::new_from_path(path)?;

    // Parse shared strings if they exist
    let shared_strings = if archive.has_file("xl/sharedStrings.xml") {
        let shared_strings_content = archive.get_file_content("xl/sharedStrings.xml")?;
        Some(crate::shared_strings::SharedStrings::parse(
            &shared_strings_content,
        )?)
    } else {
        None
    };

    // Parse styles if they exist
    let _styles = if archive.has_file("xl/styles.xml") {
        let styles_content = archive.get_file_content("xl/styles.xml")?;
        Some(crate::styles::Styles::parse(&styles_content)?)
    } else {
        None
    };

    // Parse theme if it exists
    let _theme = if archive.has_file("xl/theme/theme1.xml") {
        let theme_content = archive.get_file_content("xl/theme/theme1.xml")?;
        Some(crate::theme::Theme::parse(&theme_content)?)
    } else {
        None
    };

    // Parse workbook.xml to get sheet information
    let workbook_content = archive.get_file_content("xl/workbook.xml")?;
    let mut workbook_parser = WorkbookParser::parse(&workbook_content)?;

    // Parse workbook relationships if they exist
    let relationships_file = "xl/_rels/workbook.xml.rels";
    if archive.has_file(relationships_file) {
        let relationships_content = archive.get_file_content(relationships_file)?;
        let _relationships = WorkbookParser::parse_relationships(&relationships_content)?;

        // Update sheet paths using relationships
        for _sheet_info in &mut workbook_parser.sheets {
            // For simplicity, we're not actually updating the paths based on relationships
            // This would require parsing the sheet IDs from the workbook and mapping them to relationships
            // For now, we'll keep the existing sheet path logic
        }
    }

    // Parse each sheet
    let mut sheets = Vec::new();

    for sheet_info in workbook_parser.sheets {
        let sheet_content = archive.get_file_content(&sheet_info.path)?;
        let worksheet =
            crate::worksheet::Worksheet::parse(&sheet_content, &shared_strings, &_styles)?;

        let sheet = Sheet {
            name: sheet_info.name,
            cells: worksheet.cells,
            rows: worksheet.rows,
        };

        sheets.push(sheet);
    }

    Ok(Workbook { sheets })
}
