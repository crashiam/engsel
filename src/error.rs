use quick_xml::events::attributes::AttrError;
use quick_xml::Error as XmlError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum XlsxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("XML parsing error: {0}")]
    XmlParsing(#[from] XmlError),

    #[error("XML attribute error: {0}")]
    XmlAttr(#[from] AttrError),

    #[error("Parse error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Parse error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("Missing required part: {0}")]
    MissingPart(String),

    #[error("Invalid cell reference: {0}")]
    InvalidCellRef(String),

    #[error("Failed to parse date: {0}")]
    InvalidDate(String),

    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    #[error("Unknown cell type: {0}")]
    UnknownCellType(String),

    #[error("Other error: {0}")]
    Other(String),
}
