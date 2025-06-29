# quant-tools — Ferramenta Quantitativa Open-Source para a B3 (em Rust)

[![Docs](https://img.shields.io/badge/docs-online-blue)](https://quant-tools.com)
[![Rust CI](https://github.com/brbtavares/quant-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/brbtavares/quant-tools/actions/workflows/ci.yml)
[![Clippy Linter](https://github.com/brbtavares/quant-tools/actions/workflows/clippy.yml/badge.svg)](https://github.com/brbtavares/quant-tools/actions/workflows/clippy.yml)
[![Docs mdBook](https://img.shields.io/badge/docs-mdBook-blue?logo=githubpages&style=flat-square)](https://brbtavares.github.io/quant-tools)

> 📄 Este README também está disponível em [English](./README.en.md)

---

## 🌐 Sobre o projeto

`quant-tools` é uma biblioteca open-source em Rust para **backtesting e simulação de estratégias quantitativas no mercado brasileiro (B3)**. O projeto é focado em desempenho, segurança e reprodutibilidade, com arquitetura modular e extensível.

A biblioteca atualmente é especializada em ativos negociados na B3:

- **Ações**
- **Contratos futuros** (índice, dólar)
- **Opções** (negociadas via B3)
- **ETFs e FIIs**

---

## ✨ Funcionalidades

- ✅ Simulação de estratégias com dados OHLC (candlestick)
- ✅ Engine de execução de ordens com suporte a compra/venda
- ✅ Métricas de avaliação (PnL, retorno, drawdown, Sharpe etc.)
- 🟡 Carga e tratamento de dados históricos da B3 (em andamento)
- 🟡 Cálculo de ajustes diários para futuros
- 🟡 Simulação de proventos (dividendos, JCP)
- 🟡 Apuração de IRRF sobre lucros operacionais
- 🟡 Suporte a estruturas com opções (travas, borboletas)

---

## 🔍 Roadmap

| Status | Recurso                                                             |
| ------ | ------------------------------------------------------------------- |
| ✅      | Engine de backtest com candles diários/intradiários                 |
| ✅      | Estratégias básicas (ex: cruzamento de médias)                      |
| 🔜      | Parser de arquivos históricos da B3 (ex: arquivos BVMF, corretoras) |
| 🔜      | Execução realista com slippage e custo                              |
| 🔜      | Análise de operações com IR e custos                                |
| 🔜      | Visualização de resultados e métricas                               |
| 🔜      | Módulo de opções com múltiplas pernas                               |

---

## 🧱 Estrutura do Projeto

```text
quant-tools/
├── core/         # Motor de simulação e métricas
├── data/         # Abstrações de dados de mercado
├── b3/           # Lógica específica da B3 (ajustes, proventos etc.)
├── strategies/   # Estratégias implementadas
├── examples/     # Casos de uso
└── cli/          # Interface de linha de comando
```

---

## 🚀 Tecnologias

- **Rust**: alta performance, segurança de memória e concorrência eficiente
- Arquitetura modular com possibilidade de extensão futura (ex: Forex, cripto)
- Testes automatizados com CI, Clippy e integração contínua

---

## 📦 Como começar

```bash
git clone https://github.com/brbtavares/quant-tools.git
cd quant-tools
cargo build
cargo test
```

---

## 💡 Exemplo simples

```rust
use quant_tools::pricing::black_scholes::black_scholes_price;

let s = 100.0;   // preço do ativo
let k = 100.0;   // strike
let r = 0.05;    // taxa livre de risco
let sigma = 0.2; // volatilidade
let t = 1.0;     // tempo até o vencimento (em anos)

let call_price = black_scholes_price(s, k, r, sigma, t, true);
println!("Preço da call: {:.4}", call_price);
```

---

## 📚 Documentação

Para gerar a documentação da API localmente:

```bash
cargo doc --open
```

Ou acesse a [documentação online](https://quant-tools.com).

---

## 🔧 Desenvolvimento com Make

```bash
make              # Executa dev: check, test, clippy
make build        # Compila o projeto
make test         # Roda todos os testes
make clippy       # Executa linter (com erro em warnings)
make fmt          # Formata o código
make check        # Verifica erros sem compilar
make dev          # Atalho para check + test + clippy
```

---

## 🤝 Contribuindo

Contribuições são bem-vindas!  
Modelos, rotinas numéricas, exemplos ou melhorias na documentação — toda ajuda é valiosa.

1. Faça um fork e clone o repositório.
2. Crie uma branch: `git checkout -b feat/nome-da-feature`
3. Use o padrão [Conventional Commits](https://www.conventionalcommits.org/pt-br/v1.0.0/)
4. Abra um Pull Request para `master`.

---

### ✅ Requisitos

- O código deve compilar no Linux e Windows (CI automatizada)
- Rode `make fmt && make clippy` antes de enviar o PR

---

## 📝 Licença

Licenciado sob a [Licença Apache 2.0](LICENSE).
