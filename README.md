# Stock Portfolio Tools
- sp_report: Stock Portfolio Report
- sp_dstool: Stock Portfolio Datastore Tool

## Stock Portfolio Report
Get latest close prices and dividends and report the gains and losses of stocks in portfolio.

Given a stocks file, containing symbol, type, date purchased, quantity purchased, and purchase/base price,
get the latest close prices and dividends from the datastore and generate a stocks value report, showing base,
current and net prices and notional values, percent change, cumulative dividend, as well as cumulative dividends.

The following features are supported:
- **Group by**: Report quantities and current notional values grouped by symbol
- **Order by**: Sort by pre-defined attributes in ascending or descending order
- **Filter**: Include and/or exclude by type or list of symbols
- **Export**: Export gains and losses table to a csv file
- **Cache**: Cache latest close prices. Cache stored in temporary folder
- **Datastore**: Get latest close price from given datastore. When enabled, disables cache

```bash
USAGE:
    sp_report [FLAGS] [OPTIONS] --root <ds_root> --stocks <stocks_file>

FLAGS:
    -d, --desc            Used with order by option to sort in descending order
    -h, --help            Prints help information
    -g, --show-groupby    Show quantities and current notional values grouped by symbol
    -V, --version         Prints version information

OPTIONS:
    -n, --name <ds_name>          Datastore name, used with datastore root (default: sp_datastore)
    -r, --root <ds_root>          Datastore root path, use to update portfolio latest prices
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

## Sample Stocks File
```csv
symbol,type,date,quantity,base_price
AAPL,stock,2020-09-20,100,115.00
AAPL,stock,2020-11-12,100,118.50
DELL,stock,2021-02-10,100,75.50
```

## Example Report 1
```bash
$ sp_report --root ~/ --name sp_sample --stocks sample.csv

Stocks Value Report
-------------------
            Date: 2021-11-16
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 35870.00
       Net Value: 4970.00
  Percent Change: 16.08
    Cum Dividend: 193.50

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2021-11-16    422      100   115.00   151.00    36.00    31.30     11500.00     15100.00    3600.00   107.00
AAPL     2020-11-12 2021-11-16    369      100   118.50   151.00    32.50    27.43     11850.00     15100.00    3250.00    86.50
DELL     2021-02-10 2021-11-16    279      100    75.50    56.70   -18.80   -24.90      7550.00      5670.00   -1880.00     0.00
```

## Example Report 2
```bash
$ sp_report --root ~/ --name sp_sample --stocks sample.csv --show-groupby

Stocks Value Report
-------------------
            Date: 2021-11-16
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 35870.00
       Net Value: 4970.00
  Percent Change: 16.08
    Cum Dividend: 193.50

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2021-11-16    422      100   115.00   151.00    36.00    31.30     11500.00     15100.00    3600.00   107.00
AAPL     2020-11-12 2021-11-16    369      100   118.50   151.00    32.50    27.43     11850.00     15100.00    3250.00    86.50
DELL     2021-02-10 2021-11-16    279      100    75.50    56.70   -18.80   -24.90      7550.00      5670.00   -1880.00     0.00

GroupBy  Size     Cur Value   
-------  ----     ---------   
AAPL          200     30200.00
DELL          100      5670.00
```

## Example Report 3
```bash
$ sp_report --root ~/ --name sp_sample --stocks sample.csv --show-groupby --orderby date --desc

Stocks Value Report
-------------------
            Date: 2021-11-16
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 35870.00
       Net Value: 4970.00
  Percent Change: 16.08
    Cum Dividend: 193.50

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
DELL     2021-02-10 2021-11-16    279      100    75.50    56.70   -18.80   -24.90      7550.00      5670.00   -1880.00     0.00
AAPL     2020-11-12 2021-11-16    369      100   118.50   151.00    32.50    27.43     11850.00     15100.00    3250.00    86.50
AAPL     2020-09-20 2021-11-16    422      100   115.00   151.00    36.00    31.30     11500.00     15100.00    3600.00   107.00

GroupBy  Size     Cur Value   
-------  ----     ---------   
DELL          100      5670.00
AAPL          200     30200.00
```

## Example Report 4
```bash
$ sp_report --root ~/ --name sp_sample --stocks sample.csv --include AAPL

Stocks Value Report
-------------------
            Date: 2021-11-16
Number of Stocks: 2
      Base Value: 23350.00
    Latest Value: 30200.00
       Net Value: 6850.00
  Percent Change: 29.34
    Cum Dividend: 193.50

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2021-11-16    422      100   115.00   151.00    36.00    31.30     11500.00     15100.00    3600.00   107.00
AAPL     2020-11-12 2021-11-16    369      100   118.50   151.00    32.50    27.43     11850.00     15100.00    3250.00    86.50
```
