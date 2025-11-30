"""Analyst Recommendations - Buy/sell ratings and upgrades/downgrades."""

import yfinance_pl as yf

ticker = yf.Ticker("AAPL")

# Analyst recommendations summary (pl.DataFrame)
# Shows aggregated buy/hold/sell counts by period
print("=== Analyst Recommendations ===")
print(ticker.recommendations)

# Upgrades and downgrades history (pl.DataFrame)
# Individual analyst rating changes
print("\n=== Upgrades/Downgrades ===")
print(ticker.upgrades_downgrades)
