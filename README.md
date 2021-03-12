# Stock Portfolio Tool
Get latest close prices and report the gains and losses of stocks in portfolio.

Given a stocks file, containing symbol, date purchased, quantity purchased, and purchase/base price,
get the latest close prices and generate a stocks value report, showing base, current and net prices
and notional values.

Optionally, the tool also reports quantities and current notional values grouped by symbol.

## Usage
```bash
USAGE:
    stock_portfolio [FLAGS] [OPTIONS] --stocks <stocks_file>

FLAGS:
   -d, --desc            Used with order by option to sort in descending order
   -h, --help            Prints help information
   -g, --show-groupby    Show quantities and current notional values grouped by symbol
   -c, --use-cache       Use local cache to store latest stock prices
   -V, --version         Prints version information

OPTIONS:
   -o, --orderby <order_by>      Order stocks by one of symbol, date or value
   -s, --stocks <stocks_file>    CSV file containing stocks in portfolio, formatted as
                                 'symbol,date,quantity,base_price' including a header line
```

## Example Stocks File
```csv
symbol,date,quantity,base_price
AAPL,2020-09-20,100,115.00
DELL,2021-02-10,100,75.50
```
