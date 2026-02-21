use std::io;

use crate::common;
use crate::scripts;
use crate::stocks::data::Data;

fn print_line() {
    println!("--------------------------------------------------------------------------------");
}

pub fn menu(data: &Data) {
    println!("回歸測試選單");

    // use 2317 as base stock to find date list
    let mut date_list = Vec::new();
    let company_data = common::get_company_data(data, "2317");

    for data in &company_data.stock_data {
        date_list.push(data.date.clone());
    }

    loop {
        println!("1. 烏雲罩頂");
        println!("2. 烏鴉躍空");
        println!("q/e. 退出 (Quit/Exit)");
        println!("請輸入選項：");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("讀取失敗");

        // 去掉換行符號
        let input = input.trim();

        match input {
            "1" => menu_dark_cloud_cover(data, &date_list),
            "2" => menu_upside_gap_two_crows(data, &date_list),
            "q" | "e" => {
                println!("退出選單");
                break;
            }
            _ => {
                println!("無效的選項，請重新輸入");
            }
        }
    }
}

fn get_year() -> Option<String> {
    println!("請輸入年分 (格式: YYYY):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let input = input.trim();

    match input {
        "2020" => Some("20200102".to_string()),
        "2021" => Some("20210104".to_string()),
        "2022" => Some("20220103".to_string()),
        "2023" => Some("20230103".to_string()),
        "2024" => Some("20240102".to_string()),
        "2025" => Some("20250102".to_string()),
        "2026" => Some("20260102".to_string()),
        _ => {
            println!("無效的年分，請輸入正確的格式 (YYYY) 2020-2026");
            None
        }
    }
}

fn menu_dark_cloud_cover(data: &Data, date_list: &[String]) {
    let input = match get_year() {
        Some(year) => year,
        None => return,
    };

    let mut results = Vec::new();
    let date_index = date_list
        .iter()
        .position(|d| d == &common::convert_date_to_fugle_format(&input))
        .unwrap();
    for date in &date_list[date_index..] {
        let date_results = scripts::complex::find_complex_dark_cloud_cover(
            data,
            &common::convert_fugle_date_to_yyyymmdd(date),
        );
        results.extend(date_results);
    }

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 單日黑雲壓頂",
        results.len(),
    );
    common::print_lower_30_percent_list(data, &results);
    print_line();
}

fn menu_upside_gap_two_crows(data: &Data, date_list: &[String]) {
    let input = match get_year() {
        Some(year) => year,
        None => return,
    };

    let mut results = Vec::new();
    let date_index = date_list
        .iter()
        .position(|d| d == &common::convert_date_to_fugle_format(&input))
        .unwrap();
    for date in &date_list[date_index..] {
        let date_results = scripts::complex::find_complex_upside_gap_two_crows(
            data,
            &common::convert_fugle_date_to_yyyymmdd(date),
        );
        results.extend(date_results);
    }

    print_line();
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 烏鴉躍空",
        results.len(),
    );
    common::print_lower_30_percent_list(data, &results);
    print_line();
}
