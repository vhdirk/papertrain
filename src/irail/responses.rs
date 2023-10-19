use serde;

#[derive(Default, Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    pub version: String,
    // pub timestamp: u32,
    // pub connection: Vec<Connection>,
}

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Connection {
//     pub id: u32,
//     pub departure: Departure,
//     pub arrival: Arrival,
//     pub duration: u32,
//     pub alerts: Alerts,
//     pub vias: Vias,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Departure {
//     pub delay: u32,
//     pub station: String,
//     pub station_info: StationInfo,
//     pub time: u32,
//     pub vehicle: String,
//     pub vehicle_info: VehicleInfo,
//     pub platform: u32,
//     pub platform_info: PlatformInfo,
//     pub left: u32,
//     pub canceled: u32,
//     pub direction: Direction,
//     pub stops: Stops,
//     pub alerts: Alerts,
//     pub walking: u32,
//     pub departure_connection: String,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct StationInfo {
//     pub id: String,
//     pub location_x: f64,
//     pub location_y: f64,
//     pub standardname: String,
//     pub name: String,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct VehicleInfo {
//     pub name: String,
//     pub shortname: String,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PlatformInfo {
//     pub name: String,
//     pub normal: String,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Direction {
//     pub name: String,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Stops {
//     pub number: u32,
//     pub stop: Vec<Stop>,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Stop {
//     pub id: u32,
//     pub station: String,
//     pub station_info: StationInfo,
//     pub time: u32,
//     pub delay: u32,
//     pub canceled: u32,
//     pub departure_delay: u32,
//     pub departure_canceled: u32,
//     pub scheduled_departure_time: u32,
//     pub arrival_delay: u32,
//     pub arrival_canceled: u32,
//     pub is_extra_stop: u32,
//     pub scheduled_arrival_time: u32,
//     pub departure_connection: String,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Alerts {
//     pub number: u32,
//     pub alert: Vec<Alert>,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Alert {
//     pub id: u32,
//     pub header: String,
//     pub lead: String,
//     pub link: String,
//     pub start_time: u32,
//     pub end_time: u32,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Arrival {
//     pub delay: u32,
//     pub station: String,
//     pub station_info: StationInfo,
//     pub time: u32,
//     pub vehicle: String,
//     pub vehicle_info: VehicleInfo,
//     pub platform: u32,
//     pub platform_info: PlatformInfo,
//     pub arrived: u32,
//     pub canceled: u32,
//     pub walking: u32,
//     pub direction: Direction,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Vias {
//     pub number: u32,
//     pub via: Vec<Vum>,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Vum {
//     pub id: String,
//     pub arrival: Arrival,
//     pub departure: Departure,
//     pub time_between: u32,
//     pub station: String,
//     pub station_info: StationInfo,
//     pub vehicle: String,
//     pub direction: Direction,
// }
