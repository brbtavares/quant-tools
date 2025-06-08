<p align="center">
  <img src="assets/logo.png" alt="Quantbr logo" width="200"/>
</p>

<h1 align="center">Quantbr Core</h1>
<p align="center"><em>Trade at the Speed of Rust — Open, Modular, Uncompromising Performance.</em></p>

<p align="center">
  <a href="https://github.com/quantbr/quantbr-core/actions/workflows/ci.yml">
    <img src="https://github.com/quantbr/quantbr-core/actions/workflows/ci.yml/badge.svg" alt="Rust CI">
  </a>
  <a href="https://github.com/quantbr/quantbr-core/actions/workflows/clippy.yml">
    <img src="https://github.com/quantbr/quantbr-core/actions/workflows/clippy.yml/badge.svg" alt="Clippy Linter">
  </a>
  <a href="https://quantbr.github.io/quantbr-core">
    <img src="https://img.shields.io/badge/docs-mdBook-blue?logo=githubpages&style=flat-square" alt="Docs">
  </a>
</p>

---

## 🚀 About the Project

**Quantbr Core** is the high-performance engine behind the Quantbr trading platform — written in Rust with modularity, speed, and flexibility in mind. Designed for professional and institutional use, but open to the community.

---

## 🧱 Features

- ⚡ High-performance core in Rust
- 🧩 Modular architecture (execution, analytics, integration)
- 🔌 Pluggable connectivity (WebSocket, REST, FIX, etc.)
- 📊 Ready for backtesting, visualization, and live analytics
- 🧑‍💻 Open-source, community-driven, and extensible

---

## 📦 Getting Started

```bash
git clone https://github.com/quantbr/quantbr-core.git
cd quantbr-core
cargo build
cargo test
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

We welcome contributions!

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

Open a [Discussion](https://github.com/quantbr/quantbr-core/discussions) or [Issue](https://github.com/quantbr/quantbr-core/issues).

---

## 📜 Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org) v2.1.

To foster an open and welcoming environment, we as contributors and maintainers pledge to making participation in our project harassment-free for everyone.

For any issues, contact: [admin@quantbr.com]

---

## 📝 License

Licensed under the [Apache 2.0 License](LICENSE).
