pub mod server;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use serde_repr::*;
use std::collections::HashMap;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum StatusCode {
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
pub struct Base {
    status: String,
    status_code: StatusCode,
    r#type: String,
    operation: String,
    error: String,
    error_code: Number,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
