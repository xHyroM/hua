use std::io;

use rlua::Value;

pub mod serializer;

pub fn serialize(value: Value) -> io::Result<String> {
    let value_wrapper = serializer::ValueWrapper(value);
    let serialized = serde_json::to_string(&value_wrapper)?;
    Ok(serialized)
}
