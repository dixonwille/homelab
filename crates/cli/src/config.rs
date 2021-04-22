use homelab_networking::{HardwareAddress, Host, wol::Password};
use serde::{
    de::{self, Deserialize, MapAccess, Visitor},
    Deserialize as DDeserialize,
};
use std::{collections::HashMap, fmt, net::Ipv4Addr};

#[derive(Debug)]
pub struct Config {
    networks: Option<HashMap<String, Network>>,
    servers: HashMap<String, Server>,
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(DDeserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "network")]
            Networks,
            #[serde(rename = "server")]
            Servers,
        }

        struct ConfigVisitor;

        impl<'de> Visitor<'de> for ConfigVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Config")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut networks: Option<HashMap<String, Network>> = None;
                let mut servers: Option<HashMap<String, Server>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Networks => {
                            let network: Vec<Network> = map.next_value()?;
                            match networks.as_mut() {
                                Some(m) => m.reserve(network.len()),
                                None => networks = Some(HashMap::with_capacity(network.len())),
                            }
                            let mut nets = networks.unwrap();
                            for net in network {
                                if nets.insert(net.name.clone(), net).is_some() {
                                    return Err(de::Error::duplicate_field("network name"));
                                }
                            }
                            networks = Some(nets);
                        }
                        Field::Servers => {
                            let server: Vec<Server> = map.next_value()?;
                            match servers.as_mut() {
                                Some(m) => m.reserve(server.len()),
                                None => servers = Some(HashMap::with_capacity(server.len())),
                            }
                            let mut srvs = servers.unwrap();
                            for srv in server {
                                if srvs.insert(srv.name.clone(), srv).is_some() {
                                    return Err(de::Error::duplicate_field("server name"));
                                }
                            }
                            servers = Some(srvs);
                        }
                    }
                }

                let servers = servers.ok_or_else(|| de::Error::missing_field("server"))?;
                Ok(Config { networks, servers })
            }
        }

        const FIELDS: &[&str] = &["network", "server"];
        deserializer.deserialize_struct("Config", FIELDS, ConfigVisitor)
    }
}

#[derive(Debug, DDeserialize)]
struct Network {
    name: String,
    dns: Ipv4Addr,
    domains: Option<Vec<String>>,
    wol: Option<NetworkWol>,
}

#[derive(Debug, DDeserialize)]
struct NetworkWol {
    broadcast: Ipv4Addr,
}

#[derive(Debug, DDeserialize)]
struct Server {
    name: String,
    host: Host,
    mac: HardwareAddress,
    network: Option<String>,
    wol: Option<ServerWol>,
}

#[derive(Debug, DDeserialize)]
struct ServerWol {
    broadcast: Option<Ipv4Addr>,
    password: Option<Password>,
}
