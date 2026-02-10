use crate::analysis;
use crate::common;
use crate::scripts;
use crate::stocks::data::Data;
use crate::stocks::data_company::StockDataWithNo;

const MODULE_NAME: &str = "scripts::complex";
const LARGE_VOLUME: u64 = 2000;

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
        let company_data = data
            .data_company
            .get(&company.stock_no)
            .expect("找不到股票資料");
        let curr_date_index = match company_data
            .stock_data
            .iter()
            .position(|d| d.date == common::convert_date_to_fugle_format(date))
        {
            Some(index) => index,
            None => continue, // 如果找不到日期，跳過這家公司
        };

        if curr_date_index < 2 {
            continue; // 確保有足夠的歷史資料
        }
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
