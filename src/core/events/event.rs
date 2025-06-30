use super::{
    fill::FillEvent,
    market::MarketEvent,
    signal_order::{OrderEvent, SignalEvent},
};

/// Representa qualquer evento que trafega no sistema — feed de mercado, sinais, ordens e execuções.
#[derive(Debug, Clone)]
pub enum Event {
    Market(MarketEvent), // Evento de mercado (candle, tick, book)
    Signal(SignalEvent), // Sinal gerado por uma estratégia
    Order(OrderEvent),   // Ordem enviada para execução
    Fill(FillEvent),     // Confirmação de execução da ordem
}
