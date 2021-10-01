# Stock Portfolio Tool
Get latest close prices and report the gains and losses of stocks in portfolio.

Given a stocks file, containing symbol, type, date purchased, quantity purchased, and purchase/base price,
get the latest close prices from yahoo finance and generate a stocks value report, showing base, current and net prices
and notional values, as well as percent change.

The following features are supported:
- **Group by**: Report quantities and current notional values grouped by symbol
- **Order by**: Sort by pre-defined attributes in ascending or descending order
- **Filter**: Include and/or exclude by type or list of symbols
- **Export**: Export gains and losses table to a csv file
- **Cache**: Cache latest close prices. Cache stored in temporary folder

## Usage
```bash
USAGE:
    stock_portfolio [FLAGS] [OPTIONS] --stocks <stocks_file>

FLAGS:
   -d, --desc            Used with order by option to sort in descending order
   -h, --help            Prints help information
   -g, --show-groupby    Show quantities and current notional values grouped by symbol
   -V, --version         Prints version information

OPTIONS:
   -c, --cache <cache_file>      Local cache file to store latest stock prices
   -x, --exclude <exclude>       Exclude stocks by type or symbols; one of stock, etf or a comma separated list of symbols
   -e, --export <export_file>    Export gains and losses table to a csv file
   -i, --include <include        Include stocks by type or symbols; one of stock, etf or a comma separated list of symbols
   -o, --orderby <order_by>      Order stocks by one of symbol, type, date, days, price, net, pct, size or value
   -s, --stocks <stocks_file>    CSV file containing stocks in portfolio, formatted as 'symbol,type,date,quantity,base_price'
                                 including a header line. Supported type values include stock and etf
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
            Date: 2021-07-28
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 38585.00
       Net Value: 7685.00
  Percent Change: 24.87

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    --------- 
AAPL     2020-09-20 2021-07-28    311      100   115.00   144.98    29.98    26.07     11500.00     14498.00    2998.00
AAPL     2020-11-12 2021-07-28    258      100   118.50   144.98    26.48    22.35     11850.00     14498.00    2648.00
DELL     2021-02-10 2021-07-28    168      100    75.50    95.89    20.39    27.01      7550.00      9589.00    2039.00
```

## Example 2
```bash
$ stock_portfolio --show-groupby --stocks example_stocks.csv

Stocks Value Report
-------------------
            Date: 2021-07-28
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 38585.00
       Net Value: 7685.00
  Percent Change: 24.87

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    --------- 
AAPL     2020-09-20 2021-07-28    311      100   115.00   144.98    29.98    26.07     11500.00     14498.00    2998.00
AAPL     2020-11-12 2021-07-28    258      100   118.50   144.98    26.48    22.35     11850.00     14498.00    2648.00
DELL     2021-02-10 2021-07-28    168      100    75.50    95.89    20.39    27.01      7550.00      9589.00    2039.00

GroupBy  Size     Cur Value   
-------  ----     ---------   
AAPL          200     28996.00
DELL          100      9589.00
```

## Example 3
```bash
$ stock_portfolio --show-groupby --stocks example_stocks.csv --orderby date --desc

Stocks Value Report
-------------------
            Date: 2021-07-28
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 38585.00
       Net Value: 7685.00
  Percent Change: 24.87

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    --------- 
DELL     2021-02-10 2021-07-28    168      100    75.50    95.89    20.39    27.01      7550.00      9589.00    2039.00
AAPL     2020-11-12 2021-07-28    258      100   118.50   144.98    26.48    22.35     11850.00     14498.00    2648.00
AAPL     2020-09-20 2021-07-28    311      100   115.00   144.98    29.98    26.07     11500.00     14498.00    2998.00

GroupBy  Size     Cur Value   
-------  ----     ---------   
DELL          100      9589.00
AAPL          200     28996.00
```

## Example 4
```bash
$ stock_portfolio --stocks example_stocks.csv --include DELL

Stocks Value Report
-------------------
            Date: 2021-07-28
Number of Stocks: 1
      Base Value: 7550.00
    Latest Value: 9589.00
       Net Value: 2039.00
  Percent Change: 27.01

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    --------- 
DELL     2021-02-10 2021-07-28    168      100    75.50    95.89    20.39    27.01      7550.00      9589.00    2039.00
```
