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
