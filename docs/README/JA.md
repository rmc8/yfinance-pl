# yfinance-pl

[English](../../README.md) | [日本語](JA.md) | [中文](ZH.md) | [한국어](KO.md) | [Español](ES.md)

PyO3とPolarsを使用した[yfinance-rs](https://github.com/gramistella/yfinance-rs)のPythonラッパーです。

[yfinance](https://github.com/ranaroussi/yfinance)互換のAPIで株式市場データを取得できます。pandasではなく**Polars DataFrame**を返します。

## 特徴

- **yfinance互換API** - yfinanceユーザーに馴染みのあるインターフェース
- **Polars DataFrame** - `pd.DataFrame`ではなく`pl.DataFrame`を返却
- **Rustバックエンド** - yfinance-rsによる高速なデータ取得

## インストール

### 前提条件

protobufコンパイラをインストール:

```bash
# macOS
brew install protobuf

# Ubuntu/Debian
sudo apt install protobuf-compiler
```

### PyPIから

```bash
pip install yfinance-pl
```

### ソースからビルド

```bash
git clone https://github.com/rmc8/yfinance-pl.git
cd yfinance-pl
pip install maturin
maturin develop --release
```

## クイックスタート

```python
import yfinance_pl as yf

ticker = yf.Ticker("AAPL")
history = ticker.history(period="1mo")
print(history)
```

## APIリファレンス

### 株価履歴

| メソッド/プロパティ | 戻り値の型 | 説明 |
|----------------|-------------|-------------|
| `history(period, interval, start, end)` | `pl.DataFrame` | 過去の株価データ |

**`history()`のパラメータ:**
- `period`: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- `interval`: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- `start`: 開始日 (YYYY-MM-DD)
- `end`: 終了日 (YYYY-MM-DD)

### 企業情報

| プロパティ | 戻り値の型 | 説明 |
|----------|-------------|-------------|
| `info` | `dict` | 詳細な企業情報 |
| `fast_info` | `dict` | 主要指標への高速アクセス |
| `calendar` | `dict` | 今後のイベント (決算発表日など) |

### 配当・コーポレートアクション

| プロパティ | 戻り値の型 | 説明 |
|----------|-------------|-------------|
| `dividends` | `pl.DataFrame` | 配当履歴 |
| `splits` | `pl.DataFrame` | 株式分割履歴 |
| `actions` | `pl.DataFrame` | 配当と分割の統合データ |
| `capital_gains` | `pl.DataFrame` | キャピタルゲイン (ファンド/ETF向け) |

### 財務諸表

| プロパティ | 戻り値の型 | 説明 |
|----------|-------------|-------------|
| `income_stmt` | `pl.DataFrame` | 年次損益計算書 |
| `quarterly_income_stmt` | `pl.DataFrame` | 四半期損益計算書 |
| `balance_sheet` | `pl.DataFrame` | 年次貸借対照表 |
| `quarterly_balance_sheet` | `pl.DataFrame` | 四半期貸借対照表 |
| `cashflow` | `pl.DataFrame` | 年次キャッシュフロー計算書 |
| `quarterly_cashflow` | `pl.DataFrame` | 四半期キャッシュフロー計算書 |
| `earnings` | `dict` | 収益サマリー |

### 株主情報

| プロパティ | 戻り値の型 | 説明 |
|----------|-------------|-------------|
| `major_holders` | `pl.DataFrame` | 主要株主サマリー |
| `institutional_holders` | `pl.DataFrame` | 機関投資家 |
| `mutualfund_holders` | `pl.DataFrame` | 投資信託保有者 |
| `insider_transactions` | `pl.DataFrame` | インサイダー取引 |
| `insider_roster_holders` | `pl.DataFrame` | インサイダー一覧 |

### アナリスト推奨

| プロパティ | 戻り値の型 | 説明 |
|----------|-------------|-------------|
| `recommendations` | `pl.DataFrame` | アナリスト推奨サマリー |
| `upgrades_downgrades` | `pl.DataFrame` | 格付け変更履歴 |

### オプション (米国株のみ)

| プロパティ/メソッド | 戻り値の型 | 説明 |
|----------------|-------------|-------------|
| `options` | `list[str]` | 利用可能な満期日 |
| `option_chain(date)` | `OptionChain` | 満期日のオプションチェーン |

`option_chain()`メソッドは`OptionChain`名前付きタプルを返します:
```python
chain = ticker.option_chain("2024-01-19")
chain.calls  # pl.DataFrame
chain.puts   # pl.DataFrame
```

### その他

| メソッド | 戻り値の型 | 説明 |
|--------|-------------|-------------|
| `get_isin()` | `str` | ISIN識別子 |

## サンプル

詳細な使用例は[examples/](../../examples/)ディレクトリを参照:

- `01_quick_start.py` - 最小限のサンプル
- `02_history.py` - 株価履歴オプション
- `03_company_info.py` - 企業情報
- `04_dividends_splits.py` - コーポレートアクション
- `05_financials.py` - 財務諸表
- `06_holders.py` - 株主情報
- `07_analysis.py` - アナリスト推奨
- `08_options.py` - オプションデータ

実行方法:
```bash
uv run python examples/01_quick_start.py
```

## 開発

```bash
# 依存関係のインストール
uv sync

# Rustモジュールのビルド (変更後に実行)
uv run maturin develop

# リリースビルド
uv run maturin develop --release

# フォーマットとリント
uv run ruff format .
uv run ruff check .
```

## yfinanceとの違い

| 項目 | yfinance | yfinance-pl |
|--------|----------|-------------|
| DataFrameの型 | `pd.DataFrame` | `pl.DataFrame` |
| バックエンド | Python | Rust (yfinance-rs) |
| Seriesの型 | `pd.Series` | `pl.DataFrame` |

## ライセンス

MIT
