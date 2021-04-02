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
   -e, --export <export_file>    Export gains and losses table to a csv file
   -f, --filter <filter>         Filter stocks by type or symbols; one of stock, etf or a comma separated list of symbols
   -o, --orderby <order_by>      Order stocks by one of symbol, type, date, price, net, pct, size or value
   -s, --stocks <stocks_file>    CSV file containing stocks in portfolio, formatted as
                                 'symbol,type,date,quantity,base_price' including a header line. Supported type values
                                 include stock and etf
```

## Example Stocks File
```csv
symbol,type,date,quantity,base_price
AAPL,stock,2020-09-20,100,115.00
AAPL,stock,2020-11-12,100,118.50
DELL,stock,2021-02-10,100,75.50
```

## Example 1
```bash
$ stock_portfolio --stocks example_stocks.csv

Stocks Value Report
-------------------
            Date: 2021-04-01
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 33508.00
       Net Value: 2608.00
  Percent Change: 8.44

Ticker   Buy Date   Upd Date   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value
------   --------   --------   ----     ----     ---      ---      ---      ----------   ---------    ---------
AAPL     2020-09-20 2021-04-01      100   115.00   123.00     8.00     6.96     11500.00     12300.00     800.00
AAPL     2020-11-12 2021-04-01      100   118.50   123.00     4.50     3.80     11850.00     12300.00     450.00
DELL     2021-02-10 2021-04-01      100    75.50    89.08    13.58    17.99      7550.00      8908.00    1358.00
```

## Example 2
```bash
$ stock_portfolio --show-groupby --stocks example_stocks.csv

Stocks Value Report
-------------------
            Date: 2021-04-01
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 33508.00
       Net Value: 2608.00
  Percent Change: 8.44

Ticker   Buy Date   Upd Date   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value
------   --------   --------   ----     ----     ---      ---      ---      ----------   ---------    ---------
AAPL     2020-09-20 2021-04-01      100   115.00   123.00     8.00     6.96     11500.00     12300.00     800.00
AAPL     2020-11-12 2021-04-01      100   118.50   123.00     4.50     3.80     11850.00     12300.00     450.00
DELL     2021-02-10 2021-04-01      100    75.50    89.08    13.58    17.99      7550.00      8908.00    1358.00

GroupBy  Size     Cur Value
-------  ----     ---------
AAPL          200     24600.00
DELL          100      8908.00
```

## Example 3
```bash
$ stock_portfolio --show-groupby --stocks example_stocks.csv --orderby date --desc

Stocks Value Report
-------------------
            Date: 2021-04-01
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 33508.00
       Net Value: 2608.00
  Percent Change: 8.44

Ticker   Buy Date   Upd Date   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value
------   --------   --------   ----     ----     ---      ---      ---      ----------   ---------    ---------
DELL     2021-02-10 2021-04-01      100    75.50    89.08    13.58    17.99      7550.00      8908.00    1358.00
AAPL     2020-11-12 2021-04-01      100   118.50   123.00     4.50     3.80     11850.00     12300.00     450.00
AAPL     2020-09-20 2021-04-01      100   115.00   123.00     8.00     6.96     11500.00     12300.00     800.00

GroupBy  Size     Cur Value
-------  ----     ---------
DELL          100      8908.00
AAPL          200     24600.00
```

## Example 4
```bash
$ stock_portfolio --stocks example_stocks.csv --filter DELL

Stocks Value Report
-------------------
            Date: 2021-04-01
Number of Stocks: 1
      Base Value: 7550.00
    Latest Value: 8908.00
       Net Value: 1358.00
  Percent Change: 17.99

Ticker   Buy Date   Upd Date   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value
------   --------   --------   ----     ----     ---      ---      ---      ----------   ---------    ---------
DELL     2021-02-10 2021-04-01      100    75.50    89.08    13.58    17.99      7550.00      8908.00    1358.00
```
