use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use std::time::Duration;

use crate::error::PlatformError;

/// Store bytes in an S3 bucket.
pub async fn put_object(
    client: &Client,
    bucket: &str,
    key: &str,
    body: Vec<u8>,
    content_type: &str,
) -> Result<(), PlatformError> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body.into())
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| PlatformError::S3(e.to_string()))?;

    tracing::info!(bucket, key, "object stored");

    Ok(())
}

/// Retrieve bytes from an S3 bucket.
pub async fn get_object(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, PlatformError> {
    let result = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| PlatformError::S3(e.to_string()))?;

    let bytes = result
        .body
        .collect()
        .await
        .map_err(|e| PlatformError::S3(e.to_string()))?
        .into_bytes()
        .to_vec();

    Ok(bytes)
}

/// Generate a time-limited presigned download URL.
pub async fn presigned_get_url(
    client: &Client,
    bucket: &str,
    key: &str,
    expires_in: Duration,
) -> Result<String, PlatformError> {
    let presigning = PresigningConfig::builder()
        .expires_in(expires_in)
        .build()
        .map_err(|e| PlatformError::S3(e.to_string()))?;

    let request = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(presigning)
        .await
        .map_err(|e| PlatformError::S3(e.to_string()))?;

    Ok(request.uri().to_string())
}
