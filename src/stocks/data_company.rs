use serde::{Deserialize, Serialize};

use crate::cfg::data::CfgData;
use crate::fugle::stocks::fetch as stocks_fetch;
use crate::storage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockData {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub turnover: u64,
    pub change: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockDataWithNo {
    pub stock_no: String,
    pub stock_data: StockData,
}

impl StockData {
    // pub fn print(&self) {
    //     println!(
    //         "Date: {}, Open: {}, High: {}, Low: {}, Close: {}, Volume: {}, Turnover: {}, Change: {}",
    //         self.date,
    //         self.open,
    //         self.high,
    //         self.low,
    //         self.close,
    //         self.volume,
    //         self.turnover,
    //         self.change
    //     );
    // }
}

pub struct DataCompany {
    pub stock_no: String,
    pub stock_data: Vec<StockData>,
}

impl DataCompany {
    pub fn new(stock_no: String) -> Self {
        let mut stock_data = storage::stocks::read(&stock_no);
        stock_data.sort_by(|a, b| a.date.cmp(&b.date));

        DataCompany {
            stock_no,
            stock_data,
        }
    }

    pub fn read(&mut self) {
        self.stock_data = storage::stocks::read(&self.stock_no);
        self.stock_data.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub async fn fetch_year(&mut self, cfg: &CfgData, year: &str) {
        self.stock_data = stocks_fetch(cfg, &self.stock_no, year).await.unwrap();
        self.stock_data.sort_by(|a, b| a.date.cmp(&b.date));
        storage::stocks::save(&self.stock_no, year, &self.stock_data);

        self.read();
    }

    // pub fn print(&self) {
    //     println!("Stock No: {}", self.stock_no);
    //     for data in &self.stock_data {
    //         data.print();
    //     }
    // }

    pub fn get_stock_data_by_date(&self, date: &str) -> Option<&StockData> {
        self.stock_data.iter().find(|d| d.date == date)
    }
}
