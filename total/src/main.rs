use chrono::prelude::*;
use core::{CustomOutput, Steps};
use dynomite::FromAttributes;
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
use serde::Serialize;
use serde_json::Value;
use simple_error::bail;
use std::env;

fn main() {
    lambda!(handler)
}

#[derive(Serialize)]
struct BodyResponse {
    steps: u64,
    calculated_at: i64,
}

fn handler(_event: Value, context: Context) -> Result<CustomOutput, HandlerError> {
    let table_name = env::var("TABLE_NAME")?;
    let client = DynamoDbClient::new(Region::EuWest1);

    let scan_items = ScanInput {
        table_name,
        ..ScanInput::default()
    };

    match client.scan(scan_items).sync() {
        Ok(result) => {
            let steps = result.items.into_iter().flatten().fold(0, |agg, item| {
                let step_struct = Steps::from_attrs(item).unwrap();
                agg + step_struct.steps
            });

            let now: i64 = Utc::now().timestamp_millis();
            let body = BodyResponse {
                steps,
                calculated_at: now,
            };

            Ok(CustomOutput {
                body: serde_json::to_string(&body)?,
                status_code: 200,
                ..CustomOutput::default()
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
