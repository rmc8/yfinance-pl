# yfinance-pl

[![PyPI version](https://img.shields.io/pypi/v/yfinance-pl.svg)](https://pypi.org/project/yfinance-pl/)
[![Python](https://img.shields.io/pypi/pyversions/yfinance-pl.svg)](https://pypi.org/project/yfinance-pl/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/rmc8/yfinance-pl/blob/main/LICENSE)
[![Documentation](https://img.shields.io/badge/docs-yfinance--pl.rmc--8.com-blue.svg)](https://yfinance-pl.rmc-8.com/)

[English](../../README.md) | [日本語](JA.md) | [中文](ZH.md) | [한국어](KO.md) | [Español](ES.md)

使用 PyO3 和 Polars 的 [yfinance-rs](https://github.com/gramistella/yfinance-rs) Python 封装库。

通过兼容 [yfinance](https://github.com/ranaroussi/yfinance) 的 API 获取股票市场数据，但返回 **Polars DataFrame** 而不是 pandas。

## 特性

- **兼容 yfinance API** - yfinance 用户熟悉的接口
- **Polars DataFrame** - 返回 `pl.DataFrame` 而不是 `pd.DataFrame`
- **Rust 后端** - 由 yfinance-rs 提供的快速数据获取

## 安装

### 前置条件

安装 protobuf 编译器:

```bash
# macOS
brew install protobuf

# Ubuntu/Debian
sudo apt install protobuf-compiler
```

### 从 PyPI 安装

```bash
pip install yfinance-pl
```

### 从源码构建

```bash
git clone https://github.com/rmc8/yfinance-pl.git
cd yfinance-pl
pip install maturin
maturin develop --release
```

## 快速开始

```python
import yfinance_pl as yf

ticker = yf.Ticker("AAPL")
history = ticker.history(period="1mo")
print(history)
```

## API 参考

### 价格历史

| 方法/属性 | 返回类型 | 描述 |
|----------------|-------------|-------------|
| `history(period, interval, start, end)` | `pl.DataFrame` | 历史价格数据 |

**`history()` 参数:**
- `period`: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- `interval`: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- `start`: 开始日期 (YYYY-MM-DD)
- `end`: 结束日期 (YYYY-MM-DD)

### 公司信息

| 属性 | 返回类型 | 描述 |
|----------|-------------|-------------|
| `info` | `dict` | 完整公司信息 |
| `fast_info` | `dict` | 快速访问关键指标 |
| `calendar` | `dict` | 即将发生的事件 (财报日期等) |

### 股息和公司行动

| 属性 | 返回类型 | 描述 |
|----------|-------------|-------------|
| `dividends` | `pl.DataFrame` | 股息历史 |
| `splits` | `pl.DataFrame` | 股票拆分历史 |
| `actions` | `pl.DataFrame` | 合并的股息和拆分数据 |
| `capital_gains` | `pl.DataFrame` | 资本收益 (适用于基金/ETF) |

### 财务报表

| 属性 | 返回类型 | 描述 |
|----------|-------------|-------------|
| `income_stmt` | `pl.DataFrame` | 年度利润表 |
| `quarterly_income_stmt` | `pl.DataFrame` | 季度利润表 |
| `balance_sheet` | `pl.DataFrame` | 年度资产负债表 |
| `quarterly_balance_sheet` | `pl.DataFrame` | 季度资产负债表 |
| `cashflow` | `pl.DataFrame` | 年度现金流量表 |
| `quarterly_cashflow` | `pl.DataFrame` | 季度现金流量表 |
| `earnings` | `dict` | 收益摘要 |

### 股东信息

| 属性 | 返回类型 | 描述 |
|----------|-------------|-------------|
| `major_holders` | `pl.DataFrame` | 主要股东摘要 |
| `institutional_holders` | `pl.DataFrame` | 机构投资者 |
| `mutualfund_holders` | `pl.DataFrame` | 共同基金持有者 |
| `insider_transactions` | `pl.DataFrame` | 内部人交易 |
| `insider_roster_holders` | `pl.DataFrame` | 内部人名册 |

### 分析师推荐

| 属性 | 返回类型 | 描述 |
|----------|-------------|-------------|
| `recommendations` | `pl.DataFrame` | 分析师推荐摘要 |
| `upgrades_downgrades` | `pl.DataFrame` | 评级变更历史 |

### 期权 (仅限美股)

| 属性/方法 | 返回类型 | 描述 |
|----------------|-------------|-------------|
| `options` | `list[str]` | 可用到期日 |
| `option_chain(date)` | `OptionChain` | 到期日的期权链 |

`option_chain()` 方法返回 `OptionChain` 命名元组:
```python
chain = ticker.option_chain("2024-01-19")
chain.calls  # pl.DataFrame
chain.puts   # pl.DataFrame
```

### 其他

| 方法 | 返回类型 | 描述 |
|--------|-------------|-------------|
| `get_isin()` | `str` | ISIN 标识符 |

## 示例

详细使用示例请参阅 [examples/](../../examples/) 目录:

- `01_quick_start.py` - 最小示例
- `02_history.py` - 价格历史选项
- `03_company_info.py` - 公司信息
- `04_dividends_splits.py` - 公司行动
- `05_financials.py` - 财务报表
- `06_holders.py` - 股东信息
- `07_analysis.py` - 分析师推荐
- `08_options.py` - 期权数据

运行示例:
```bash
uv run python examples/01_quick_start.py
```

## 开发

```bash
# 安装依赖
uv sync

# 构建 Rust 模块 (更改后运行)
uv run maturin develop

# 发布版本构建
uv run maturin develop --release

# 格式化和检查
uv run ruff format .
uv run ruff check .
```

## 与 yfinance 的区别

| 方面 | yfinance | yfinance-pl |
|--------|----------|-------------|
| DataFrame 类型 | `pd.DataFrame` | `pl.DataFrame` |
| 后端 | Python | Rust (yfinance-rs) |
| Series 类型 | `pd.Series` | `pl.DataFrame` |

## 许可证

MIT
