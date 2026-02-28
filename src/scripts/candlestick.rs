use crate::analysis;
use crate::common;
use crate::stocks::data::Data;
use crate::stocks::data_company::StockDataWithNo;

const MODULE_NAME: &str = "scripts::long_red_candle";
const RANGE: usize = 20 * 6;

pub fn find_doji_date_range_max_min(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    let mut results = Vec::new();

    for company in &data.company_map.stock_map {
        let stock_no = &company.stock_no;

        let company_data = common::get_company_data(data, &company.stock_no);
        let curr_date_index =
            match common::get_current_index_by_date(&company_data.stock_data, date, 2) {
                Some(index) => index,
                None => continue, // 如果找不到日期，跳過這家公司
            };
        let curr_stock_data = &company_data.stock_data[curr_date_index];

        if analysis::candlestick::anal_candlestick(curr_stock_data)
            == analysis::candlestick::CandlestickType::Doji
            && let Some((max_price, min_price)) =
                analysis::volume::find_max_min_date_range_company(company_data, date, RANGE)
        {
            if max_price > curr_stock_data.close * 1.3 {
                results.push(StockDataWithNo {
                    stock_no: stock_no.clone(),
                    stock_data: curr_stock_data.clone(),
                });
            }

            if min_price < curr_stock_data.close * 0.7 {
                results.push(StockDataWithNo {
                    stock_no: stock_no.clone(),
                    stock_data: curr_stock_data.clone(),
                });
            }
        }
    }

    results
}

#[allow(dead_code)]
pub fn find_lower_shadow_date(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的長下影線");

    let mut result = Vec::new();

    for company in &data.company_map.stock_map {
        let stock_no = &company.stock_no;
        let company_name = data.company_map.get_name(stock_no);

        let data_company = data.data_company.get(stock_no).expect("找不到股票資料");
        let date_fugle_format = common::convert_date_to_fugle_format(date);
        let stock_data_for_date = data_company.get_stock_data_by_date(&date_fugle_format);

        if let Some(stock_data) = stock_data_for_date {
            if analysis::candlestick::anal_candlestick(stock_data)
                == analysis::candlestick::CandlestickType::LongLowerShadow
            {
                result.push(StockDataWithNo {
                    stock_no: stock_no.clone(),
                    stock_data: stock_data.clone(),
                });
            }
        } else {
            println!("[{MODULE_NAME}] 無法找到 {stock_no} ({company_name}) 在 {date} 的資料");
        }
    }

    result
}

#[allow(dead_code)]
pub fn find_hanging_man_date(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的吊人線");

    let mut result = Vec::new();

    for company in &data.company_map.stock_map {
        let stock_no = &company.stock_no;
        let company_name = data.company_map.get_name(stock_no);

        let data_company = data.data_company.get(stock_no).expect("找不到股票資料");
        let date_fugle_format = common::convert_date_to_fugle_format(date);
        let stock_data_for_date = data_company.get_stock_data_by_date(&date_fugle_format);

        if let Some(stock_data) = stock_data_for_date {
            if analysis::candlestick::anal_candlestick(stock_data)
                == analysis::candlestick::CandlestickType::HangingMan
            {
                result.push(StockDataWithNo {
                    stock_no: stock_no.clone(),
                    stock_data: stock_data.clone(),
                });
            }
        } else {
            println!("[{MODULE_NAME}] 無法找到 {stock_no} ({company_name}) 在 {date} 的資料");
        }
    }

    result
}
