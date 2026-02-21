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

use crate::stocks::data_company::StockData;

pub fn get_current_index_by_date(
    company_data: &[StockData],
    date: &str,
    range: usize,
) -> Option<usize> {
    let curr_date_index = company_data
        .iter()
        .position(|d| d.date == convert_date_to_fugle_format(date))?;
    if curr_date_index < range {
        None
    } else {
        Some(curr_date_index)
    }
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

pub fn print_line() {
    println!("--------------------------------------------------------------------------------");
}

use std::io;

use chrono::Local;

// get YYYYMMDD format of today's date
fn get_today_date() -> String {
    let today = Local::now().date_naive();
    today.format("%Y%m%d").to_string()
}

pub fn get_date_input() -> String {
    println!("請輸入日期 (YYYYMMDD):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let mut input = input.trim().to_string();
    if input.is_empty() {
        input = get_today_date();
        println!("使用今天的日期: {input}");
    }
    input
}

pub fn print_lower_upper_30_percent_list(data: &Data, results: &[StockDataWithNo]) {
    println!(
        "{:<9}{:<5}{:>6}{:>6}{:>8}{:>8}  公司名稱",
        "日期", "台股", "成交張數", "收盤價", "+30%", "-30%",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * 1.3,
            result.stock_data.close * 0.7,
            data.company_map.get_name(&result.stock_no),
        );
    }
}
