use aws_sdk_eventbridge::Client;
use serde::Serialize;

use crate::error::PlatformError;

/// Publish an event to an EventBridge bus.
///
/// - `bus_name`: The EventBridge bus name (e.g. from SSM parameter)
/// - `detail_type`: Event type identifier (e.g. "contact.submitted", "lead.captured")
/// - `payload`: Any serializable struct — serialized to JSON as the event detail
pub async fn publish(
    client: &Client,
    bus_name: &str,
    detail_type: &str,
    payload: &impl Serialize,
) -> Result<(), PlatformError> {
    let detail =
        serde_json::to_string(payload).map_err(|e| PlatformError::Events(e.to_string()))?;

    client
        .put_events()
        .entries(
            aws_sdk_eventbridge::types::PutEventsRequestEntry::builder()
                .event_bus_name(bus_name)
                .source("orca")
                .detail_type(detail_type)
                .detail(detail)
                .build(),
        )
        .send()
        .await
        .map_err(|e| PlatformError::Events(e.to_string()))?;

    tracing::info!(bus = bus_name, detail_type, "event published");

    Ok(())
}
