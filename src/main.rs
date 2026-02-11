mod analysis;
mod cfg;
mod common;
mod fugle;
mod menu;
mod scripts;
mod stocks;
mod storage;
mod twse;

#[tokio::main]
async fn main() {
    let mut data = stocks::data::Data::new().await;

    let results = scripts::complex::find_complex_dark_cloud_cover(&data, "20260211");
    println!("符合條件的股票數量: {}", results.len());
    for stock in &results {
        println!(
            "股票代號: {}, 日期: {}, 收盤價: {}, -30%: {}",
            stock.stock_no,
            stock.stock_data.date,
            stock.stock_data.close,
            stock.stock_data.close * 0.7,
        );
    }

    menu::main_menu::main_menu(&mut data).await;
}
