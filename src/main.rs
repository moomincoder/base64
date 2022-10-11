use std::fmt;
use std::io::{self, Read};
use crate::alphabet::Classic;

mod alphabet;
mod decoder;
mod encoder;


fn read_stdin() -> Result<String, CLIError> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|_| CLIError::StdInUnreadable)?;

    Ok(input.trim().to_string())
}

fn encode(input: &String) -> String {
    encoder::encode_using_alphabet(&Classic, input.as_bytes())
}

fn decode(input: &String) -> Result<String, CLIError> {
    let decoded =
        decoder::decode_using_alphabet(Classic, input).map_err(|_| CLIError::DecodingError)?;
    
    let decoded_as_string = std::str::from_utf8(&decoded).map_err(|_| CLIError::DecodingError)?;

    Ok(decoded_as_string.to_owned())
}

enum CLIError {
    TooFewArguments,
    InvalidSubCommand(String),
    StdInUnreadable,
    DecodingError,
}

impl std::fmt::Debug for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::TooFewArguments =>
                write!(f, "Not enough arguments provided"),

            Self::InvalidSubCommand(cmd) =>
                write!(f, "Invalid subcommand provided: \"{}\"", cmd),

            Self::StdInUnreadable =>
                write!(f, "Unable to read STDIN"),

            Self::DecodingError =>
                write!(f, "An error occured while decoding"),
        }
    }
}

fn main() -> Result<(), CLIError> {
    if std::env::args().count() < 2 {
        return Err(CLIError::TooFewArguments);
    }

    let subcommand = std::env::args()
        .nth(1)
        .ok_or_else(|| CLIError::TooFewArguments)?;

    let input = read_stdin()?;

    let output = match subcommand.as_str() {
        "encode" => Ok(encode(&input)),
        "decode" => Ok(decode(&input)?),
        cmd => Err(CLIError::InvalidSubCommand(cmd.to_string())),
    }?;

    println!("{}", output);

    Ok(())
}
