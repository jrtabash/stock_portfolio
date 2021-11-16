# Stock Portfolio Tools
- sp_report: Stock Portfolio Report
- sp_dstool: Stock Portfolio Datastore Tool

## Stock Portfolio Report
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
- **Datastore**: Get latest close price from given datastore. When enabled, disables cache

```bash
USAGE:
    sp_report [FLAGS] [OPTIONS] --stocks <stocks_file>

FLAGS:
    -d, --desc            Used with order by option to sort in descending order
    -h, --help            Prints help information
    -g, --show-groupby    Show quantities and current notional values grouped by symbol
    -V, --version         Prints version information

OPTIONS:
    -c, --cache <cache_file>      Local cache file to store latest stock prices. Ignored when datastore root is specified
    -n, --name <ds_name>          Datastore name, used with datastore root (default: sp_datastore)
    -r, --root <ds_root>          Datastore root path, use to update portfolio latest prices. When specified,
                                  local cache file will be ignored
    -x, --exclude <exclude>       Exclude stocks by type or symbols; one of stock, etf or a comma separated list of symbols
    -e, --export <export_file>    Export gains and losses table to a csv file
    -i, --include <include>       Include stocks by type or symbols; one of stock, etf or a comma separated list of symbols
    -o, --orderby <order_by>      Order stocks by one of symbol, type, date, days, price, net, pct, size or value
    -s, --stocks <stocks_file>    CSV file containing stocks in portfolio, formatted as 'symbol,type,date,quantity,base_price'
                                  including a header line. Supported type values include stock and etf
```

## Stock Portfolio Datastore Tool
Manage datastore and symbol price and size data. Data includes open, high, low, and close prices, trading volumes, and dividends.

The following operations are supported:
- **Create**: Create datastore
- **Delete**: Delete datastore
- **Update**: Update price and size data
- **Drop**: Drop symbol
- **Check**: Check price and size data

```bash
USAGE:
    sp_dstool [OPTIONS] --dsop <ds_operation> --root <ds_root>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -n, --name <ds_name>          Datastore name (default: sp_datastore)
    -o, --dsop <ds_operation>     Datastore tool operation, one of create, delete, update, drop, check
    -r, --root <ds_root>          Datastore root path
    -s, --stocks <stocks_file>    CSV file containing stocks in portfolio, refer to sp_report --help for more
                                  information. File is required with update operation
    -y, --symbol <symbol>         Stock symbol. Optional with update and check operations. Required with drop symbol operation
```

## Example Stocks File
```csv
symbol,type,date,quantity,base_price
AAPL,stock,2020-09-20,100,115.00
AAPL,stock,2020-11-12,100,118.50
DELL,stock,2021-02-10,100,75.50
```

## Example Report 1
```bash
$ sp_report --stocks example_stocks.csv

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

## Example Report 2
```bash
$ sp_report --show-groupby --stocks example_stocks.csv

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

## Example Report 3
```bash
$ sp_report --show-groupby --stocks example_stocks.csv --orderby date --desc

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

## Example Report 4
```bash
$ sp_report --stocks example_stocks.csv --include DELL

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
