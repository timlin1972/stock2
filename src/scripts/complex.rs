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
