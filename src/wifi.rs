use log::*;

use esp_backtrace as _;
use esp_println as _;

use embassy_time::{Duration, Timer};

use embedded_svc::wifi::{Wifi, AuthMethod, ClientConfiguration, Configuration};
use esp_wifi::wifi::{WifiController, WifiEvent, WifiState};

// #[derive(Debug, Clone)]
pub struct WifiConfig {
    pub ssid: &'static str,
    pub password: &'static str,
    pub auth_method: AuthMethod,
    pub channel: Option<u8>,
}

pub async fn connection(controller: &mut WifiController<'static>, config: &'static WifiConfig) {
    info!("start connection task");
    // info!("Device capabilities: {:?}", controller.get_capabilities());
    match esp_wifi::wifi::get_wifi_state() {
        WifiState::StaConnected => {
            // wait until we're no longer connected
            controller.wait_for_event(WifiEvent::StaDisconnected).await;
            Timer::after(Duration::from_millis(5000)).await
        }
        _ => {}
    }
    if !matches!(controller.is_started(), Ok(true)) {
        let client_config = Configuration::Client(ClientConfiguration {
            ssid: config.ssid.into(),
            password: config.password.into(),
            auth_method: config.auth_method,
            channel: config.channel,
            ..Default::default()
        });
        controller.set_configuration(&client_config).unwrap();
        info!("Starting wifi");
        controller.start().await.unwrap();
        info!("Wifi started!");
    }
    info!("About to connect...");

    match controller.connect().await {
        Ok(_) => info!("Wifi connected!"),
        Err(e) => {
            info!("Failed to connect to wifi: {:?}", e);
            Timer::after(Duration::from_millis(5000)).await
        }
    }
}

// pub fn wifi(
//     config: &WifiConfig,
//     modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
//     sysloop: EspSystemEventLoop,
//     _storage: EspDefaultNvsPartition,
// ) -> Result<BlockingWifi<EspWifi<'static>>> {
//     log::info!("connecting to wifi");

//     if config.ssid.is_empty() {
//         bail!("Missing WiFi SSID")
//     }

//     let esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
//     let mut wifi = BlockingWifi::wrap(esp_wifi, sysloop).expect("failed to create blocking wifi");

//     wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

//     info!("Starting wifi...");

//     wifi.start()?;

//     let channel: Option<u8> = if let Some(channel) = config.channel {
//         Some(channel)
//     } else {
//         info!("Scanning...");

//         let ap_infos = wifi.scan()?;

//         let ours = ap_infos.into_iter().find(|a| a.ssid == config.ssid);

//         if let Some(ours) = ours {
//             info!(
//                 "Found configured access point '{}' on channel {}",
//                 config.ssid, ours.channel
//             );
//             Some(ours.channel)
//         } else {
//             info!(
//                 "Configured access point '{}' not found during scanning, will go with unknown channel",
//                 config.ssid
//             );
//             None
//         }
//     };

//     wifi.set_configuration(&Configuration::Client(ClientConfiguration {
//         ssid: config.ssid.into(),
//         password: config.password.into(),
//         auth_method: config.auth_method,
//         channel,
//         ..Default::default()
//     }))?;

//     info!("Connecting wifi...");

//     wifi.connect()?;

//     info!("Waiting for DHCP lease...");

//     wifi.wait_netif_up()?;

//     let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

//     info!("Wifi DHCP info: '{:?}'", ip_info);

//     Ok(wifi)
// }
