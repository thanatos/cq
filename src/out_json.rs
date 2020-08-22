use std::convert::TryFrom;
use std::error::Error;
use std::io::Write;

use serde_cbor::Value;
use serde_json::Value as JsonValue;

pub fn output_json<W: Write>(output: W, value: Value) -> Result<(), Box<dyn Error>> {
    serde_json::to_writer(output, &to_json(&value))?;
    Ok(())
}

fn to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => JsonValue::Null,
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Integer(i) => {
            let number = match u64::try_from(*i) {
                Ok(i) => serde_json::Number::from(i),
                Err(_) => match i64::try_from(*i) {
                    Ok(i) => serde_json::Number::from(i),
                    Err(_) => unimplemented!(
                        "CBOR input contained an integer that was larger than serde_json could \
                         express."
                    ),
                }
            };
            JsonValue::Number(number)
        },
        Value::Float(f) => {
            match serde_json::Number::from_f64(*f) {
                Some(n) => JsonValue::Number(n),
                None => unimplemented!(
                    "CBOR input contained a floating point value that was one of NaN, +∞, or -∞; \
                     JSON is not capable of representing these values.",
                ),
            }
        }
        Value::Text(t) => JsonValue::String(t.to_owned()),
        Value::Array(arr) => JsonValue::Array(arr.iter().map(to_json).collect()),
        Value::Map(map) => {
            let mut output_map = serde_json::Map::new();
            for (key, value) in map.iter() {
                let key = match key {
                    Value::Text(v) => v.to_owned(),
                    _ => unimplemented!(
                        "A CBOR input contained a map with a non-string key {:?}; JSON is not \
                         capable of representing maps with keys other than strings.",
                        key,
                    ),
                };
                let value = to_json(value);
                output_map.insert(key, value);
            }
            JsonValue::Object(output_map)
        }
        _ => unimplemented!(
            "CBOR contained {:?}; no conversion is implemented for this type.",
            value,
        )
    }
}
