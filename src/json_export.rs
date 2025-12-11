use super::XlsxError;
use std::path::Path;

pub fn parse_xlsx_to_json<P: AsRef<Path>>(path: P) -> Result<String, XlsxError> {
    let workbook = crate::workbook::parse_xlsx_to_workbook(path)?;
    let json = serde_json::to_string(&workbook)?;
    Ok(json)
}
