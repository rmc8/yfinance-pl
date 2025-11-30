APIリファレンス
===============

このページはyfinance-plのAPIドキュメントです。

Tickerクラス
------------

株式データを取得するためのメインクラスです。

.. code-block:: python

   import yfinance_pl as yf
   ticker = yf.Ticker("AAPL")

株価履歴
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - メソッド/プロパティ
     - 戻り値の型
     - 説明
   * - ``history(period, interval, start, end)``
     - ``pl.DataFrame``
     - 過去の株価データ（``date`` 列はDatetime型）

**history()のパラメータ:**

- ``period``: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- ``interval``: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- ``start``: 開始日 (YYYY-MM-DD)
- ``end``: 終了日 (YYYY-MM-DD)

企業情報
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - プロパティ
     - 戻り値の型
     - 説明
   * - ``info``
     - ``dict``
     - 詳細な企業情報
   * - ``fast_info``
     - ``dict``
     - 主要指標への高速アクセス
   * - ``calendar``
     - ``dict``
     - 今後のイベント (決算発表日など)

配当・コーポレートアクション
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - プロパティ
     - 戻り値の型
     - 説明
   * - ``dividends``
     - ``pl.DataFrame``
     - 配当履歴
   * - ``splits``
     - ``pl.DataFrame``
     - 株式分割履歴
   * - ``actions``
     - ``pl.DataFrame``
     - 配当と分割の統合データ
   * - ``capital_gains``
     - ``pl.DataFrame``
     - キャピタルゲイン (ファンド/ETF向け)

財務諸表
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - プロパティ
     - 戻り値の型
     - 説明
   * - ``income_stmt``
     - ``pl.DataFrame``
     - 年次損益計算書
   * - ``quarterly_income_stmt``
     - ``pl.DataFrame``
     - 四半期損益計算書
   * - ``balance_sheet``
     - ``pl.DataFrame``
     - 年次貸借対照表
   * - ``quarterly_balance_sheet``
     - ``pl.DataFrame``
     - 四半期貸借対照表
   * - ``cashflow``
     - ``pl.DataFrame``
     - 年次キャッシュフロー計算書
   * - ``quarterly_cashflow``
     - ``pl.DataFrame``
     - 四半期キャッシュフロー計算書
   * - ``earnings``
     - ``dict``
     - 収益サマリー

株主情報
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - プロパティ
     - 戻り値の型
     - 説明
   * - ``major_holders``
     - ``pl.DataFrame``
     - 主要株主サマリー
   * - ``institutional_holders``
     - ``pl.DataFrame``
     - 機関投資家
   * - ``mutualfund_holders``
     - ``pl.DataFrame``
     - 投資信託保有者
   * - ``insider_transactions``
     - ``pl.DataFrame``
     - インサイダー取引
   * - ``insider_roster_holders``
     - ``pl.DataFrame``
     - インサイダー一覧

アナリスト推奨
~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - プロパティ
     - 戻り値の型
     - 説明
   * - ``recommendations``
     - ``pl.DataFrame``
     - アナリスト推奨サマリー
   * - ``upgrades_downgrades``
     - ``pl.DataFrame``
     - 格付け変更履歴

オプション (米国株のみ)
~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - プロパティ/メソッド
     - 戻り値の型
     - 説明
   * - ``options``
     - ``list[str]``
     - 利用可能な満期日
   * - ``option_chain(date)``
     - ``OptionChain``
     - 満期日のオプションチェーン

``option_chain()`` メソッドは ``OptionChain`` 名前付きタプルを返します:

.. code-block:: python

   chain = ticker.option_chain("2024-01-19")
   chain.calls  # pl.DataFrame
   chain.puts   # pl.DataFrame

その他
~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - メソッド
     - 戻り値の型
     - 説明
   * - ``get_isin()``
     - ``str``
     - ISIN識別子
