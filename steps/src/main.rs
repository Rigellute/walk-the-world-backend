use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use chrono::prelude::*;
use core::{CustomOutput, Steps};
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
use serde::Deserialize;
use simple_error::bail;
use std::env;
use uuid::Uuid;

#[derive(Deserialize)]
struct Body {
    steps: u64,
}

fn main() {
    lambda!(handler)
}

fn handler(event: ApiGatewayProxyRequest, context: Context) -> Result<CustomOutput, HandlerError> {
    let table_name = env::var("TABLE_NAME")?;
    let client = DynamoDbClient::new(Region::EuWest1);

    let utc: DateTime<Utc> = Utc::now();
    let now_as_millis = utc.timestamp_millis();
    let user_id = event
        .request_context
        .identity
        .cognito_identity_id
        .expect("Not authorized: no user_id present.");

    let body = event
        .body
        .expect("No `body` property present in the request");

    let steps = match serde_json::from_str::<Body>(&body) {
        Ok(body) => body.steps,
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
        steps,
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
                "Error saving steps: id - {}, error - {}",
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
