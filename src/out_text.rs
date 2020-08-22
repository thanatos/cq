use std::io::{self, Write};

use serde_cbor::Value;

pub fn pretty_print<W: Write>(output: &mut W, value: &Value, indent: u32, final_nl: bool) -> io::Result<()> {
    match value {
        Value::Null => write!(output, "null")?,
        Value::Bool(b) => write!(output, "{}", b)?,
        Value::Integer(i) => write!(output, "{}", i)?,
        Value::Float(f) => write!(output, "{}", f)?,
        Value::Text(t) => output_text(output, t)?,
        Value::Array(arr) => {
            write!(output, "[\n")?;
            for value in arr {
                pretty_print(output, value, indent + 1, false)?;
                write!(output, ",\n")?;
            }
            output_indent(output, indent)?;
            write!(output, "]")?;
        }
        Value::Map(map) => {
            write!(output, "{{\n")?;
            for (key, value) in map.iter() {
                output_indent(output, indent + 1)?;
                pretty_print(output, key, indent + 1, false)?;
                write!(output, ": ")?;
                pretty_print(output, value, indent + 1, false)?;
                write!(output, ",\n")?;
            }
            output_indent(output, indent)?;
            write!(output, "}}")?;
        }
        _ => write!(output, "(unknown value: {:?})", value)?,
    }
    if final_nl {
        write!(output, "\n")?;
    }
    Ok(())
}

fn output_text<W: Write>(output: &mut W, text: &str) -> io::Result<()> {
    write!(output, "\"")?;
    for c in text.chars() {
        match c {
            '\\' => write!(output, "\\")?,
            '"' => write!(output, "\\\"")?,
            _ => write!(output, "{}", c)?,
        }
    }
    write!(output, "\"")
}

fn output_indent<W: Write>(output: &mut W, indent: u32) -> io::Result<()> {
    for _ in 0 .. indent {
        write!(output, "\t")?;
    }
    Ok(())
}
