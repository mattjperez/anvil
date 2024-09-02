pub mod server;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    metadata: Value,
    status: String,
    status_code: Number,
    r#type: String,
}
