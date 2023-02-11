# Stock Portfolio Tools
- sp_report: Stock Portfolio Report
- sp_dstool: Stock Portfolio Datastore Tool
- sp_stats: Stock Portfolio Stats Tool
- sp_yhist: Stock Portfolio YFinance History Tool

## Stock Portfolio Report
Generate reports for portfolio stocks.

The following reports are supported:
- **Value**: Gains and losses of stocks in portfolio
- **Top**: Top/bottom performers in portfolio
- **Volat**: Volatility of stocks in portfolio
- **Daych**: Day change of stocks in portfolio

Given a stocks file, containing symbol, type, date purchased, quantity purchased, and purchase/base price,
get the latest close prices and dividends from the datastore and generate a report. The value report shows base,
current and net prices and notional values, percent change, cumulative dividend, as well as cumulative dividends.
The top report shows top and bottom performing stocks in several categories. The volatility report shows overall
volatility and 22 day volatility per stock. The day change report shows previous price, price, change, percent
change, low, high and volume.

The following features are supported:
- **Group by**: Report quantities and current notional values grouped by symbol
- **Order by**: Sort by pre-defined attributes in ascending or descending order
- **Filter**: Include and/or exclude by type, list of symbols, or expression
- **Export**: Export gains and losses table to a csv file

```bash
USAGE:
    sp_report [FLAGS] [OPTIONS] --config <stocks_config>

FLAGS:
    -d, --desc            Used with order by option to sort in descending order
    -h, --help            Prints help information
    -g, --show-groupby    Show quantities and current notional values grouped by symbol
    -V, --version         Prints version information

OPTIONS:
    -x, --exclude <exclude>         Filter stocks by type, symbols or expression;
                                    If type, must be one of 'stock', 'etf', or 'index'.
                                    If symbols, must be a comma separated list of symbol names.
                                    If expression, must follow the format '<field> <op> <value>', where:
                                    <field> : one of days, price, net, pct, div, size, value
                                    <op>    : one of =, !=, <, >, <=, >=
                                    Example : 'days > 365'
    -e, --export <export_file>      Export gains and losses table to a csv file
    -i, --include <include>         Filter stocks by type, symbols or expression;
                                    If type, must be one of 'stock', 'etf', or 'index'.
                                    If symbols, must be a comma separated list of symbol names.
                                    If expression, must follow the format '<field> <op> <value>', where:
                                    <field> : one of days, price, net, pct, div, size, value
                                    <op>    : one of =, !=, <, >, <=, >=
                                    Example : 'days > 365'
    -o, --orderby <order_by>        Order stocks by one of:
                                    symbol : stock symbol        | type    : stock type
                                    date   : base date           | days    : days held
                                    price  : latest price        | size    : quantity
                                    net    : net price           | pct     : percent change
                                    value  : notional value      | div     : cumulative dividend
                                    volat  : orderall volatility | volat22 : 22 day volatility
                                    prevpr : previous day price  | volume  : day volume
                                    change : day change          | pctchg  : day percent change
                                    low    : day low price       | high    : day high price
    -p, --type <report_type>        Report type, one of value, top, volat (default: value)
                                    value : stocks value (gains & losses)
                                    top   : Top/Bottom performing stocks
                                    volat : Stocks volatility
                                    daych : Stocks day change
    -l, --config <stocks_config>    Config file containing datastore root and name, as well as stocks in portfolio.
                                    Both root and name can be set to "$default" which will use home path for root and
                                    sp_datastore for name.
                                    The CSV block "csv{" should contain stocks in portfolio, formatted as
                                    'symbol,type,date,quantity,base_price' including a header line. Supported type
                                    values include stock, etf and index.
                                    A CSV file block "csv_file{" can be used instead of a CSV block. It should contain
                                    the path to a CSV file. The file should contain the CSV symbol data.
                                    Sample config 1:
                                        root: $default
                                        name: my_datastore
                                        stocks: csv{
                                          symbol,type,date,quantity,base_price
                                          AAPL,stock,2020-09-20,100,115.00
                                        }
                                    Sample config 2:
                                        root: $default
                                        name: my_datastore
                                        stocks: csv_file{
                                          /path/to/my/stocks.csv
                                        }
```

## Stock Portfolio Datastore Tool
Manage datastore and symbol price and size data. Data includes open, high, low, and close prices, trading volumes, dividends and splits.

The following operations are supported:
- **Create**: Create datastore
- **Delete**: Delete datastore
- **Update**: Update history, dividend and split data
- **Drop**: Drop symbol
- **Reset**: Reset symbol, equivalent to drop + update
- **Showh**: Show symbol history
- **Showd**: Show symbol dividends
- **Shows**: Show symbol splits
- **Export**: Export symbol history and dividends
- **Check**: Check history, dividend and split data
- **Stat**: Calculate files count and size

```bash
USAGE:
    sp_dstool [FLAGS] [OPTIONS] --dsop <ds_operation> --config <stocks_config>

FLAGS:
    -a, --auto-reset    Auto reset stocks on dividend and split updates
    -h, --help          Prints help information
    -V, --version       Prints version information
    -v, --verbose       Verbose mode

OPTIONS:
    -o, --dsop <ds_operation>       Datastore tool operation, one of create, delete, update, drop, reset, showh, showd,
                                    shows, export, check, stat.
                                    create : create empty datastore
                                    delete : delete existing datastore
                                    update : update history, dividend and split data
                                    drop   : drop a symbol
                                    reset  : Reset a symbol. Equivalent to drop + update
                                    showh  : show history for symbol
                                    showd  : show dividends for symbol
                                    shows  : show splits for symbol
                                    export : export symbol history and dividends
                                    check  : check history, dividend and split data
                                    stat   : calculate files count and size
    -e, --export <export_file>      Export symbol history and dividends to csv file. Required with export operation
    -l, --config <stocks_config>    Config file containing datastore root and name, as well as stocks in portfolio.
                                    Both root and name can be set to "$default" which will use home path for root and
                                    sp_datastore for name.
                                    The CSV block "csv{" should contain stocks in portfolio, formatted as
                                    'symbol,type,date,quantity,base_price' including a header line. Supported type
                                    values include stock, etf and index.
                                    A CSV file block "csv_file{" can be used instead of a CSV block. It should contain
                                    the path to a CSV file. The file should contain the CSV symbol data.
                                    Sample config 1:
                                        root: $default
                                        name: my_datastore
                                        stocks: csv{
                                          symbol,type,date,quantity,base_price
                                          AAPL,stock,2020-09-20,100,115.00
                                        }
                                    Sample config 2:
                                        root: $default
                                        name: my_datastore
                                        stocks: csv_file{
                                          /path/to/my/stocks.csv
                                        }
    -y, --symbol <symbol>           Stock symbol. Optional with update and check operations. Required with drop, reset,
                                    showh, showd, shows, and export operations
```

## Stock Portfolio Stats Tool
Describe and calculate symbol stats.

The following calculations are supported:
- **desc**: Describe symbol history
- **divdesc**: Describe symbol dividends
- **sa**: Calculate symbol simple average price
- **vwap**: Calculate symbol volume weighted average price
- **volat**: Calculate symbol volatility
- **sma**: Calculate symbol simple moving average price
- **mvwap**: Calculate symbol moving volume weighted average price
- **roc**: Calculate symbol rate of change
- **pctch**: Calculate symbol percent change relative to from date
- **mvolat**: Calculate symbol moving volatility
- **rsi**: Calculate symbol Relative Strength Index

```bash
USAGE:
    sp_stats [OPTIONS] --calc <calculate> --config <stocks_config> --symbol <symbol>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --calc <calculate>          Calculate stats, one of desc, divdesc, sa, vwap, volat, sma, mvwap, roc, pctch,
                                    mvolat, rsi.
                                    desc    : describe history
                                    divdesc : describe dividends
                                    sa      : calculate simple average price
                                    vwap    : calculate volume weighted average price
                                    volat   : calculate volatility
                                    sma     : calculate simple moving average price
                                    mvwap   : calculate moving volume weighted average price
                                    roc     : calculate rate of change
                                    pctch   : calculate percent change relative to from date
                                    mvolat  : calculate moving volatility
                                    rsi     : Calculate Relative Strength Index
    -i, --field <field>             Symbol history field to use in calculation.
                                    One of open, high, low, close, adj_close. Default adj_close.
                                    Applies to sa, vwap, volat, sma, mvwap, roc, pctch and mvolat
    -f, --from <from_date>          Start from date YYYY-MM-DD
    -l, --config <stocks_config>    Config file containing datastore root and name, as well as stocks in portfolio.
                                    Both root and name can be set to "$default" which will use home path for root and
                                    sp_datastore for name.
                                    The CSV block "csv{" should contain stocks in portfolio, formatted as
                                    'symbol,type,date,quantity,base_price' including a header line. Supported type
                                    values include stock, etf and index.
                                    A CSV file block "csv_file{" can be used instead of a CSV block. It should contain
                                    the path to a CSV file. The file should contain the CSV symbol data.
                                    Sample config 1:
                                        root: $default
                                        name: my_datastore
                                        stocks: csv{
                                          symbol,type,date,quantity,base_price
                                          AAPL,stock,2020-09-20,100,115.00
                                        }
                                    Sample config 2:
                                        root: $default
                                        name: my_datastore
                                        stocks: csv_file{
                                          /path/to/my/stocks.csv
                                        }
    -y, --symbol <symbol>           Stock symbol
    -w, --window <window>           Number of days, required with sma, mvwap, roc, mvolat and rsi calculations
                                    Required minimum: sma=1, mvwap=1, roc=2, mvolat=1, rsi=2
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

## Sample config file
```
root: $default
name: sp_sample
stocks: csv{
  symbol,type,date,quantity,base_price
  AAPL,stock,2020-09-20,100,115.00
  AAPL,stock,2020-11-12,100,118.50
  DELL,stock,2021-02-10,100,75.50
}
```

## Example Report 1
```bash
$ sp_report --config ~/sp_sample.cfg

Stocks Value Report
-------------------
            Date: 2023-02-03
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 35124.00
       Net Value: 4224.00
    Cum Dividend: 507.50
  Percent Change: 13.67
  Pct Chg w/ Div: 15.31

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2023-02-03    866      100   115.00   154.50    39.50    34.35     11500.00     15450.00    3950.00   198.00
AAPL     2020-11-12 2023-02-03    813      100   118.50   154.50    36.00    30.38     11850.00     15450.00    3600.00   177.50
DELL     2021-02-10 2023-02-03    723      100    75.50    42.24   -33.26   -44.05      7550.00      4224.00   -3326.00   132.00
```

## Example Report 2
```bash
$ sp_report --config ~/sp_sample.cfg --show-groupby

Stocks Value Report
-------------------
            Date: 2023-02-03
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 35124.00
       Net Value: 4224.00
    Cum Dividend: 507.50
  Percent Change: 13.67
  Pct Chg w/ Div: 15.31

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2023-02-03    866      100   115.00   154.50    39.50    34.35     11500.00     15450.00    3950.00   198.00
AAPL     2020-11-12 2023-02-03    813      100   118.50   154.50    36.00    30.38     11850.00     15450.00    3600.00   177.50
DELL     2021-02-10 2023-02-03    723      100    75.50    42.24   -33.26   -44.05      7550.00      4224.00   -3326.00   132.00

GroupBy  Size     Cur Value   
-------  ----     ---------   
AAPL          200     30900.00
DELL          100      4224.00
```

## Example Report 3
```bash
$ sp_report --config ~/sp_sample.cfg --show-groupby --orderby date --desc

Stocks Value Report
-------------------
            Date: 2023-02-03
Number of Stocks: 3
      Base Value: 30900.00
    Latest Value: 35124.00
       Net Value: 4224.00
    Cum Dividend: 507.50
  Percent Change: 13.67
  Pct Chg w/ Div: 15.31

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
DELL     2021-02-10 2023-02-03    723      100    75.50    42.24   -33.26   -44.05      7550.00      4224.00   -3326.00   132.00
AAPL     2020-11-12 2023-02-03    813      100   118.50   154.50    36.00    30.38     11850.00     15450.00    3600.00   177.50
AAPL     2020-09-20 2023-02-03    866      100   115.00   154.50    39.50    34.35     11500.00     15450.00    3950.00   198.00

GroupBy  Size     Cur Value   
-------  ----     ---------   
DELL          100      4224.00
AAPL          200     30900.00
```

## Example Report 4
```bash
$ sp_report --config ~/sp_sample.cfg --include AAPL

Stocks Value Report
-------------------
            Date: 2023-02-03
Number of Stocks: 2
      Base Value: 23350.00
    Latest Value: 30900.00
       Net Value: 7550.00
    Cum Dividend: 375.50
  Percent Change: 32.33
  Pct Chg w/ Div: 33.94

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2023-02-03    866      100   115.00   154.50    39.50    34.35     11500.00     15450.00    3950.00   198.00
AAPL     2020-11-12 2023-02-03    813      100   118.50   154.50    36.00    30.38     11850.00     15450.00    3600.00   177.50
```

## Example Report 5
```bash
$ sp_report --config ~/sp_sample.cfg --include 'pct > 0.0' --orderby pct --desc

Stocks Value Report
-------------------
            Date: 2023-02-03
Number of Stocks: 2
      Base Value: 23350.00
    Latest Value: 30900.00
       Net Value: 7550.00
    Cum Dividend: 375.50
  Percent Change: 32.33
  Pct Chg w/ Div: 33.94

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2023-02-03    866      100   115.00   154.50    39.50    34.35     11500.00     15450.00    3950.00   198.00
AAPL     2020-11-12 2023-02-03    813      100   118.50   154.50    36.00    30.38     11850.00     15450.00    3600.00   177.50
```

## Example Report 6
```bash
$ sp_report --config ~/sp_sample.cfg -i 'days > 850'

Stocks Value Report
-------------------
            Date: 2023-02-03
Number of Stocks: 1
      Base Value: 11500.00
    Latest Value: 15450.00
       Net Value: 3950.00
    Cum Dividend: 198.00
  Percent Change: 34.35
  Pct Chg w/ Div: 36.07

Symbol   Buy Date   Upd Date   Days   Size     Base     Cur      Net      Pct      Base Value   Cur Value    Net Value  Cum Div 
------   --------   --------   ----   ----     ----     ---      ---      ---      ----------   ---------    ---------  ------- 
AAPL     2020-09-20 2023-02-03    866      100   115.00   154.50    39.50    34.35     11500.00     15450.00    3950.00   198.00
```

## Example Stats 1
```bash
$ sp_stats -l ~/sp_research.cfg -c desc -y MSFT -f 2021-11-22
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
$ sp_stats -l ~/sp_research.cfg -c mvwap -y MSFT -f 2021-11-22 -w 5
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
