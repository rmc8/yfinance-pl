"""yfinance-pl: Python wrapper for yfinance-rs using PyO3 and Polars."""

from collections import namedtuple
from typing import Annotated, Literal, TypedDict

import polars as pl

from yfinance_pl._yfinance_pl import Ticker as _RustTicker

# Type aliases for IDE autocompletion
Period = Literal["1d", "5d", "1mo", "3mo", "6mo", "1y", "2y", "5y", "10y", "ytd", "max"]
Interval = Literal[
    "1m", "2m", "5m", "15m", "30m", "60m", "90m", "1h", "1d", "5d", "1wk", "1mo", "3mo"
]

# Date format type alias
DateStr = Annotated[str, "Date string in YYYY-MM-DD format"]


# TypedDict definitions for structured return types
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

# Named tuple for option chain (yfinance-compatible)
OptionChain = namedtuple("OptionChain", ["calls", "puts"])


def _wrap_option_chain(original_method):
    """Wrapper to convert option_chain tuple to OptionChain named tuple."""

    def wrapper(self, date=None):
        calls, puts = original_method(date)
        return OptionChain(calls=calls, puts=puts)

    return wrapper


# Wrap the Rust Ticker class to provide OptionChain named tuple
class Ticker:
    """Ticker class for fetching stock data (yfinance-compatible API)."""

    def __init__(self, symbol):
        self._ticker = _RustTicker(symbol)

    def __getattr__(self, name):
        return getattr(self._ticker, name)

    def __repr__(self):
        return repr(self._ticker)

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
        """Get historical OHLCV data as a Polars DataFrame.

        Args:
            period: Data period (1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max)
            interval: Data interval (1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo)
            start: Start date (YYYY-MM-DD) - not yet implemented
            end: End date (YYYY-MM-DD) - not yet implemented
            prepost: Include pre and post market data
            auto_adjust: Adjust prices for splits and dividends
            actions: Include dividends and stock splits

        Returns:
            pl.DataFrame: Historical OHLCV data with date column
        """
        return self._ticker.history(
            period=period,
            interval=interval,
            start=start,
            end=end,
            prepost=prepost,
            auto_adjust=auto_adjust,
            actions=actions,
        )

    def option_chain(self, date: DateStr | None = None) -> OptionChain:
        """Get option chain for a specific expiration date.

        Args:
            date: Expiration date in YYYY-MM-DD format (optional)

        Returns:
            OptionChain: Named tuple with calls and puts DataFrames
        """
        calls, puts = self._ticker.option_chain(date)
        return OptionChain(calls=calls, puts=puts)


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
__version__ = "0.7.2.1"
