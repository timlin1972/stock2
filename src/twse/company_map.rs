use std::error::Error;

use crate::stocks::company_map::CompanyInfo;

/// 從 TWSE API 抓取上市公司代號 → 中文名稱
pub async fn fetch() -> Result<Vec<CompanyInfo>, Box<dyn Error>> {
    let url = "https://openapi.twse.com.tw/v1/opendata/t187ap03_L";
    let resp = reqwest::get(url).await?.json::<Vec<CompanyInfo>>().await?;

    Ok(resp)
}
