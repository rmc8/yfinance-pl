Documentación de yfinance-pl
============================

Obtén datos de mercado con Polars DataFrame
-------------------------------------------

**yfinance-pl** es un wrapper de Python para `yfinance-rs <https://github.com/gramistella/yfinance-rs>`_
usando PyO3 y Polars.

Obtén datos del mercado de valores con una API compatible con `yfinance <https://github.com/ranaroussi/yfinance>`_.
Devuelve **Polars DataFrame** en lugar de pandas.

Características
---------------

- **API compatible con yfinance** - Interfaz familiar para usuarios de yfinance
- **Polars DataFrame** - Devuelve ``pl.DataFrame`` en lugar de ``pd.DataFrame``
- **Backend Rust** - Obtención rápida de datos mediante yfinance-rs
- **API con tipos seguros** - Autocompletado del IDE para ``period``, ``interval`` y tipos de retorno

Instalación
-----------

.. code-block:: bash

    $ pip install yfinance-pl

Inicio rápido
-------------

.. code-block:: python

   import yfinance_pl as yf

   ticker = yf.Ticker("AAPL")

   # Obtener historial de precios
   history = ticker.history(period="1mo")
   print(history)

   # Obtener información de la empresa
   info = ticker.info

   # Obtener estados financieros
   income = ticker.income_stmt
   balance = ticker.balance_sheet

   # Obtener recomendaciones de analistas
   recommendations = ticker.recommendations

   # Obtener datos de opciones (solo acciones de EE.UU.)
   if ticker.options:
       chain = ticker.option_chain(ticker.options[0])
       print(chain.calls)
       print(chain.puts)

Diferencias con yfinance
------------------------

.. list-table::
   :header-rows: 1

   * - Aspecto
     - yfinance
     - yfinance-pl
   * - Tipo de DataFrame
     - ``pd.DataFrame``
     - ``pl.DataFrame``
   * - Backend
     - Python
     - Rust (yfinance-rs)
   * - Tipo de Series
     - ``pd.Series``
     - ``pl.DataFrame``

.. toctree::
   :maxdepth: 2
   :caption: Contenido

   reference/index

Licencia
--------

MIT License
