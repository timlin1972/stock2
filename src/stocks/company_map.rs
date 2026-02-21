use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::twse;

const MODULE_NAME: &str = "stocks::company_map";
const IGNORE_STOCKS_FILE: &str = "ignore_stocks.txt";
const COMPANY_MAP: &str = "company_map.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct CompanyInfo {
    #[serde(rename = "公司代號")]
    pub stock_no: String,
    #[serde(rename = "公司簡稱")]
    pub name: String,
    #[serde(rename = "產業別")]
    pub industry: String,
}

pub struct CompanyMap {
    pub stock_map: Vec<CompanyInfo>,
    industry_map: HashMap<String, String>,
}

impl CompanyMap {
    pub async fn new() -> Self {
        let industry_map = build_industry_map();
        let ignore_stocks = read_lines_to_vec(IGNORE_STOCKS_FILE).unwrap();

        let stock_map = get_company_map(&ignore_stocks).await;

        CompanyMap {
            stock_map,
            industry_map,
        }
    }

    // pub fn print(&self) {
    //     for company in &self.stock_map {
    //         println!("{}: {}", company.stock_no, self.get_name(&company.stock_no));
    //     }
    // }

    pub fn get_name(&self, stock_no: &str) -> String {
        for company in &self.stock_map {
            if company.stock_no == stock_no {
                return format!(
                    "{}/{}",
                    company.name,
                    self.industry_map
                        .get(&company.industry)
                        .unwrap_or(&"未知產業".to_string())
                );
            }
        }

        panic!("[{MODULE_NAME}] Cannot find company name for stock no: {stock_no}");
    }
}

fn read_lines_to_vec<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for l in reader.lines().map_while(Result::ok) {
        // 如果有註解，用 "//" 切開，只取前半段
        let clean = l.split("//").next().unwrap().trim();
        if !clean.is_empty() {
            result.push(clean.to_string());
        }
    }
    Ok(result)
}

fn build_industry_map() -> HashMap<String, String> {
    let industries = vec![
        ("01", "水泥工業"),
        ("02", "食品工業"),
        ("03", "塑膠工業"),
        ("04", "紡織纖維"),
        ("05", "電機機械"),
        ("06", "電器電纜"),
        ("08", "玻璃陶瓷"),
        ("09", "造紙工業"),
        ("10", "鋼鐵工業"),
        ("11", "橡膠工業"),
        ("12", "汽車工業"),
        ("13", "電子工業"),
        ("14", "建材營造業"),
        ("15", "航運業"),
        ("16", "觀光餐旅"),
        ("17", "金融保險業"),
        ("18", "貿易百貨業"),
        ("19", "綜合"),
        ("20", "其他業"),
        ("21", "化學工業"),
        ("22", "生技醫療業"),
        ("23", "油電燃氣業"),
        ("24", "半導體業"),
        ("25", "電腦及週邊設備業"),
        ("26", "光電業"),
        ("27", "通信網路業"),
        ("28", "電子零組件業"),
        ("29", "電子通路業"),
        ("30", "資訊服務業"),
        ("31", "其他電子業"),
        ("32", "文化創意業"),
        ("33", "農業科技業"),
        ("34", "電子商務"),
        ("35", "綠能環保"),
        ("36", "數位雲端"),
        ("37", "運動休閒"),
        ("38", "居家生活"),
    ];

    // 將 Vec 轉換為 HashMap
    industries
        .into_iter()
        .map(|(code, name)| (code.to_string(), name.to_string()))
        .collect()
}

async fn get_company_map(ignore_stocks: &[String]) -> Vec<CompanyInfo> {
    // if the company map JSON file already exists, read from it instead of fetching from the API
    let mut save_map = false;
    let mut stock_map = if Path::new(COMPANY_MAP).exists() {
        let file = File::open(COMPANY_MAP).unwrap();
        let reader = io::BufReader::new(file);
        let stock_map: Vec<CompanyInfo> = serde_json::from_reader(reader).unwrap();
        stock_map
    } else {
        save_map = true;
        twse::company_map::fetch().await.unwrap()
    };

    // 過濾掉不需要的股票
    stock_map.retain(|company| {
        // skip 金融保險業 and stocks in the ignore list
        !ignore_stocks.contains(&company.stock_no) && company.industry != "17"
    });

    stock_map.sort_by(|a, b| a.stock_no.cmp(&b.stock_no));

    // Save the company map to a JSON file for future use
    if save_map {
        let json = serde_json::to_string_pretty(&stock_map).unwrap();
        std::fs::write(COMPANY_MAP, json).unwrap();
    }

    stock_map
}
