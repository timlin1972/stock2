use std::collections::HashMap;

use crate::cfg::data::CfgData;
use crate::stocks::company_map::CompanyMap;
use crate::stocks::data_company::DataCompany;

const MODULE_NAME: &str = "stocks::data";

pub struct Data {
    pub cfg: CfgData,
    pub company_map: CompanyMap,
    pub data_company: HashMap<String, DataCompany>,
}

impl Data {
    pub async fn new() -> Self {
        let cfg = CfgData::new();
        let company_map = CompanyMap::new().await;
        let mut data_company = HashMap::new();

        println!("[{MODULE_NAME}] Reading data for all companies...");

        for company in &company_map.stock_map {
            data_company.insert(
                company.stock_no.clone(),
                DataCompany::new(company.stock_no.clone()),
            );
        }

        Data {
            cfg,
            company_map,
            data_company,
        }
    }

    pub async fn fetch_year(&mut self, year: &str) {
        println!("[{MODULE_NAME}] Fetching data for all companies for year: {year}...");
        let mut index = 1;
        let total = self.data_company.len();
        for data_company in self.data_company.values_mut() {
            println!(
                "[{MODULE_NAME}] [{index}/{total}] Fetching and writing data for stock: {} ({})...",
                data_company.stock_no,
                self.company_map.get_name(&data_company.stock_no)
            );
            data_company.fetch_year(&self.cfg, year).await;

            // sleep 1 second to avoid hitting API rate limits
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            index += 1;
        }
    }

    pub async fn fetch_company_year(&mut self, stock_no: &str, year: &str) {
        if let Some(data_company) = self.data_company.get_mut(stock_no) {
            data_company.fetch_year(&self.cfg, year).await;
        } else {
            panic!("[{MODULE_NAME}] 找不到股票代號: {stock_no}",);
        }
    }

    // pub fn print(&self) {
    //     self.cfg.print();
    //     self.company_map.print();
    //     for data_company in self.data_company.values() {
    //         data_company.print();
    //     }
    // }
}
