use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("could not parse mac address")]
    MacParse(#[from] MacParseError),
    #[error("wol password was not in 4 byte or 6 byte format")]
    WolPasswordParse,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MacParseError {
    #[error("could not parse \"{byte}\" as byte")]
    ParseByte {
        #[source]
        source: ParseIntError,
        byte: String,
    },
    #[error("mac address is too long, got {0} bytes")]
    TooLong(usize),
    #[error("mac address is too short, got {0} bytes")]
    TooShort(usize),
}
