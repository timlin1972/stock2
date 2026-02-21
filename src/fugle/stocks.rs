use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio::time::Duration;

use crate::cfg::data::CfgData;
use crate::consts;
use crate::stocks::data_company::StockData;

const MODULE_NAME: &str = "fugle::stocks";

#[derive(Debug, Serialize, Deserialize)]
struct FugleStockResponse {
    pub timeframe: String,
    pub data: Vec<StockData>,
}

pub async fn fetch(
    cfg: &CfgData,
    stock_no: &str,
    year: &str,
) -> Result<Vec<StockData>, Box<dyn Error>> {
    let mut retry_count = 0;
    let max_retries = 3;
    loop {
        let from = format!("{}-01-01", year);
        let to = format!("{}-12-31", year);
        let url = format!(
            "https://api.fugle.tw/marketdata/v1.0/stock/historical/candles/{stock_no}?from={from}&to={to}&timeframe=D&adjusted=true&fields=open,high,low,close,volume,turnover,change&sort=asc",
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        let response = client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36")
            .header("Accept", "application/json")
            .header("X-API-KEY", &cfg.fugle_api_key)
            .send()
            .await?;

        // 先拿到原始文字，避免解析 JSON 失敗時看不到原因
        let body_text = response.text().await?;
        // println!("[{MODULE_NAME}] Raw response: {body_text}");
        let parsed: FugleStockResponse = match serde_json::from_str(&body_text) {
            Ok(data) => data,
            Err(e) => {
                if body_text.contains("Rate limit exceeded") {
                    println!("[{MODULE_NAME}] API rate limit exceeded. 5 秒後重試...");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue; // 等待後重試
                }
                if consts::IGNORED_STOCKS.contains(&stock_no) {
                    println!("[{MODULE_NAME}] 股票 {stock_no} 在 {year} 沒有資料，列管已忽略");
                    return Ok(vec![]); // 回傳空資料
                }
                println!("[{MODULE_NAME}] JSON 解析失敗: {e}");
                println!("[{MODULE_NAME}] body_text: {body_text}");
                println!("[{MODULE_NAME}] 3 秒後重試...");
                tokio::time::sleep(Duration::from_secs(3)).await;
                retry_count += 1;
                if retry_count >= max_retries {
                    println!("[{MODULE_NAME}] 已達最大重試次數，放棄 {stock_no} 的資料");
                    return Ok(vec![]); // 回傳空資料
                }
                continue; // 解析失敗，等待後重試
            }
        };

        return Ok(parsed.data);
    }
}
