use clap::Parser;
use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, error, info};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::sync::Mutex;
use yahoo_finance_api::{Decimal, Quote, YResponse, YahooConnector, YahooError};

// Implements lazy singleton pattern for Yahoo Connector
lazy_static! {
    static ref CONNECTOR: Mutex<YahooConnector> = Mutex::new(YahooConnector::new().expect("Unable to Create a Yahoo Connector"));
}
// Creating a parser for the CLI program that takes in the name of the stock
#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    about = "CLI Program to fetch stock using Yahoo finance",
    long_about = "CLI Program to fetch stock using Yahoo finance. The CLI is build using the clap crate in rust"
)]
struct Stock {
    /// Ticker name of a stock.
    #[arg(short, long)]
    name: String,
    /// Interval of the stock, by default set to 6 months
    #[arg(short, long, default_value = "6mo")]
    interval: String,
}
fn init_log()
{
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .build(LevelFilter::Debug)).unwrap();

    let _ = log4rs::init_config(config);
}
#[tokio::main]
async fn main() {
    init_log();
    let stock_args = Stock::parse();
    debug!("Ticker Name set :{}",stock_args.name);
    debug!("Interval Set:{}",stock_args.interval);
    debug!("About to fetch Stock from Yahoo");
    let response = fetch_stock(&stock_args.name, &stock_args.name).await;
    match response {
        Ok(data) => {
            let quotes = data.quotes();
            match quotes {
                Ok(stock_quotes) => {
                    let mut closing_prices:Vec<(u64,Decimal,Decimal,Decimal)> = Vec::new();
                    for quote in stock_quotes {
                        let s = (quote.timestamp,quote.close,quote.low,quote.high);
                        closing_prices.push(s);
                        println!("{:?}", s)
                    }
                }
                Err(e) => {
                    error!("Error with the quotes retrieved: {}",e);
                    println!("Error with the quotes retrieved");
                }
            }
        }
        Err(e) => {
            error!("Yahoo finance errored out with the following error:{}",e);
            println!("Error occured check log file for more details");
        }
    }
}

async fn fetch_stock(stock_name: &str, interval: &str) -> Result<YResponse, YahooError> {
    CONNECTOR.lock().unwrap().get_latest_quotes(stock_name, interval).await
}


