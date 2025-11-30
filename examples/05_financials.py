"""Financial Statements - Income, balance sheet, and cash flow."""

import yfinance_pl as yf

ticker = yf.Ticker("AAPL")

# Income Statement (annual)
print("=== Income Statement (Annual) ===")
print(ticker.income_stmt)

# Income Statement (quarterly)
print("\n=== Income Statement (Quarterly) ===")
print(ticker.quarterly_income_stmt)

# Balance Sheet (annual)
print("\n=== Balance Sheet (Annual) ===")
print(ticker.balance_sheet)

# Balance Sheet (quarterly)
print("\n=== Balance Sheet (Quarterly) ===")
print(ticker.quarterly_balance_sheet)

# Cash Flow (annual)
print("\n=== Cash Flow (Annual) ===")
print(ticker.cashflow)

# Cash Flow (quarterly)
print("\n=== Cash Flow (Quarterly) ===")
print(ticker.quarterly_cashflow)

# Earnings summary (dict)
print("\n=== Earnings ===")
print(ticker.earnings)
