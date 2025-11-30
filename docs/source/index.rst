yfinance-pl documentation
=========================

Download Market Data with Polars DataFrames
-------------------------------------------

**yfinance-pl** is a Python wrapper for `yfinance-rs <https://github.com/gramistella/yfinance-rs>`_
using PyO3 and Polars.

Get stock market data with a `yfinance <https://github.com/ranaroussi/yfinance>`_-compatible API,
but with **Polars DataFrames** instead of pandas.

Features
--------

- **yfinance-compatible API** - Familiar interface for yfinance users
- **Polars DataFrames** - Returns ``pl.DataFrame`` instead of ``pd.DataFrame``
- **Rust backend** - Fast data fetching powered by yfinance-rs
- **Type-safe API** - IDE autocompletion for ``period``, ``interval``, and return types

Install
-------

.. code-block:: bash

    $ pip install yfinance-pl

Quick Start
-----------

.. code-block:: python

   import yfinance_pl as yf

   ticker = yf.Ticker("AAPL")

   # Get price history
   history = ticker.history(period="1mo")
   print(history)

   # Get company info
   info = ticker.info

   # Get financial statements
   income = ticker.income_stmt
   balance = ticker.balance_sheet

   # Get analyst recommendations
   recommendations = ticker.recommendations

   # Get options data (US stocks only)
   if ticker.options:
       chain = ticker.option_chain(ticker.options[0])
       print(chain.calls)
       print(chain.puts)

Differences from yfinance
-------------------------

.. list-table::
   :header-rows: 1

   * - Aspect
     - yfinance
     - yfinance-pl
   * - DataFrame type
     - ``pd.DataFrame``
     - ``pl.DataFrame``
   * - Backend
     - Python
     - Rust (yfinance-rs)
   * - Series type
     - ``pd.Series``
     - ``pl.DataFrame``

.. toctree::
   :maxdepth: 2
   :caption: Contents

   reference/index

License
-------

MIT License
