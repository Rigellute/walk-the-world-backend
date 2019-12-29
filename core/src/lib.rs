use dynomite::Item;
use serde::Serialize;
use std::default::Default;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn get_now() -> Duration {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

#[derive(Item, Debug, Clone, Default, Serialize)]
pub struct Steps {
    #[dynomite(partition_key)]
    pub user_id: String,
    #[dynomite(sort_key)]
    pub step_id: Uuid,
    pub timestamp: u64,
    pub steps: u64,
}

#[derive(Serialize)]
pub struct Headers {
    #[serde(rename = "Access-Control-Allow-Origin")]
    pub access_control_allow_origin: char,
    #[serde(rename = "Access-Control-Allow-Credentials")]
    pub access_control_allow_credentials: bool,
}

impl Default for Headers {
    fn default() -> Self {
        Headers {
            access_control_allow_origin: '*',
            access_control_allow_credentials: true,
        }
    }
}

#[derive(Serialize)]
pub struct CustomOutput {
    pub body: String,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub headers: Headers,
}

impl Default for CustomOutput {
    fn default() -> Self {
        CustomOutput {
            body: "".to_string(),
            status_code: 404,
            headers: Headers::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
