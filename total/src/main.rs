use dynomite::{FromAttributes, Item};
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
use serde::Serialize;
use serde_json::Value;
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

#[derive(Serialize)]
struct CustomOutput {
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: u32,
}

fn main() {
    lambda!(handler)
}

fn handler(_event: Value, context: Context) -> Result<CustomOutput, HandlerError> {
    let client = DynamoDbClient::new(Region::EuWest1);

    let scan_items = ScanInput {
        table_name: "uclsteps".to_string(),
        ..ScanInput::default()
    };

    match client.scan(scan_items).sync() {
        Ok(result) => {
            let steps = result.items.into_iter().flatten().fold(0, |agg, item| {
                let step_struct = Steps::from_attrs(item).unwrap();
                agg + step_struct.steps
            });

            Ok(CustomOutput {
                body: steps,
                status_code: 200,
            })
        }
        Err(e) => {
            println!(
                "Something went wrong: id - {}, error - {}",
                context.aws_request_id, e
            );

            bail!("Error scanning table");
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
