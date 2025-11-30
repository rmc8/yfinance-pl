"""yfinance-pl: Python wrapper for yfinance-rs using PyO3 and Polars."""

from collections import namedtuple

from yfinance_pl._yfinance_pl import Ticker as _RustTicker

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

    def option_chain(self, date=None):
        """Get option chain for a specific expiration date.

        Args:
            date: Expiration date in YYYY-MM-DD format (optional)

        Returns:
            OptionChain: Named tuple with calls and puts DataFrames
        """
        calls, puts = self._ticker.option_chain(date)
        return OptionChain(calls=calls, puts=puts)


__all__ = ["Ticker", "OptionChain"]
__version__ = "0.7.2.0"
