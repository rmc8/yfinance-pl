use paft::prelude::ToDataFrameVec;
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_polars::PyDataFrame;
use yfinance_rs::core::{Interval, Range};
use yfinance_rs::{Ticker as YfTicker, YfClient};

fn create_runtime() -> PyResult<tokio::runtime::Runtime> {
    tokio::runtime::Runtime::new()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

fn to_py_err<E: std::fmt::Display>(e: E) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
}

fn parse_range(period: &str) -> Range {
    match period {
        "1d" => Range::D1,
        "5d" => Range::D5,
        "1mo" => Range::M1,
        "3mo" => Range::M3,
        "6mo" => Range::M6,
        "1y" => Range::Y1,
        "2y" => Range::Y2,
        "5y" => Range::Y5,
        "10y" => Range::Y10,
        "ytd" => Range::Ytd,
        "max" => Range::Max,
        _ => Range::M1,
    }
}

fn parse_interval(interval: &str) -> Interval {
    match interval {
        "1m" => Interval::I1m,
        "2m" => Interval::I2m,
        "5m" => Interval::I5m,
        "15m" => Interval::I15m,
        "30m" => Interval::I30m,
        "90m" => Interval::I90m,
        "60m" | "1h" => Interval::I1h,
        "1d" => Interval::D1,
        "5d" => Interval::D5,
        "1wk" => Interval::W1,
        "1mo" => Interval::M1,
        "3mo" => Interval::M3,
        _ => Interval::D1,
    }
}

/// Ticker class for fetching stock data (yfinance-compatible API)
#[pyclass]
struct Ticker {
    symbol: String,
}

#[pymethods]
impl Ticker {
    #[new]
    fn new(symbol: String) -> Self {
        Ticker { symbol }
    }

    /// Get historical OHLCV data as a Polars DataFrame
    ///
    /// Args:
    ///     period: Data period (1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max)
    ///     interval: Data interval (1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo)
    ///     start: Start date (not yet implemented)
    ///     end: End date (not yet implemented)
    ///     prepost: Include pre and post market data
    ///     auto_adjust: Adjust prices for splits and dividends
    ///     actions: Include dividends and stock splits
    #[pyo3(signature = (period=None, interval=None, start=None, end=None, prepost=false, auto_adjust=true, actions=true))]
    fn history(
        &self,
        period: Option<&str>,
        interval: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
        prepost: bool,
        auto_adjust: bool,
        actions: bool,
    ) -> PyResult<PyDataFrame> {
        let _ = (start, end); // TODO: implement start/end date filtering

        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);

            let range_val = parse_range(period.unwrap_or("1mo"));
            let interval_val = parse_interval(interval.unwrap_or("1d"));

            let mut builder = ticker.history_builder();
            builder = builder
                .range(range_val)
                .interval(interval_val)
                .auto_adjust(auto_adjust)
                .prepost(prepost)
                .actions(actions);

            let candles = builder.fetch().await.map_err(to_py_err)?;
            let df = candles.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get ticker info as a dictionary
    #[getter]
    fn info(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let info = ticker.info().await.map_err(to_py_err)?;

            let dict = PyDict::new(py);

            // Basic info
            dict.set_item("symbol", &symbol)?;

            // Name
            if let Some(name) = &info.name {
                dict.set_item("shortName", name)?;
            }

            // ISIN
            if let Some(isin) = &info.isin {
                dict.set_item("isin", isin.to_string())?;
            }

            // Exchange
            if let Some(exchange) = &info.exchange {
                dict.set_item("exchange", format!("{:?}", exchange))?;
            }

            // Market state
            if let Some(state) = &info.market_state {
                dict.set_item("marketState", format!("{:?}", state))?;
            }

            // Currency
            if let Some(currency) = &info.currency {
                dict.set_item("currency", format!("{:?}", currency))?;
            }

            // Prices
            if let Some(last) = &info.last {
                dict.set_item("regularMarketPrice", last.amount().to_string())?;
            }
            if let Some(open) = &info.open {
                dict.set_item("regularMarketOpen", open.amount().to_string())?;
            }
            if let Some(high) = &info.high {
                dict.set_item("regularMarketDayHigh", high.amount().to_string())?;
            }
            if let Some(low) = &info.low {
                dict.set_item("regularMarketDayLow", low.amount().to_string())?;
            }
            if let Some(prev) = &info.previous_close {
                dict.set_item("regularMarketPreviousClose", prev.amount().to_string())?;
            }

            // Volume
            if let Some(vol) = info.volume {
                dict.set_item("regularMarketVolume", vol)?;
            }
            if let Some(avg_vol) = info.average_volume {
                dict.set_item("averageVolume", avg_vol)?;
            }

            // Fundamentals
            if let Some(mc) = &info.market_cap {
                dict.set_item("marketCap", mc.amount().to_string())?;
            }
            if let Some(shares) = info.shares_outstanding {
                dict.set_item("sharesOutstanding", shares)?;
            }
            if let Some(eps) = &info.eps_ttm {
                dict.set_item("trailingEps", eps.amount().to_string())?;
            }
            if let Some(pe) = &info.pe_ttm {
                dict.set_item("trailingPE", pe.to_string())?;
            }
            if let Some(div_yield) = &info.dividend_yield {
                dict.set_item("dividendYield", div_yield.to_string())?;
            }

            // 52-week range
            if let Some(low52) = &info.fifty_two_week_low {
                dict.set_item("fiftyTwoWeekLow", low52.amount().to_string())?;
            }
            if let Some(high52) = &info.fifty_two_week_high {
                dict.set_item("fiftyTwoWeekHigh", high52.amount().to_string())?;
            }

            Ok(dict.into())
        })
    }

    /// Get fast info as a dictionary
    #[getter]
    fn fast_info(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let fast_info = ticker.fast_info().await.map_err(to_py_err)?;

            let dict = PyDict::new(py);
            dict.set_item("symbol", &symbol)?;

            if let Some(name) = &fast_info.name {
                dict.set_item("name", name)?;
            }
            if let Some(exchange) = &fast_info.exchange {
                dict.set_item("exchange", format!("{:?}", exchange))?;
            }
            if let Some(currency) = &fast_info.currency {
                dict.set_item("currency", format!("{:?}", currency))?;
            }
            if let Some(volume) = fast_info.volume {
                dict.set_item("volume", volume)?;
            }

            Ok(dict.into())
        })
    }

    /// Get dividend history as a Polars DataFrame
    #[getter]
    fn dividends(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let divs = ticker.dividends(None).await.map_err(to_py_err)?;

            let timestamps: Vec<i64> = divs.iter().map(|(ts, _)| *ts).collect();
            let amounts: Vec<f64> = divs.iter().map(|(_, amt)| *amt).collect();

            let df = DataFrame::new(vec![
                Series::new("date".into(), timestamps).into(),
                Series::new("dividends".into(), amounts).into(),
            ])
            .map_err(to_py_err)?;

            Ok(PyDataFrame(df))
        })
    }

    /// Get stock splits history as a Polars DataFrame
    #[getter]
    fn splits(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let splits = ticker.splits(None).await.map_err(to_py_err)?;

            let timestamps: Vec<i64> = splits.iter().map(|(ts, _, _)| *ts).collect();
            let ratios: Vec<f64> = splits
                .iter()
                .map(|(_, num, den)| *num as f64 / *den as f64)
                .collect();

            let df = DataFrame::new(vec![
                Series::new("date".into(), timestamps).into(),
                Series::new("stock_splits".into(), ratios).into(),
            ])
            .map_err(to_py_err)?;

            Ok(PyDataFrame(df))
        })
    }

    /// Get corporate actions (dividends + splits) as a Polars DataFrame
    #[getter]
    fn actions(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let acts = ticker.actions(None).await.map_err(to_py_err)?;

            let mut timestamps: Vec<i64> = Vec::new();
            let mut dividends: Vec<Option<f64>> = Vec::new();
            let mut splits: Vec<Option<f64>> = Vec::new();

            for action in acts {
                match action {
                    yfinance_rs::core::Action::Dividend { ts, amount } => {
                        timestamps.push(yfinance_rs::core::conversions::datetime_to_i64(ts));
                        dividends.push(Some(yfinance_rs::core::conversions::money_to_f64(&amount)));
                        splits.push(None);
                    }
                    yfinance_rs::core::Action::Split {
                        ts,
                        numerator,
                        denominator,
                    } => {
                        timestamps.push(yfinance_rs::core::conversions::datetime_to_i64(ts));
                        dividends.push(None);
                        splits.push(Some(numerator as f64 / denominator as f64));
                    }
                    yfinance_rs::core::Action::CapitalGain { .. } => {}
                }
            }

            let df = DataFrame::new(vec![
                Series::new("date".into(), timestamps).into(),
                Series::new("dividends".into(), dividends).into(),
                Series::new("stock_splits".into(), splits).into(),
            ])
            .map_err(to_py_err)?;

            Ok(PyDataFrame(df))
        })
    }

    /// Get capital gains as a Polars DataFrame
    #[getter]
    fn capital_gains(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let gains = ticker.capital_gains(None).await.map_err(to_py_err)?;

            let timestamps: Vec<i64> = gains.iter().map(|(ts, _)| *ts).collect();
            let amounts: Vec<f64> = gains.iter().map(|(_, amt)| *amt).collect();

            let df = DataFrame::new(vec![
                Series::new("date".into(), timestamps).into(),
                Series::new("capital_gains".into(), amounts).into(),
            ])
            .map_err(to_py_err)?;

            Ok(PyDataFrame(df))
        })
    }

    /// Get the ISIN for this ticker
    fn get_isin(&self) -> PyResult<Option<String>> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            ticker.isin().await.map_err(to_py_err)
        })
    }

    // ============ Phase 2: Financial Statements ============

    /// Get annual income statement as a Polars DataFrame
    #[getter]
    fn income_stmt(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let stmt = ticker.income_stmt(None).await.map_err(to_py_err)?;
            let df = stmt.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get quarterly income statement as a Polars DataFrame
    #[getter]
    fn quarterly_income_stmt(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let stmt = ticker.quarterly_income_stmt(None).await.map_err(to_py_err)?;
            let df = stmt.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get annual balance sheet as a Polars DataFrame
    #[getter]
    fn balance_sheet(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let stmt = ticker.balance_sheet(None).await.map_err(to_py_err)?;
            let df = stmt.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get quarterly balance sheet as a Polars DataFrame
    #[getter]
    fn quarterly_balance_sheet(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let stmt = ticker.quarterly_balance_sheet(None).await.map_err(to_py_err)?;
            let df = stmt.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get annual cash flow statement as a Polars DataFrame
    #[getter]
    fn cashflow(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let stmt = ticker.cashflow(None).await.map_err(to_py_err)?;
            let df = stmt.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get quarterly cash flow statement as a Polars DataFrame
    #[getter]
    fn quarterly_cashflow(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let stmt = ticker.quarterly_cashflow(None).await.map_err(to_py_err)?;
            let df = stmt.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get earnings data as a dictionary
    #[getter]
    fn earnings(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let earnings = ticker.earnings(None).await.map_err(to_py_err)?;

            let dict = PyDict::new(py);
            dict.set_item("symbol", &symbol)?;

            // Add earnings counts
            dict.set_item("yearly_count", earnings.yearly.len())?;
            dict.set_item("quarterly_count", earnings.quarterly.len())?;
            dict.set_item("quarterly_eps_count", earnings.quarterly_eps.len())?;

            Ok(dict.into())
        })
    }

    /// Get calendar events as a dictionary
    #[getter]
    fn calendar(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let cal = ticker.calendar().await.map_err(to_py_err)?;

            let dict = PyDict::new(py);
            dict.set_item("symbol", &symbol)?;

            // earnings_dates is Vec, not Option
            let date_strs: Vec<String> = cal.earnings_dates.iter().map(|d| d.to_string()).collect();
            dict.set_item("earningsDates", date_strs)?;

            if let Some(date) = cal.ex_dividend_date {
                dict.set_item("exDividendDate", date.to_string())?;
            }
            if let Some(date) = cal.dividend_payment_date {
                dict.set_item("dividendDate", date.to_string())?;
            }

            Ok(dict.into())
        })
    }

    // ============ Phase 3: Analysis & Holders ============

    /// Get analyst recommendations as a Polars DataFrame
    #[getter]
    fn recommendations(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let recs = ticker.recommendations().await.map_err(to_py_err)?;
            let df = recs.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get analyst upgrades/downgrades history as a Polars DataFrame
    #[getter]
    fn upgrades_downgrades(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let data = ticker.upgrades_downgrades().await.map_err(to_py_err)?;
            let df = data.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get major holders breakdown as a Polars DataFrame
    #[getter]
    fn major_holders(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let holders = ticker.major_holders().await.map_err(to_py_err)?;

            let categories: Vec<String> = holders.iter().map(|h| h.category.clone()).collect();
            let values: Vec<String> = holders.iter().map(|h| h.value.to_string()).collect();

            let df = DataFrame::new(vec![
                Series::new("Breakdown".into(), categories).into(),
                Series::new("Value".into(), values).into(),
            ])
            .map_err(to_py_err)?;

            Ok(PyDataFrame(df))
        })
    }

    /// Get institutional holders as a Polars DataFrame
    #[getter]
    fn institutional_holders(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let holders = ticker.institutional_holders().await.map_err(to_py_err)?;
            let df = holders.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get mutual fund holders as a Polars DataFrame
    #[getter]
    fn mutualfund_holders(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let holders = ticker.mutual_fund_holders().await.map_err(to_py_err)?;
            let df = holders.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get insider transactions as a Polars DataFrame
    #[getter]
    fn insider_transactions(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let txns = ticker.insider_transactions().await.map_err(to_py_err)?;
            let df = txns.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get insider roster holders as a Polars DataFrame
    #[getter]
    fn insider_roster_holders(&self) -> PyResult<PyDataFrame> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let roster = ticker.insider_roster_holders().await.map_err(to_py_err)?;
            let df = roster.to_dataframe().map_err(to_py_err)?;
            Ok(PyDataFrame(df))
        })
    }

    // ============ Phase 4: Options ============

    /// Get available option expiration dates as a list of strings (YYYY-MM-DD)
    #[getter]
    fn options(&self) -> PyResult<Vec<String>> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);
            let timestamps = ticker.options().await.map_err(to_py_err)?;

            // Convert Unix timestamps to YYYY-MM-DD strings
            let dates: Vec<String> = timestamps
                .iter()
                .map(|ts| {
                    chrono::DateTime::from_timestamp(*ts, 0)
                        .map(|dt| dt.format("%Y-%m-%d").to_string())
                        .unwrap_or_default()
                })
                .collect();

            Ok(dates)
        })
    }

    /// Get option chain for a specific expiration date
    ///
    /// Args:
    ///     date: Expiration date in YYYY-MM-DD format (optional, uses nearest if not provided)
    ///
    /// Returns:
    ///     tuple: (calls DataFrame, puts DataFrame)
    #[pyo3(signature = (date=None))]
    fn option_chain(&self, date: Option<&str>) -> PyResult<(PyDataFrame, PyDataFrame)> {
        let runtime = create_runtime()?;
        let symbol = self.symbol.clone();
        let date_str = date.map(|s| s.to_string());

        runtime.block_on(async move {
            let client = YfClient::default();
            let ticker = YfTicker::new(&client, &symbol);

            // Convert date string to Unix timestamp if provided
            let timestamp = if let Some(ref date) = date_str {
                let naive = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
                    .map_err(|e| to_py_err(format!("Invalid date format: {}", e)))?;
                Some(
                    naive
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                        .and_utc()
                        .timestamp(),
                )
            } else {
                None
            };

            let chain = ticker.option_chain(timestamp).await.map_err(to_py_err)?;

            let calls_df = chain.calls.to_dataframe().map_err(to_py_err)?;
            let puts_df = chain.puts.to_dataframe().map_err(to_py_err)?;

            Ok((PyDataFrame(calls_df), PyDataFrame(puts_df)))
        })
    }

    fn __repr__(&self) -> String {
        format!("yfinance.Ticker('{}')", self.symbol)
    }
}

/// Python module for yfinance-pl
#[pymodule]
fn _yfinance_pl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Ticker>()?;
    Ok(())
}
