# quantbr — Open-Source Quantitative Finance Library in Rust (QuantLib-inspired)

[![Docs](https://img.shields.io/badge/docs-online-blue)](https://quantbr.com)
[![Rust CI](https://github.com/quantbr/quantbr/actions/workflows/ci.yml/badge.svg)](https://github.com/quantbr/quantbr/actions/workflows/ci.yml)
[![Clippy Linter](https://github.com/quantbr/quantbr/actions/workflows/clippy.yml/badge.svg)](https://github.com/quantbr/quantbr/actions/workflows/clippy.yml)
[![Docs mdBook](https://img.shields.io/badge/docs-mdBook-blue?logo=githubpages&style=flat-square)](https://quantbr.github.io/quantbr)

---

## 🌐 About quantbr

`quantbr` is an open-source Rust library for **quantitative finance models**, designed to provide high-performance, modular, and idiomatic tools for financial engineering.  

It serves as a comprehensive framework that integrates models for:

- **Pricing** of derivatives and structured products
- **Forecasting** of asset prices, volatility, and risk factors
- **Hedging** strategies and optimal replication techniques
- **Risk measurement** including exposure analysis and scenario testing
- **Execution** models for market microstructure and trading algorithms  

The project is inspired by industry standards like QuantLib, but leverages the safety, speed, and modern design of Rust to deliver robust financial components.

Whether you're building pricing engines, risk systems, trading platforms, or research tools, `quantbr` provides a unified foundation for quantitative finance applications.
g, yield curve construction, risk measurement, and numerical methods, with the safety and speed of Rust.

---

## 🧱 Features

`quantbr` provides a unified suite of quantitative finance models, covering the full spectrum of pricing, forecasting, hedging, risk measurement, and execution foundations.

- **Derivative Pricing**
  - Black-Scholes (closed-form)
  - Binomial and Trinomial trees
  - Monte Carlo simulation (planned)
  - Stochastic processes: GBM, Heston, Hull-White, CIR, Vasicek (planned)

- **Interest Rate & Yield Curves**
  - Term structure of interest rates
  - Discount curves, forward curves
  - Piecewise and spline interpolation (planned)

- **Fixed Income Instruments**
  - Bonds, swaps
  - Cash flow models (fixed and floating rate)
  - Day count conventions, business day adjustment

- **Risk Models & Metrics**
  - Duration, convexity
  - Value-at-Risk (VaR), Conditional VaR (CVaR) (planned)
  - Scenario analysis (planned)

- **Forecasting & Time Series Tools**
  - OHLC series types and helpers
  - Volatility estimators (historical, EWMA - planned)
  - Foundations for ARIMA, GARCH (planned)

- **Hedging & Replication**
  - Delta, gamma hedging (planned)
  - Optimal hedge ratio calculators (planned)

- **Numerical Methods**
  - Root-finding algorithms
  - Integration routines
  - Finite difference grids (planned)

- **Calendars**
  - Business calendars and holiday adjustments

---

## 📦 Getting Started

```bash
git clone https://github.com/brbtavares/quantbr.git
cd quantbr
cargo build
cargo test
```

---

## ✨ Example

```rust
use quantbr::pricing::black_scholes::black_scholes_price;

let s = 100.0;   // spot price
let k = 100.0;   // strike price
let r = 0.05;    // risk-free rate
let sigma = 0.2; // volatility
let t = 1.0;     // time to maturity

let call_price = black_scholes_price(s, k, r, sigma, t, true);
println!("Call option price: {:.4}", call_price);
```

---

## 📃 Documentation

For detailed documentation on each module, generate the API docs using:

```bash
cargo doc --open
```

Or visit the [online documentation](https://quantbr.com).

---

## 🔧 Development with Make

```bash
make              # Runs dev: check, test, clippy
make build        # Builds the project
make test         # Runs all tests
make clippy       # Runs linter (fails on warnings)
make fmt          # Formats the code
make check        # Checks code without compiling
make dev          # Runs check, test, clippy (default target)
```

Run `make` to execute the default `dev` workflow.

---

## 🤝 Contributing

We welcome contributions of new models, numerical routines, documentation, or examples!

1. Fork the repo and clone your fork.
2. Create a branch: `git checkout -b feat/your-feature`
3. Use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
4. Push and open a Pull Request (PR) to `master`.

---

### ✅ Requirements

- Code must build on Linux and Windows (via CI)
- Run `make fmt && make clippy` before opening a PR

---

## ✍️ Commit Message Format

**Format:**

```bash
<type>(optional-scope): short description
```

---

**Common Types:**

| Type     | Description                                    |
| -------- | ---------------------------------------------- |
| feat     | New feature                                    |
| fix      | Bug fix                                        |
| docs     | Documentation updates                          |
| style    | Code style changes                             |
| refactor | Code changes that neither fix nor add features |
| test     | Adding or updating tests                       |
| chore    | Build process, tooling, or dependencies        |
| ci       | CI/CD workflow changes                         |

**Examples:**

```bash
feat(pricing): add Black-Scholes option pricing
fix(yield): correct interpolation on forward curve
docs: update README with example usage
ci(clippy): enforce no warnings in Clippy

```

---

## 💬 Questions or Ideas?

Open a [Discussion](https://github.com/brbtavares/quantbr/discussions) or [Issue](https://github.com/brbtavares/quantbr/issues).

---

## 📜 Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org) v2.1.

---

## 📝 License

Licensed under the [Apache 2.0 License](LICENSE).
