use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellStyle {
    pub bg_color: Option<String>, // e.g. "#FFEEAA"
    pub fg_color: Option<String>,
    pub font_family: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_size: Option<f64>,
}

impl Default for CellStyle {
    fn default() -> Self {
        Self {
            bg_color: None,
            fg_color: None,
            font_family: None,
            bold: false,
            italic: false,
            underline: false,
            font_size: None,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub name: String,
    pub cells: Vec<Cell>,
    pub rows: Vec<Vec<Cell>>,
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            name: String::new(),
            cells: Vec::new(),
            rows: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workbook {
    pub sheets: Vec<Sheet>,
}

impl Default for Workbook {
    fn default() -> Self {
        Self { sheets: Vec::new() }
    }
}
