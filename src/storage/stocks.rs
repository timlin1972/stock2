use std::fs;
use std::fs::File;
use std::io::BufWriter;

use crate::stocks::data_company::StockData;

const MODULE_NAME: &str = "storage::stocks";
const DATA_DIR: &str = "data";
const YEAR_FROM: i32 = 2025;
const YEAR_TO: i32 = 2026;

pub fn save(stock_no: &str, year: &str, data: &Vec<StockData>) {
    let data_company_dir = format!("{DATA_DIR}/{stock_no}");
    if fs::metadata(&data_company_dir).is_err() {
        fs::create_dir_all(&data_company_dir).unwrap();
    }

    let data_company_file = format!("{data_company_dir}/{year}.json");

    // Implementation for writing data to storage
    let file = File::create(&data_company_file).unwrap();
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &data).unwrap();
}

pub fn read(stock_no: &str) -> Vec<StockData> {
    let mut stock_data = Vec::new();
    for year in YEAR_FROM..=YEAR_TO {
        let data_company_file = format!("{DATA_DIR}/{stock_no}/{year}.json");
        // println!("[{MODULE_NAME}] Reading data from {data_company_file}");
        if fs::metadata(&data_company_file).is_ok() {
            let file = File::open(&data_company_file).unwrap();
            let reader = std::io::BufReader::new(file);
            let yearly_data: Vec<StockData> = serde_json::from_reader(reader).unwrap();
            stock_data.extend(yearly_data);
        } else {
            println!(
                "[{MODULE_NAME}] Data file not found for stock: {stock_no}, year: {year}. Please run the fetch function first."
            );
        }
    }

    stock_data.sort_by(|a, b| a.date.cmp(&b.date));
    stock_data
}
