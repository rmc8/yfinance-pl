"""Options Data - Call and put options (US stocks only)."""

import yfinance_pl as yf

ticker = yf.Ticker("AAPL")

# List of available expiration dates (list[str])
print("=== Available Expiration Dates ===")
expiration_dates = ticker.options
print(expiration_dates)

# Get option chain for first expiration date
if expiration_dates:
    date = expiration_dates[0]
    print(f"\n=== Option Chain for {date} ===")

    # Returns OptionChain named tuple with calls and puts
    chain = ticker.option_chain(date)

    print("\n--- Call Options ---")
    print(chain.calls)

    print("\n--- Put Options ---")
    print(chain.puts)
else:
    print("No options data available for this ticker.")

# Note: Japanese stocks (e.g., 7203.T) do not have options data on Yahoo Finance
