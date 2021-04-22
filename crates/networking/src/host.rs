use crate::error::Error;
use std::{fmt, net::Ipv4Addr, str::FromStr};

#[cfg(feature = "deserialize")]
use serde::Deserialize;

#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "deserialize", serde(from = "&str"))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(into = "String"))]
pub enum Host {
    Ip(Ipv4Addr),
    Domain(String),
}

impl FromStr for Host {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = Ipv4Addr::from_str(s).map(|ip| ip.into()) {
            Ok(i)
        } else {
            Ok(s.to_string().into())
        }
    }
}

impl From<&str> for Host {
    fn from(s: &str) -> Self {
        s.parse()
            .expect("host should never fail when converting from &str")
    }
}

impl From<String> for Host {
    fn from(s: String) -> Self {
        Host::Domain(s)
    }
}

impl From<Ipv4Addr> for Host {
    fn from(ip: Ipv4Addr) -> Self {
        Host::Ip(ip)
    }
}

impl From<Host> for String {
    fn from(h: Host) -> Self {
        h.to_string()
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Host::Ip(ip) => write!(f, "{}", ip),
            Host::Domain(d) => write!(f, "{}", d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ip() {
        let h: Result<Host, _> = "1.2.3.4".parse();
        match h {
            Ok(Host::Ip(ip)) => assert_eq!(ip, Ipv4Addr::new(1, 2, 3, 4)),
            _ => panic!("expected an IP type"),
        }
    }

    #[test]
    fn from_str_domain() {
        let h: Result<Host, _> = "mine.local".parse();
        match h {
            Ok(Host::Domain(d)) => assert_eq!(d, "mine.local"),
            _ => panic!("expected an Domain type"),
        }
    }
}
