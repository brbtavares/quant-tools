# quant-tools — Open-Source Quantitative Trading Toolkit for B3 (Rust-based)

[![Docs](https://img.shields.io/badge/docs-online-blue)](https://quant-tools.com)
[![Rust CI](https://github.com/brbtavares/quant-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/brbtavares/quant-tools/actions/workflows/ci.yml)
[![Clippy Linter](https://github.com/brbtavares/quant-tools/actions/workflows/clippy.yml/badge.svg)](https://github.com/brbtavares/quant-tools/actions/workflows/clippy.yml)
[![Docs mdBook](https://img.shields.io/badge/docs-mdBook-blue?logo=githubpages&style=flat-square)](https://brbtavares.github.io/quant-tools)

> 📄 This README is also available in [Portuguese](./README.md)

---

## 🌐 About the project

`quant-tools` is an open-source Rust library for **backtesting and simulating quantitative trading strategies on the Brazilian stock exchange (B3)**. It is designed for performance, safety, and reproducibility, with a modular and extensible architecture.

Currently, the library focuses on B3-traded assets:

- **Equities**
- **Futures contracts** (e.g., index, dollar)
- **Options** (exchange-traded)
- **ETFs and REITs**

---

## ✨ Features

- ✅ Strategy simulation using OHLC (candlestick) data
- ✅ Order execution engine (buy/sell)
- ✅ Performance metrics (PnL, return, drawdown, Sharpe, etc.)
- 🟡 Historical data loading and processing from B3 (in progress)
- 🟡 Daily adjustment calculations for futures
- 🟡 Dividend and JCP simulation
- 🟡 Tax (IRRF) reporting on capital gains
- 🟡 Support for multi-leg option strategies

---

## 🔍 Roadmap

| Status | Feature                                             |
| ------ | --------------------------------------------------- |
| ✅      | Backtest engine with daily/intraday candles         |
| ✅      | Basic strategies (e.g., moving averages)            |
| 🔜      | Parser for B3 historical data (CSV, broker formats) |
| 🔜      | Realistic order execution (slippage, costs)         |
| 🔜      | Profit and tax reporting                            |
| 🔜      | Metrics visualization                               |
| 🔜      | Options module with advanced structures             |

---

## 🧱 Project Structure

```text
quant-tools/
├── core/         # Simulation engine and metrics
├── data/         # Market data abstractions
├── b3/           # B3-specific logic (adjustments, dividends, etc.)
├── strategies/   # Built-in strategies
├── examples/     # Usage examples
└── cli/          # Command-line interface
```

---

## 🚀 Technologies

- **Rust**: high performance, memory safety, and concurrency
- Modular architecture ready for future expansion (e.g., Forex, crypto)
- CI-integrated testing, Clippy, and linting

---

## 📦 Getting Started

```bash
git clone https://github.com/brbtavares/quant-tools.git
cd quant-tools
cargo build
cargo test
```

---

## 💡 Simple Example

```rust
use quant_tools::pricing::black_scholes::black_scholes_price;

let s = 100.0;   // spot price
let k = 100.0;   // strike price
let r = 0.05;    // risk-free rate
let sigma = 0.2; // volatility
let t = 1.0;     // time to maturity (in years)

let call_price = black_scholes_price(s, k, r, sigma, t, true);
println!("Call option price: {:.4}", call_price);
```

---

## 📚 Documentation

To generate local API documentation:

```bash
cargo doc --open
```

Or visit the [online documentation](https://quant-tools.com).

---

## 🔧 Development with Make

```bash
make              # Runs check, test, clippy
make build        # Builds the project
make test         # Runs all tests
make clippy       # Runs the linter (fails on warnings)
make fmt          # Formats the code
make check        # Checks without compiling
make dev          # Runs check + test + clippy
```

---

## 🤝 Contributing

We welcome contributions of models, numerical routines, examples, or docs!

1. Fork the repository and clone it
2. Create a branch: `git checkout -b feat/your-feature`
3. Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
4. Open a Pull Request to `master`

---

### ✅ Requirements

- Code must build on both Linux and Windows (CI)
- Run `make fmt && make clippy` before opening a PR

---

## 📝 License

Licensed under the [Apache 2.0 License](LICENSE).
