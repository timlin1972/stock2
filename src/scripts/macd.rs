use crate::analysis;
use crate::common;
use crate::stocks::data::Data;
use crate::stocks::data_company::StockDataWithNo;

const MODULE_NAME: &str = "scripts::macd";

pub fn find_macd_golden_cross_date_company(
    data: &Data,
    stock_no: String,
    date: &str,
) -> Vec<StockDataWithNo> {
    let date_fugle_format = common::convert_date_to_fugle_format(date);
    let mut macd_golden_cross_data_all = Vec::new();

    let data_company = data.data_company.get(&stock_no).expect("找不到股票資料");

    let mut macd_calculator = analysis::macd::MacdCalculator::new(stock_no.clone());
    if let Some((_res, crosses)) = macd_calculator.calc(data_company, date) {
        for cross in &crosses {
            if cross.date == date_fugle_format
                && cross.cross_type == analysis::macd::MacdCrossType::GoldenCross
            {
                let stock_date = data_company
                    .stock_data
                    .iter()
                    .find(|d| d.date == date_fugle_format)
                    .unwrap();
                let clone_cross = StockDataWithNo {
                    stock_no: cross.stock_no.clone(),
                    stock_data: stock_date.clone(),
                };
                macd_golden_cross_data_all.push(clone_cross);
            }
        }
    }

    macd_golden_cross_data_all
}

pub fn find_macd_golden_cross_date(data: &Data, date: &str) -> Vec<StockDataWithNo> {
    println!("[{MODULE_NAME}] 分析 {date} 的 MACD 黃金交叉");

    // let date_fugle_format = common::convert_date_to_fugle_format(date);
    let mut macd_golden_cross_data_all = Vec::new();

    for company in &data.company_map.stock_map {
        let macd_golden_cross_data_company =
            find_macd_golden_cross_date_company(data, company.stock_no.clone(), date);
        macd_golden_cross_data_all.extend(macd_golden_cross_data_company);
    }

    macd_golden_cross_data_all
}
