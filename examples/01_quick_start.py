"""Quick Start Example - Get stock price in 3 lines."""

import yfinance_pl as yf

# Create a Ticker object
ticker = yf.Ticker("AAPL")

# Get last month's price history
history = ticker.history(period="1mo")
print(history)
