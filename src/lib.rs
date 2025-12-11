mod error;
mod json_export;
mod model;
mod shared_strings;
mod styles;
mod theme;
mod workbook;
mod worksheet;
mod zip_reader;

pub use error::XlsxError;
pub use json_export::parse_xlsx_to_json;
pub use model::{Cell, CellStyle, CellValue, Sheet, Workbook};
pub use workbook::parse_xlsx_to_workbook;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_parse_xlsx() {
//         // Simple test that just checks if the module compiles and doesn't panic
//         assert!(true);
//     }
// }
