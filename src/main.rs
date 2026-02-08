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

    menu::main_menu::main_menu(&mut data).await;
}
