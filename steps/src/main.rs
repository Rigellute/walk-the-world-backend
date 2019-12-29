use chrono::prelude::*;
use core::{CustomOutput, Steps};
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, QueryInput};
use serde::Deserialize;
use simple_error::bail;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

#[derive(Deserialize)]
struct Identity {
    #[serde(rename = "cognitoIdentityId")]
    cognito_identity_id: String,
}

#[derive(Deserialize)]
struct RequestContext {
    identity: Identity,
}

#[derive(Deserialize)]
struct Body {
    steps: u64,
}

#[derive(Deserialize)]
struct CustomEvent {
    body: String,
    #[serde(rename = "requestContext")]
    request_context: RequestContext,
}

fn main() {
    lambda!(handler)
}

fn handler(event: CustomEvent, context: Context) -> Result<CustomOutput, HandlerError> {
    let table_name = env::var("TABLE_NAME")?;
    let client = DynamoDbClient::new(Region::EuWest1);

    let utc: DateTime<Utc> = Utc::now();
    let now_as_millis = utc.timestamp_millis();
    let num_millis_from_midnight = utc.num_seconds_from_midnight() * 1000;
    let user_id = event.request_context.identity.cognito_identity_id;

    /*
     * First, validate that the user has not added any steps within the last day (i.e. since midnight)
     */
    // The property "timestamp" is a reserved word in DynamoDb, so we must rename it
    let mut expression_attribute_names = HashMap::new();
    expression_attribute_names.insert("#ts".to_string(), "timestamp".to_string());

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":user_id".to_string(),
        AttributeValue {
            s: Some(user_id.clone()),
            ..Default::default()
        },
    );
    expression_attribute_values.insert(
        ":midnight".to_string(),
        AttributeValue {
            n: Some((now_as_millis - num_millis_from_midnight as i64).to_string()),
            ..Default::default()
        },
    );

    let query = QueryInput {
        table_name: table_name.clone(),
        key_condition_expression: Some("user_id = :user_id".to_string()),
        filter_expression: Some("#ts > :midnight".to_string()),
        expression_attribute_values: Some(expression_attribute_values),
        expression_attribute_names: Some(expression_attribute_names),
        ..QueryInput::default()
    };

    let has_added_steps_since_midnight = match client.query(query).sync() {
        Ok(res) => match res.items {
            Some(items) => !items.is_empty(),
            None => false,
        },
        Err(e) => {
            println!(
                "Error checking if user has posted in last day: id - {}, error - {}",
                context.aws_request_id, e
            );
            bail!("Something went wrong");
        }
    };

    if has_added_steps_since_midnight {
        let not_allowed = CustomOutput {
            status_code: 400,
            ..Default::default()
        };

        return Ok(not_allowed);
    }

    let body = match serde_json::from_str::<Body>(&event.body) {
        Ok(body) => body,
        Err(e) => {
            println!(
                "Error parsing body: id - {}, error - {}",
                context.aws_request_id, e
            );
            bail!("Error parsing body");
        }
    };

    let new_step_entry = Steps {
        user_id,
        step_id: Uuid::new_v4(),
        steps: body.steps,
        timestamp: now_as_millis as u64,
    };

    let put_item = PutItemInput {
        table_name,
        item: new_step_entry.clone().into(),
        ..Default::default()
    };

    match client.put_item(put_item).sync() {
        Ok(_result) => Ok(CustomOutput {
            body: serde_json::to_string(&new_step_entry)?,
            status_code: 200,
            ..Default::default()
        }),
        Err(e) => {
            println!(
                "Something went wrong: id - {}, error - {}",
                context.aws_request_id, e
            );
            bail!("Error saving steps");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn handler_handles() {
        let event = json!({
            "answer": 42
        });
    }
}
