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
