use crate::common::{Price, Quantity, Timestamp, Volume};

/// Evento de mercado especializado por tipo de dado.
#[derive(Debug, Clone)]
pub enum MarketEvent {
    Candle(CandleEvent), // OHLCV por período
    Tick(TickEvent),     // Negociação individual (tick)
    Book(BookEvent),     // Atualização de livro de ofertas
}

/// Evento que representa um candle completo (barra de preço).
#[derive(Debug, Clone)]
pub struct CandleEvent {
    pub timestamp: Timestamp, // Início do candle
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub volume: Price,
}

/// Evento que representa um único tick de negociação.
#[derive(Debug, Clone)]
pub struct TickEvent {
    pub timestamp: Timestamp,
    pub price: Price,   // Preço da negociação
    pub volume: Volume, // Volume negociado
    pub side: TickSide, // Lado agressor
}

/// Lado do tick: compra (agressor comprador) ou venda (agressor vendedor).
#[derive(Debug, Clone)]
pub enum TickSide {
    Buy,
    Sell,
}

/// Evento que representa uma atualização do livro de ofertas (DOM).
#[derive(Debug, Clone)]
pub struct BookEvent {
    pub timestamp: Timestamp,
    pub bids: Vec<Level>, // Lado da compra (ordens de compra)
    pub asks: Vec<Level>, // Lado da venda (ordens de venda)
}

/// Nível de preço no livro de ofertas.
#[derive(Debug, Clone)]
pub struct Level {
    pub price: Price,
    pub quantity: Quantity,
}
