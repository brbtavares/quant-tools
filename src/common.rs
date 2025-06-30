// This file contains common traits and types used across the project.

// Time
pub use chrono::{DateTime, Utc};
pub type Timestamp = DateTime<Utc>;

// Tipos numéricos básicos
pub type Money = f64; // Quantia em dinheiro (ex: saldo, valor de posição)
pub type Price = f64; // Preço unitário
pub type Volume = f64; // Volume financeiro (preço * quantidade)
pub type Quantity = u32; // Número de contratos/lotes
pub type Percent = f64; // 0.0 → 1.0 (ex: 0.05 = 5%)
pub type Points = f64; // Pontos (índice, preço alvo etc.)
pub type Multiplier = f64; // Multiplicador de contrato ou risco

// Funções auxiliares (exemplo)
pub fn round_money(value: Money, tick_size: f64) -> Money {
    (value / tick_size).round() * tick_size
}
