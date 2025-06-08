# Architecture

Quantbr Core is composed of loosely coupled modules:

- **Market Data Ingestion**: Connectors for exchanges (WebSocket, REST, FIX)
- **Execution Engine**: Manages orders, accounts, and trade lifecycle
- **Analytics Module**: Real-time metrics, indicators, strategy hooks
- **Interface Layer**: Exposes APIs to UI, bots or external systems

The project uses Rust's crate system to maintain strong boundaries and separation of concerns between modules.
