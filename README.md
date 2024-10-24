# Project1
Rust Project 1 for 522 - Stock Market Monitor

1. The crates used in this project and their uses are described as follows:

  clap: ‘clap’ crate is used for building flexible command line interface applications. [Link]
  
  yahoo_finance_api: ‘yahoo_finance_api’ has been used in this project to fetch the stock quotes of the past six months using the yahoo! finance API. [Link]
  
  tokio:
  
  lazy_static:
  
  log:
  
  log4rs:
  
  plotters: ‘plotters’ crate is used to plot various types of graphs like line chart, histogram, candlesticks plot etc. [Link]
  
  chrono: The ‘chrono’ crate is a library which contains a list of functions which can be used to perform various operations on dates and times in the Proleptic Gregorian calendar. [Link] 
  
  ta: ‘ta’ crate is used for technical analysis tasks. We have used this crate to calculate a variety of financial analysis indicators like RSI (Relative Strength   Index), MACD (Moving Average Convergence Divergence) etc. [Link]


2. Financial Analysis Indicators/Algorithms used:

Volatile Days: On the stock chart volatile days are plotted in order to indicate the days where stock price varied by more than 2% of the total price.

Bollinger Bands: We have implemented the Bollinger Bands Indicators by using the ta crate. For Bollinger Bands, three lines are plotted (middle band, upper band, lower band). The middle band is usually calculated as a simple moving average over a particular period (a period of 20 has been used in our program). The upper band and lower band are calculated by adding and subtracting the multiple of standard deviation respectively to the average value. Mathematically, these bands can be written as follows: 

BollingerBandsMiddle Band = Simple Moving Average (SMA).
BollingerBandsUpper Band = SMA + SD of observation * multiplier (usually 2.0)
BollingerBandsLower Band = SMA - SD of observation * multiplier (usually 2.0)

Relative Strength Index (RSI): We have implemented the Relative Strength Index (RSI) Indicators by using the ta crate. Mathematically RSI can be defined as follows:
RSIt = EMAUt * 100 / (EMAUt + EMADt)

Where:

RSIt : value of RSI indicator in a moment of time t
EMAUt : value of EMA of up periods in a moment of time t
EMADt : value of EMA of down periods in a moment of time t

The RSI indicator is studied in order to identify the oversold or overbought conditions of a stock.

Fast/Slow Exponential Moving Average (EMA):

Fast/Slow Simple Moving Average (SMA):

Moving Average Convergence Divergence (MACD):


3. Charting Setup

4. Project Setup

5. Usage Instructions

References:
https://docs.rs/ta/0.5.0/ta/
https://crates.io/crates/chrono

