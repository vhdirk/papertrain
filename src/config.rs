use crate::irail::IRailConfig;
use core::option::Option;
// use crate::wifi::WifiConfig;

use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};


pub struct WifiConfig {
    pub ssid: &'static str,
    pub password: &'static str,
    pub auth_method: AuthMethod,
    pub channel: Option<u8>
}


// [derive(Debug, Clone)]
pub struct Connection {
    pub from: &'static str,
    pub to: &'static str,
}

// [derive(Debug, Clone)]
pub struct AppConfig<const N: usize> {
    pub wifi: WifiConfig,
    pub irail: IRailConfig,
    pub connections: [Connection; N],
}
