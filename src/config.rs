use crate::irail::IRailConfig;
use crate::wifi::WifiConfig;

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
