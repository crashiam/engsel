use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CellStyle {
    pub bg_color: Option<String>, // e.g. "#FFEEAA"
    pub fg_color: Option<String>,
    pub font_family: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_size: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellValue {
    String(String),
    Number(f64),
    Bool(bool),
    Date(DateTime<Utc>),
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub address: String, // e.g. "A1"
    pub value: CellValue,
    pub style: Option<CellStyle>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub name: String,
    pub cells: Vec<Cell>,
    pub rows: Vec<Vec<Cell>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Workbook {
    pub sheets: Vec<Sheet>,
}
