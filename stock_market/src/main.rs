
use clap::Parser;
use yahoo_finance_api as yahoo;

use lazy_static::lazy_static;
use std::sync::Mutex;
use yahoo_finance_api::YahooConnector;

// Implements lazy singleton pattern for Yahoo Connector
lazy_static! {
    static ref CONNECTOR: Mutex<YahooConnector> = Mutex::new(YahooConnector::new().expect("Unable to Create a Yahoo Connector"));
}
// Creating a parser for the CLI program that takes in the name of the stock
#[derive(Parser, Debug)]
#[command(version="0.1.0", about="CLI Program to fetch stock using Yahoo finance", long_about = "CLI Program to fetch stock using Yahoo finance. The CLI is build using the clap crate in rust")]
struct Stock {
    /// Short name of a stock.
    #[arg(short, long)]
    name : String
}

fn main() {
    let stock_args = Stock::parse();
}

async fn fetch_stock(stock_name: String) {
    CONNECTOR.lock().unwrap().get_latest_quotes()
}


