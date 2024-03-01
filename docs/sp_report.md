## Stock Portfolio Report
Generate reports for portfolio stocks.

The following reports are supported:
- **Value**: Gains and losses of stocks in portfolio
- **Top**: Top/bottom performers in portfolio
- **Volat**: Volatility of stocks in portfolio
- **Daych**: Day change of stocks in portfolio
- **Closed**: Closed positions value
- **Divid**: Dividends of stocks in porfolio
- **Sum**: Summary of stocks in portfolio

Given a stocks file, containing symbol, type, date purchased, quantity purchased, and purchase/base price,
get the latest close prices and dividends from the datastore and generate a report. The value report shows base,
current and net prices and notional values, percent change, cumulative dividend, as well as cumulative dividends.
The top report shows top and bottom performing stocks in several categories. The volatility report shows overall
volatility and 22 day volatility per stock. The day change report shows previous price, price, change, percent
change, low, high and volume. The dividends report shows latest dividend, cumulative dividend, yearly dividend
and daily unit dividend. The summary report shows value, minimum, average and maximum over aggregated base,
latest and net prices as well as percent change.

The following features are supported:
- **Group by**: Report quantities, base notional and current notional values grouped by symbol
- **Order by**: Sort by pre-defined attributes in ascending or descending order
- **Filter**: Include and/or exclude by type, list of symbols, or expression
- **Export**: Export gains and losses table to a csv file

```bash
USAGE:
    sp_report [FLAGS] [OPTIONS] --config <stocks_config>

FLAGS:
    -d, --desc             Used with order by option to sort in descending order
    -h, --help             Prints help information
    -m, --match-symbols    Match closed positions to configured stock symbols post filtering and ordering
    -g, --show-groupby     Show quantities, base notional and current notional values grouped by symbol
    -V, --version          Prints version information

OPTIONS:
    -x, --exclude <exclude>         Filter stocks by type, symbols or expression;
                                    If type, must be one of 'cash', 'etf', or 'index'.
                                    If symbols, must be a comma separated list of symbol names.
                                    If expression, must follow the format '<field> <op> <value>', where:
                                    <field> : one of days, price, net, pct, div, size, value
                                    <op>    : one of =, !=, <, >, <=, >=
                                    Example : 'days > 365'
    -e, --export <export_file>      Export gains and losses table to a csv file
    -i, --include <include>         Filter stocks by type, symbols or expression;
                                    If type, must be one of 'cash', 'etf', or 'index'.
                                    If symbols, must be a comma separated list of symbol names.
                                    If expression, must follow the format '<field> <op> <value>', where:
                                    <field> : one of days, price, net, pct, div, size, value
                                    <op>    : one of =, !=, <, >, <=, >=
                                    Example : 'days > 365'
    -o, --orderby <order_by>        Order stocks by one of:
                                    symbol : stock symbol       | type    : stock type
                                    date   : base date          | days    : days held
                                    price  : latest price       | size    : quantity
                                    net    : net price          | pct     : percent change
                                    value  : notional value     | prevpr  : previous day price
                                    ladiv  : latest dividend    | div     : cumulative dividend
                                    yrdiv  : yearly dividend    | dudiv   : daily unit dividend
                                    volat  : overall volatility | volat22 : 22 day volatility
                                    volume : day volume         | change  : day change
                                    pctchg : day percent change | valchg  : day value change
                                    low    : day low price      | high    : day high price
    -p, --type <report_type>        Report type, one of value, top, volat (default: value)
                                    value  : stocks value (gains & losses)
                                    top    : Top/Bottom performing stocks
                                    volat  : Stocks volatility
                                    daych  : Stocks day change
                                    closed : Closed positions value
                                    divid  : Stoks dividend
                                    sum    : Stocks summary
    -l, --config <stocks_config>    Config file containing datastore root and name, stocks, closed positions and cash in
                                    portfolio. Both root and name can be set to "$default" which will use home path for
                                    root and sp_datastore for name.
                                    
                                    The stocks CSV block "csv{" should contain stocks in portfolio, with the following
                                    columns:
                                        symbol
                                        type
                                        date
                                        quantity
                                        base_price
                                    including a header line. Supported type values include cash, etf and index. A stocks
                                    CSV file block "csv_file{" can be used instead of a stocks CSV block. It should
                                    contain the path to a CSV file. The file should contain the CSV stocks data.
                                    
                                    The closed positions CSV block "csv{" should contain closed positions in portfolio,
                                    with the following columns:
                                        symbol
                                        type
                                        base_date
                                        exit_date
                                        quantity
                                        base_price
                                        exit_price
                                        base_fee
                                        exit_fee
                                        dividend
                                    including a header line. Supported type values include cash, etf and index. The
                                    closed positions CSV file block "csv_file{" can be used instead of a closed
                                    positions CSV block. It should contain the path to a CSV file. The file should
                                    contain the CSV closed positions data.
                                    
                                    Sample config 1:
                                        ds_root: $default
                                        ds_name: my_datastore
                                        stocks: csv{
                                          symbol,type,date,quantity,base_price
                                          AAPL,cash,2020-09-20,100,115.00
                                        }
                                    
                                    Sample config 2:
                                        ds_root: $default
                                        ds_name: my_datastore
                                        cash: 1250.00
                                        stocks: csv_file{
                                          /path/to/my/stocks.csv
                                        }
                                        closed_positions: csv_file{
                                          /path/to/my/closed_positions.csv
                                        }
```
