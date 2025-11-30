"""Type stubs for yfinance-pl."""

from typing import Annotated, Literal, NamedTuple, TypedDict

import polars as pl

# Type aliases
Period = Literal["1d", "5d", "1mo", "3mo", "6mo", "1y", "2y", "5y", "10y", "ytd", "max"]
Interval = Literal[
    "1m", "2m", "5m", "15m", "30m", "60m", "90m", "1h", "1d", "5d", "1wk", "1mo", "3mo"
]
DateStr = Annotated[str, "Date string in YYYY-MM-DD format"]


class TickerInfo(TypedDict, total=False):
    """Ticker information dictionary."""

    symbol: str
    shortName: str
    isin: str
    exchange: str
    marketState: str
    currency: str
    regularMarketPrice: str
    regularMarketOpen: str
    regularMarketDayHigh: str
    regularMarketDayLow: str
    regularMarketPreviousClose: str
    regularMarketVolume: int
    averageVolume: int
    marketCap: str
    sharesOutstanding: int
    trailingEps: str
    trailingPE: str
    dividendYield: str
    fiftyTwoWeekLow: str
    fiftyTwoWeekHigh: str


class FastInfo(TypedDict, total=False):
    """Fast ticker information dictionary."""

    symbol: str
    name: str
    exchange: str
    currency: str
    volume: int


class CalendarInfo(TypedDict, total=False):
    """Calendar events dictionary."""

    symbol: str
    earningsDates: list[str]
    exDividendDate: str
    dividendDate: str


class EarningsInfo(TypedDict):
    """Earnings data dictionary."""

    symbol: str
    yearly_count: int
    quarterly_count: int
    quarterly_eps_count: int


class OptionChain(NamedTuple):
    """Option chain data."""

    calls: pl.DataFrame
    puts: pl.DataFrame


class Ticker:
    """Ticker class for fetching stock data (yfinance-compatible API)."""

    def __init__(self, symbol: str) -> None: ...

    # Price History
    def history(
        self,
        period: Period | None = None,
        interval: Interval | None = None,
        start: DateStr | None = None,
        end: DateStr | None = None,
        prepost: bool = False,
        auto_adjust: bool = True,
        actions: bool = True,
    ) -> pl.DataFrame:
        """Get historical OHLCV data as a Polars DataFrame."""
        ...

    # Company Info
    @property
    def info(self) -> TickerInfo:
        """Get ticker info as a dictionary."""
        ...

    @property
    def fast_info(self) -> FastInfo:
        """Get fast info as a dictionary."""
        ...

    @property
    def calendar(self) -> CalendarInfo:
        """Get calendar events as a dictionary."""
        ...

    # Dividends & Corporate Actions
    @property
    def dividends(self) -> pl.DataFrame:
        """Get dividend history as a Polars DataFrame."""
        ...

    @property
    def splits(self) -> pl.DataFrame:
        """Get stock splits history as a Polars DataFrame."""
        ...

    @property
    def actions(self) -> pl.DataFrame:
        """Get corporate actions (dividends + splits) as a Polars DataFrame."""
        ...

    @property
    def capital_gains(self) -> pl.DataFrame:
        """Get capital gains as a Polars DataFrame."""
        ...

    # Financial Statements
    @property
    def income_stmt(self) -> pl.DataFrame:
        """Get annual income statement as a Polars DataFrame."""
        ...

    @property
    def quarterly_income_stmt(self) -> pl.DataFrame:
        """Get quarterly income statement as a Polars DataFrame."""
        ...

    @property
    def balance_sheet(self) -> pl.DataFrame:
        """Get annual balance sheet as a Polars DataFrame."""
        ...

    @property
    def quarterly_balance_sheet(self) -> pl.DataFrame:
        """Get quarterly balance sheet as a Polars DataFrame."""
        ...

    @property
    def cashflow(self) -> pl.DataFrame:
        """Get annual cash flow statement as a Polars DataFrame."""
        ...

    @property
    def quarterly_cashflow(self) -> pl.DataFrame:
        """Get quarterly cash flow statement as a Polars DataFrame."""
        ...

    @property
    def earnings(self) -> EarningsInfo:
        """Get earnings data as a dictionary."""
        ...

    # Shareholders
    @property
    def major_holders(self) -> pl.DataFrame:
        """Get major holders breakdown as a Polars DataFrame."""
        ...

    @property
    def institutional_holders(self) -> pl.DataFrame:
        """Get institutional holders as a Polars DataFrame."""
        ...

    @property
    def mutualfund_holders(self) -> pl.DataFrame:
        """Get mutual fund holders as a Polars DataFrame."""
        ...

    @property
    def insider_transactions(self) -> pl.DataFrame:
        """Get insider transactions as a Polars DataFrame."""
        ...

    @property
    def insider_roster_holders(self) -> pl.DataFrame:
        """Get insider roster holders as a Polars DataFrame."""
        ...

    # Analyst Recommendations
    @property
    def recommendations(self) -> pl.DataFrame:
        """Get analyst recommendations as a Polars DataFrame."""
        ...

    @property
    def upgrades_downgrades(self) -> pl.DataFrame:
        """Get analyst upgrades/downgrades history as a Polars DataFrame."""
        ...

    # Options
    @property
    def options(self) -> list[str]:
        """Get available option expiration dates as a list of strings (YYYY-MM-DD)."""
        ...

    def option_chain(self, date: DateStr | None = None) -> OptionChain:
        """Get option chain for a specific expiration date."""
        ...

    # Other
    def get_isin(self) -> str | None:
        """Get the ISIN for this ticker."""
        ...


__all__ = [
    "Ticker",
    "OptionChain",
    "Period",
    "Interval",
    "DateStr",
    "TickerInfo",
    "FastInfo",
    "CalendarInfo",
    "EarningsInfo",
]
__version__: str
