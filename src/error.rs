use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("DynamoDB: {0}")]
    DynamoDb(String),

    #[error("EventBridge: {0}")]
    Events(String),

    #[error("{0}")]
    Internal(String),

    #[error("x402: {0}")]
    X402(String),
}

impl From<aws_sdk_dynamodb::Error> for PlatformError {
    fn from(err: aws_sdk_dynamodb::Error) -> Self {
        PlatformError::DynamoDb(err.to_string())
    }
}

impl From<aws_sdk_eventbridge::Error> for PlatformError {
    fn from(err: aws_sdk_eventbridge::Error) -> Self {
        PlatformError::Events(err.to_string())
    }
}
