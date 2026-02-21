use std::io;

use chrono::Local;

use crate::common;
use crate::menu::regression_menu;
use crate::scripts;
use crate::stocks::data::Data;
use crate::stocks::data_company::StockDataWithNo;

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
        println!("8. 烏雲罩頂");
        println!("9. 多頭母子");
        println!("10. 空頭母子");
        println!("11. 內困三日翻紅");
        println!("12. 內困三日翻黑");
        println!("13. 烏鴉躍空");
        println!("99. 回歸測試");
        println!("q/e. 退出 (Quit/Exit)");
        println!("h. Help");
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
            "8" => menu_dark_cloud_cover_date(data),
            "9" => menu_bullish_harami_pattern_date(data),
            "10" => menu_bearish_harami_pattern_date(data),
            "11" => menu_complex_bullish_harami_three_day_reversal_date(data),
            "12" => menu_complex_bearish_harami_three_day_reversal_date(data),
            "13" => menu_upside_gap_two_crows_date(data),
            "99" => regression_menu::menu(data),
            "h" => menu_help(),
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

fn menu_help() {
    println!("烏雲罩頂");
    println!("    1. 前面上漲 30%");
    println!("    2. 收黑K且創新高，前一天要紅K");
    println!("    3. 與前一天紅K有部分重疊");
    println!("    ⇒");
    println!("    1. 如果當天爆大量 ⇒ 視為陰吞噬");
    println!("    2. 如果沒有 ⇒ 拉回找買點");
    println!("        1. 十字線");
    println!("        2. 長下影線");
    println!("        3. 長紅K");
    println!("烏鴉躍空");
    println!("    1. 波段創新高後");
    println!("    2. 連續兩日出現留下缺口之跳空黑K線");
    println!("    3. 不用看量，不用看影線");
    println!("    ⇒");
    println!("    賣");
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
    let input = get_date_input();

    print_line();
    let mut results = scripts::candlestick::find_long_red_candle_date(data, &input);
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序
    println!("總共有 {} 支股票在 {} 是長紅 K 棒", results.len(), input);
    print_lower_upper_30_percent_list(data, &results);
    print_line();
}

fn menu_doji_date_range_max_min(data: &Data) {
    let input = get_date_input();

    let mut results = scripts::candlestick::find_doji_date_range_max_min(data, &input);
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    print_line();
    println!("總共有 {} 支股票在 {} 是十字線", results.len(), input);
    print_lower_upper_30_percent_list(data, &results);
    print_line();
}

fn menu_macd_golden_cross_date(data: &Data) {
    let input = get_date_input();

    print_line();
    let mut results = scripts::complex::find_macd_golden_cross_date_large_volume(data, &input);
    results.sort_by(|a, b| b.stock_data.volume.cmp(&a.stock_data.volume)); // 按照成交量排序

    println!(
        "總共有 {} 支股票在 {input} 是 MACD 黃金交叉且大成交量",
        results.len(),
    );
    print_detail_list(data, &results);
    print_line();
}

fn menu_bullish_engulfing_date(data: &Data) {
    let input = get_date_input();

    let results = scripts::bullish_engulfing::find_bullish_engulfing_date(data, &input);

    print_line();
    println!("總共有 {} 支股票在 {input} 是 陽吞噬形態", results.len());
    print_lower_upper_30_percent_list(data, &results);
    print_line();
}

fn menu_complex_hanging_man_date(data: &Data) {
    let input = get_date_input();

    print_line();
    let results = scripts::complex::find_complex_hanging_man_date(data, &input);
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 單日吊人線且前兩天都是漲停",
        results.len(),
    );
    print_detail_list(data, &results);
    print_line();
}

fn menu_dark_cloud_cover_date(data: &Data) {
    let input = get_date_input();

    print_line();
    let results = scripts::complex::find_complex_dark_cloud_cover(data, &input);
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 單日黑雲壓頂",
        results.len(),
    );
    common::print_lower_30_percent_list(data, &results);
    print_line();
}

fn menu_bullish_harami_pattern_date(data: &Data) {
    let input = get_date_input();

    print_line();
    let results = scripts::complex::find_complex_bullish_harami_pattern(data, &input);
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 多頭母子",
        results.len(),
    );
    print_upper_30_percent_list(data, &results);
    print_line();
}

fn menu_bearish_harami_pattern_date(data: &Data) {
    let input = get_date_input();

    print_line();
    let results = scripts::complex::find_complex_bearish_harami_pattern(data, &input);
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 空頭母子",
        results.len(),
    );
    common::print_lower_30_percent_list(data, &results);
    print_line();
}

fn menu_complex_bullish_harami_three_day_reversal_date(data: &Data) {
    let input = get_date_input();

    print_line();
    let results =
        scripts::complex::find_complex_bullish_harami_three_day_reversal_date(data, &input);
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 內困三日翻紅",
        results.len(),
    );
    print_upper_30_percent_list(data, &results);
    print_line();
}

fn menu_complex_bearish_harami_three_day_reversal_date(data: &Data) {
    let input = get_date_input();

    print_line();
    let results =
        scripts::complex::find_complex_bearish_harami_three_day_reversal_date(data, &input);
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 內困三日翻黑",
        results.len(),
    );
    common::print_lower_30_percent_list(data, &results);
    print_line();
}

fn menu_upside_gap_two_crows_date(data: &Data) {
    let input = get_date_input();

    print_line();
    let results = scripts::complex::find_complex_upside_gap_two_crows(data, &input);
    println!(
        "總共有 {} 支股票在 {input} 是 複合條件: 烏鴉躍空",
        results.len(),
    );
    common::print_lower_30_percent_list(data, &results);
    print_line();
}

//
//  Helper functions
//

fn get_date_input() -> String {
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

fn print_upper_30_percent_list(data: &Data, results: &[StockDataWithNo]) {
    println!(
        "{:<9}{:<5}{:>6}{:>6}{:>8}  公司名稱",
        "日期", "台股", "成交張數", "收盤價", "+30%",
    );
    for result in results {
        println!(
            "{:<11}{:<6}{:>10}{:>9.2}{:>9.2}  {:<20}",
            result.stock_data.date,
            result.stock_no,
            common::str_volume(result.stock_data.volume),
            result.stock_data.close,
            result.stock_data.close * 1.3,
            data.company_map.get_name(&result.stock_no),
        );
    }
}

fn print_lower_upper_30_percent_list(data: &Data, results: &[StockDataWithNo]) {
    println!(
        "{:<9}{:<5}{:>6}{:>6}{:>8}{:>8}  公司名稱",
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
}

fn print_detail_list(data: &Data, results: &[StockDataWithNo]) {
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
}
