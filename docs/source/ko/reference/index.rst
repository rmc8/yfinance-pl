API 레퍼런스
============

이 페이지는 yfinance-pl API 문서입니다.

Ticker 클래스
-------------

주식 데이터를 가져오기 위한 메인 클래스입니다.

.. code-block:: python

   import yfinance_pl as yf
   ticker = yf.Ticker("AAPL")

주가 이력
~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 메서드/속성
     - 반환 타입
     - 설명
   * - ``history(period, interval, start, end)``
     - ``pl.DataFrame``
     - 과거 주가 데이터

**history() 파라미터:**

- ``period``: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- ``interval``: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- ``start``: 시작일 (YYYY-MM-DD)
- ``end``: 종료일 (YYYY-MM-DD)

기업 정보
~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 속성
     - 반환 타입
     - 설명
   * - ``info``
     - ``dict``
     - 상세 기업 정보
   * - ``fast_info``
     - ``dict``
     - 주요 지표 빠른 액세스
   * - ``calendar``
     - ``dict``
     - 예정 이벤트 (실적 발표일 등)

배당 및 기업 활동
~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 속성
     - 반환 타입
     - 설명
   * - ``dividends``
     - ``pl.DataFrame``
     - 배당 이력
   * - ``splits``
     - ``pl.DataFrame``
     - 주식 분할 이력
   * - ``actions``
     - ``pl.DataFrame``
     - 배당과 분할 통합 데이터
   * - ``capital_gains``
     - ``pl.DataFrame``
     - 자본 이득 (펀드/ETF용)

재무제표
~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 속성
     - 반환 타입
     - 설명
   * - ``income_stmt``
     - ``pl.DataFrame``
     - 연간 손익계산서
   * - ``quarterly_income_stmt``
     - ``pl.DataFrame``
     - 분기 손익계산서
   * - ``balance_sheet``
     - ``pl.DataFrame``
     - 연간 대차대조표
   * - ``quarterly_balance_sheet``
     - ``pl.DataFrame``
     - 분기 대차대조표
   * - ``cashflow``
     - ``pl.DataFrame``
     - 연간 현금흐름표
   * - ``quarterly_cashflow``
     - ``pl.DataFrame``
     - 분기 현금흐름표
   * - ``earnings``
     - ``dict``
     - 수익 요약

주주 정보
~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 속성
     - 반환 타입
     - 설명
   * - ``major_holders``
     - ``pl.DataFrame``
     - 주요 주주 요약
   * - ``institutional_holders``
     - ``pl.DataFrame``
     - 기관 투자자
   * - ``mutualfund_holders``
     - ``pl.DataFrame``
     - 뮤추얼펀드 보유자
   * - ``insider_transactions``
     - ``pl.DataFrame``
     - 내부자 거래
   * - ``insider_roster_holders``
     - ``pl.DataFrame``
     - 내부자 명단

애널리스트 추천
~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 속성
     - 반환 타입
     - 설명
   * - ``recommendations``
     - ``pl.DataFrame``
     - 애널리스트 추천 요약
   * - ``upgrades_downgrades``
     - ``pl.DataFrame``
     - 등급 변경 이력

옵션 (미국 주식만 해당)
~~~~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 속성/메서드
     - 반환 타입
     - 설명
   * - ``options``
     - ``list[str]``
     - 가능한 만기일
   * - ``option_chain(date)``
     - ``OptionChain``
     - 만기일의 옵션 체인

``option_chain()`` 메서드는 ``OptionChain`` 네임드 튜플을 반환합니다:

.. code-block:: python

   chain = ticker.option_chain("2024-01-19")
   chain.calls  # pl.DataFrame
   chain.puts   # pl.DataFrame

기타
~~~~

.. list-table::
   :header-rows: 1
   :widths: 30 20 50

   * - 메서드
     - 반환 타입
     - 설명
   * - ``get_isin()``
     - ``str``
     - ISIN 식별자
