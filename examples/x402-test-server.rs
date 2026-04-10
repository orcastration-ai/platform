use alloy_primitives::address;
use axum::{routing::get, Json, Router};
use platform::x402::{middleware, V1Eip155Exact, USDC};
use serde_json::{json, Value};
use tracing_subscriber::EnvFilter;
use x402_chain_eip155::KnownNetworkEip155;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let x402 = middleware()?;

    // Test wallet (from x402-rs examples) — Base Sepolia testnet
    let pay_to = address!("BAc675C310721717Cd4A37F6cbeA1F081b1C2a07");
    let price = USDC::base_sepolia().parse("0.01")?;
    let price_tag = V1Eip155Exact::price_tag(pay_to, price);

    let app = Router::new().route("/health", get(health)).route(
        "/paid",
        get(paid_resource).layer(x402.with_price_tag(price_tag)),
    );

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(4020);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    tracing::info!("x402 test server on http://localhost:{port}");
    tracing::info!("  GET /health  — free");
    tracing::info!("  GET /paid    — requires 0.01 USDC (Base Sepolia)");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn paid_resource() -> Json<Value> {
    Json(json!({ "message": "Payment verified. You have access." }))
}
