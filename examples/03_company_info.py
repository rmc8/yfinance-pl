"""Company Information - Get company details and metadata."""

import yfinance_pl as yf

ticker = yf.Ticker("AAPL")

# Full company information (dict)
print("=== Company Info ===")
info = ticker.info
for key, value in list(info.items())[:10]:  # Show first 10 items
    print(f"{key}: {value}")

# Fast info - quick access to key metrics (dict)
print("\n=== Fast Info ===")
fast = ticker.fast_info
for key, value in fast.items():
    print(f"{key}: {value}")

# Calendar - upcoming events like earnings dates (dict)
print("\n=== Calendar ===")
calendar = ticker.calendar
print(calendar)
