use rlua::Value;
use serde::{ser::SerializeMap, Serialize};

pub struct ValueWrapper<'lua>(pub Value<'lua>);

impl<'lua> Serialize for ValueWrapper<'lua> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            ValueWrapper(value) => match value {
                Value::String(s) => serializer.serialize_str(s.to_str().unwrap()),
                Value::Boolean(b) => serializer.serialize_bool(*b),
                Value::Integer(i) => serializer.serialize_i64(*i),
                Value::Number(n) => serializer.serialize_f64(*n),
                Value::Table(t) => {
                    let mut map = serializer.serialize_map(None)?;
                    for pair in t.clone().pairs::<Value, Value>() {
                        let (key, value) = pair.unwrap();
                        map.serialize_entry(&ValueWrapper(key), &ValueWrapper(value))?;
                    }
                    map.end()
                }
                _ => Err(serde::ser::Error::custom(format!(
                    "Unsupported type: {:?}",
                    value
                ))),
            },
        }
    }
}
