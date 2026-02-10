use std::io;

use chrono::Local;

use crate::common;
use crate::scripts;
use crate::stocks::data::Data;

fn print_line() {
    println!("--------------------------------------------------------------------------------");
}

// get YYYYMMDD format of today's date
fn get_today_date() -> String {
    let today = Local::now().date_naive();
    today.format("%Y%m%d").to_string()
}

pub async fn main_menu(data: &mut Data) {
    loop {
        println!("Main Menu");
        println!("每日工作: 1/4/5/6");
        println!("1. 抓 2026 全部股票資料");
        println!("2. 抓 年度個股股票資料");
        println!("3. 單日長紅 K 棒");
        println!("4. 單日十字線配合前 20*6 日最大最小值");
        println!("5. 單日陽吞噬形態");
        println!("6. 單日 MACD 黃金交叉且大成交量");
        println!("7. 複合條件: 單日吊人線且前兩天都是漲停");
        println!("q/e. 退出 (Quit/Exit)");
        println!("請輸入選項：");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("讀取失敗");

        // 去掉換行符號
        let input = input.trim();

        match input {
            "1" => menu_fetch_data_all_companies(data).await,
            "2" => menu_fetch_data_company(data).await,
            "3" => menu_long_red_candle_date(data),
            "4" => menu_doji_date_range_max_min(data),
            "5" => menu_bullish_engulfing_date(data),
            "6" => menu_macd_golden_cross_date(data),
            "7" => menu_complex_hanging_man_date(data),
            "q" | "e" => {
                println!("退出程式");
                break;
            }
            _ => {
                println!("無效的選項，請重新輸入。");
            }
        }
    }
}

async fn menu_fetch_data_all_companies(data: &mut Data) {
    print_line();
    data.fetch_year("2026").await;
    print_line();
}

async fn menu_fetch_data_company(data: &mut Data) {
    println!("請輸入年分 (格式: YYYY):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let input = input.trim();

    println!("請輸入股票代號:");
    let mut stock_no = String::new();
    io::stdin().read_line(&mut stock_no).expect("讀取失敗");
    let stock_no = stock_no.trim();

    print_line();
    data.fetch_company_year(stock_no, input).await;
    print_line();
}

fn menu_long_red_candle_date(data: &Data) {
    println!("請輸入日期 (YYYYMMDD):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let mut input = input.trim().to_string();
    if input.is_empty() {
        input = get_today_date();
        println!("使用今天的日期: {input}");
    }

    print_line();
    let mut results = scripts::candlestick::find_long_red_candle_date(data, &input);
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序
    println!("總共有 {} 支股票在 {} 是長紅 K 棒", results.len(), input);

    println!(
        "{:<9}{:<5}{:>6}{:>5}{:>9}{:>9}  公司名稱",
        "日期", "台股", "成交張數", "收盤價", "+30%", "-30%",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * 1.3,
            result.stock_data.close * 0.7,
            data.company_map.get_name(&result.stock_no),
        );
    }
    print_line();
}

fn menu_doji_date_range_max_min(data: &Data) {
    println!("請輸入日期 (YYYYMMDD):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let mut input = input.trim().to_string();
    if input.is_empty() {
        input = get_today_date();
        println!("使用今天的日期: {input}");
    }

    print_line();
    let mut doji_data_all = scripts::candlestick::find_doji_date_range_max_min(data, &input);
    doji_data_all.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序
    println!("總共有 {} 支股票在 {} 是十字線", doji_data_all.len(), input);
    println!(
        "{:<9}{:<5}{:>6}{:>5}{:>9}{:>9}  公司名稱",
        "日期", "台股", "成交張數", "收盤價", "+30%", "-30%",
    );
    for doji_data in doji_data_all {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            doji_data.stock_data.date,
            doji_data.stock_no,
            common::str_volume(doji_data.stock_data.volume),
            doji_data.stock_data.close,
            doji_data.stock_data.close * 1.3,
            doji_data.stock_data.close * 0.7,
            data.company_map.get_name(&doji_data.stock_no),
        );
    }

    print_line();
}

fn menu_macd_golden_cross_date(data: &Data) {
    println!("請輸入日期 (YYYYMMDD):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let mut input = input.trim().to_string();
    if input.is_empty() {
        input = get_today_date();
        println!("使用今天的日期: {input}");
    }

    print_line();
    let mut macd_golden_cross_data_all =
        scripts::complex::find_macd_golden_cross_date_large_volume(data, &input);
    macd_golden_cross_data_all.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    println!(
        "總共有 {} 支股票在 {} 是 MACD 黃金交叉且大成交量",
        macd_golden_cross_data_all.len(),
        input
    );
    println!(
        "{:<9}{:<5}{:>6}{:>6}{:>6}{:>6}{:>6}{:>6}  公司名稱",
        "日期", "台股", "成交張數", "開盤價", "收盤價", "最高價", "最低價", "漲跌",
    );
    for macd_golden_cross_data in macd_golden_cross_data_all {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            macd_golden_cross_data.stock_data.date,
            macd_golden_cross_data.stock_no,
            common::str_volume(macd_golden_cross_data.stock_data.volume),
            macd_golden_cross_data.stock_data.open,
            macd_golden_cross_data.stock_data.close,
            macd_golden_cross_data.stock_data.high,
            macd_golden_cross_data.stock_data.low,
            macd_golden_cross_data.stock_data.change,
            data.company_map.get_name(&macd_golden_cross_data.stock_no),
        );
    }

    print_line();
}

fn menu_bullish_engulfing_date(data: &Data) {
    println!("請輸入日期 (YYYYMMDD):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let mut input = input.trim().to_string();
    if input.is_empty() {
        input = get_today_date();
        println!("使用今天的日期: {input}");
    }

    let results = scripts::bullish_engulfing::find_bullish_engulfing_date(data, &input);

    print_line();

    println!("總共有 {} 支股票在 {} 是 陽吞噬形態", results.len(), input);
    println!(
        "{:<9}{:<5}{:>6}{:>5}{:>9}{:>9}  公司名稱",
        "日期", "台股", "成交張數", "收盤價", "+30%", "-30%",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * 1.3,
            result.stock_data.close * 0.7,
            data.company_map.get_name(&result.stock_no),
        );
    }
    print_line();
}

fn menu_complex_hanging_man_date(data: &Data) {
    println!("請輸入日期 (YYYYMMDD):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let mut input = input.trim().to_string();
    if input.is_empty() {
        input = get_today_date();
        println!("使用今天的日期: {input}");
    }

    print_line();
    let results = scripts::complex::find_complex_hanging_man_date(data, &input);
    println!(
        "總共有 {} 支股票在 {} 是 複合條件: 單日吊人線且前兩天都是漲停",
        results.len(),
        input
    );
    println!(
        "{:<9}{:<5}{:>6}{:>6}{:>6}{:>6}{:>6}{:>6}  公司名稱",
        "日期", "台股", "成交張數", "開盤價", "收盤價", "最高價", "最低價", "漲跌",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}{:>9.2}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.open,
            result.stock_data.close,
            result.stock_data.high,
            result.stock_data.low,
            result.stock_data.change,
            data.company_map.get_name(&result.stock_no),
        );
    }
    print_line();
}
