use rand;
use rand::Rng;
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Settings {
    pub comit: Comit,
    pub network: Network,
    pub http_api: HttpSocket,
    pub logging: Logging,
    pub bitcoin: Bitcoin,
    pub ethereum: Ethereum,
}

#[derive(Clone, Debug, Serialize)]
pub struct Comit {
    #[serde(with = "hex_serde")]
    pub secret_seed: [u8; 32],
}

#[derive(Clone, Debug, Serialize)]
pub struct Network {
    pub listen: Vec<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct HttpSocket {
    pub address: IpAddr,
    pub port: u16,
}

#[derive(Clone, Debug, Serialize)]
pub struct Logging {
    pub level: String,
    pub structured: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Bitcoin {
    pub network: String,
    pub node_url: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Ethereum {
    pub network: String,
    pub node_url: String,
}

impl Default for Comit {
    fn default() -> Comit {
        let mut secret_seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret_seed);

        Comit { secret_seed }
    }
}

impl Default for Network {
    fn default() -> Network {
        Network {
            listen: vec!["/ip4/0.0.0.0/tcp/9939".into()],
        }
    }
}

impl Default for HttpSocket {
    fn default() -> HttpSocket {
        HttpSocket {
            address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 8080,
        }
    }
}

impl Default for Logging {
    fn default() -> Self {
        Logging {
            level: "DEBUG".to_string(),
            structured: false,
        }
    }
}

impl Default for Bitcoin {
    fn default() -> Self {
        Bitcoin {
            network: "regtest".to_string(),
            node_url: "http://localhost:18443".to_string(),
        }
    }
}

impl Default for Ethereum {
    fn default() -> Self {
        Ethereum {
            network: "regtest".to_string(),
            node_url: "http://localhost:8545".to_string(),
        }
    }
}
