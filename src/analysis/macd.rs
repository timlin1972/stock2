use ta::Next;
use ta::indicators::ExponentialMovingAverage as Ema;

use crate::common;
use crate::stocks::data_company::DataCompany;

const MODULE_NAME: &str = "analysis::macd";
const INTERVALS: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacdCrossType {
    GoldenCross,
    DeathCross,
}

// #[derive(Debug, Clone)]
pub struct MacdCross {
    pub stock_no: String,
    pub date: String,
    // pub dif: f64,
    // pub macd_signal: f64,
    pub cross_type: MacdCrossType,
}

// #[derive(Debug, Clone)]
pub struct MacdResult {
    // dif: f64,
    // macd_signal: f64,
    // histogram: f64,
}

pub struct MacdCalculator {
    stock_no: String,
    ema12: Ema,
    ema26: Ema,
    signal_ema9: Ema,
    prev_dif: f64,
    prev_signal: f64,
}

impl MacdCalculator {
    pub fn new(stock_no: String) -> Self {
        Self {
            stock_no,
            ema12: Ema::new(12).unwrap(),
            ema26: Ema::new(26).unwrap(),
            signal_ema9: Ema::new(9).unwrap(),
            prev_dif: 0.0,
            prev_signal: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.ema12 = Ema::new(12).unwrap();
        self.ema26 = Ema::new(26).unwrap();
        self.signal_ema9 = Ema::new(9).unwrap();
        self.prev_dif = 0.0;
        self.prev_signal = 0.0;
    }

    pub fn calc(
        &mut self,
        data_company: &DataCompany,
        date: &str,
    ) -> Option<(Vec<MacdResult>, Vec<MacdCross>)> {
        self.reset();

        let mut results = Vec::new();
        let mut macd_crosses = Vec::new();

        // find index of date in data_company.stock_data
        let date_fugle_format = common::convert_date_to_fugle_format(date);
        let end_index = match data_company
            .stock_data
            .iter()
            .position(|d| d.date == date_fugle_format)
        {
            Some(i) => i,
            None => {
                println!(
                    "[{MODULE_NAME}] 找不到 {} 日期 {} 的資料",
                    self.stock_no, date
                );
                return None;
            }
        };

        let start_index = if end_index >= INTERVALS {
            end_index - INTERVALS
        } else {
            println!("[{MODULE_NAME}] {} 資料不足，無法計算 MACD", self.stock_no);
            return None;
        };

        for daily in &data_company.stock_data[start_index..=end_index] {
            let close_price = daily.close;
            let (res, cross) = self.feed(&daily.date, close_price);
            results.push(res);
            if let Some(c) = cross {
                macd_crosses.push(c);
            }
        }

        Some((results, macd_crosses))
    }

    fn feed(&mut self, date: &str, close_price: f64) -> (MacdResult, Option<MacdCross>) {
        let e12 = self.ema12.next(close_price);
        let e26 = self.ema26.next(close_price);

        let dif = e12 - e26;
        let signal = self.signal_ema9.next(dif);
        let _histogram = dif - signal;

        let res = MacdResult {
            // dif,
            // macd_signal: signal,
            // histogram,
        };

        // 判斷交叉邏輯
        let mut cross = None;
        if self.prev_dif <= self.prev_signal && dif > signal {
            // println!("🚀 【黃金交叉】{} Date: {} DIF({:.2}) 向上突破 MACD({:.2})", self.stock_no, date, dif, signal);
            cross = Some(MacdCross {
                stock_no: self.stock_no.clone(),
                date: date.to_string(),
                // dif,
                // macd_signal: signal,
                cross_type: MacdCrossType::GoldenCross,
            });
        } else if self.prev_dif >= self.prev_signal && dif < signal {
            // println!("💀 【死亡交叉】{} Date: {} DIF({:.2}) 向下貫穿 MACD({:.2})", self.stock_no, date, dif, signal);
            cross = Some(MacdCross {
                stock_no: self.stock_no.clone(),
                date: date.to_string(),
                // dif,
                // macd_signal: signal,
                cross_type: MacdCrossType::DeathCross,
            });
        }

        self.prev_dif = dif;
        self.prev_signal = signal;
        (res, cross)
    }
}
