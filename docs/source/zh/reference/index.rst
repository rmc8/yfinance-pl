API 参考
========

本页面记录 yfinance-pl API。

Ticker 类
---------

获取股票数据的主要类。

.. code-block:: python

   import yfinance_pl as yf
   ticker = yf.Ticker("AAPL")

价格历史
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 方法/属性
     - 返回类型
     - 描述
   * - ``history(period, interval, start, end)``
     - ``pl.DataFrame``
     - 历史价格数据

**history() 参数:**

- ``period``: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- ``interval``: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- ``start``: 开始日期 (YYYY-MM-DD)
- ``end``: 结束日期 (YYYY-MM-DD)

公司信息
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 属性
     - 返回类型
     - 描述
   * - ``info``
     - ``dict``
     - 完整公司信息
   * - ``fast_info``
     - ``dict``
     - 快速访问关键指标
   * - ``calendar``
     - ``dict``
     - 即将发生的事件 (财报日期等)

股息和公司行动
~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 属性
     - 返回类型
     - 描述
   * - ``dividends``
     - ``pl.DataFrame``
     - 股息历史
   * - ``splits``
     - ``pl.DataFrame``
     - 股票拆分历史
   * - ``actions``
     - ``pl.DataFrame``
     - 合并的股息和拆分数据
   * - ``capital_gains``
     - ``pl.DataFrame``
     - 资本收益 (适用于基金/ETF)

财务报表
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 属性
     - 返回类型
     - 描述
   * - ``income_stmt``
     - ``pl.DataFrame``
     - 年度利润表
   * - ``quarterly_income_stmt``
     - ``pl.DataFrame``
     - 季度利润表
   * - ``balance_sheet``
     - ``pl.DataFrame``
     - 年度资产负债表
   * - ``quarterly_balance_sheet``
     - ``pl.DataFrame``
     - 季度资产负债表
   * - ``cashflow``
     - ``pl.DataFrame``
     - 年度现金流量表
   * - ``quarterly_cashflow``
     - ``pl.DataFrame``
     - 季度现金流量表
   * - ``earnings``
     - ``dict``
     - 收益摘要

股东信息
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 属性
     - 返回类型
     - 描述
   * - ``major_holders``
     - ``pl.DataFrame``
     - 主要股东摘要
   * - ``institutional_holders``
     - ``pl.DataFrame``
     - 机构投资者
   * - ``mutualfund_holders``
     - ``pl.DataFrame``
     - 共同基金持有者
   * - ``insider_transactions``
     - ``pl.DataFrame``
     - 内部人交易
   * - ``insider_roster_holders``
     - ``pl.DataFrame``
     - 内部人名册

分析师推荐
~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 属性
     - 返回类型
     - 描述
   * - ``recommendations``
     - ``pl.DataFrame``
     - 分析师推荐摘要
   * - ``upgrades_downgrades``
     - ``pl.DataFrame``
     - 评级变更历史

期权 (仅限美股)
~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 属性/方法
     - 返回类型
     - 描述
   * - ``options``
     - ``list[str]``
     - 可用到期日
   * - ``option_chain(date)``
     - ``OptionChain``
     - 到期日的期权链

``option_chain()`` 方法返回 ``OptionChain`` 命名元组:

.. code-block:: python

   chain = ticker.option_chain("2024-01-19")
   chain.calls  # pl.DataFrame
   chain.puts   # pl.DataFrame

其他
~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 方法
     - 返回类型
     - 描述
   * - ``get_isin()``
     - ``str``
     - ISIN 标识符
