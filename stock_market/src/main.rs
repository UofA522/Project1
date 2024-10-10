
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version="0.1.0", about="CLI Program to fetch stock using Yahoo finance", long_about = "CLI Program to fetch stock using Yahoo finance. The CLI is build using the clap crate in rust")]
struct Stock {
    /// Short name of a stock.
    #[arg(short, long)]
    name : String
}
fn main() {
    let stock_args = Stock::parse();
    fetch_stock(stock_args.name)
}

fn fetch_stock(stock_name: String) {

}


