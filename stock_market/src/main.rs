
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version="0.1.0", about, long_about = None)]
struct Stock {
    #[arg(short, long)]
    name : String
}
fn main() {
    let stock_args = Stock::parse();
    println!("Stock:{}",stock_args.name);
}


