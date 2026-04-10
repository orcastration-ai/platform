//! x402 payment middleware for Axum services.
//!
//! Services use `middleware()` to get a configured middleware instance, then
//! apply `.with_price_tag(tag)` as a layer on payment-gated routes.

use std::sync::Arc;

use crate::error::PlatformError;

pub use alloy_primitives::address;
pub use x402_axum::facilitator_client::FacilitatorClient;
pub use x402_axum::X402Middleware;
pub use x402_chain_eip155::{KnownNetworkEip155, V1Eip155Exact, V2Eip155Exact};
pub use x402_types::networks::USDC;

pub const DEFAULT_FACILITATOR_URL: &str = "https://x402.org/facilitator";

/// Create x402 middleware using `X402_FACILITATOR_URL` env var (or default).
pub fn middleware() -> Result<X402Middleware<Arc<FacilitatorClient>>, PlatformError> {
    let url = std::env::var("X402_FACILITATOR_URL")
        .unwrap_or_else(|_| DEFAULT_FACILITATOR_URL.to_string());
    X402Middleware::try_new(&url).map_err(|e| PlatformError::X402(e.to_string()))
}
