use crate::error::MacParseError;
use std::{convert::TryFrom, fmt, str::FromStr};

#[cfg(feature = "deserialize")]
use serde::Deserialize;

#[cfg(feature = "serialize")]
use serde::Serialize;

/// A Hardware Address.
#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "deserialize", serde(try_from = "&str"))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(into = "String"))]
pub struct HardwareAddress {
    octets: [u8; 6],
}

impl HardwareAddress {
    /// Create a new Hardware Address.
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        HardwareAddress {
            octets: [a, b, c, d, e, f],
        }
    }
    /// Return the octects of the HardwareAddress.
    pub fn octects(&self) -> &[u8; 6] {
        &self.octets
    }
}

impl FromStr for HardwareAddress {
    type Err = MacParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(|c| c == ':' || c == '-').collect();
        if parts.len() > 6 {
            return Err(MacParseError::TooLong(parts.len()));
        }
        if parts.len() < 6 {
            return Err(MacParseError::TooShort(parts.len()));
        }
        let value = parts
            .into_iter()
            .map(|byte| {
                u8::from_str_radix(byte, 16).map_err(|e| MacParseError::ParseByte {
                    source: e,
                    byte: byte.into(),
                })
            })
            .enumerate()
            .try_fold([0; 6], |mut acc, (i, byte)| {
                acc[i] = byte?;
                Ok(acc)
            })?;
        Ok(HardwareAddress { octets: value })
    }
}

impl TryFrom<&str> for HardwareAddress {
    type Error = MacParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<[u8; 6]> for HardwareAddress {
    fn from(octets: [u8; 6]) -> Self {
        HardwareAddress { octets }
    }
}

impl From<HardwareAddress> for String {
    fn from(addr: HardwareAddress) -> Self {
        addr.to_string()
    }
}

impl fmt::Display for HardwareAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.octets[0],
            self.octets[1],
            self.octets[2],
            self.octets[3],
            self.octets[4],
            self.octets[5]
        )
    }
}

impl fmt::Debug for HardwareAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_dashes() {
        let addr: Result<HardwareAddress, _> = "00-11-22-33-44-55".parse();
        match addr {
            Ok(addr) => assert_eq!(
                addr,
                HardwareAddress::new(0x0, 0x11, 0x22, 0x33, 0x44, 0x55)
            ),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn from_str_colons() {
        let addr: Result<HardwareAddress, _> = "00:11:22:33:44:55".parse();
        match addr {
            Ok(addr) => assert_eq!(
                addr,
                HardwareAddress::new(0x0, 0x11, 0x22, 0x33, 0x44, 0x55)
            ),
            Err(e) => panic!("didn't expect an error: {}", e),
        }
    }

    #[test]
    fn from_str_too_short() {
        let addr: Result<HardwareAddress, _> = "00:11:22:33:44".parse();
        match addr {
            Err(MacParseError::TooShort(e)) => assert_eq!(e, 5),
            _ => panic!("should have gotten a TooShort error"),
        }
    }

    #[test]
    fn from_str_too_long() {
        let addr: Result<HardwareAddress, _> = "00:11:22:33:44:55:66".parse();
        match addr {
            Err(MacParseError::TooLong(e)) => assert_eq!(e, 7),
            _ => panic!("should have gotten a TooLong error"),
        }
    }

    #[test]
    fn from_str_byte_parse() {
        let addr: Result<HardwareAddress, _> = "00:11:22:g1:44:55".parse();
        match addr {
            Err(MacParseError::ParseByte { byte, source: _ }) => assert_eq!(byte, "g1"),
            _ => panic!("expected ParseByte error"),
        }
    }
}
