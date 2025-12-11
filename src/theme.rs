use super::XlsxError;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Cursor;

pub struct Theme {
    #[allow(dead_code)]
    pub colors: std::collections::HashMap<String, String>,
}

impl Theme {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            colors: std::collections::HashMap::new(),
        }
    }

    pub fn parse(content: &[u8]) -> Result<Self, XlsxError> {
        let mut reader = Reader::from_reader(Cursor::new(content));
        reader.check_end_names(true);
        reader.expand_empty_elements(true);

        let mut buffer = Vec::new();
        let mut colors = std::collections::HashMap::new();

        loop {
            let event = reader.read_event_into(&mut buffer)?;
            match event {
                Event::Start(e) => {
                    // Create a local name slice that lives longer
                    let local_name_slice = e.local_name().as_ref().to_owned();
                    let local_name = &local_name_slice;

                    // Check for color elements
                    if let Some(color_name) = match local_name.as_slice() {
                        b"dk1" => Some("dk1"),
                        b"lt1" => Some("lt1"),
                        b"dk2" => Some("dk2"),
                        b"lt2" => Some("lt2"),
                        b"accent1" => Some("accent1"),
                        b"accent2" => Some("accent2"),
                        b"accent3" => Some("accent3"),
                        b"accent4" => Some("accent4"),
                        b"accent5" => Some("accent5"),
                        b"accent6" => Some("accent6"),
                        b"hlink" => Some("hlink"),
                        b"folHlink" => Some("folHlink"),
                        _ => None,
                    } {
                        let color_name_str = color_name.to_string();
                        // Parse the srgbClr element inside
                        let mut found_srgb = false;
                        let mut inner_buffer = Vec::new();

                        loop {
                            let inner_event = reader.read_event_into(&mut inner_buffer)?;
                            match inner_event {
                                Event::Start(inner_e) => {
                                    if inner_e.local_name().as_ref() == b"srgbClr" {
                                        // Get the val attribute which contains the RGB value
                                        for attr in inner_e.attributes() {
                                            let attr = attr?;
                                            if attr.key.local_name().as_ref() == b"val" {
                                                let rgb =
                                                    String::from_utf8_lossy(attr.value.as_ref())
                                                        .to_string();
                                                colors.insert(
                                                    color_name_str.clone(),
                                                    format!("#{}", rgb),
                                                );
                                                found_srgb = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                                Event::End(inner_e)
                                    if inner_e.local_name().as_ref() == local_name.as_slice() =>
                                {
                                    break;
                                }
                                Event::Eof => {
                                    break;
                                }
                                _ => (),
                            }
                            inner_buffer.clear();
                        }
                        if !found_srgb {
                            // If no srgbClr, check for sysClr (system color)
                            // For simplicity, we'll skip sysClr for now
                        }
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => (),
            }
            buffer.clear();
        }

        Ok(Self { colors })
    }

    #[allow(dead_code)]
    pub fn get_color(&self, name: &str) -> Option<&str> {
        self.colors.get(name).map(|s| s.as_str())
    }
}
