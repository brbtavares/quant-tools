# 🚧 quant-tools — Roadmap de Desenvolvimento

> Última atualização: junho de 2025

---

## ✅ Mínimo Viável (MVP)

- [ ] Engine de backtest com candles OHLC
- [ ] Execução básica de ordens (buy/sell, market)
- [ ] Controle de posição, saldo, custo
- [ ] Estratégia SMA cruzada (exemplo)
- [ ] Cálculo de PnL, retorno e drawdown

---

## 🔜 Próximas Fases

### 📊 Indicadores Técnicos (src/indicators)

- [ ] SMA, EMA, WMA
- [ ] ATR
- [ ] RSI, ROC
- [ ] MACD

### 📁 Dados e Parsing (src/data)

- [ ] Loader de arquivos B3 (ações e futuros)
- [ ] Normalizador de símbolos (PETR4.SA → PETR4)
- [ ] Suporte a dados intraday
- [ ] Validador de gaps/datas

### 💰 B3-Specifics (src/b3)

- [ ] Ajuste diário de futuros
- [ ] Controle de proventos (dividendo, JCP)
- [ ] Apuração de IRRF mensal
- [ ] Calendário oficial da B3 (negociação e liquidação)

---

## 🎯 Longo Prazo

### 🔧 Estratégias

- [ ] Breakout de máximas/mínimas
- [ ] VWAP e médias móveis adaptativas
- [ ] Estratégias com opções (trava, borboleta, etc.)

### 🧠 Métricas e Risco

- [ ] Sharpe, Sortino
- [ ] Volatilidade anualizada
- [ ] Heatmap de drawdown
- [ ] Value-at-Risk (VaR)

### 🛠️ Arquitetura

- [ ] Separação em crates (`core`, `b3`, `strategies`, etc.)
- [ ] Publicação no crates.io
- [ ] CLI para rodar backtest por linha de comando

---

## 📚 Documentação

- [ ] Doc mdBook com tutoriais
- [ ] Exemplos por tipo de ativo
- [ ] Diagrama de arquitetura
