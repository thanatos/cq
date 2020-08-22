use std::error::Error;
use std::fmt;
use std::io;

use serde_cbor::Value;
use structopt::StructOpt;

mod out_json;
mod out_text;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(long = "output", short, default_value = "auto")]
    output_format: OutputFormat,
}

#[derive(Debug)]
enum OutputFormat {
    Auto,
    Cbor,
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = OutputFormatParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "auto" => OutputFormat::Auto,
            "cbor" => OutputFormat::Cbor,
            "text" => OutputFormat::Text,
            "json" => OutputFormat::Json,
            _ => return Err(OutputFormatParseErr(s.to_owned())),
        })
    }
}

#[derive(Debug)]
struct OutputFormatParseErr(String);

impl fmt::Display for OutputFormatParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown value \"{}\" for --output", self.0)
    }
}

impl Error for OutputFormatParseErr {}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    let input: Value = serde_cbor::from_reader(io::stdin())?;

    match args.output_format {
        OutputFormat::Auto => out_text::pretty_print(&mut io::stdout(), &input, 0, true)?,
        OutputFormat::Text => out_text::pretty_print(&mut io::stdout(), &input, 0, true)?,
        OutputFormat::Json => out_json::output_json(&mut io::stdout(), input)?,
        _ => unimplemented!(),
    }

    Ok(())
}
