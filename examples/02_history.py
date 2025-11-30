"""Stock Price History - Various ways to fetch historical data."""

import yfinance_pl as yf

ticker = yf.Ticker("AAPL")

# Using period parameter
# Valid periods: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
print("=== Last 5 days ===")
print(ticker.history(period="5d"))

print("\n=== Year to date ===")
print(ticker.history(period="ytd"))

# Using start and end dates
print("\n=== Custom date range ===")
print(ticker.history(start="2024-01-01", end="2024-06-30"))

# Different intervals
# Valid intervals: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
print("\n=== Weekly data ===")
print(ticker.history(period="3mo", interval="1wk"))

# Japanese stock example
print("\n=== Toyota (7203.T) ===")
toyota = yf.Ticker("7203.T")
print(toyota.history(period="1mo"))
