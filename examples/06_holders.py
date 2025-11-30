"""Shareholders Information - Institutional and insider holdings."""

import yfinance_pl as yf

ticker = yf.Ticker("AAPL")

# Major holders summary (pl.DataFrame)
print("=== Major Holders ===")
print(ticker.major_holders)

# Institutional holders (pl.DataFrame)
print("\n=== Institutional Holders ===")
print(ticker.institutional_holders)

# Mutual fund holders (pl.DataFrame)
print("\n=== Mutual Fund Holders ===")
print(ticker.mutualfund_holders)

# Insider transactions (pl.DataFrame)
print("\n=== Insider Transactions ===")
print(ticker.insider_transactions)

# Insider roster - list of insiders (pl.DataFrame)
print("\n=== Insider Roster ===")
print(ticker.insider_roster_holders)
