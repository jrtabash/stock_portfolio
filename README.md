# Stock Portfolio Tools
- sp_report: Stock Portfolio Report
- sp_dstool: Stock Portfolio Datastore Tool
- sp_stats: Stock Portfolio Stats Tool
- sp_yhist: Stock Portfolio YFinance History Tool

## Stock Portfolio Report
Get latest close prices and dividends and generate report. Supported reports:
- value : Gains and losses of stocks in portfolio
- top   : Top/bottom performers in portfolio

Given a stocks file, containing symbol, type, date purchased, quantity purchased, and purchase/base price,
get the latest close prices and dividends from the datastore and generate a report. The value report shows base,
current and net prices and notional values, percent change, cumulative dividend, as well as cumulative dividends.
The top report shows top and bottom performing stocks in several categories.

The following features are supported:
- **Group by**: Report quantities and current notional values grouped by symbol
- **Order by**: Sort by pre-defined attributes in ascending or descending order
- **Filter**: Include and/or exclude by type, list of symbols, or expression
- **Export**: Export gains and losses table to a csv file

```bash
USAGE:
    sp_report [FLAGS] [OPTIONS] --stocks <stocks_file>

FLAGS:
    -d, --desc            Used with order by option to sort in descending order
    -h, --help            Prints help information
    -g, --show-groupby    Show quantities and current notional values grouped by symbol
    -V, --version         Prints version information

OPTIONS:
    -n, --name <ds_name>          Datastore name (default: sp_datastore)
    -r, --root <ds_root>          Datastore root path (default: value of HOME environment variable)
    -x, --exclude <exclude>       Filter stocks by type, symbols or expression;
                                  If type, must be one of 'stock' or 'etf'.
                                  If symbols, must be a comma separated list of symbol names.
                                  If expression, must follow the format '<field> <op> <value>', where:
                                  <field> : one of days, price, net, pct, div, size, value
                                  <op>    : one of =, !=, <, >, <=, >=
                                  Example : 'days > 365'
    -e, --export <export_file>    Export gains and losses table to a csv file
    -i, --include <include>       Filter stocks by type, symbols or expression;
                                  If type, must be one of 'stock' or 'etf'.
                                  If symbols, must be a comma separated list of symbol names.
                                  If expression, must follow the format '<field> <op> <value>', where:
                                  <field> : one of days, price, net, pct, div, size, value
                                  <op>    : one of =, !=, <, >, <=, >=
                                  Example : 'days > 365'
    -o, --orderby <order_by>      Order stocks by one of symbol, type, date, days, price, net, pct, div, size or value
    -p, --type <report_type>      Report type, one of value, top (default: value)
                                  value : stocks value (gains & losses)
                                  top   : Top/Bottom performing stocks
    -s, --stocks <stocks_file>    CSV file containing stocks in portfolio, formatted as
                                  'symbol,type,date,quantity,base_price' including a header line. Supported type values
                                  include stock and etf
```

## Stock Portfolio Datastore Tool
Manage datastore and symbol price and size data. Data includes open, high, low, and close prices, trading volumes, and dividends.

The following operations are supported:
- **Create**: Create datastore
- **Delete**: Delete datastore
- **Update**: Update price and size data
- **Drop**: Drop symbol
- **Export**: Export symbol history and dividends
- **Check**: Check price and size data
- **Stat**: Calculate files count and size

```bash
USAGE:
    sp_dstool [FLAGS] [OPTIONS] --dsop <ds_operation>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -n, --name <ds_name>          Datastore name (default: sp_datastore)
    -o, --dsop <ds_operation>     Datastore tool operation, one of create, delete, update, drop, export, check, stat.
                                  create : create empty datastore
                                  delete : delete existing datastore
                                  update : update price and size data
                                  drop   : drop a symbol
                                  export : export symbol history and dividends
                                  check  : check price and size data
                                  stat   : calculate files count and size
    -r, --root <ds_root>          Datastore root path (default: value of HOME environment variable)
    -e, --export <export_file>    Export symbol history and dividends to csv file. Required with export operation
    -s, --stocks <stocks_file>    CSV file containing stocks in portfolio, formatted as
                                  'symbol,type,date,quantity,base_price' including a header line. Supported type values
                                  include stock and etf
    -y, --symbol <symbol>         Stock symbol. Optional with update and check operations. Required with drop and export
                                  symbol operation
```

## Stock Portfolio Stats Tool
Describe and calculate symbol stats.

The following calculations are supported:
- **desc**: Describe symbol history
- **divdesc**: Describe symbol dividends
- **vwap**: Calculate symbol adjusted close volume weighted average price
- **mvwap**: Calculate symbol adjusted close moving volume weighted average price
- **roc**: Calculate symbol adjusted close rate of change
- **pctch**: Calculate symbol adjusted close percent change relative to from date

```bash
USAGE:
    sp_stats [OPTIONS] --calc <calculate> --symbol <symbol>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --calc <calculate>    Calculate stats, one of desc, divdesc, vwap, mvwap, roc, pctch.
                              desc    : describe symbol history
                              divdesc : describe symbol dividends
                              vwap    : calculate symbol adjusted close volume weighted average price
                              mvwap   : calculate symbol adjusted close moving volume weighted average price
                              roc     : calculate symbol adjusted close rate of change
                              pctch   : calculate symbol adjusted close percent change relative to from date
    -n, --name <ds_name>      Datastore name (default: sp_datastore)
    -r, --root <ds_root>      Datastore root path (default: value of HOME environment variable)
    -f, --from <from_date>    Start from date YYYY-MM-DD
    -y, --symbol <symbol>     Stock symbol
    -w, --window <window>     Number of days window, required with mvwap and roc calculations
```

## Stock Portfolio YHistory Tool
Query yfinance history.

The following events are supported:
- **history**: Historical open, high, low, close, adj_close, volume data
- **dividend**: Historical dividend data
- **split**: Historical split data

The following intervals are supported:
- **day**: Daily intervals
- **week**: Weekly intervals
- **month**: Monthly intervals

```bash
USAGE:
    sp_yhist [OPTIONS] --events <events> --interval <interval> --symbol <symbol>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --events <events>        Events to query, one of history, dividend, split
    -f, --from <from_date>       Start date YYYY-MM-DD (default: today - 7days)
    -i, --interval <interval>    Interval to query, one of day, week, month
    -y, --symbol <symbol>        Stock symbol
    -t, --to <to_date>           Stop date YYYY-MM-DD (default: today)
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

## Example Report 5
```bash
$ sp_report --name sp_sample --stocks sample.csv --include 'pct > 0.0' --orderby pct --desc
Stocks Value Report
-------------------
            Date: 2022-01-10
Number of Stocks: 2
      Base Value: 23350.00
    Latest Value: 34438.00
       Net Value: 11088.00
  Percent Change: 47.49
    Cum Dividend: 193.50

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2022-01-10    477      100   115.00   172.19    57.19    49.73     11500.00     17219.00    5719.00   107.00
AAPL     2020-11-12 2022-01-10    424      100   118.50   172.19    53.69    45.31     11850.00     17219.00    5369.00    86.50
```

## Example Report 6
```bash
$ sp_report --name sp_sample --stocks sample.csv -i 'days < 365'
Stocks Value Report
-------------------
            Date: 2022-01-10
Number of Stocks: 1
      Base Value: 7550.00
    Latest Value: 5988.00
       Net Value: -1562.00
  Percent Change: -20.69
    Cum Dividend: 0.00

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
DELL     2021-02-10 2022-01-10    334      100    75.50    59.88   -15.62   -20.69      7550.00      5988.00   -1562.00     0.00
```

## Example Stats 1
```bash
$ sp_stats -n sp_research -c desc -y MSFT -f 2021-11-22
  from: 2021-11-22
    to: 2021-12-10
symbol: MSFT
 field:         open         high          low        close    adj_close           volume
 count:           14           14           14           14           14               14
   min:     323.9500     327.4200     318.0300     323.0100     323.0100    21561374.0000
   max:     344.6200     349.6700     339.5500     340.7900     340.7900    42885600.0000
  mean:     334.3050     337.5271     330.0829     333.2050     333.2050    29581302.5000
   std:       4.4476       5.0684       5.8090       5.2430       5.2430     6674763.8635
   25%:     331.9900     335.5000     328.1200     329.6800     329.6800    24217200.0000
   50%:     334.9600     337.8550     330.4500     334.0100     334.0100    30173951.5000
   75%:     335.3200     339.2800     333.9100     337.6800     337.6800    31031100.0000
```

## Example Stats 2
```bash
$ sp_stats -n sp_research -c mvwap -y MSFT -f 2021-11-22 -w 5
  from: 2021-11-22
    to: 2021-12-10
symbol: MSFT
 field: adj_close
 mvwap: 
2021-11-29 336.5613
2021-11-30 334.1417
2021-12-01 332.5284
2021-12-02 331.2141
2021-12-03 329.4912
2021-12-06 327.7966
2021-12-07 328.4034
2021-12-08 329.0780
2021-12-09 329.5750
2021-12-10 333.6194
```
