use std::fs;

use serde::{Deserialize, Serialize};

const CFG_FILE: &str = "cfg.json";

#[derive(Serialize, Deserialize)]
pub struct CfgData {
    pub fugle_api_key: String,
}

impl CfgData {
    fn read_from_file() -> Self {
        let file = fs::File::open(CFG_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader::<_, CfgData>(reader).unwrap()
    }

    pub fn new() -> Self {
        Self::read_from_file()
    }

    // pub fn print(&self) {
    //     println!("Fugle API Key: {}", self.fugle_api_key);
    // }
}
