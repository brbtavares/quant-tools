<p align="center">
  <img src="assets/logo.png" alt="Quantbr logo" width="200"/>
</p>

<h1 align="center">quantbr — Open-Source Quantitative Finance Tools in Rust.</h1>
<p align="center"><em>.Technical indicators, time series models, and risk metrics for algorithmic trading and portfolio analytics.</em></p>

<p align="center">
  <a href="https://github.com/quantbr/quantbr/actions/workflows/ci.yml">
    <img src="https://github.com/quantbr/quantbr/actions/workflows/ci.yml/badge.svg" alt="Rust CI">
  </a>
  <a href="https://github.com/quantbr/quantbr/actions/workflows/clippy.yml">
    <img src="https://github.com/quantbr/quantbr/actions/workflows/clippy.yml/badge.svg" alt="Clippy Linter">
  </a>
  <a href="https://quantbr.github.io/quantbr">
    <img src="https://img.shields.io/badge/docs-mdBook-blue?logo=githubpages&style=flat-square" alt="Docs">
  </a>
</p>

---

## 🚀 About the Project

*quantbr** is a Rust library providing tools for quantitative finance.  
It includes technical indicators, statistical models for time series analysis, and risk metrics — designed for use in trading strategies, portfolio analytics, and research.

---

## 🧱 Features

- 📈 Technical indicators (e.g. RSI, MACD, EMA, Bollinger Bands)
- 📊 Time series models (e.g. ARIMA, GARCH, Kalman Filter)
- ⚖️ Risk metrics (e.g. VaR, CVaR, Sharpe, Sortino, drawdown)
- 🧑‍💻 Written in Rust for performance and safety
- 🔌 Easy integration into trading engines and analytics platforms

---

## 📦 Getting Started

```bash
git clone https://github.com/quantbr/quantbr.git
cd quantbr
cargo build
cargo test
```

---

## ✨ Example

```rust
use quantbr::indicators::rsi;
let data = vec![45.0, 46.0, 44.5, 47.0, 48.0];
let rsi_val = rsi(&data, 14);
println!("RSI: {:?}", rsi_val);
```

---

## 🔧 Development with Just (`justfile`)

This project uses [`just`](https://github.com/casey/just) to streamline common development tasks.

### 📦 Installing `just`

If you already have Rust:

```bash
cargo install just
```

Or use a package manager:

- macOS: `brew install just`
- Windows: `choco install just` or `cargo install just`
- Linux: via `cargo` or from the [releases page](https://github.com/casey/just)

### 🚀 Available Commands

```bash
just             # Runs build, test, clippy
just build       # Builds the project
just test        # Runs all tests
just clippy      # Runs linter (fails on warnings)
just fmt         # Formats the code
just check       # Checks code without compiling
```

💡 You can simply run:

```bash
just
```

This will run the default task: `build`, `test`, and `clippy`.

---

## 🤝 Contributing

We welcome contributions! Add new indicators, models, or enhancements. Please follow Conventional Commits.

1. Fork the repo and clone your fork.
2. Create a branch: `git checkout -b feat/your-feature`
3. Use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format.
4. Push and open a Pull Request (PR) to `master`.

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

For any issues, contact: [admin@quantbr.com]

---

## 📝 License

Licensed under the [Apache 2.0 License](LICENSE).
