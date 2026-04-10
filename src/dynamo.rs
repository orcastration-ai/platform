use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::error::PlatformError;

/// Put an item into a DynamoDB table.
pub async fn put_item(
    client: &Client,
    table_name: &str,
    item: HashMap<String, AttributeValue>,
) -> Result<(), PlatformError> {
    client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await
        .map_err(|e| PlatformError::DynamoDb(e.to_string()))?;

    Ok(())
}

/// Put an item with a condition expression (e.g. `attribute_not_exists(email)` for idempotency).
/// Returns `Ok(true)` if the item was written, `Ok(false)` if the condition failed.
pub async fn put_item_if(
    client: &Client,
    table_name: &str,
    item: HashMap<String, AttributeValue>,
    condition: &str,
) -> Result<bool, PlatformError> {
    let result = client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .condition_expression(condition)
        .send()
        .await;

    match result {
        Ok(_) => Ok(true),
        Err(err) => {
            if err
                .as_service_error()
                .is_some_and(|e| e.is_conditional_check_failed_exception())
            {
                Ok(false)
            } else {
                Err(PlatformError::DynamoDb(err.to_string()))
            }
        }
    }
}

/// Get a single item by its key.
pub async fn get_item(
    client: &Client,
    table_name: &str,
    key: HashMap<String, AttributeValue>,
) -> Result<Option<HashMap<String, AttributeValue>>, PlatformError> {
    let result = client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
        .map_err(|e| PlatformError::DynamoDb(e.to_string()))?;

    Ok(result.item)
}

/// Query items using a key condition expression.
///
/// - `index_name`: Optional GSI name (e.g. "by-source")
/// - `key_condition`: KeyConditionExpression (e.g. "#src = :src")
/// - `names`: ExpressionAttributeNames (e.g. {"#src" => "source"})
/// - `values`: ExpressionAttributeValues
pub async fn query(
    client: &Client,
    table_name: &str,
    index_name: Option<&str>,
    key_condition: &str,
    names: Option<HashMap<String, String>>,
    values: HashMap<String, AttributeValue>,
) -> Result<Vec<HashMap<String, AttributeValue>>, PlatformError> {
    let mut builder = client
        .query()
        .table_name(table_name)
        .key_condition_expression(key_condition)
        .set_expression_attribute_values(Some(values));

    if let Some(idx) = index_name {
        builder = builder.index_name(idx);
    }

    if let Some(n) = names {
        builder = builder.set_expression_attribute_names(Some(n));
    }

    let result = builder
        .send()
        .await
        .map_err(|e| PlatformError::DynamoDb(e.to_string()))?;

    Ok(result.items.unwrap_or_default())
}
