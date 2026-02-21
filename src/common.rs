pub fn convert_date_to_fugle_format(date: &str) -> String {
    // Convert date from "YYYYMMDD" to "YYYY-MM-DD"
    format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
}

#[allow(dead_code)]
pub fn convert_fugle_date_to_yyyymmdd(date: &str) -> String {
    // Convert date from "YYYY-MM-DD" to "YYYYMMDD"
    date.replace("-", "")
}

fn format_commas(value: u64) -> String {
    let s = value.to_string();
    let bytes = s.as_bytes();
    let mut result = String::new();
    let len = bytes.len();
    for (i, &b) in bytes.iter().enumerate() {
        result.push(b as char);
        if (len - i - 1).is_multiple_of(3) && i != len - 1 {
            result.push(',');
        }
    }
    result
}

pub fn str_volume(volume: u64) -> String {
    format_commas((volume as f64 / 1000.0) as u64)
}

use crate::stocks::data::Data;
use crate::stocks::data_company::DataCompany;

pub fn get_company_data<'a>(data: &'a Data, stock_no: &str) -> &'a DataCompany {
    data.data_company.get(stock_no).expect("找不到股票資料")
}

use crate::stocks::data_company::StockDataWithNo;

pub fn print_lower_30_percent_list(data: &Data, results: &[StockDataWithNo]) {
    println!(
        "{:<9}{:<5}{:>6}{:>6}{:>8}  公司名稱",
        "日期", "台股", "成交張數", "收盤價", "-30%",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * 0.7,
            data.company_map.get_name(&result.stock_no),
        );
    }
}
