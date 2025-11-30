yfinance-pl 문서
================

Polars DataFrame으로 시장 데이터 가져오기
-----------------------------------------

**yfinance-pl**은 `yfinance-rs <https://github.com/gramistella/yfinance-rs>`_\ 의
PyO3와 Polars를 사용한 Python 래퍼입니다.

`yfinance <https://github.com/ranaroussi/yfinance>`_ 호환 API로 주식 시장 데이터를 가져올 수 있습니다.
pandas 대신 **Polars DataFrame**을 반환합니다.

특징
----

- **yfinance 호환 API** - yfinance 사용자에게 익숙한 인터페이스
- **Polars DataFrame** - ``pd.DataFrame`` 대신 ``pl.DataFrame`` 반환
- **Rust 백엔드** - yfinance-rs를 통한 빠른 데이터 가져오기

설치
----

.. code-block:: bash

    $ pip install yfinance-pl

빠른 시작
---------

.. code-block:: python

   import yfinance_pl as yf

   ticker = yf.Ticker("AAPL")

   # 주가 이력 가져오기
   history = ticker.history(period="1mo")
   print(history)

   # 기업 정보 가져오기
   info = ticker.info

   # 재무제표 가져오기
   income = ticker.income_stmt
   balance = ticker.balance_sheet

   # 애널리스트 추천 가져오기
   recommendations = ticker.recommendations

   # 옵션 데이터 가져오기 (미국 주식만 해당)
   if ticker.options:
       chain = ticker.option_chain(ticker.options[0])
       print(chain.calls)
       print(chain.puts)

yfinance와의 차이점
-------------------

.. list-table::
   :header-rows: 1

   * - 항목
     - yfinance
     - yfinance-pl
   * - DataFrame 타입
     - ``pd.DataFrame``
     - ``pl.DataFrame``
   * - 백엔드
     - Python
     - Rust (yfinance-rs)
   * - Series 타입
     - ``pd.Series``
     - ``pl.DataFrame``

.. toctree::
   :maxdepth: 2
   :caption: 목차

   reference/index

라이선스
--------

MIT License
