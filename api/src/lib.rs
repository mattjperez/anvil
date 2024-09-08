pub mod server;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    OperationCreated = 100,
    Started = 101,
    Stopped = 102,
    Running = 103,
    Canceling = 104,
    Pending = 105,
    Starting = 106,
    Stopping = 107,
    Aborting = 108,
    Freezing = 109,
    Frozen = 110,
    Thawed = 111,
    Error = 112,
    Ready = 113,
    Success = 200,
    Failure = 400,
    Canceled = 401,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Type {
    /// standard synchronous operation
    Sync,
    /// background asynchronous operation
    Async,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    status: Status,
    status_code: u16,
    r#type: Type,
    operation: String,
    error: String,
    error_code: u16,
    // #[serde(flatten)]
    // extra: HashMap<String, Value>,
}
