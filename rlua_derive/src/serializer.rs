use rlua::{Context, Error, FromLua, Result, ToLua, Value};
use serde_json::{Number as JsonNumber, Value as JsonValue};

struct JsonValueWrapper(pub serde_json::Value);

impl<'lua> ToLua<'lua> for &'_ JsonValueWrapper {
    fn to_lua(self, lua: Context<'lua>) -> Result<Value<'lua>> {
        Ok(match &self.0 {
            JsonValue::Null => Value::Nil,
            JsonValue::Bool(b) => Value::Boolean(*b),
            JsonValue::Number(n) => {
                if let Some(n) = n.as_i64() {
                    Value::Integer(n)
                } else if let Some(n) = n.as_f64() {
                    Value::Number(n)
                } else {
                    Err(Error::ToLuaConversionError {
                        from: "serde_json::Number",
                        to: "Integer",
                        message: Some(format!("value {} too large", n)),
                    })?
                }
            }
            JsonValue::String(s) => lua.create_string(s).map(Value::String)?,
            _ => Err(Error::ToLuaConversionError {
                from: "serde_json::Value",
                to: "AnyLuaData",
                message: Some("unsupported value".to_string()),
            })?,
        })
    }
}

impl<'lua> FromLua<'lua> for JsonValueWrapper {
    fn from_lua(lua_value: Value<'lua>, lua: Context<'lua>) -> Result<Self> {
        Ok(match lua_value {
            Value::Nil => JsonValueWrapper(JsonValue::Null),
            Value::Boolean(b) => JsonValueWrapper(JsonValue::Bool(b)),
            Value::LightUserData(_) => Err(Error::FromLuaConversionError {
                from: "LightUserData",
                to: "serde_json::Value",
                message: Some("not supported".to_string()),
            })?,
            Value::Integer(i) => JsonValueWrapper(JsonValue::Number(i.into())),
            Value::Number(n) => JsonValueWrapper(JsonValue::Number(
                JsonNumber::from_f64(n).ok_or_else(|| Error::FromLuaConversionError {
                    from: "Number",
                    to: "serde_json::Number",
                    message: Some(format!("value {} not supported", n)),
                })?,
            )),
            Value::String(s) => JsonValueWrapper(JsonValue::String(s.to_str()?.to_string())),
            /*Value::Table(t) => {
                if t.len()? == 0 {
                    // There's no way to know whether it's supposed to be an
                    // object or an array.
                    JsonValueWrapper(JsonValue::Object(JsonMap::new()))
                } else if let Ok(Nil) = t.get(1) {
                    // It's probably a sequence.
                    let values = t
                        .sequence_values()
                        .map(|r: Result<Value>| {
                            r.and_then(|v| JsonValueWrapper(JsonValue::from_lua(v, lua))?)
                        })
                        .collect::<Result<_>>()?;

                    JsonValueWrapper(JsonValue::Array(values))
                } else {
                    // XXX: maybe call a metamethod here?
                    let items = t
                        .pairs()
                        .map(|r: Result<(String, Value)>| {
                            r.and_then(|(k, v)| {
                                Ok((
                                    k.as_str().to_string(),
                                    JsonValueWrapper(JsonValue::from_lua(v, lua)?),
                                ))
                            })
                        })
                        .collect::<Result<_>>()?;

                    JsonValueWrapper(JsonValue::Object(items))
                }
            }*/
            Value::Function(_) => Err(Error::FromLuaConversionError {
                from: "Function",
                to: "serde_json::Value",
                message: Some("not supported".to_string()),
            })?,
            Value::Thread(_) => Err(Error::FromLuaConversionError {
                from: "Thread",
                to: "serde_json::Value",
                message: Some("not supported".to_string()),
            })?,
            Value::Table(_) => Err(Error::FromLuaConversionError {
                from: "Table",
                to: "serde_json::Value",
                message: Some("not supported".to_string()),
            })?,
            Value::UserData(_) => Err(Error::FromLuaConversionError {
                from: "AnyUserData",
                to: "serde_json::Value",
                message: Some("not supported".to_string()),
            })?,
            Value::Error(e) => Err(e)?,
        })
    }
}
