# engsel - xlsx parser for Rust

A library for parsing `xlsx` files.

## Features

- Parse `xlsx` files into structured data models
- Export `xlsx` files to JSON format
- Support for multiple sheets in a single workbook
- Cell value types: String, Number, Boolean, Date, and Empty
- Basic cell styling support
- Shared strings parsing
- Worksheet parsing with cell and row-based access

## Installation

Add this to Cargo.toml

```toml
[dependencies]
engsel = { git = "https://github.com/crashiam/engsel.git", branch = "main" } 
```

## Usage

### Parsing to Structured Workbook

```rust
use std::path::Path;
use engsel::{parse_xlsx_to_workbook, Workbook};

fn main() {
    let path = Path::new("example.xlsx");
    
    match parse_xlsx_to_workbook(path) {
        Ok(workbook) => {
            println!("Successfully parsed XLSX file to Workbook structure");
            println!("Number of sheets: {}", workbook.sheets.len());
            
            // Access sheets
            for (i, sheet) in workbook.sheets.iter().enumerate() {
                println!("Sheet {}: {}", i + 1, sheet.name);
                println!("Number of cells: {}", sheet.cells.len());
                println!("Number of rows: {}", sheet.rows.len());
            }
        }
        Err(e) => {
            println!("Error parsing XLSX file: {}", e);
        }
    }
}
```

## API Reference

### Core Functions

#### `parse_xlsx_to_json<P: AsRef<Path>>(path: P) -> Result<String, XlsxError>`
Parses an `xlsx` file and returns its contents as a JSON string.

#### `parse_xlsx_to_workbook<P: AsRef<Path>>(path: P) -> Result<Workbook, XlsxError>`
Parses an `xlsx` file and returns a structured `Workbook` object.

### Data Structures

#### `Workbook`
Represents an entire `xlsx` workbook containing multiple sheets.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workbook {
    pub sheets: Vec<Sheet>,
}
```

#### `Sheet`
Represents a single sheet within a workbook.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub name: String,         // Sheet name
    pub cells: Vec<Cell>,     // All cells in the sheet
    pub rows: Vec<Vec<Cell>>, // Cells organized by rows
}
```

#### `Cell`
Represents a single cell in a worksheet.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub address: String,    // Cell address (e.g., "A1")
    pub value: CellValue,   // Cell value
    pub style: Option<CellStyle>, // Cell styling
}
```

#### `CellValue`
Represents the value of a cell with different types.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellValue {
    String(String),      // Text values
    Number(f64),         // Numeric values
    Bool(bool),          // Boolean values
    Date(DateTime<Utc>), // Date/time values
    Empty,               // Empty cells
}
```

#### `CellStyle`
Represents the styling information for a cell.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellStyle {
    pub bg_color: Option<String>,   // Background color (e.g., "#FFEEAA")
    pub fg_color: Option<String>,   // Foreground color
    pub font_family: Option<String>, // Font family
    pub bold: bool,                 // Bold text
    pub italic: bool,               // Italic text
    pub underline: bool,            // Underlined text
    pub font_size: Option<f64>,     // Font size
}
```

## Architecture

The engsel library is structured as follows:

- **lib.rs**: Main library entry point exposing public API
- **model.rs**: Core data structures (Workbook, Sheet, Cell, etc.)
- **workbook.rs**: Workbook parsing logic
- **worksheet.rs**: Worksheet parsing logic
- **shared_strings.rs**: Shared strings table parsing
- **styles.rs**: Styles parsing
- **theme.rs**: Theme parsing
- **zip_reader.rs**: ZIP archive handling for XLSX files
- **json_export.rs**: JSON export functionality
- **error.rs**: Error handling

### Parsing Flow

1. **Zip Archive Handling**: The XLSX file is opened as a ZIP archive
2. **Shared Strings Parsing**: If present, the shared strings table is parsed
3. **Styles Parsing**: If present, style information is parsed
4. **Workbook Parsing**: The workbook structure is parsed to get sheet information
5. **Worksheet Parsing**: Each worksheet is parsed individually
6. **Cell Processing**: Cells are extracted with their values and styles
7. **Data Structuring**: The parsed data is organized into the Workbook structure
8. **Optional JSON Conversion**: The Workbook can be converted to JSON format

## Examples

### Reading Cell Values

```rust
use std::path::Path;
use engsel::parse_xlsx_to_workbook;

fn main() {
    let path = Path::new("example.xlsx");
    
    if let Ok(workbook) = parse_xlsx_to_workbook(path) {
        // Get the first sheet
        if let Some(sheet) = workbook.sheets.first() {
            // Access a specific cell (e.g., A1)
            if let Some(cell) = sheet.cells.iter().find(|c| c.address == "A1") {
                match &cell.value {
                    engsel::CellValue::String(s) => println!("A1: {}", s),
                    engsel::CellValue::Number(n) => println!("A1: {}", n),
                    engsel::CellValue::Bool(b) => println!("A1: {}", b),
                    engsel::CellValue::Date(d) => println!("A1: {}", d),
                    engsel::CellValue::Empty => println!("A1: Empty"),
                }
            }
        }
    }
}
```

### Iterating Over Rows

```rust
use std::path::Path;
use engsel::parse_xlsx_to_workbook;

fn main() {
    let path = Path::new("example.xlsx");
    
    if let Ok(workbook) = parse_xlsx_to_workbook(path) {
        if let Some(sheet) = workbook.sheets.first() {
            println!("Sheet: {}", sheet.name);
            println!("Row data:");
            
            for (row_idx, row) in sheet.rows.iter().enumerate() {
                print!("Row {}: ", row_idx + 1);
                
                for (col_idx, cell) in row.iter().enumerate() {
                    if col_idx > 0 {
                        print!(" | ");
                    }
                    
                    match &cell.value {
                        engsel::CellValue::String(s) => print!("{}", s),
                        engsel::CellValue::Number(n) => print!("{}", n),
                        engsel::CellValue::Bool(b) => print!("{}", b),
                        engsel::CellValue::Date(d) => print!("{}", d.format("%Y-%m-%d")),
                        engsel::CellValue::Empty => print!("(empty)"),
                    }
                }
                println!();
            }
        }
    }
}
```

## Error Handling

The library returns an `XlsxError` enum with the following variants:

- `FileNotFound`: The specified XLSX file was not found
- `ZipError`: Error reading the ZIP archive
- `XmlParsing`: Error parsing XML content
- `IoError`: I/O error during file operations
- `Utf8Error`: UTF-8 encoding error
- `ParseError`: General parsing error

## Limitations

- Currently only supports reading `xlsx` files (no writing support)
- Some advanced `xlsx` features may not be supported
- Formula evaluation is not implemented
- Macro-enabled files `xlsm` are not supported

## Contributing

Forked from [rlsx](https://crates.io/crates/rlsx) as a learning purpose. Still under a heavy construction. Not ready for external contributions!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
