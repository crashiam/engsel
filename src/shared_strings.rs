use super::XlsxError;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Cursor;

pub struct SharedStrings {
    pub strings: Vec<String>,
}

impl SharedStrings {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
        }
    }

    pub fn parse(content: &[u8]) -> Result<Self, XlsxError> {
        let mut reader = Reader::from_reader(Cursor::new(content));
        reader.check_end_names(true);
        reader.expand_empty_elements(true);

        let mut buffer = Vec::new();
        let mut strings = Vec::new();
        let mut in_t = false;
        let mut current_string = String::new();

        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Start(e)) => {
                    if e.local_name().as_ref() == b"si" {
                        current_string.clear();
                    } else if e.local_name().as_ref() == b"t" {
                        in_t = true;
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_t {
                        current_string.push_str(&e.unescape()?);
                    }
                }
                Ok(Event::End(e)) => {
                    if e.local_name().as_ref() == b"t" {
                        in_t = false;
                    } else if e.local_name().as_ref() == b"si" {
                        strings.push(current_string.clone());
                        current_string.clear();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(XlsxError::XmlParsing(e)),
                _ => (),
            }
            buffer.clear();
        }

        Ok(Self { strings })
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.strings.get(index).map(|s| s.as_str())
    }
}
