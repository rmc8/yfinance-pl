Referencia de API
=================

Esta página documenta la API de yfinance-pl.

Clase Ticker
------------

Clase principal para obtener datos de acciones.

.. code-block:: python

   import yfinance_pl as yf
   ticker = yf.Ticker("AAPL")

Historial de precios
~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Método/Propiedad
     - Tipo de retorno
     - Descripción
   * - ``history(period, interval, start, end)``
     - ``pl.DataFrame``
     - Datos históricos de precios (columna ``date`` como Datetime)

**Parámetros de history():**

- ``period``: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- ``interval``: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- ``start``: Fecha de inicio (YYYY-MM-DD)
- ``end``: Fecha de fin (YYYY-MM-DD)

Información de la empresa
~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Propiedad
     - Tipo de retorno
     - Descripción
   * - ``info``
     - ``dict``
     - Información detallada de la empresa
   * - ``fast_info``
     - ``dict``
     - Acceso rápido a métricas clave
   * - ``calendar``
     - ``dict``
     - Próximos eventos (fechas de ganancias, etc.)

Dividendos y acciones corporativas
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Propiedad
     - Tipo de retorno
     - Descripción
   * - ``dividends``
     - ``pl.DataFrame``
     - Historial de dividendos
   * - ``splits``
     - ``pl.DataFrame``
     - Historial de splits de acciones
   * - ``actions``
     - ``pl.DataFrame``
     - Datos combinados de dividendos y splits
   * - ``capital_gains``
     - ``pl.DataFrame``
     - Ganancias de capital (para fondos/ETF)

Estados financieros
~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Propiedad
     - Tipo de retorno
     - Descripción
   * - ``income_stmt``
     - ``pl.DataFrame``
     - Estado de resultados anual
   * - ``quarterly_income_stmt``
     - ``pl.DataFrame``
     - Estado de resultados trimestral
   * - ``balance_sheet``
     - ``pl.DataFrame``
     - Balance general anual
   * - ``quarterly_balance_sheet``
     - ``pl.DataFrame``
     - Balance general trimestral
   * - ``cashflow``
     - ``pl.DataFrame``
     - Estado de flujo de efectivo anual
   * - ``quarterly_cashflow``
     - ``pl.DataFrame``
     - Estado de flujo de efectivo trimestral
   * - ``earnings``
     - ``dict``
     - Resumen de ganancias

Información de accionistas
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Propiedad
     - Tipo de retorno
     - Descripción
   * - ``major_holders``
     - ``pl.DataFrame``
     - Resumen de principales accionistas
   * - ``institutional_holders``
     - ``pl.DataFrame``
     - Inversores institucionales
   * - ``mutualfund_holders``
     - ``pl.DataFrame``
     - Tenedores de fondos mutuos
   * - ``insider_transactions``
     - ``pl.DataFrame``
     - Transacciones de insiders
   * - ``insider_roster_holders``
     - ``pl.DataFrame``
     - Lista de insiders

Recomendaciones de analistas
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Propiedad
     - Tipo de retorno
     - Descripción
   * - ``recommendations``
     - ``pl.DataFrame``
     - Resumen de recomendaciones de analistas
   * - ``upgrades_downgrades``
     - ``pl.DataFrame``
     - Historial de cambios de calificación

Opciones (solo acciones de EE.UU.)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Propiedad/Método
     - Tipo de retorno
     - Descripción
   * - ``options``
     - ``list[str]``
     - Fechas de vencimiento disponibles
   * - ``option_chain(date)``
     - ``OptionChain``
     - Cadena de opciones para la fecha de vencimiento

El método ``option_chain()`` devuelve una tupla nombrada ``OptionChain``:

.. code-block:: python

   chain = ticker.option_chain("2024-01-19")
   chain.calls  # pl.DataFrame
   chain.puts   # pl.DataFrame

Otros
~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - Método
     - Tipo de retorno
     - Descripción
   * - ``get_isin()``
     - ``str``
     - Identificador ISIN
