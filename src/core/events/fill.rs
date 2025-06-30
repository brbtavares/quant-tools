use super::signal_order::{Action, Direction};
use crate::common::{Price, Quantity, Timestamp};

/// Evento de execução de uma ordem.
/// Confirma que a ordem foi executada no mercado.
#[derive(Debug, Clone)]
pub struct FillEvent {
    pub timestamp: Timestamp,
    pub filled_price: Price,
    pub quantity: Quantity,
    pub direction: Direction,
    pub action: Action,
    // Futuro: comissão, order_id, instrumento, etc.
}
