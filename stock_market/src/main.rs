use clap::Parser;
use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, error};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
// use std::error::Error;
use std::sync::Mutex;
use yahoo_finance_api::{YResponse, YahooConnector, YahooError};
use plotters::prelude::*;
use chrono::{DateTime,Utc};


fn stock_picture_creator(stock_data: &Vec<(u64,f64,f64,f64,bool)>) -> Result<(), Box<dyn std::error::Error>> {
    // generate a stock price chart and save it as a PNG file
    // fn stock_picture_creator(heading: &str, prices: &[f64]) -> Result<(), Box<dyn std::error::Error>> {}
    // Sample stock price data: (timestamp, close, low, high, status)
    // whereever true candle stick needs to be there

    // Convert UNIX timestamp to DateTime for x-axis
    let times: Vec<DateTime<Utc>> = stock_data.iter()
        .map(|&(unix_timestamp, _, _, _, _)| DateTime::from_timestamp(unix_timestamp as i64, 0).unwrap())
        .collect();

    // let open_prices: Vec<f64> = stock_data.iter().map(|&(_, close, _, _, _, _)| close).collect();
    let high_prices: Vec<f64> = stock_data.iter().map(|&(_, _, _, high, _)| high).collect();
    let low_prices: Vec<f64> = stock_data.iter().map(|&(_, _, low, _, _)| low).collect();
    let close_prices: Vec<f64> = stock_data.iter().map(|&(_, close, _, _ ,_)| close).collect();
    let variations: Vec<f64> = stock_data.iter()
        .map(|&(_,_, low, high, _)| ((high - low) / low) * 100.0)
        .collect();

    // Create a drawing area
    let root = BitMapBackend::new("stock_chart.png", (680, 680)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Stock Prices Chart", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(55)
        .y_label_area_size(40)
        .build_cartesian_2d(times[0]..times[times.len() - 1], 160.0..250.0)?;

    chart.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_label_formatter(&|x| x.format("%Y-%m-%d").to_string())
        .draw()?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(close_prices.iter()).map(|(&time, &close)| (time, close)),
        &RED,
    ))?;

    // circle points where variation is > 2% and plot low, high, and connect them with a line
    for (i, &time) in times.iter().enumerate() {
        let low = low_prices[i];
        let high = high_prices[i];
        let close = close_prices[i];
        let variation = variations[i];

        if variation > 2.0 {
            // Draw a vertical line from low to high
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(time, low), (time, high)],
                &BLUE, // Color for the high-low line
            )))?;

            // Draw a flat line for the low price
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(time - chrono::Duration::seconds(3600), low), (time + chrono::Duration::seconds(3600), low)],
                &BLUE, // Flat line for low price
            )))?;

            // Draw a flat line for the high price
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(time - chrono::Duration::seconds(3600), high), (time + chrono::Duration::seconds(3600), high)],
                &BLUE, // Flat line for high price
            )))?;

            // Draw a hollow circle for the close price
            chart.draw_series(std::iter::once(Circle::new(
                (time, close),
                5, // size of the marker
                ShapeStyle {
                    color: BLUE.to_rgba(), // Hollow circle for close price
                    filled: false,
                    stroke_width: 1,
                }, // Hollow circle for the close price
            )))?;
        }
    }
    root.present()?;

    Ok(())
}

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
    /// Interval of the stock, by default set to 1 day to retrieve daily stock prices
    #[arg(short, long, default_value = "1d")]
    interval: String,
    /// Range of date you are interested in getting the data for
    #[arg(short, long, default_value = "6mo")]
    range: String,
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
    let response = fetch_stock(&stock_args.name, &stock_args.interval, &stock_args.range).await;
    match response {
        Ok(data) => {
            let quotes = data.quotes();
            match quotes {
                Ok(stock_quotes) => {
                    let mut stock_prices: Vec<(u64, f64, f64, f64, bool)> = Vec::new();
                    for quote in stock_quotes {
                        let mut volatile:bool = false;
                        let intra_day_high_low:f64 = (quote.high - quote.low) as f64;
                        let threshold = 0.02 * (quote.close as f64);
                        if intra_day_high_low > threshold {
                            volatile = true;
                        }
                        let s = (quote.timestamp, quote.close as f64, quote.low as f64, quote.high as f64,volatile);
                        stock_prices.push(s);
                        println!("{:?}", s);
                        //Plot graph here
                        match stock_picture_creator(&stock_prices) {
                            Err(e) => println!("{:?}", e),
                            _ => ()
                        }
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

async fn fetch_stock(stock_name: &str, interval: &str, range: &str) -> Result<YResponse, YahooError> {
    CONNECTOR.lock().unwrap().get_quote_range(stock_name, interval, range).await
}
