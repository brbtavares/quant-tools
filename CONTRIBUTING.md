# Contributing to Quantbr

Thank you for considering contributing to Quantbr — a high-performance, modular trading platform in Rust.

This guide outlines how to get started and the conventions we follow.

---

## 🚀 Getting Started

1. Fork the repository and clone your fork.
2. Create a new branch:
   ```bash
   git checkout -b feat/my-feature
   ```
3. Commit changes using the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format (see below).
4. Push and open a pull request against `master`.

---

## 🔧 Development

- This project uses [Rust](https://www.rust-lang.org/).
- Clone the repo and run:

  ```bash
  cargo build && cargo test
  ```

  ***

## 🧪 Contributing Guidelines

- Fork the repository
- Create a new branch (`git checkout -b feature/feature-name`)
- Commit with clear messages (`feat: add order book support`)
- Open a pull request (PR)

---

## ✍️ Commit Message Format (Conventional Commits)

We use the **Conventional Commits** standard to maintain clarity and automation.

### Format:

```
<type>(optional-scope): short description
```

### Common Types:

| Type       | Description                                       |
| ---------- | ------------------------------------------------- |
| `feat`     | New feature                                       |
| `fix`      | Bug fix                                           |
| `docs`     | Documentation only changes                        |
| `style`    | Code style changes (formatting, spacing...)       |
| `refactor` | Code change that doesn't fix a bug or add feature |
| `test`     | Adding or refactoring tests                       |
| `chore`    | Maintenance, tooling, or dependency updates       |
| `ci`       | CI/CD workflow changes                            |

### Examples:

```bash
feat: add WebSocket connector for Binance
fix(orderbook): correct price level merging bug
docs: update README with installation steps
ci(clippy): add Clippy workflow for static analysis
```

---

## ✅ Requirements

- Code must build on both Linux and Windows using GitHub Actions
- Run `just fmt && just clippy` before submitting

---

## 💬 Questions or Ideas?

Open a [Discussion](https://github.com/quantbr/quantbr-core/discussions) or [Issue](https://github.com/quantbr/quantbr-core/issues) to start a conversation.

---

## 📄 License

By contributing, you agree that your code will be licensed under the Apache 2.0 License.
