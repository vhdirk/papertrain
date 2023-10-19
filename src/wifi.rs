use anyhow::{bail, Result};
use embedded_svc::wifi::{
    AuthMethod, ClientConfiguration, Configuration,
};
use esp_idf_hal::peripheral;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{EspWifi, BlockingWifi},
};
use log::info;


#[derive(Debug, Clone)]
pub struct WifiConfig {
    pub ssid: &'static str,
    pub password: &'static str,
    pub auth_method: AuthMethod,
}

pub fn wifi(
    config: &WifiConfig,
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
    _storage: EspDefaultNvsPartition,
) -> Result<BlockingWifi<EspWifi<'static>>> {
    log::info!("connecting to wifi");

    if config.ssid.is_empty() {
        bail!("Missing WiFi SSID")
    }

    let esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(esp_wifi, sysloop).expect("failed to create blocking wifi");

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    info!("Starting wifi...");

    wifi.start()?;

    info!("Scanning...");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == config.ssid);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            config.ssid, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            config.ssid
        );
        None
    };

    wifi.set_configuration(&Configuration::Client(
        ClientConfiguration {
            ssid: config.ssid.into(),
            password: config.password.into(),
            auth_method: AuthMethod::WPA2WPA3Personal,//config.auth,
            channel,
            ..Default::default()
        },
    ))?;

    info!("Connecting wifi...");

    wifi.connect()?;

    info!("Waiting for DHCP lease...");

    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(wifi)
}
