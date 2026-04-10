# Platform

Shared Rust crate providing cross-cutting platform primitives consumed by all Orca services.

## Modules

- **`events`** — EventBridge publish (bus name + detail type + payload)
- **`dynamo`** — Generic DynamoDB operations (table name passed by caller)
- **`postgres`** — Postgres connection pool (placeholder — services own their queries)
- **`x402`** — x402 payment middleware for Axum services

## Commands

```sh
cargo build
cargo test
```

## Design Principles

- Modules are named for **what they do**, not which AWS service they wrap
- Table/bus names are always passed in by the caller — platform is config-agnostic
- Services own their domain logic; platform only provides shared transport
