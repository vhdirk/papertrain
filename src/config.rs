use crate::wifi::WifiConfig;
use crate::irail::IRailConfig;

#[derive(Debug, Clone)]
pub struct Connection {
    pub from: &'static str,
    pub to: &'static str,
}

#[derive(Debug, Clone)]
pub struct Config<const N: usize> {
    pub wifi: WifiConfig,
    pub irail: IRailConfig,
    pub connections: [Connection; N]
}

