use crate::common;
use crate::stocks::data_company::DataCompany;
use crate::stocks::data_company::StockData;
use crate::stocks::data_company::StockDataWithNo;

const MODULE_NAME: &str = "analysis::bullish_engulfing";

pub fn find_bullish_engulfing_date_company(
    data_company: &DataCompany,
    date: &str,
) -> Vec<StockDataWithNo> {
    let mut results = Vec::new();

    // find index of date in data_company.stock_data
    let date_fugle_format = common::convert_date_to_fugle_format(date);
    let curr_index = match data_company
        .stock_data
        .iter()
        .position(|d| d.date == date_fugle_format)
    {
        Some(i) => i,
        None => {
            println!(
                "[{MODULE_NAME}] 找不到 {} 日期 {} 的資料",
                data_company.stock_no, date
            );
            return Vec::new();
        }
    };

    let prev_index = if curr_index >= 1 {
        curr_index - 1
    } else {
        println!(
            "[{MODULE_NAME}] {} 資料不足，無法計算 陽吞噬形態",
            data_company.stock_no
        );
        return Vec::new();
    };

    let curr_data = &data_company.stock_data[curr_index];
    let prev_data = &data_company.stock_data[prev_index];

    if is_bullish_engulfing(prev_data, curr_data) {
        results.push(StockDataWithNo {
            stock_no: data_company.stock_no.clone(),
            stock_data: curr_data.clone(),
        });
    }

    results
}

fn is_bullish_engulfing(prev: &StockData, curr: &StockData) -> bool {
    // 前一天是黑K
    let prev_black = prev.close < prev.open;
    // 當天是紅K
    let curr_red = curr.close > curr.open;

    // 當天的實體完全包覆前一天的實體和影線
    let engulf = curr.open < prev.low && curr.close > prev.high;

    prev_black && curr_red && engulf
}
