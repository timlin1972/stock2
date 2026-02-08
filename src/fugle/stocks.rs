use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio::time::Duration;

use crate::cfg::data::CfgData;
use crate::stocks::data_company::StockData;

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
    let parsed: FugleStockResponse = serde_json::from_str(&body_text).unwrap();

    Ok(parsed.data)
}
