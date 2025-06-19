# quantbr — Open-Source Quantitative Finance Tools in Rust

[![Docs](https://img.shields.io/badge/docs-online-blue)](https://quantbr.com)
[![Rust CI](https://github.com/quantbr/quantbr/actions/workflows/ci.yml/badge.svg)](https://github.com/quantbr/quantbr/actions/workflows/ci.yml)
[![Clippy Linter](https://github.com/quantbr/quantbr/actions/workflows/clippy.yml/badge.svg)](https://github.com/quantbr/quantbr/actions/workflows/clippy.yml)
[![Docs mdBook](https://img.shields.io/badge/docs-mdBook-blue?logo=githubpages&style=flat-square)](https://quantbr.github.io/quantbr)

---

## 🚀 About the Project

`quantbr` is a Rust library designed to provide a suite of tools for quantitative finance. This library aims to facilitate the development of financial applications by offering a modular and idiomatic Rust interface.

---

## 🧱 Features

- **Portfolio Optimization** (e.g., mean-variance optimization, risk parity, Black-Litterman)
- **Asset Allocation** (strategic, tactical, dynamic allocation methods)
Market Microstructure Analysis (order book modeling, slippage, transaction cost analysis)
- **Execution Algorithms** (VWAP, TWAP, implementation shortfall, smart order routing)
- **Factor Investing & Smart Beta** (multi-factor models, Fama-French, momentum, value, quality)
- **Machine Learning for Finance**(classification, regression, clustering, reinforcement learning)
**Alternative Data Analysis (sentiment analysis, satellite imagery, web scraping)**
- **Credit Risk Modeling (PD/LGD/EAD estimation, credit scoring, structural models)**
- **Interest Rate Modeling (yield curve modeling, term structure, short-rate models)**
- **Liquidity Risk & Stress Testing** (liquidity metrics, scenario analysis)
- **Regulatory & Compliance Analytics** (Basel, MiFID, risk reporting)
- **Simulation & Scenario Analysis** (Monte Carlo, historical, what-if analysis)
**Performance Attribution** (Brinson model, multi-factor attribution)
- **Backtesting & Strategy Evaluation** (robustness checks, walk-forward analysis)

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
use quantbr::indicators::rsi;
let data = vec![45.0, 46.0, 44.5, 47.0, 48.0];
let period = 14
let rsi_val = rsi(&data, period);
println!("RSI: {:?}", rsi_val);
```

---

## 📃 Documentation

For detailed documentation on each module and its functions, refer to the inline documentation within the source code or generate the documentation using:

```bash
cargo doc --open
```

---

## 🔧 Development with Make

This project uses a `Makefile` to streamline common development tasks.  
No external dependencies are required — `make` is widely available on Linux and macOS.  
On Windows, you can use it via [WSL](https://learn.microsoft.com/en-us/windows/wsl/), [MSYS2](https://www.msys2.org/), or [Git Bash](https://gitforwindows.org/).

---

### 🚀 Available Commands

```bash
make              # Runs dev: check, test, clippy
make build        # Builds the project
make test         # Runs all tests
make clippy       # Runs linter (fails on warnings)
make fmt          # Formats the code
make check        # Checks code without compiling
make dev          # Runs check, test, clippy (default target)
```

💡 You can simply run:

```bash
make
```

This will run the default `dev` target: `cargo check`, `cargo test`, and `cargo clippy -- -D warnings`.

---

## 🤝 Contributing

We welcome contributions! Add new indicators, models, or enhancements. Please follow Conventional Commits.

1. Fork the repo and clone your fork.
2. Create a branch: `git checkout -b feat/your-feature`
3. Use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format.
4. Push and open a Pull Request (PR) to `master`.

---

### ✅ Requirements

- Code must build on Linux and Windows (verified via GitHub Actions)
- Run `just fmt && just clippy` before opening a PR

---

## ✍️ Commit Message Format

We follow **Conventional Commits**.

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
feat: add WebSocket connector for Binance
fix(orderbook): fix price level merge bug
docs: update README instructions
ci(clippy): add Clippy static analysis
```

---

## 💬 Questions or Ideas?

Open a [Discussion](https://github.com/quantbr/quantbr/discussions) or [Issue](https://github.com/quantbr/quantbr/issues).

---

## 📜 Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org) v2.1.

To foster an open and welcoming environment, we as contributors and maintainers pledge to making participation in our project harassment-free for everyone.

For any issues, contact: [brbtavares@outlook.com]

---

## 📝 License

Licensed under the [Apache 2.0 License](LICENSE).
