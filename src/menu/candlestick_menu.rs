use std::io;

use crate::common;
use crate::scripts;
use crate::stocks::data::Data;

pub fn menu(data: &Data) {
    println!("K線選單");
    common::print_line();

    loop {
        println!("1. 單日十字線");
        println!("q/e. 退出 (Quit/Exit)");
        println!("請輸入選項：");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("讀取失敗");

        // 去掉換行符號
        let input = input.trim();

        match input {
            "1" => menu_doji(data),
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

fn menu_doji(data: &Data) {
    let input = common::get_date_input();

    let mut results = scripts::candlestick::find_doji_date(data, &input);
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    common::print_line();
    println!("總共有 {} 支股票在 {} 是十字線", results.len(), input);
    common::print_lower_upper_30_percent_list(data, &results);
    common::print_line();
}
