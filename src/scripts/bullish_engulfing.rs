use crate::analysis;
use crate::stocks::data::Data;
use crate::stocks::data_company::StockDataWithNo;

const MODULE_NAME: &str = "scripts::bullish_engulfing";

pub fn find_bullish_engulfing_date(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的陽吞噬形態");

    let mut bullish_engulfing_data_all = Vec::new();

    for company in &data.company_map.stock_map {
        let stock_no = &company.stock_no;

        let data_company = data.data_company.get(stock_no).expect("找不到股票資料");
        let result =
            analysis::bullish_engulfing::find_bullish_engulfing_date_company(data_company, date);
        bullish_engulfing_data_all.extend(result);
    }

    bullish_engulfing_data_all
}
