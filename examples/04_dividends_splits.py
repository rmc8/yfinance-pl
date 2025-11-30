"""Dividends and Splits - Corporate action history."""

import yfinance_pl as yf

ticker = yf.Ticker("AAPL")

# Dividend history (pl.DataFrame)
print("=== Dividends ===")
print(ticker.dividends)

# Stock split history (pl.DataFrame)
print("\n=== Stock Splits ===")
print(ticker.splits)

# Combined actions (pl.DataFrame)
print("\n=== All Corporate Actions ===")
print(ticker.actions)

# Capital gains (for funds/ETFs)
print("\n=== Capital Gains ===")
print(ticker.capital_gains)
