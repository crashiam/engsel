use engsel::parse_xlsx_to_json;

fn main() {
    if let Err(err) = parse_xlsx_to_json("test.xlsx") {
        eprintln!("{}", err);
    }
}
