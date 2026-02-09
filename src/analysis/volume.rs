use crate::common;
use crate::stocks::data_company::DataCompany;

const MODULE_NAME: &str = "analysis::volume";

pub fn find_max_min_date_range_company(
    data_company: &DataCompany,
    date: &str,
    range: usize,
) -> Option<(f64, f64)> {
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
            return None;
        }
    };

    let start_index = if curr_index >= range {
        curr_index - range + 1
    } else {
        println!(
            "[{MODULE_NAME}] {} 日期 {} 前面資料不足，無法計算最大最小值",
            data_company.stock_no, date
        );
        return None;
    };

    let mut max_price = f64::MIN;
    let mut min_price = f64::MAX;

    for i in start_index..=curr_index {
        let data = &data_company.stock_data[i];
        if data.high > max_price {
            max_price = data.high;
        }
        if data.low < min_price {
            min_price = data.low;
        }
    }

    Some((max_price, min_price))
}
