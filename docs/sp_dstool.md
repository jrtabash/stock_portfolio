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
- **Consym**: Check datastore contains symbol
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
                                    consym : check datastore contains symbol
                                    check  : check history, dividend and split data
                                    stat   : calculate files count and size
    -e, --export <export_file>      Export symbol history and dividends to csv file. Required with export operation
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
    -y, --symbol <symbol>           Stock symbol. Optional with update and check operations. Required with drop, reset,
                                    showh, showd, shows, consym and export operations
```
