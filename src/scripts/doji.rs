use crate::analysis;
use crate::common;
use crate::stocks::data::Data;
use crate::stocks::data_company::StockDataWithNo;

const MODULE_NAME: &str = "scripts::doji";

pub fn find_doji_date(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的十字線");

    let mut doji_data_all = Vec::new();

    for company in &data.company_map.stock_map {
        let stock_no = &company.stock_no;
        let company_name = data.company_map.get_name(stock_no);

        let data_company = data.data_company.get(stock_no).expect("找不到股票資料");
        let date_fugle_format = common::convert_date_to_fugle_format(date);
        let stock_data_for_date = data_company.get_stock_data_by_date(&date_fugle_format);

        if let Some(stock_data) = stock_data_for_date {
            if analysis::candlestick::anal_candlestick(stock_data)
                == analysis::candlestick::CandlestickType::Doji
            {
                doji_data_all.push(StockDataWithNo {
                    stock_no: stock_no.clone(),
                    stock_data: stock_data.clone(),
                });
            }
        } else {
            println!("[{MODULE_NAME}] 無法找到 {stock_no} ({company_name}) 在 {date} 的資料");
        }
    }

    doji_data_all
}
