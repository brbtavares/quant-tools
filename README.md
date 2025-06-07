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
</p>

---

## 🚀 About the Project

Quantbr Core is the high-performance engine behind the Quantbr trading platform — built in Rust with modularity, speed and flexibility in mind. Designed for professional and institutional use, but open for everyone.

## 🧱 Features

- ⚡ High-performance core in Rust
- 🧩 Modular design (data ingestion, execution, analytics, UI integration)
- 🔌 Easy integration with exchanges (via WebSocket, REST, FIX)
- 📊 Future-ready for backtesting, visualization, and real-time analytics
- 🧑‍💻 Open-source and community-driven

## 📦 Getting Started

```bash
git clone https://github.com/quantbr/quantbr-core.git
cd quantbr-core
cargo build
cargo test
```

## 🤝 Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for how to get started. All contributions are welcome!

## 📝 License

This project is licensed under the [Apache 2.0 License](LICENSE).

## 🔧 Automação com Just (`justfile`)

Este projeto utiliza [`just`](https://github.com/casey/just) para simplificar tarefas comuns de desenvolvimento. O `justfile` funciona como um script multiplataforma e facilita comandos como build, teste e lint.

### 📦 Instalação do `just`

Se você já tem o Rust instalado:

```bash
cargo install just
```

Alternativas:

- macOS: `brew install just`
- Windows: `choco install just` ou `cargo install just`
- Linux: use `cargo` ou baixe o binário na [página do projeto](https://github.com/casey/just)

---

### 🚀 Comandos disponíveis

Após instalar, você pode usar os seguintes comandos:

```bash
just             # Roda build, test e clippy
just build       # Compila o projeto (cargo build)
just test        # Roda os testes
just clippy      # Linter (com falha se houver warning)
just fmt         # Formata o código-fonte
just check       # Verifica o código sem compilar
```

---

### 💡 Dica

Você também pode rodar apenas:

```bash
just
```

Isso executa a tarefa padrão (`default`), que inclui `build`, `test` e `clippy`.
