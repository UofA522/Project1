
use clap::Parser;
use yahoo_finance_api as yahoo;

use lazy_static::lazy_static;
use std::sync::Mutex;
#[derive(Parser, Debug)]
#[command(version="0.1.0", about="CLI Program to fetch stock using Yahoo finance", long_about = "CLI Program to fetch stock using Yahoo finance. The CLI is build using the clap crate in rust")]
struct Stock {
    /// Short name of a stock.
    #[arg(short, long)]
    name : String
}
#[tokio::main]
async fn main() {
    let stock_args = Stock::parse();
    let provider = yahoo::YahooConnector::new().expect("Failed to Create Yahoo connector");
}

async fn fetch_stock(stock_name: String) {

}


