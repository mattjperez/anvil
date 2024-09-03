pub mod server;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    status: String,
    status_code: Number,
    r#type: String,
    operation: String,
    error: String,
    error_code: Number,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
