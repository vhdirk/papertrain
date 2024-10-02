use defmt::*;

use esp_backtrace as _;
use esp_println as _;

use embassy_time::{Duration, Timer};
use esp_wifi::wifi::{ClientConfiguration, AuthMethod, Configuration, WifiController, WifiError, WifiEvent, WifiState};

// #[derive(Debug, Clone)]
pub struct WifiConfig {
    pub ssid: &'static str,
    pub password: &'static str,
    pub auth_method: AuthMethod,
    pub channel: Option<u8>,
}

pub async fn connection(controller: &mut WifiController<'static>, config: &'static WifiConfig) -> Result<(), WifiError> {
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
            ssid: heaples::String::<32>::new(config.ssid),
            password: heaples::String::<64>::new(config.password),
            auth_method: config.auth_method,
            channel: config.channel,
            ..Default::default()
        });
        if let Err(err) = controller.set_configuration(&client_config) {
            warn!("Error setting wifi config {:?}", err);
            Timer::after(Duration::from_millis(5000)).await;
            return Err(err);
        }
        info!("Starting wifi");
        controller.start().await.unwrap();
        info!("Wifi started!");
    }
    info!("About to connect...");

    match controller.connect().await {
        Ok(_) => info!("Wifi connected!"),
        Err(e) => {
            info!("Failed to connect to wifi '{}': {:?}", config.ssid, e);
            Timer::after(Duration::from_millis(5000)).await
        }
    }
    Ok(())
}