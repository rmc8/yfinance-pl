# yfinance-pl

[English](../../README.md) | [日本語](JA.md) | [中文](ZH.md) | [한국어](KO.md) | [Español](ES.md)

PyO3와 Polars를 사용한 [yfinance-rs](https://github.com/gramistella/yfinance-rs) Python 래퍼입니다.

[yfinance](https://github.com/ranaroussi/yfinance) 호환 API로 주식 시장 데이터를 가져오되, pandas 대신 **Polars DataFrame**을 반환합니다.

## 특징

- **yfinance 호환 API** - yfinance 사용자에게 익숙한 인터페이스
- **Polars DataFrame** - `pd.DataFrame` 대신 `pl.DataFrame` 반환
- **Rust 백엔드** - yfinance-rs 기반의 빠른 데이터 가져오기

## 설치

### 사전 요구 사항

protobuf 컴파일러 설치:

```bash
# macOS
brew install protobuf

# Ubuntu/Debian
sudo apt install protobuf-compiler
```

### PyPI에서 설치

```bash
pip install yfinance-pl
```

### 소스에서 빌드

```bash
git clone https://github.com/rmc8/yfinance-pl.git
cd yfinance-pl
pip install maturin
maturin develop --release
```

## 빠른 시작

```python
import yfinance_pl as yf

ticker = yf.Ticker("AAPL")
history = ticker.history(period="1mo")
print(history)
```

## API 레퍼런스

### 가격 히스토리

| 메서드/속성 | 반환 타입 | 설명 |
|----------------|-------------|-------------|
| `history(period, interval, start, end)` | `pl.DataFrame` | 과거 가격 데이터 |

**`history()` 매개변수:**
- `period`: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- `interval`: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- `start`: 시작 날짜 (YYYY-MM-DD)
- `end`: 종료 날짜 (YYYY-MM-DD)

### 회사 정보

| 속성 | 반환 타입 | 설명 |
|----------|-------------|-------------|
| `info` | `dict` | 전체 회사 정보 |
| `fast_info` | `dict` | 주요 지표 빠른 액세스 |
| `calendar` | `dict` | 예정된 이벤트 (실적 발표일 등) |

### 배당 및 기업 활동

| 속성 | 반환 타입 | 설명 |
|----------|-------------|-------------|
| `dividends` | `pl.DataFrame` | 배당 이력 |
| `splits` | `pl.DataFrame` | 주식 분할 이력 |
| `actions` | `pl.DataFrame` | 배당 및 분할 통합 데이터 |
| `capital_gains` | `pl.DataFrame` | 자본 이득 (펀드/ETF용) |

### 재무제표

| 속성 | 반환 타입 | 설명 |
|----------|-------------|-------------|
| `income_stmt` | `pl.DataFrame` | 연간 손익계산서 |
| `quarterly_income_stmt` | `pl.DataFrame` | 분기별 손익계산서 |
| `balance_sheet` | `pl.DataFrame` | 연간 대차대조표 |
| `quarterly_balance_sheet` | `pl.DataFrame` | 분기별 대차대조표 |
| `cashflow` | `pl.DataFrame` | 연간 현금흐름표 |
| `quarterly_cashflow` | `pl.DataFrame` | 분기별 현금흐름표 |
| `earnings` | `dict` | 수익 요약 |

### 주주 정보

| 속성 | 반환 타입 | 설명 |
|----------|-------------|-------------|
| `major_holders` | `pl.DataFrame` | 주요 주주 요약 |
| `institutional_holders` | `pl.DataFrame` | 기관 투자자 |
| `mutualfund_holders` | `pl.DataFrame` | 뮤추얼 펀드 보유자 |
| `insider_transactions` | `pl.DataFrame` | 내부자 거래 |
| `insider_roster_holders` | `pl.DataFrame` | 내부자 명단 |

### 애널리스트 추천

| 속성 | 반환 타입 | 설명 |
|----------|-------------|-------------|
| `recommendations` | `pl.DataFrame` | 애널리스트 추천 요약 |
| `upgrades_downgrades` | `pl.DataFrame` | 등급 변경 이력 |

### 옵션 (미국 주식만 해당)

| 속성/메서드 | 반환 타입 | 설명 |
|----------------|-------------|-------------|
| `options` | `list[str]` | 사용 가능한 만기일 |
| `option_chain(date)` | `OptionChain` | 만기일 옵션 체인 |

`option_chain()` 메서드는 `OptionChain` 네임드 튜플을 반환합니다:
```python
chain = ticker.option_chain("2024-01-19")
chain.calls  # pl.DataFrame
chain.puts   # pl.DataFrame
```

### 기타

| 메서드 | 반환 타입 | 설명 |
|--------|-------------|-------------|
| `get_isin()` | `str` | ISIN 식별자 |

## 예제

자세한 사용 예제는 [examples/](../../examples/) 디렉토리를 참조하세요:

- `01_quick_start.py` - 최소 예제
- `02_history.py` - 가격 히스토리 옵션
- `03_company_info.py` - 회사 정보
- `04_dividends_splits.py` - 기업 활동
- `05_financials.py` - 재무제표
- `06_holders.py` - 주주 정보
- `07_analysis.py` - 애널리스트 추천
- `08_options.py` - 옵션 데이터

예제 실행:
```bash
uv run python examples/01_quick_start.py
```

## 개발

```bash
# 의존성 설치
uv sync

# Rust 모듈 빌드 (변경 후 실행)
uv run maturin develop

# 릴리스 빌드
uv run maturin develop --release

# 포맷 및 린트
uv run ruff format .
uv run ruff check .
```

## yfinance와의 차이점

| 항목 | yfinance | yfinance-pl |
|--------|----------|-------------|
| DataFrame 타입 | `pd.DataFrame` | `pl.DataFrame` |
| 백엔드 | Python | Rust (yfinance-rs) |
| Series 타입 | `pd.Series` | `pl.DataFrame` |

## 라이선스

MIT
