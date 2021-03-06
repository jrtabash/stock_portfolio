# Stock Portfolio Tool
Get latest close prices and report gain and loss of stocks in portfolio.

Given a stocks file, containing ticker, date purchased, quantity purchased, and purchase/base price,
get the latest close prices and generate a stocks value report, showing base, current and net prices
and notional values.

Optionally, the tool also reports quantities and current notional values grouped by ticker.

## Usage
```bash
USAGE:
    stock_portfolio [FLAGS] --stocks <stocks_file>

FLAGS:
   -h, --help            Prints help information
   -g, --show-groupby    Show quantities and current notional values grouped by ticker
   -V, --version         Prints version information

OPTIONS:
   -s, --stocks <stocks_file>    CSV file containing stocks in portfolio, formatted as
                                 'symbol,date,quantity,base_price' including a header line
```

## Example Stocks File
```csv
symbol,date,quantity,base_price
AAPL,2020-09-20,100,115.00
DELL,2021-02-10,100,75.50
```
