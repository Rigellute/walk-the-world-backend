use core::get_now;
use dynomite::Item;
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
use serde::{Deserialize, Serialize};
use simple_error::bail;
use uuid::Uuid;

#[derive(Item, Debug, Clone, Default, Serialize)]
pub struct Steps {
    #[dynomite(partition_key)]
    user_id: String,
    #[dynomite(sort_key)]
    step_id: Uuid,
    timestamp: u64,
    steps: u32,
}

#[derive(Deserialize)]
struct Identity {
    #[serde(rename = "cognitoIdentityId")]
    cognito_identity_id: String,
}

#[derive(Deserialize)]
struct RequestContext {
    identity: Identity,
}

#[derive(Serialize)]
struct CustomOutput {
    body: String,
    #[serde(rename = "statusCode")]
    status_code: u16,
}

#[derive(Deserialize)]
struct Body {
    steps: u32,
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
    let client = DynamoDbClient::new(Region::EuWest1);
    let since_the_epoch = get_now();

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
        user_id: event.request_context.identity.cognito_identity_id,
        step_id: Uuid::new_v4(),
        steps: body.steps,
        timestamp: since_the_epoch.as_millis() as u64,
    };

    let put_item = PutItemInput {
        table_name: "uclsteps".to_string(),
        item: new_step_entry.clone().into(),
        ..PutItemInput::default()
    };

    match client.put_item(put_item).sync() {
        Ok(_result) => Ok(CustomOutput {
            body: serde_json::to_string(&new_step_entry)?,
            status_code: 200,
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
