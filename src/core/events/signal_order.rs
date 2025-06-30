use crate::common::{Quantity, Timestamp};

/// Direção da posição ou ordem: comprada ou vendida.
#[derive(Debug, Clone)]
pub enum Direction {
    Long,
    Short,
}

/// Ação da operação: abrir ou fechar posição.
#[derive(Debug, Clone)]
pub enum Action {
    Open,
    Close,
}

/// Ordem gerada a partir de um sinal e enviada para execução (real ou simulada).
#[derive(Debug, Clone)]
pub struct OrderEvent {
    pub timestamp: Timestamp,
    pub quantity: Quantity,
    pub direction: Direction,
    pub action: Action,
    // Futuro: instrument, limit_price, slippage, etc.
}

/// Representa um sinal gerado por uma estratégia de trading.
/// Indica a intenção de abrir ou fechar uma posição.
#[derive(Debug, Clone)]
pub struct SignalEvent {
    pub timestamp: Timestamp,
    pub direction: Direction, // Long ou Short
    pub action: Action,       // Open ou Close
}
