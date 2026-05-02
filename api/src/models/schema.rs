use serde_json::{Value, json};

use super::RepositoryData;

pub const INDEX_SCHEMA_URL: &str = "https://luna.linwood.dev/schemas/index-v1.schema.json";

pub fn index_schema() -> Value {
    let mut schema = json!(schemars::schema_for!(RepositoryData));
    if let Some(object) = schema.as_object_mut() {
        object.insert("$id".to_string(), json!(INDEX_SCHEMA_URL));
        object.insert("title".to_string(), json!("Luna index v1"));
    }
    if let Some(file_version) = schema
        .pointer_mut("/properties/file_version")
        .and_then(Value::as_object_mut)
    {
        file_version.insert("const".to_string(), json!(1));
    }
    schema
}

pub fn index_schema_pretty() -> serde_json::Result<String> {
    serde_json::to_string_pretty(&index_schema())
}
