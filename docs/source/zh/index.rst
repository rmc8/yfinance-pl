yfinance-pl 文档
================

使用 Polars DataFrame 获取市场数据
----------------------------------

**yfinance-pl** 是 `yfinance-rs <https://github.com/gramistella/yfinance-rs>`_ 的
Python 封装库，使用 PyO3 和 Polars。

通过兼容 `yfinance <https://github.com/ranaroussi/yfinance>`_ 的 API 获取股票市场数据，
但返回 **Polars DataFrame** 而不是 pandas。

特性
----

- **兼容 yfinance API** - yfinance 用户熟悉的接口
- **Polars DataFrame** - 返回 ``pl.DataFrame`` 而不是 ``pd.DataFrame``
- **Rust 后端** - 由 yfinance-rs 提供的快速数据获取
- **类型安全 API** - 为 ``period``、``interval`` 和返回类型提供 IDE 自动补全

安装
----

.. code-block:: bash

    $ pip install yfinance-pl

快速开始
--------

.. code-block:: python

   import yfinance_pl as yf

   ticker = yf.Ticker("AAPL")

   # 获取价格历史
   history = ticker.history(period="1mo")
   print(history)

   # 获取公司信息
   info = ticker.info

   # 获取财务报表
   income = ticker.income_stmt
   balance = ticker.balance_sheet

   # 获取分析师推荐
   recommendations = ticker.recommendations

   # 获取期权数据 (仅限美股)
   if ticker.options:
       chain = ticker.option_chain(ticker.options[0])
       print(chain.calls)
       print(chain.puts)

与 yfinance 的区别
------------------

.. list-table::
   :header-rows: 1

   * - 方面
     - yfinance
     - yfinance-pl
   * - DataFrame 类型
     - ``pd.DataFrame``
     - ``pl.DataFrame``
   * - 后端
     - Python
     - Rust (yfinance-rs)
   * - Series 类型
     - ``pd.Series``
     - ``pl.DataFrame``

.. toctree::
   :maxdepth: 2
   :caption: 目录

   reference/index

许可证
------

MIT License
