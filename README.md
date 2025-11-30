# yfinance-pl

[![PyPI version](https://img.shields.io/pypi/v/yfinance-pl.svg)](https://pypi.org/project/yfinance-pl/)
[![Python](https://img.shields.io/pypi/pyversions/yfinance-pl.svg)](https://pypi.org/project/yfinance-pl/)
[![License](https://img.shields.io/pypi/l/yfinance-pl.svg)](https://github.com/rmc8/yfinance-pl/blob/main/LICENSE)
[![Downloads](https://img.shields.io/pypi/dm/yfinance-pl.svg)](https://pypi.org/project/yfinance-pl/)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue.svg)](https://rmc8.github.io/yfinance-pl/)

[English](README.md) | [日本語](docs/README/JA.md) | [中文](docs/README/ZH.md) | [한국어](docs/README/KO.md) | [Español](docs/README/ES.md)

A Python wrapper for [yfinance-rs](https://github.com/gramistella/yfinance-rs) using PyO3 and Polars.

Get stock market data with a [yfinance-compatible](https://github.com/ranaroussi/yfinance) API, but with **Polars DataFrames** instead of pandas.

## Features

- **yfinance-compatible API** - Familiar interface for yfinance users
- **Polars DataFrames** - Returns `pl.DataFrame` instead of `pd.DataFrame`
- **Rust backend** - Fast data fetching powered by yfinance-rs

## Installation

### Prerequisites

Install protobuf compiler:

```bash
# macOS
brew install protobuf

# Ubuntu/Debian
sudo apt install protobuf-compiler
```

### From PyPI

```bash
pip install yfinance-pl
```

### From Source

```bash
git clone https://github.com/rmc8/yfinance-pl.git
cd yfinance-pl
pip install maturin
maturin develop --release
```

## Quick Start

```python
import yfinance_pl as yf

ticker = yf.Ticker("AAPL")
history = ticker.history(period="1mo")
print(history)
```

## API Reference

### Price History

| Method/Property | Return Type | Description |
|----------------|-------------|-------------|
| `history(period, interval, start, end)` | `pl.DataFrame` | Historical price data |

**Parameters for `history()`:**
- `period`: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- `interval`: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- `start`: Start date (YYYY-MM-DD)
- `end`: End date (YYYY-MM-DD)

### Company Info

| Property | Return Type | Description |
|----------|-------------|-------------|
| `info` | `dict` | Full company information |
| `fast_info` | `dict` | Quick access to key metrics |
| `calendar` | `dict` | Upcoming events (earnings dates, etc.) |

### Dividends & Corporate Actions

| Property | Return Type | Description |
|----------|-------------|-------------|
| `dividends` | `pl.DataFrame` | Dividend history |
| `splits` | `pl.DataFrame` | Stock split history |
| `actions` | `pl.DataFrame` | Combined dividends and splits |
| `capital_gains` | `pl.DataFrame` | Capital gains (for funds/ETFs) |

### Financial Statements

| Property | Return Type | Description |
|----------|-------------|-------------|
| `income_stmt` | `pl.DataFrame` | Annual income statement |
| `quarterly_income_stmt` | `pl.DataFrame` | Quarterly income statement |
| `balance_sheet` | `pl.DataFrame` | Annual balance sheet |
| `quarterly_balance_sheet` | `pl.DataFrame` | Quarterly balance sheet |
| `cashflow` | `pl.DataFrame` | Annual cash flow statement |
| `quarterly_cashflow` | `pl.DataFrame` | Quarterly cash flow statement |
| `earnings` | `dict` | Earnings summary |

### Shareholders

| Property | Return Type | Description |
|----------|-------------|-------------|
| `major_holders` | `pl.DataFrame` | Major holders summary |
| `institutional_holders` | `pl.DataFrame` | Institutional holders |
| `mutualfund_holders` | `pl.DataFrame` | Mutual fund holders |
| `insider_transactions` | `pl.DataFrame` | Insider transactions |
| `insider_roster_holders` | `pl.DataFrame` | Insider roster |

### Analyst Recommendations

| Property | Return Type | Description |
|----------|-------------|-------------|
| `recommendations` | `pl.DataFrame` | Analyst recommendations summary |
| `upgrades_downgrades` | `pl.DataFrame` | Rating changes history |

### Options (US stocks only)

| Property/Method | Return Type | Description |
|----------------|-------------|-------------|
| `options` | `list[str]` | Available expiration dates |
| `option_chain(date)` | `OptionChain` | Option chain for expiration date |

The `option_chain()` method returns an `OptionChain` named tuple:
```python
chain = ticker.option_chain("2024-01-19")
chain.calls  # pl.DataFrame
chain.puts   # pl.DataFrame
```

### Other

| Method | Return Type | Description |
|--------|-------------|-------------|
| `get_isin()` | `str` | ISIN identifier |

## Examples

See the [examples/](examples/) directory for detailed usage examples:

- `01_quick_start.py` - Minimal example
- `02_history.py` - Price history options
- `03_company_info.py` - Company information
- `04_dividends_splits.py` - Corporate actions
- `05_financials.py` - Financial statements
- `06_holders.py` - Shareholder information
- `07_analysis.py` - Analyst recommendations
- `08_options.py` - Options data

Run examples:
```bash
uv run python examples/01_quick_start.py
```

## Development

```bash
# Install dependencies
uv sync

# Build Rust module (run after changes)
uv run maturin develop

# Release build
uv run maturin develop --release

# Format and lint
uv run ruff format .
uv run ruff check .
```

## Differences from yfinance

| Aspect | yfinance | yfinance-pl |
|--------|----------|-------------|
| DataFrame type | `pd.DataFrame` | `pl.DataFrame` |
| Backend | Python | Rust (yfinance-rs) |
| Series type | `pd.Series` | `pl.DataFrame` |

## License

MIT
