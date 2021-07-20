# Order Top of Book (Given Symbol + Time)

```
USAGE:
cargo run <INPUT> <SYMBOL> <TIME>
```

INPUT: 
- file path (i.e. `./*.pcap`)

SYMBOL:
- max length 8 character symbol (i.e. `AAPL`) ["Quoted security represented in Nasdaq Integrated symbology"](http://www.nasdaqtrader.com/trader.aspx?id=CQSsymbolconvention)

TIME:
- `%H:%M:%S.%N` (i.e. HOUR:MINUTE:SECOND.NANOS)

## Pre-Reqs

Download a DEEP file from [Market Data | IEX](https://iextrading.com/trading/market-data/).

I personally used:
- [2021-07-12 (click to download)](https://www.googleapis.com/download/storage/v1/b/iex/o/data%2Ffeeds%2F20210712%2F20210712_IEXTP1_DEEP1.0.pcap.gz?generation=1626135690115652&alt=media)

Expand from `.gz` to `.pcap` file in order to use with this tool.

## Example (only cmpiled for macOS currently)

```bash
$ ./bin/main ./data/20210712_IEXTP1_DEEP1.0.pcap NET 9:31:7.398847
```

## To Build & Run

```bash
$ cargo run ./data/20210712_IEXTP1_DEEP1.0.pcap NET 9:31:7.398847
```

Optionally, you may add ENVS for debug logging such as `RUST_LOG=info RUST_BACKTRACE=1`.

## Problem Description

> IEX makes depth of book packet captures available for free from their website (https://iextrading.com/trading/market-data/). Make a program which will read this depth of book data, and use it to reconstruct full order books for any listed stock. Use the program to output  the top of book quote for any stock at any point in time. For the purpose of this exercise, you can just use the sample data available from the IEX website.

Let's parse this out:
1. IEX DEEP files is what we want
2. we don't care about trades only price level updates
3. "full order books for any listed stock"
4. "output top of book quote for any stock at any point in time"

Two deliverables here?

## Reference Documentation/Schemas

[IEX DEEP Specification.pdf](https://iextrading.com/docs/IEX%20DEEP%20Specification.pdf)

See page 35 of 44.

Price Level Update Messages in a Single Segment:

```
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-
| Transport Header | B 0-3
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 4-7
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 8-11
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 12-15
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 16-19
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 20-23
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 24-27
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 28-31
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 32-35
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Transport Header | B 36-39
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Message Length | Message Type | (Event Flags) | B 40-43
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Timestamp | B 44-47
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Timestamp | B 48-51
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Symbol | B 52-55
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Symbol | B 56-59
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Size | B 60-63
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Price | B 64-67
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Price | B 68-71
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

## First Thoughts:

1. ~~HashMap~~ (no wait, I need min/max/sorted keys) BTreeMap
2. One BTreeMap per symbol, per side

```
Symbol: String -> {
  Buys: BTreeMap<Price, Size>,
  Sells: BTreeMap<Price, Size>,
}
```

3. Testing purposes, let's first focus on a single symbol (my favorite stock $NET)
4. new packet -> parse messages -> check symbol -> add/remove to/from maps
5. while loop until lte timestamp

Advantages:
- quick dev time
- yay for data structure usage \*smile\*

Disadvantages:
- slow
- need to start from top of pcap file each time

Let's review our goals:
1. "full order books for any listed stock"
2. "output top of book quote for any stock at any point in time"

Well?
1. Yes, but slow
2. Yes, but slow

## What Do Other Providers Offer?

I believe Polygon does not offer book data for stocks but for crypto: 
- saves aggregates
- candles/bars
- snapshot book real-time

![polygon-crypto](../polygon-pcp-book/images/polygon-book-data.png)

TOPS is the max bid and lowest ask at a given timetamp and looks like it's atomic based on price level action. Meanwhile Polygon's lowest time window is 1 minute for crypto.

Only other choice is coming up with a crazy cluster mem/disk DB with a finite/consistent latency and a warning for traders. I guess this tool could be used to check how your trades stacked up relative to the book and your brokerage/clearing house partnership.

## Forward Thinking

So I now understand why TOPS is more or less built this way... My first leap was for a BTreeMap on each side which would mean 2 BTreeMaps per each symbol. This would for sure need redundancy and I'm curious what IEX uses as a architecture/data structure/etc.

If a data provider would want to offer this as a feature historically it would make sense to store individual messages/ticks inside a disk data structure and query individual symbols. This disk storage can then have an import side and a query side.

The query side is where this gets difficult. Because it implies always needing a window and never know the _exact_ top of book.

My idea of query side:
- in parallel:
  - 1 query for buy searching backwards by timestamp until window size for valid price/size
  - 1 query for sell side searching backwards by timestamp until window size for valid price/size
- return top of book after parallel queries finish
- optionally add depth/level for this crawler to return after top 3 valid

## Easy Things to Add

```
for each price status change:
  save (symbol, min, max) to disk/db/store by timestamp
```

This would be inline with TOPS but would require a time series DB/disk/mem solution.

## Checking My Work

I checked my work on an early intraday "TOP of book" quote_update from TOPS using [https://pypi.org/project/iex/](https://pypi.org/project/iex/).

```python
{
  'type': 'quote_update',
  'flags': 0,
  'timestamp': datetime.datetime(2021, 7, 12, 13, 31, 7, 398847, tzinfo=datetime.timezone.utc),
  'symbol': b'NET',
  'bid_size': 300,
  'bid_price': Decimal('111.2'),
  'ask_size': 100,
  'ask_price': Decimal('116.87'),
}
```

## Other References

- [iex · PyPI](https://pypi.org/project/iex/)
- [Order Book Definition](https://www.investopedia.com/terms/o/order-book.asp)
- [timpalpant/go-iex: A Go library for accessing the IEX Developer API.](https://github.com/timpalpant/go-iex)
- [market microstructure - What is an efficient data structure to model order book? - Quantitative Finance Stack Exchange](https://quant.stackexchange.com/questions/3783/what-is-an-efficient-data-structure-to-model-order-book)
- [How to Build a Fast Limit Order Book « WK's High Frequency Trading Blog](https://web.archive.org/web/20110219163448/http://howtohft.wordpress.com/2011/02/15/how-to-build-a-fast-limit-order-book/)
- [Show HN: A first project in Rust – in-memory order book | Hacker News](https://news.ycombinator.com/item?id=22389239)
