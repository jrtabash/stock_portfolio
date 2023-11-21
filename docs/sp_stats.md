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
                                    including a header line. Supported type values include stock, etf and index. A
                                    stocks CSV file block "csv_file{" can be used instead of a stocks CSV block. It
                                    should contain the path to a CSV file. The file should contain the CSV stocks data.
                                    
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
                                    including a header line. Supported type values include stock, etf and index. The
                                    closed positions CSV file block "csv_file{" can be used instead of a closed
                                    positions CSV block. It should contain the path to a CSV file. The file should
                                    contain the CSV closed positions data.
                                    
                                    Sample config 1:
                                        ds_root: $default
                                        ds_name: my_datastore
                                        stocks: csv{
                                          symbol,type,date,quantity,base_price
                                          AAPL,stock,2020-09-20,100,115.00
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
    -y, --symbol <symbol>           Stock symbol
    -w, --window <window>           Number of days, required with sma, mvwap, roc, mvolat and rsi calculations
                                    Required minimum: sma=1, mvwap=1, roc=2, mvolat=1, rsi=2
```
