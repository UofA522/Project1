# Rust Project 1 for 522 - Stock Market Monitor

## Overview 

The goal of the project is as follows:
1. Write a simple stock monitoring program that takes a stock symbol as input
2. The output fo the program is a chart showing the daily closing price for the last 6 months
3. Print the minimum and maximum closing price for the last 6 months 
4. Plot the days when volatility in the price was greater than 2% of the closing price.
5. Also plot one of the financial analysis algorithms like RSI or MACD.

## The crates used in this project and their uses are described as follows:

  1. `clap`: ‘clap’ crate is used for building flexible command line interface applications. [Link]
  
  2. `yahoo_finance_api`: ‘yahoo_finance_api’ has been used in this project to fetch the stock quotes of the past six months using the yahoo! finance API. [Link]
  
  3. `tokio`:
  
  4. `lazy_static`:
  
  5. `log`:
  
  6. `log4rs`:
  
  7. `plotters`: ‘plotters’ crate is used to plot various types of graphs like line chart, histogram, candlesticks plot etc. [Link]
  
  8. `chrono`: The ‘chrono’ crate is a library which contains a list of functions which can be used to perform various operations on dates and times in the Proleptic Gregorian calendar. [Link] 
  
  9. `ta`: ‘ta’ crate is used for technical analysis tasks. We have used this crate to calculate a variety of financial analysis indicators like RSI (Relative Strength   Index), MACD (Moving Average Convergence Divergence) etc. [Link]


## Financial Analysis Indicators/Algorithms used:

1. Volatile Days: On the stock chart volatile days are plotted in order to indicate the days where stock price varied by more than 2% of the total price.

2. Bollinger Bands: We have implemented the Bollinger Bands Indicators by using the ta crate. For Bollinger Bands, three lines are plotted (middle band, upper band, lower band). The middle band is usually calculated as a simple moving average over a particular period (a period of 20 has been used in our program). The upper band and lower band are calculated by adding and subtracting the multiple of standard deviation respectively to the average value. Mathematically, these bands can be written as follows: 


```
BollingerBandsMiddle Band = Simple Moving Average (SMA).
BollingerBandsUpper Band = SMA + SD of observation * multiplier (usually 2.0)
BollingerBandsLower Band = SMA - SD of observation * multiplier (usually 2.0)

Relative Strength Index (RSI): We have implemented the Relative Strength Index (RSI) Indicators by using the ta crate. Mathematically RSI can be defined as follows:
RSIt = EMAUt * 100 / (EMAUt + EMADt)
```


### Symbol Definitions

1. RSIt : value of RSI indicator in a moment of time t
2. EMAUt : value of EMA of up periods in a moment of time t
3. EMADt : value of EMA of down periods in a moment of time t

The RSI indicator is studied in order to identify the oversold or overbought conditions of a stock.

Fast/Slow Exponential Moving Average (EMA):

Fast/Slow Simple Moving Average (SMA):

Moving Average Convergence Divergence (MACD):

## Charting Setup

## Project Setup

## Usage Instructions

## References:
1. [ta](https://docs.rs/ta/0.5.0/ta/)
2. [chrono](https://crates.io/crates/chrono)

