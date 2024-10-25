# Rust Project 1 for 522 - Stock Market Monitor

## Group-4
- Alton Lavin D'souza (1814026)
- Kalyana Abenanth Gurunathan (1836448)
- Prabhroop Singh (1865486)

## Overview 

The goal of the project is as follows:
1. Write a simple stock monitoring program that takes a stock symbol as input
2. The output fo the program is a chart showing the daily closing price for the last 6 months
3. Print the minimum and maximum closing prices for the last 6 months along with the respective dates of those closing prices.
4. Plot the days when volatility in the price was greater than 2% of the closing price.
5. Also plot one of the financial analysis algorithms like RSI or MACD.

## The crates used in this project and their uses are described as follows:

  1. `clap`: crate is used for building flexible command line interface applications. [Link]
  
  2. `yahoo_finance_api`:  has been used in this project to fetch the stock quotes of the past six months using the yahoo! finance API. [Link]
  
  3. `tokio`: A runtime for writing reliable asynchronous applications in Rust.
  
  4. `lazy_static`: A crate for declaring lazily evaluated static variables
  
  5. `log`: A crate used for providing a single API that abstracts various logging implementations
  
  6. `log4rs`: Logging implementation used for our application, modelled after popular Java logging libraries like `logback` and `log4j`
  
  7. `plotters`:  crate is used to plot various types of graphs like line chart, histogram, candlesticks plot etc. [Link]
  
  8. `chrono`: The ‘chrono’ crate is a library which contains a list of functions which can be used to perform various operations on dates and times in the Proleptic Gregorian calendar. [Link] 
  
  9. `ta`: crate is used for technical analysis tasks. We have used this crate to calculate a variety of financial analysis indicators like RSI (Relative Strength   Index), MACD (Moving Average Convergence Divergence) etc.


## Financial Analysis Indicators/Algorithms used:

### 1. Volatile Days 
On the stock chart volatile days are plotted in order to indicate the days where stock price varied by more than 2% of the total price.

### 2. Bollinger Bands 
We have implemented the Bollinger Bands Indicators by using the ta crate. For Bollinger Bands, three lines are plotted (middle band, upper band, lower band). The middle band is usually calculated as a simple moving average over a particular period (a period of 20 has been used in our program). The upper band and lower band are calculated by adding and subtracting the multiple of standard deviation respectively to the average value. Mathematically, these bands can be written as follows: 


```
BollingerBandsMiddle Band = Simple Moving Average (SMA).
BollingerBandsUpper Band = SMA + SD of observation * multiplier (usually 2.0)
BollingerBandsLower Band = SMA - SD of observation * multiplier (usually 2.0)

```
### 3. Relative Strength Index (RSI)
We have implemented the Relative Strength Index (RSI) Indicators by using the ta crate. Mathematically RSI can be defined as follows:
```
RSIt = EMAUt * 100 / (EMAUt + EMADt)
```
where 
1. RSIt : value of RSI indicator in a moment of time t
2. EMAUt : value of EMA of up periods in a moment of time t
3. EMADt : value of EMA of down periods in a moment of time t

The RSI indicator is studied in order to identify the oversold or overbought conditions of a stock. A period of 14 has been used in this project to calculate the RSI.

### 4. Fast/Slow Exponential Moving Average (EMA):

### 5. Fast/Slow Simple Moving Average (SMA):

### 6. Moving Average Convergence Divergence (MACD):

## Charting Setup

For plotting all the charts we used plotters and labelled the x and y axis where and when needed.

We have used Line and Histogram Charts primarily.
The charts looks like as follows:

![stock_chart_TSLA](https://github.com/user-attachments/assets/afc0b38d-afe9-4aa6-9d2d-ea325377133a)

![stock_chart_BNY](https://github.com/user-attachments/assets/2602b544-ffc0-4582-9fa9-a21f52faa18b)

![macd_TSLA](https://github.com/user-attachments/assets/f97ef130-f6f6-4115-b6e6-2dcfb6902467)

![macd_BNY](https://github.com/user-attachments/assets/dd5af0f4-c8e3-4354-a5cb-df871baf1b6f)

![sma_TSLA](https://github.com/user-attachments/assets/d8327974-b596-4c3e-a67e-792074a4ab68)

![sma_BNY](https://github.com/user-attachments/assets/341217f6-5fcf-4b18-a603-b17d0b1178a2)


![rsiTSLA](https://github.com/user-attachments/assets/4fc46294-a1e7-4c5f-882f-20b5be03abd2)

![sma_BNY](https://github.com/user-attachments/assets/011434a6-5afb-4fdf-bcaa-3c04eb2e4e90)

![sma_TSLA](https://github.com/user-attachments/assets/1b741162-cb45-496c-8811-9e3704a6ad51)

![stock_chart_BNY](https://github.com/user-attachments/assets/dde48da4-1efd-435f-aa04-91144b5bb971)

![stock_chart_TSLA](https://github.com/user-attachments/assets/4c39ed70-6203-4bbd-b21f-4615608a42c4)







Autoscaling is supported for height not width, width is fixed.


## Project Setup
To setup the project lets look at the project setup that you might have:

![image](https://github.com/user-attachments/assets/e4e89dfa-0567-4f46-9064-735ff605ba18)

After you extract the file your file directory should something as above. Follow the steps.

1. Install Rust from [here](https://www.rust-lang.org/tools/install)
2. Run  `cd ./stock_market`
3. Run `cargo build --release`
4. You should now see a `target` directory
5. Run  `cd ./target/release`
6. Then run `./stock_marker.exe --name AAPL`
![image](https://github.com/user-attachments/assets/b8148f23-519c-4c1f-a3c9-bddbbb7075c8)

`Note: All errors are logged to a log file under the 'log' directory`


## Usage Instructions
### Help
```
 ./stock_market.exe --help
```
or 

```
 ./stock_market.exe -h
```
Output:

![image](https://github.com/user-attachments/assets/91fc9570-894a-4523-a6a7-4823225e4070)

### Version
```
./stock_market.exe --version
```
or 
```
./stock_market.exe -v
```
### Basic Usage
```
./stock_market --name <STOCK_TICKER_NAME>
```
This shows the daily stock quotes for the last 6 months
#### Example
![image](https://github.com/user-attachments/assets/cf07acfc-1d69-406e-80b7-a7f331053888)

### Usages with different range and interval
```
./stock_market --name <STOCK_TICKER_NAME> --range <range> --interval <interval>
```
#### Example
![image](https://github.com/user-attachments/assets/c09a6e1e-3c50-4647-947f-792d39fa48a0)

The above example works only if you supported ranges and intervals as shown in the below table.
##### Table for Supported Range and interval 
![image](https://github.com/user-attachments/assets/b6c952ed-dd8a-4f6f-a4fd-8c102faf6d5a)


## References:
1. [ta](https://docs.rs/ta/0.5.0/ta/)
2. [chrono](https://crates.io/crates/chrono)
3. [yahoo_finance_api](https://crates.io/crates/yahoo_finance_api)


