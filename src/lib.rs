//! # quantbr
//!
//! Biblioteca para análise quantitativa de mercados financeiros no Brasil.
//!
//! Módulos disponíveis:
//! - `common`: utilidades comuns
//! - `indicators`: indicadores técnicos
//! - `time_series`: manipulação de séries temporais
//! - `pricing`: modelos de precificação
//! - `risk`: análise de risco

// Re-exporting modules for public API
pub mod common;
pub mod indicators;
pub mod time_series;
pub mod pricing;
pub mod risk;
