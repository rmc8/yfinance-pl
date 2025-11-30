# yfinance-pl

[![PyPI version](https://img.shields.io/pypi/v/yfinance-pl.svg)](https://pypi.org/project/yfinance-pl/)
[![Python](https://img.shields.io/pypi/pyversions/yfinance-pl.svg)](https://pypi.org/project/yfinance-pl/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/rmc8/yfinance-pl/blob/main/LICENSE)
[![Documentation](https://img.shields.io/badge/docs-yfinance--pl.rmc--8.com-blue.svg)](https://yfinance-pl.rmc-8.com/)

[English](../../README.md) | [日本語](JA.md) | [中文](ZH.md) | [한국어](KO.md) | [Español](ES.md)

Un wrapper de Python para [yfinance-rs](https://github.com/gramistella/yfinance-rs) usando PyO3 y Polars.

Obtén datos del mercado de valores con una API compatible con [yfinance](https://github.com/ranaroussi/yfinance), pero con **Polars DataFrames** en lugar de pandas.

## Características

- **API compatible con yfinance** - Interfaz familiar para usuarios de yfinance
- **Polars DataFrames** - Devuelve `pl.DataFrame` en lugar de `pd.DataFrame`
- **Backend en Rust** - Obtención rápida de datos impulsada por yfinance-rs
- **API con tipos seguros** - Autocompletado del IDE para `period`, `interval` y tipos de retorno

## Instalación

### Desde PyPI

```bash
pip install yfinance-pl
```

### Desde el código fuente

Requiere compilador de protobuf:

```bash
# macOS
brew install protobuf

# Ubuntu/Debian
sudo apt install protobuf-compiler
```

Luego compilar:

```bash
git clone https://github.com/rmc8/yfinance-pl.git
cd yfinance-pl
pip install maturin
maturin develop --release
```

## Inicio rápido

```python
import yfinance_pl as yf

ticker = yf.Ticker("AAPL")
history = ticker.history(period="1mo")
print(history)
```

## Referencia de API

### Historial de precios

| Método/Propiedad | Tipo de retorno | Descripción |
|----------------|-------------|-------------|
| `history(period, interval, start, end)` | `pl.DataFrame` | Datos históricos de precios (columna `date` como Datetime) |

**Parámetros de `history()`:**
- `period`: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max
- `interval`: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo
- `start`: Fecha de inicio (YYYY-MM-DD)
- `end`: Fecha de fin (YYYY-MM-DD)

### Información de la empresa

| Propiedad | Tipo de retorno | Descripción |
|----------|-------------|-------------|
| `info` | `dict` | Información completa de la empresa |
| `fast_info` | `dict` | Acceso rápido a métricas clave |
| `calendar` | `dict` | Próximos eventos (fechas de resultados, etc.) |

### Dividendos y acciones corporativas

| Propiedad | Tipo de retorno | Descripción |
|----------|-------------|-------------|
| `dividends` | `pl.DataFrame` | Historial de dividendos |
| `splits` | `pl.DataFrame` | Historial de splits de acciones |
| `actions` | `pl.DataFrame` | Dividendos y splits combinados |
| `capital_gains` | `pl.DataFrame` | Ganancias de capital (para fondos/ETFs) |

### Estados financieros

| Propiedad | Tipo de retorno | Descripción |
|----------|-------------|-------------|
| `income_stmt` | `pl.DataFrame` | Estado de resultados anual |
| `quarterly_income_stmt` | `pl.DataFrame` | Estado de resultados trimestral |
| `balance_sheet` | `pl.DataFrame` | Balance general anual |
| `quarterly_balance_sheet` | `pl.DataFrame` | Balance general trimestral |
| `cashflow` | `pl.DataFrame` | Estado de flujo de efectivo anual |
| `quarterly_cashflow` | `pl.DataFrame` | Estado de flujo de efectivo trimestral |
| `earnings` | `dict` | Resumen de ganancias |

### Información de accionistas

| Propiedad | Tipo de retorno | Descripción |
|----------|-------------|-------------|
| `major_holders` | `pl.DataFrame` | Resumen de principales accionistas |
| `institutional_holders` | `pl.DataFrame` | Inversores institucionales |
| `mutualfund_holders` | `pl.DataFrame` | Fondos mutuos |
| `insider_transactions` | `pl.DataFrame` | Transacciones de insiders |
| `insider_roster_holders` | `pl.DataFrame` | Lista de insiders |

### Recomendaciones de analistas

| Propiedad | Tipo de retorno | Descripción |
|----------|-------------|-------------|
| `recommendations` | `pl.DataFrame` | Resumen de recomendaciones |
| `upgrades_downgrades` | `pl.DataFrame` | Historial de cambios de calificación |

### Opciones (solo acciones de EE.UU.)

| Propiedad/Método | Tipo de retorno | Descripción |
|----------------|-------------|-------------|
| `options` | `list[str]` | Fechas de vencimiento disponibles |
| `option_chain(date)` | `OptionChain` | Cadena de opciones para fecha de vencimiento |

El método `option_chain()` devuelve una tupla nombrada `OptionChain`:
```python
chain = ticker.option_chain("2024-01-19")
chain.calls  # pl.DataFrame
chain.puts   # pl.DataFrame
```

### Otros

| Método | Tipo de retorno | Descripción |
|--------|-------------|-------------|
| `get_isin()` | `str` | Identificador ISIN |

## Ejemplos

Consulta el directorio [examples/](../../examples/) para ejemplos detallados:

- `01_quick_start.py` - Ejemplo mínimo
- `02_history.py` - Opciones de historial de precios
- `03_company_info.py` - Información de la empresa
- `04_dividends_splits.py` - Acciones corporativas
- `05_financials.py` - Estados financieros
- `06_holders.py` - Información de accionistas
- `07_analysis.py` - Recomendaciones de analistas
- `08_options.py` - Datos de opciones

Ejecutar ejemplos:
```bash
uv run python examples/01_quick_start.py
```

## Desarrollo

```bash
# Instalar dependencias
uv sync

# Compilar módulo Rust (ejecutar después de cambios)
uv run maturin develop

# Compilación de producción
uv run maturin develop --release

# Formatear y verificar
uv run ruff format .
uv run ruff check .
```

## Diferencias con yfinance

| Aspecto | yfinance | yfinance-pl |
|--------|----------|-------------|
| Tipo de DataFrame | `pd.DataFrame` | `pl.DataFrame` |
| Backend | Python | Rust (yfinance-rs) |
| Tipo de Series | `pd.Series` | `pl.DataFrame` |

## Licencia

MIT
