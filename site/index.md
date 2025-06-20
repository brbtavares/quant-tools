---
layout: home
nav_order: 1
title: quantbr
---

Ferramentas quantitativas para finanças em Rust.

Explore:

- [Docs.rs](https://quantbr.com/docs)
- [mdBook](https://quantbr.com/book)
- [GitHub](https://github.com/brbtavares/quantbr)

## Exemplo de uso

```rust
use quantbr::indicators::rsi::Rsi;

let mut rsi = Rsi::new(14);
rsi.update(45.0);
println!("{:?}", rsi.value());
```
