use crate::{Error, Result};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // (should be iso8601)

    // -- User and context attributes.
    user_id: Option<u64>,

    // -- http request attributes.
    req_path: String,
    req_methos: String,

    // -- Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
