use chrono::{DateTime, Utc};
use clap::Parser;
use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, error};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use plotters::prelude::*;
use std::sync::Mutex;
use ta::indicators::{BollingerBands, ExponentialMovingAverage, MovingAverageConvergenceDivergence as Macd, RelativeStrengthIndex, SimpleMovingAverage};
use ta::Next;
use yahoo_finance_api::{YResponse, YahooConnector, YahooError};

// Implements lazy singleton pattern for Yahoo Connector
lazy_static! {
    static ref CONNECTOR: Mutex<YahooConnector> = Mutex::new(YahooConnector::new().expect("Unable to Create a Yahoo Connector"));
}

const WINDOW_WIDTH: u32 = 2000;
const WINDOW_HEIGHT: u32 = 480;
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

// generate a stock price chart and save it as a PNG file
// fn stock_picture_creator(heading: &str, prices: &[f64]) -> Result<(), Box<dyn std::error::Error>> {}
// Sample stock price data: (timestamp, close, low, high, status)
// whereever true candle stick needs to be there

// Convert UNIX timestamp to DateTime for x-axis
fn stock_picture_creator(stock_data: &Vec<(u64, f64, f64, f64, bool)>, ticker_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let times: Vec<DateTime<Utc>> = stock_data.iter()
        .map(|&(unix_timestamp, _, _, _, _)| DateTime::from_timestamp(unix_timestamp as i64, 0).unwrap())
        .collect();

    // let open_prices: Vec<f64> = stock_data.iter().map(|&(_, close, _, _, _, _)| close).collect();
    let high_prices: Vec<f64> = stock_data.iter().map(|&(_, _, _, high, _)| high).collect();
    let low_prices: Vec<f64> = stock_data.iter().map(|&(_, _, low, _, _)| low).collect();
    let close_prices: Vec<f64> = stock_data.iter().map(|&(_, close, _, _, _)| close).collect();
    let variations: Vec<f64> = stock_data.iter()
        .map(|&(_, _, low, high, _)| ((high - low) / low) * 100.0)
        .collect();

    // Create a drawing area
    let stock_chart_file_name = "stock_chart_".to_owned() + ticker_name + ".png";
    let root = BitMapBackend::new(&stock_chart_file_name, (WINDOW_WIDTH, WINDOW_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;
    let y_max = close_prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = close_prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let mut chart = ChartBuilder::on(&root)
        .caption("Stock Prices Chart ".to_owned() + ticker_name, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(55)
        .y_label_area_size(40)
        .build_cartesian_2d(times[0]..times[times.len() - 1], y_min..y_max)?;

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
    let mut closing_prices_vec: Vec<f64> = Vec::new();
    let response = fetch_stock(&stock_args.name, &stock_args.interval, &stock_args.range).await;
    match response {
        Ok(data) => {
            let quotes = data.quotes();
            match quotes {
                Ok(stock_quotes) => {
                    let mut stock_prices: Vec<(u64, f64, f64, f64, bool)> = Vec::new();
                    for quote in stock_quotes {
                        let mut volatile: bool = false;
                        let intra_day_high_low: f64 = (quote.high - quote.low) as f64;
                        let threshold = 0.02 * (quote.close as f64);
                        if intra_day_high_low > threshold {
                            volatile = true;
                        }
                        let s = (quote.timestamp, quote.close as f64, quote.low as f64, quote.high as f64, volatile);
                        stock_prices.push(s);
                        closing_prices_vec.push(quote.close);
                    }
                    let (max_date,max_price, min_date, min_price) = fetch_min_max_closing_pricing_with_dates(&stock_prices);
                    println!("Max closing Price For {} : {} on {}",&stock_args.name,max_price,DateTime::from_timestamp(max_date as i64, 0).unwrap());
                    println!("Min closing Price For {} : {} on {}",&stock_args.name,min_price,DateTime::from_timestamp(min_date as i64,0).unwrap());

                    let times: Vec<DateTime<Utc>> = stock_prices.iter()
                        .map(|&(unix_timestamp, _, _, _, _)| DateTime::from_timestamp(unix_timestamp as i64, 0).unwrap())
                        .collect();
                    match stock_picture_creator(&stock_prices, &stock_args.name) {
                        Err(e) => {
                            println!("Error Occured while creating the chart");
                            error!("Error Occured while creating the chart:{:?}",e)
                        }
                        _ => ()
                    }

                    //Bollinger Bands
                    match bollinger_bands(closing_prices_vec.clone(), 20, 2.0,&stock_args.name,times.clone()) {
                        Ok(_) => {debug!("Bollinger Bands Created")}
                        Err(e) => {println!("Error while creating bolLinger bands check log file for more details");
                            error!("Error while creating Bollinger Bands:{:?}",e)
                        }
                    }

                    //RSI
                    match rsi(closing_prices_vec.clone(), 14, &stock_args.name,times.clone()) {
                        Ok(_) => {debug!("RSI Chart Created Successfully")}
                        Err(e) => {
                            println!("Error While creating RSI chart check log file for more details");
                            error!("Error while creating RSI Chart :{:?}",e);}
                    }

                    //MACD
                    match macd(closing_prices_vec.clone(), 12, 26, 9,&stock_args.name) {
                        Ok(_) => {
                            debug!("MACD Chart Created Successfully")
                        }
                        Err(e) => {
                            println!("Error while creating MACD Chart check log file for details");
                            error!("Error while creating MACD chart: {:?}",e)
                        }
                    }

                    // Fast/Slow Exponential Moving Averages
                    match exponential_moving_averages(closing_prices_vec.clone(), 20, 50,&stock_args.name,times.clone()) {
                        Ok(_) => {
                            debug!("Graph for Exponential Moving Averages Created")
                        }
                        Err(e) => {
                            println!("Error while creating Exponential Moving Averages Graph check log file for more details");
                            error!("Error while creating Exponential Moving Averages Graph:{:?}",e);
                        }
                    };

                    //Fast/Slow Simple Moving Averages
                    match simple_moving_averages(closing_prices_vec.clone(), 20, 50, &stock_args.name,times.clone()) {
                        Ok(_) => {debug!("Graph for simple moving averages created")}
                        Err(e) => {
                            println!("Error while creating simple moving averages graph check log file for more details");
                            debug!("Error while creating simple moving Average {:?}",e)}
                    }
                }
                Err(e) => {
                    error!("Error with the quotes retrieved: {}",e);
                    println!("Error with the quotes retrieved");
                }
            }
        }
        Err(e) => {
            error!("Yahoo finance errored out with the following error, ensure you have specified the right ticker and have used supported ranges and intervals also ensure you are connected to the internet:{}",e);
            println!("Error occured check log file for more details");
        }
    }
}

fn fetch_min_max_closing_pricing_with_dates(stock_prices: &Vec<(u64, f64, f64, f64, bool)>) -> (u64,f64,u64,f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut min_date = 0;
    let mut max_date = 0;
    for stock_price in stock_prices{
        // println!("{:?}",stock_price);
        if stock_price.1 < min {
            min = stock_price.1;
            min_date = stock_price.0;
        }
        if stock_price.1 > max {
            max = stock_price.1;
            max_date = stock_price.0;
        }
    }
    (max_date,max,min_date,min)
}

async fn fetch_stock(stock_name: &str, interval: &str, range: &str) -> Result<YResponse, YahooError> {
    CONNECTOR.lock().unwrap().get_quote_range(stock_name, interval, range).await
}

fn bollinger_bands(closing_prices: Vec<f64>, period: usize, multiplier: f64, ticker_name: &str, times: Vec<DateTime<Utc>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut average = Vec::new();
    let mut upper_bands = Vec::new();
    let mut lower_bands = Vec::new();

    let mut bb = BollingerBands::new(period, multiplier).unwrap();

    closing_prices.iter()
        .for_each(|&price| {
            let output = bb.next(price);
            average.push(output.average);
            upper_bands.push(output.upper);
            lower_bands.push(output.lower);
        });

    let bollinger_bands_file_name = "bollinger_bands_".to_owned() + ticker_name + ".png";
    let root = BitMapBackend::new(&bollinger_bands_file_name, (WINDOW_WIDTH, WINDOW_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find the minimum and maximum values in the vector, handling negative values
    let min_value = lower_bands.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_value = upper_bands.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Set the chart area, handling the range from min_value to max_value
    let mut chart = ChartBuilder::on(&root)
        .caption("Bollinger Bands Graph For ".to_owned() + ticker_name, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(55)
        .y_label_area_size(40)
        .build_cartesian_2d(times[0]..times[times.len() - 1], min_value..max_value)?;

    chart.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_label_formatter(&|x| x.format("%Y-%m-%d").to_string())
        .draw()?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(average.iter()).map(|(&time, &avg)| (time, avg)),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(upper_bands.iter()).map(|(&time, &upper)| (time, upper)),
        &BLUE,
    ))?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(lower_bands.iter()).map(|(&time, &lower)| (time, lower)),
        &BLACK,
    ))?;

    root.present()?;
    Ok(())
}

fn rsi(closing_prices: Vec<f64>, period: usize, ticker_name: &str, times: Vec<DateTime<Utc>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut rsi = RelativeStrengthIndex::new(period).unwrap();
    let mut vector: Vec<f64> = Vec::new();

    for price in closing_prices {
        let rsi_value = rsi.next(price);
        vector.push(rsi_value);
        // println!("{:?}", rsi_value);
    }

    let rsi_file_name = "rsi".to_owned() + ticker_name + ".png";
    let root = BitMapBackend::new(&rsi_file_name, (WINDOW_WIDTH, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find the minimum and maximum values in the vector, handling negative values
    //let min_value = vector.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let min_value = 0.0;
    let max_value = vector.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Set the chart area, handling the range from min_value to max_value
    let mut chart = ChartBuilder::on(&root)
        .caption("RSI Chart for ".to_owned() + ticker_name, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(55)
        .y_label_area_size(40)
        .build_cartesian_2d(times[0]..times[times.len() - 1], min_value..max_value)?;

    chart.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_label_formatter(&|x| x.format("%Y-%m-%d").to_string())
        .draw()?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(vector.iter()).map(|(&time, &price)| (time, price)),
        &RED,
    ))?;
    //chart.configure_series_labels().border_style(&BLACK).draw()?;

    root.present()?;
    Ok(())
}

fn macd(
    closing_prices: Vec<f64>,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    ticker_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut macd = Macd::new(fast_period, slow_period, signal_period).unwrap();

    let mut macd_line: Vec<f64> = Vec::new();
    let mut signal_line: Vec<f64> = Vec::new();
    let mut histogram: Vec<f64> = Vec::new();

    for price in closing_prices {
        let macd_result = macd.next(price);
        macd_line.push(macd_result.macd);
        signal_line.push(macd_result.signal);
        histogram.push(macd_result.histogram);
    }

    let macd_chart_file_name = "macd_".to_owned()+ticker_name+".png";
    let root = BitMapBackend::new(&macd_chart_file_name, (WINDOW_WIDTH, WINDOW_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find the minimum and maximum values in the MACD and Signal line
    let min_value = macd_line.iter().chain(macd_line.iter()).cloned().fold(f64::INFINITY, f64::min);
    let max_value = macd_line.iter().chain(macd_line.iter()).cloned().fold(f64::NEG_INFINITY, f64::max);

    // Set the chart area, handling the range from min_value to max_value
    let mut chart = ChartBuilder::on(&root)
        .caption("MACD Graph For ".to_owned() + ticker_name, ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..signal_line.len(), min_value..max_value)?;

    // Configure the mesh (grid) and labels
    chart.configure_mesh().draw()?;

    // Draw the MACD line in black
    chart.draw_series(LineSeries::new(
        macd_line.iter().enumerate().map(|(i, &y)| (i, y)),
        &BLACK,
    ))?;

    // Draw the Signal line in red
    chart.draw_series(LineSeries::new(
        signal_line.iter().enumerate().map(|(i, &y)| (i, y)),
        &RED,
    ))?;

    // Draw the histogram, with positive values in green and negative values in red
    for (i, &hist_value) in histogram.iter().enumerate() {
        let bar_color = if hist_value >= 0.0 { GREEN.filled() } else { RED.filled() };
        chart.draw_series(Histogram::vertical(&chart)
                              .style(bar_color)
                              .data(std::iter::once((i, hist_value))),
        )?;
    }

    root.present()?;
    Ok(())
}


fn exponential_moving_averages(closing_prices_vec: Vec<f64>, fast_period: usize, slow_period: usize, ticker_name:&str, times: Vec<DateTime<Utc>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut fast_ema = ExponentialMovingAverage::new(fast_period).unwrap(); // Fast EMA
    let mut slow_ema = ExponentialMovingAverage::new(slow_period).unwrap(); // Slow EMA

    let mut fast_ema_values = Vec::new();
    let mut slow_ema_values = Vec::new();

    for price in closing_prices_vec {
        let fast_value = fast_ema.next(price);
        let slow_value = slow_ema.next(price);

        fast_ema_values.push(fast_value);
        slow_ema_values.push(slow_value);
    }

    let ema_chart_filename = "ema_".to_owned()+ticker_name+".png";
    let root = BitMapBackend::new(&ema_chart_filename, (WINDOW_WIDTH, WINDOW_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find the minimum and maximum values in the vector, handling negative values
    let min_value = fast_ema_values.iter().chain(slow_ema_values.iter()).fold(f64::INFINITY, |a, &b| a.min(b));
    let max_value = fast_ema_values.iter().chain(slow_ema_values.iter()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Set the chart area, handling the range from min_value to max_value
    let mut chart = ChartBuilder::on(&root)
        .caption("Exponential Moving Averages for ".to_owned() + ticker_name, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(55)
        .y_label_area_size(40)
        .build_cartesian_2d(times[0]..times[times.len() - 1], min_value..max_value)?;

    chart.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_label_formatter(&|x| x.format("%Y-%m-%d").to_string())
        .draw()?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(fast_ema_values.iter()).map(|(&time, &price)| (time, price)),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(slow_ema_values.iter()).map(|(&time, &price)| (time, price)),
        &BLACK,
    ))?;

    root.present()?;
    Ok(())
}

fn simple_moving_averages(closing_prices_vec: Vec<f64>, fast_period: usize, slow_period: usize, ticker_name:&str, times: Vec<DateTime<Utc>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut fast_sma = SimpleMovingAverage::new(fast_period).unwrap(); // Fast SMA
    let mut slow_sma = SimpleMovingAverage::new(slow_period).unwrap(); // Slow EMA

    let mut fast_sma_values = Vec::new();
    let mut slow_sma_values = Vec::new();

    for price in closing_prices_vec {
        let fast_value = fast_sma.next(price);
        let slow_value = slow_sma.next(price);

        fast_sma_values.push(fast_value);
        slow_sma_values.push(slow_value);
    }

    let sma_chart_file_name = "sma_".to_owned() + ticker_name + ".png";
    let root = BitMapBackend::new(&sma_chart_file_name, (WINDOW_WIDTH, WINDOW_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find the minimum and maximum values in the vector, handling negative values
    let min_value = fast_sma_values.iter().chain(slow_sma_values.iter()).fold(f64::INFINITY, |a, &b| a.min(b));
    let max_value = fast_sma_values.iter().chain(slow_sma_values.iter()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Set the chart area, handling the range from min_value to max_value
    // Set the chart area, handling the range from min_value to max_value
    let mut chart = ChartBuilder::on(&root)
        .caption("Simple Moving Averages Chart for ".to_owned() + ticker_name, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(55)
        .y_label_area_size(40)
        .build_cartesian_2d(times[0]..times[times.len() - 1], min_value..max_value)?;

    chart.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_label_formatter(&|x| x.format("%Y-%m-%d").to_string())
        .draw()?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(fast_sma_values.iter()).map(|(&time, &price)| (time, price)),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        times.iter().zip(slow_sma_values.iter()).map(|(&time, &price)| (time, price)),
        &BLACK,
    ))?;

    root.present()?;
    Ok(())
}

 
