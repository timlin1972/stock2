use std::error::Error;

use crate::stocks::company_map::CompanyInfo;

/// 從 TWSE API 抓取上市公司代號 → 中文名稱
pub async fn fetch(ignore_stocks: &[String]) -> Result<Vec<CompanyInfo>, Box<dyn Error>> {
    let url = "https://openapi.twse.com.tw/v1/opendata/t187ap03_L";
    let resp = reqwest::get(url).await?.json::<Vec<CompanyInfo>>().await?;

    let mut ret_results = Vec::new();

    for company in resp {
        if ignore_stocks.contains(&company.stock_no) {
            continue;
        }
        if company.industry == "17" {
            // skip 金融保險業
            continue;
        }

        ret_results.push(company);
    }

    Ok(ret_results)
}
