yfinance-pl ドキュメント
========================

Polars DataFrameで市場データを取得
----------------------------------

**yfinance-pl** は `yfinance-rs <https://github.com/gramistella/yfinance-rs>`_ の
PyO3とPolarsを使用したPythonラッパーです。

`yfinance <https://github.com/ranaroussi/yfinance>`_ 互換のAPIで株式市場データを取得できます。
pandasではなく **Polars DataFrame** を返します。

特徴
----

- **yfinance互換API** - yfinanceユーザーに馴染みのあるインターフェース
- **Polars DataFrame** - ``pd.DataFrame`` ではなく ``pl.DataFrame`` を返却
- **Rustバックエンド** - yfinance-rsによる高速なデータ取得
- **型安全なAPI** - ``period``、``interval``、戻り値の型のIDE自動補完

インストール
------------

.. code-block:: bash

    $ pip install yfinance-pl

クイックスタート
----------------

.. code-block:: python

   import yfinance_pl as yf

   ticker = yf.Ticker("AAPL")

   # 株価履歴を取得
   history = ticker.history(period="1mo")
   print(history)

   # 企業情報を取得
   info = ticker.info

   # 財務諸表を取得
   income = ticker.income_stmt
   balance = ticker.balance_sheet

   # アナリスト推奨を取得
   recommendations = ticker.recommendations

   # オプションデータを取得 (米国株のみ)
   if ticker.options:
       chain = ticker.option_chain(ticker.options[0])
       print(chain.calls)
       print(chain.puts)

yfinanceとの違い
----------------

.. list-table::
   :header-rows: 1

   * - 項目
     - yfinance
     - yfinance-pl
   * - DataFrameの型
     - ``pd.DataFrame``
     - ``pl.DataFrame``
   * - バックエンド
     - Python
     - Rust (yfinance-rs)
   * - Seriesの型
     - ``pd.Series``
     - ``pl.DataFrame``

.. toctree::
   :maxdepth: 2
   :caption: 目次

   reference/index

ライセンス
----------

MIT License
