use crate::analysis;
use crate::common;
use crate::scripts;
use crate::stocks::data::Data;
use crate::stocks::data_company::DataCompany;
use crate::stocks::data_company::{StockData, StockDataWithNo};

const MODULE_NAME: &str = "scripts::complex";
const LARGE_VOLUME: u64 = 2000;
const LOOK_BACK_DAYS: usize = 20 * 3; // 看三個月的資料

pub fn find_macd_golden_cross_date_large_volume(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的 MACD 黃金交叉且大成交量");

    let mut crosses = scripts::macd::find_macd_golden_cross_date(data, date);

    // remove the crosses that the volume is smaller than LARGE_VOLUME
    crosses.retain(|cross| cross.stock_data.volume >= LARGE_VOLUME * 1000);

    crosses
}

#[allow(clippy::collapsible_if)]
pub fn find_complex_hanging_man_date(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    // println!("[{MODULE_NAME}] 分析 {date} 的複合條件: 單日吊人線且前兩天都是漲停");
    let mut results = Vec::new();
    for company in &data.company_map.stock_map {
        let company_data = common::get_company_data(data, &company.stock_no);
        let curr_date_index = match get_current_index_by_date(&company_data.stock_data, date, 2) {
            Some(index) => index,
            None => continue, // 如果找不到日期，跳過這家公司
        };

        let prev_stock_data = &company_data.stock_data[curr_date_index - 1];
        let prev_prev_stock_data = &company_data.stock_data[curr_date_index - 2];
        let curr_stock_data = &company_data.stock_data[curr_date_index];

        // 兩根漲停+吊人線
        if analysis::candlestick::anal_candlestick(curr_stock_data)
            == analysis::candlestick::CandlestickType::HangingMan
        {
            if analysis::candlestick::anal_limit_up(prev_stock_data, curr_stock_data) {
                if analysis::candlestick::anal_limit_up(prev_prev_stock_data, prev_stock_data) {
                    results.push(StockDataWithNo {
                        stock_no: company.stock_no.clone(),
                        stock_data: curr_stock_data.clone(),
                    });
                }
            }
        }

        // 一根漲停+當天是漲停且吊人線
        if analysis::candlestick::anal_candlestick(curr_stock_data)
            == analysis::candlestick::CandlestickType::HangingMan
        {
            if analysis::candlestick::anal_limit_up(curr_stock_data, curr_stock_data) {
                if analysis::candlestick::anal_limit_up(prev_stock_data, curr_stock_data) {
                    results.push(StockDataWithNo {
                        stock_no: company.stock_no.clone(),
                        stock_data: curr_stock_data.clone(),
                    });
                }
            }
        }
    }

    results
}

pub fn find_complex_dark_cloud_cover(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的複合條件: 單日黑雲罩頂");
    let mut results = Vec::new();
    for company in &data.company_map.stock_map {
        let company_data = common::get_company_data(data, &company.stock_no);
        let curr_date_index =
            match get_current_index_by_date(&company_data.stock_data, date, LOOK_BACK_DAYS) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };

        let prev_stock_data = &company_data.stock_data[curr_date_index - 1];
        let curr_stock_data = &company_data.stock_data[curr_date_index];

        // 當天是黑K
        if curr_stock_data.close >= curr_stock_data.open {
            continue;
        }
        // 昨天是紅K
        if prev_stock_data.close <= prev_stock_data.open {
            continue;
        }

        // 黑K創新高
        if curr_stock_data.open <= prev_stock_data.close {
            continue;
        }

        // 黑K創新高，創新高看個 5 天
        let mut ignore = false;
        for i in 1..=5 {
            let past_stock_data = &company_data.stock_data[curr_date_index - i];
            if curr_stock_data.open <= past_stock_data.close
                || curr_stock_data.open <= past_stock_data.open
            {
                ignore = true;
                break;
            }
        }
        if ignore {
            continue;
        }

        // 黑K包覆部分紅K
        if curr_stock_data.close <= prev_stock_data.open
            || curr_stock_data.close >= prev_stock_data.close
        {
            continue;
        }

        // 檢視一下波段，要有低點
        if !is_swing_low(company_data, curr_stock_data, date) {
            continue;
        }

        results.push(StockDataWithNo {
            stock_no: company.stock_no.clone(),
            stock_data: curr_stock_data.clone(),
        });
    }

    results
}

fn is_bullish_harami_pattern(prev: &StockData, curr: &StockData) -> bool {
    // 當天是紅K
    if curr.close <= curr.open {
        return false;
    }
    // 昨天是黑K
    if prev.close >= prev.open {
        return false;
    }

    // 紅K實體被黑K實體包覆
    if curr.open < prev.close || curr.close > prev.open {
        return false;
    }

    true
}

pub fn find_complex_bullish_harami_pattern(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!(
        "[{MODULE_NAME}] 分析 {date} 的複合條件: 單日多頭孕線: 當天是紅K且昨天是黑K且紅K實體被黑K實體包覆"
    );
    let mut results = Vec::new();
    for company in &data.company_map.stock_map {
        let company_data = common::get_company_data(data, &company.stock_no);
        let curr_date_index =
            match get_current_index_by_date(&company_data.stock_data, date, LOOK_BACK_DAYS) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };

        let prev_stock_data = &company_data.stock_data[curr_date_index - 1];
        let curr_stock_data = &company_data.stock_data[curr_date_index];

        if !is_bullish_harami_pattern(prev_stock_data, curr_stock_data) {
            continue;
        }

        // 檢視一下波段，要有高點
        if !is_swing_high(company_data, curr_stock_data, date) {
            continue;
        }

        results.push(StockDataWithNo {
            stock_no: company.stock_no.clone(),
            stock_data: curr_stock_data.clone(),
        });
    }

    results
}

fn is_bearish_harami_pattern(prev: &StockData, curr: &StockData) -> bool {
    // 當天是黑K
    if curr.close >= curr.open {
        return false;
    }
    // 昨天是紅K
    if prev.close <= prev.open {
        return false;
    }

    // 黑K實體被紅K實體包覆
    if curr.open > prev.close || curr.close < prev.open {
        return false;
    }

    true
}

pub fn find_complex_bearish_harami_pattern(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!(
        "[{MODULE_NAME}] 分析 {date} 的複合條件: 單日空頭孕線: 當天是黑K且昨天是紅K且黑K實體被紅K實體包覆"
    );
    let mut results = Vec::new();
    for company in &data.company_map.stock_map {
        let company_data = common::get_company_data(data, &company.stock_no);
        let curr_date_index =
            match get_current_index_by_date(&company_data.stock_data, date, LOOK_BACK_DAYS) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };

        let prev_stock_data = &company_data.stock_data[curr_date_index - 1];
        let curr_stock_data = &company_data.stock_data[curr_date_index];

        if !is_bearish_harami_pattern(prev_stock_data, curr_stock_data) {
            continue;
        }

        // 檢視一下波段，要有低點
        if !is_swing_low(company_data, curr_stock_data, date) {
            continue;
        }

        results.push(StockDataWithNo {
            stock_no: company.stock_no.clone(),
            stock_data: curr_stock_data.clone(),
        });
    }

    results
}

pub fn find_complex_bullish_harami_three_day_reversal_date(
    data: &Data,
    date: &str,
) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的複合條件: 內困三日翻紅");
    let mut results = Vec::new();
    for company in &data.company_map.stock_map {
        let company_data = common::get_company_data(data, &company.stock_no);
        let curr_date_index =
            match get_current_index_by_date(&company_data.stock_data, date, LOOK_BACK_DAYS) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };

        let prev_prev_stock_data = &company_data.stock_data[curr_date_index - 2];
        let prev_stock_data = &company_data.stock_data[curr_date_index - 1];
        let curr_stock_data = &company_data.stock_data[curr_date_index];

        // 前兩天是多頭孕線
        if !is_bullish_harami_pattern(prev_prev_stock_data, prev_stock_data) {
            continue;
        }

        // 當天是紅K
        if curr_stock_data.close <= curr_stock_data.open {
            continue;
        }

        // 當天收盤價高於前兩天的黑K實體
        if curr_stock_data.close < prev_prev_stock_data.open {
            continue;
        }

        // 當天紅K要帶量
        let yesterday_date = common::convert_fugle_date_to_yyyymmdd(&prev_stock_data.date);
        if let Some(mv) = analysis::volume::find_mv(company_data, &yesterday_date) {
            // println!(
            //     "[{MODULE_NAME}] {} 昨天 MV5: {}, MV10: {}, MV20: {}",
            //     company.stock_no,
            //     common::str_volume(mv.0 as u64),
            //     common::str_volume(mv.1 as u64),
            //     common::str_volume(mv.2 as u64),
            // );
            if curr_stock_data.volume as f64 <= mv.0 * 1.5
                || curr_stock_data.volume as f64 <= mv.1 * 1.5
                || curr_stock_data.volume as f64 <= mv.2 * 1.5
            {
                continue;
            }
        } else {
            println!(
                "[{MODULE_NAME}] 找不到 {} 日期 {} 的成交金額資料，無法判斷是否帶量，跳過這家公司",
                company.stock_no, date
            );
            continue;
        }

        // 檢視一下波段，要有高點
        if !is_swing_high(company_data, curr_stock_data, date) {
            continue;
        }

        results.push(StockDataWithNo {
            stock_no: company.stock_no.clone(),
            stock_data: curr_stock_data.clone(),
        });
    }

    results
}

fn get_current_index_by_date(
    company_data: &[StockData],
    date: &str,
    range: usize,
) -> Option<usize> {
    let curr_date_index = company_data
        .iter()
        .position(|d| d.date == common::convert_date_to_fugle_format(date))?;
    if curr_date_index < range {
        None
    } else {
        Some(curr_date_index)
    }
}

pub fn find_complex_bearish_harami_three_day_reversal_date(
    data: &Data,
    date: &str,
) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的複合條件: 內困三日翻黑");
    let mut results = Vec::new();
    for company in &data.company_map.stock_map {
        let company_data = common::get_company_data(data, &company.stock_no);
        let curr_date_index =
            match get_current_index_by_date(&company_data.stock_data, date, LOOK_BACK_DAYS) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };

        let prev_prev_stock_data = &company_data.stock_data[curr_date_index - 2];
        let prev_stock_data = &company_data.stock_data[curr_date_index - 1];
        let curr_stock_data = &company_data.stock_data[curr_date_index];

        // 前兩天是空頭孕線
        if !is_bearish_harami_pattern(prev_prev_stock_data, prev_stock_data) {
            continue;
        }

        // 當天是黑K
        if curr_stock_data.close >= curr_stock_data.open {
            continue;
        }

        // 當天收盤價低於前兩天的紅K實體
        if curr_stock_data.close > prev_prev_stock_data.open {
            continue;
        }

        // 當天黑K不能帶量
        let yesterday_date = common::convert_fugle_date_to_yyyymmdd(&prev_stock_data.date);
        if let Some(mv) = analysis::volume::find_mv(company_data, &yesterday_date) {
            // println!(
            //     "[{MODULE_NAME}] {} 昨天 MV5: {}, MV10: {}, MV20: {}",
            //     company.stock_no,
            //     common::str_volume(mv.0 as u64),
            //     common::str_volume(mv.1 as u64),
            //     common::str_volume(mv.2 as u64),
            // );
            if curr_stock_data.volume as f64 >= mv.0 * 1.5
                || curr_stock_data.volume as f64 >= mv.1 * 1.5
                || curr_stock_data.volume as f64 >= mv.2 * 1.5
            {
                continue;
            }
        } else {
            println!(
                "[{MODULE_NAME}] 找不到 {} 日期 {} 的成交金額資料，無法判斷是否帶量，跳過這家公司",
                company.stock_no, date
            );
            continue;
        }

        // 檢視一下波段，要有低點
        if !is_swing_low(company_data, curr_stock_data, date) {
            continue;
        }

        results.push(StockDataWithNo {
            stock_no: company.stock_no.clone(),
            stock_data: curr_stock_data.clone(),
        });
    }

    results
}

fn is_swing_low(company_data: &DataCompany, curr_stock_data: &StockData, date: &str) -> bool {
    let (_max_price, min_price) =
        match analysis::volume::find_max_min_date_range_company(company_data, date, LOOK_BACK_DAYS)
        {
            Some((_max_price, min_price)) => (_max_price, min_price),
            None => return false,
        };

    if curr_stock_data.close * 0.7 < min_price {
        return false;
    }

    true
}

fn is_swing_high(company_data: &DataCompany, curr_stock_data: &StockData, date: &str) -> bool {
    let (max_price, _min_price) =
        match analysis::volume::find_max_min_date_range_company(company_data, date, LOOK_BACK_DAYS)
        {
            Some((max_price, _min_price)) => (max_price, _min_price),
            None => return false,
        };

    if curr_stock_data.close * 1.3 > max_price {
        return false;
    }

    true
}

pub fn find_complex_upside_gap_two_crows(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的複合條件: 烏鴉躍空");

    let mut results = Vec::new();
    for company in &data.company_map.stock_map {
        let company_data = common::get_company_data(data, &company.stock_no);
        let curr_date_index =
            match get_current_index_by_date(&company_data.stock_data, date, LOOK_BACK_DAYS) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };

        let prev_prev_stock_data = &company_data.stock_data[curr_date_index - 2];
        let prev_stock_data = &company_data.stock_data[curr_date_index - 1];
        let curr_stock_data = &company_data.stock_data[curr_date_index];

        // 前兩天是紅K
        if prev_prev_stock_data.close <= prev_prev_stock_data.open {
            continue;
        }

        // 前一天是黑K
        if prev_stock_data.close >= prev_stock_data.open {
            continue;
        }

        // 當天是黑K
        if curr_stock_data.close >= curr_stock_data.open {
            continue;
        }

        // 要有缺口
        if curr_stock_data.close <= prev_prev_stock_data.close
            || prev_stock_data.close <= prev_prev_stock_data.close
        {
            continue;
        }

        // 黑K創新高，創新高看個 5 天
        let mut ignore = false;
        for i in 2..=6 {
            let past_stock_data = &company_data.stock_data[curr_date_index - i];
            if curr_stock_data.open <= past_stock_data.close
                || curr_stock_data.open <= past_stock_data.open
            {
                ignore = true;
                break;
            }
            if prev_stock_data.open <= past_stock_data.close
                || prev_stock_data.open <= past_stock_data.open
            {
                ignore = true;
                break;
            }
        }
        if ignore {
            continue;
        }

        // 檢視一下波段，要有低點
        if !is_swing_low(company_data, curr_stock_data, date) {
            continue;
        }

        results.push(StockDataWithNo {
            stock_no: company.stock_no.clone(),
            stock_data: curr_stock_data.clone(),
        });
    }

    results
}
