use crate::irail::IRailConfig;
use crate::wifi::WifiConfig;


// [derive(Debug, Clone)]
pub struct AppConfig<const N: usize> {
    pub wifi: WifiConfig,
    pub irail: IRailConfig,
    pub connection: [&'static str; N],
}
