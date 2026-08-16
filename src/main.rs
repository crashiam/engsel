use rlsx::parse_xlsx_to_json;
use std::path::Path;

fn main() {
    // Example usage of the rlsx crate
    println!("rlsx example usage");

    // In a real application, you would provide a path to an actual XLSX file
    let path = Path::new("example.xlsx");

    match parse_xlsx_to_json(path) {
        Ok(json) => {
            println!("Successfully parsed XLSX file to JSON");
            println!("JSON: {}", json);
        }
        Err(e) => {
            println!("Error parsing XLSX file: {}", e);
        }
    }
}
