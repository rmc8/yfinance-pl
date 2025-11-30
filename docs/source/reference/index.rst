API Reference
=============

This page documents the yfinance-pl API.

Ticker Class
------------

The main class for fetching stock data.

.. code-block:: python

   import yfinance_pl as yf
   ticker = yf.Ticker("AAPL")

Price History
~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Method/Property
     - Return Type
     - Description
   * - ``history(period, interval, start, end)``
     - ``pl.DataFrame``
     - Historical price data (``date`` column as Datetime)

**Parameters for history():**

- ``period``: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- ``interval``: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- ``start``: Start date (YYYY-MM-DD)
- ``end``: End date (YYYY-MM-DD)

Company Info
~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Property
     - Return Type
     - Description
   * - ``info``
     - ``dict``
     - Full company information
   * - ``fast_info``
     - ``dict``
     - Quick access to key metrics
   * - ``calendar``
     - ``dict``
     - Upcoming events (earnings dates, etc.)

Dividends & Corporate Actions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Property
     - Return Type
     - Description
   * - ``dividends``
     - ``pl.DataFrame``
     - Dividend history
   * - ``splits``
     - ``pl.DataFrame``
     - Stock split history
   * - ``actions``
     - ``pl.DataFrame``
     - Combined dividends and splits
   * - ``capital_gains``
     - ``pl.DataFrame``
     - Capital gains (for funds/ETFs)

Financial Statements
~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Property
     - Return Type
     - Description
   * - ``income_stmt``
     - ``pl.DataFrame``
     - Annual income statement
   * - ``quarterly_income_stmt``
     - ``pl.DataFrame``
     - Quarterly income statement
   * - ``balance_sheet``
     - ``pl.DataFrame``
     - Annual balance sheet
   * - ``quarterly_balance_sheet``
     - ``pl.DataFrame``
     - Quarterly balance sheet
   * - ``cashflow``
     - ``pl.DataFrame``
     - Annual cash flow statement
   * - ``quarterly_cashflow``
     - ``pl.DataFrame``
     - Quarterly cash flow statement
   * - ``earnings``
     - ``dict``
     - Earnings summary

Shareholders
~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Property
     - Return Type
     - Description
   * - ``major_holders``
     - ``pl.DataFrame``
     - Major holders summary
   * - ``institutional_holders``
     - ``pl.DataFrame``
     - Institutional holders
   * - ``mutualfund_holders``
     - ``pl.DataFrame``
     - Mutual fund holders
   * - ``insider_transactions``
     - ``pl.DataFrame``
     - Insider transactions
   * - ``insider_roster_holders``
     - ``pl.DataFrame``
     - Insider roster

Analyst Recommendations
~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Property
     - Return Type
     - Description
   * - ``recommendations``
     - ``pl.DataFrame``
     - Analyst recommendations summary
   * - ``upgrades_downgrades``
     - ``pl.DataFrame``
     - Rating changes history

Options (US stocks only)
~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Property/Method
     - Return Type
     - Description
   * - ``options``
     - ``list[str]``
     - Available expiration dates
   * - ``option_chain(date)``
     - ``OptionChain``
     - Option chain for expiration date

The ``option_chain()`` method returns an ``OptionChain`` named tuple:

.. code-block:: python

   chain = ticker.option_chain("2024-01-19")
   chain.calls  # pl.DataFrame
   chain.puts   # pl.DataFrame

Other
~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Method
     - Return Type
     - Description
   * - ``get_isin()``
     - ``str``
     - ISIN identifier

OptionChain
-----------

Named tuple returned by ``option_chain()``.

.. code-block:: python

   from yfinance_pl import OptionChain

   # OptionChain has two attributes:
   # - calls: pl.DataFrame
   # - puts: pl.DataFrame
