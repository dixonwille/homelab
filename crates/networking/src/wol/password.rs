use crate::error::Error;
use crate::hardware_address::HardwareAddress;
use std::{convert::TryFrom, fmt, net::Ipv4Addr, str::FromStr};

#[cfg(feature = "deserialize")]
use serde::Deserialize;

#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "deserialize", serde(try_from = "&str"))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(into = "String"))]
pub enum Password {
    Four(Ipv4Addr),
    Six(HardwareAddress),
}

impl FromStr for Password {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Ipv4Addr::from_str(s)
            .map(|ip| ip.into())
            .ok()
            .or_else(|| HardwareAddress::from_str(s).map(|hw| hw.into()).ok())
        {
            Some(pass) => Ok(pass),
            None => Err(Error::WolPasswordParse),
        }
    }
}

impl TryFrom<&str> for Password {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<Ipv4Addr> for Password {
    fn from(ip: Ipv4Addr) -> Self {
        Password::Four(ip)
    }
}

impl From<HardwareAddress> for Password {
    fn from(hw: HardwareAddress) -> Self {
        Password::Six(hw)
    }
}

impl From<Password> for String {
    fn from(pass: Password) -> Self {
        pass.to_string()
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Password::Four(ip) => write!(f, "{}", ip),
            Password::Six(hw) => write!(f, "{}", hw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_four() {
        let pass: Result<Password, _> = "1.2.3.4".parse();
        match pass {
            Ok(Password::Four(ip)) => {
                assert_eq!(ip, Ipv4Addr::new(1, 2, 3, 4))
            }
            Ok(_) => panic!("expected to be of type Four"),
            Err(e) => panic!("didn't expect an error: {}", e),
        }
    }

    #[test]
    fn from_str_six() {
        let pass: Result<Password, _> = "00:11:22:33:44:55".parse();
        match pass {
            Ok(Password::Six(hw)) => {
                assert_eq!(hw, HardwareAddress::new(0x0, 0x11, 0x22, 0x33, 0x44, 0x55))
            }
            Ok(_) => panic!("expected to be of type Six"),
            Err(e) => panic!("didn't expect an error: {}", e),
        }
    }

    #[test]
    fn from_str_error() {
        let pass: Result<Password, _> = "NotAPassword".parse();
        match pass {
            Err(Error::WolPasswordParse) => {}
            _ => panic!("expected WOLPasswordParse error"),
        }
    }
}
