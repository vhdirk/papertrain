// use std::{str::FromStr, fmt::Display};

use serde::{self, Deserialize};

use core::{str::FromStr, fmt::Display};

use alloc::{string::String, vec::Vec};

pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    pub version: String,
    #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
    pub timestamp: u32,
    // pub connection: Vec<Connection>,
}

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Connection {
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub id: u32,
//     pub departure: Departure,
//     pub arrival: Arrival,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub duration: u32,
//     pub alerts: Option<Alerts>,
//     pub vias: Option<Vias>,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Departure {
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub delay: u32,
//     pub station: Option<String>,
//     #[serde(rename(deserialize = "stationinfo"))]
//     pub station_info: StationInfo,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub time: u32,
//     pub vehicle: String,
//     #[serde(rename(deserialize = "vehicleinfo"))]
//     pub vehicle_info: VehicleInfo,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub platform: u32,
//     #[serde(rename(deserialize = "platforminfo"))]
//     pub platform_info: PlatformInfo,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub left: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub canceled: u32,
//     pub direction: Direction,
//     pub stops: Option<Stops>,
//     pub alerts: Option<Alerts>,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub walking: u32,
//     pub departure_connection: String,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct StationInfo {
//     pub id: String,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub location_x: f32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub location_y: f32,
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
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub number: u32,
//     pub stop: Vec<Stop>,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Stop {
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub id: u32,
//     pub station: Option<String>,
//     #[serde(rename(deserialize = "stationinfo"))]
//     pub station_info: StationInfo,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub time: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub delay: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub canceled: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub departure_delay: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub departure_canceled: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub scheduled_departure_time: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub arrival_delay: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub arrival_canceled: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub is_extra_stop: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub scheduled_arrival_time: u32,
//     pub departure_connection: String,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Alerts {
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub number: u32,
//     pub alert: Vec<Alert>,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Alert {
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub id: u32,
//     pub header: String,
//     pub lead: String,
//     pub link: String,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub start_time: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub end_time: u32,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Arrival {
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub delay: u32,
//     pub station: Option<String>,
//     #[serde(rename(deserialize = "stationinfo"))]
//     pub station_info: StationInfo,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub time: u32,
//     pub vehicle: String,
//     #[serde(rename(deserialize = "vehicleinfo"))]
//     pub vehicle_info: VehicleInfo,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub platform: u32,
//     #[serde(rename(deserialize = "platforminfo"))]
//     pub platform_info: PlatformInfo,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub arrived: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub canceled: u32,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub walking: u32,
//     pub direction: Direction,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Vias {
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub number: u32,
//     pub via: Vec<Vum>,
// }

// #[derive(Default, Debug, Clone, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Vum {
//     pub id: String,
//     pub arrival: Arrival,
//     pub departure: Departure,
//     #[serde(deserialize_with = "crate::irail::responses::deserialize_number_from_string")]
//     pub time_between: u32,
//     pub station: Option<String>,
//     #[serde(rename(deserialize = "stationinfo"))]
//     pub station_info: StationInfo,
//     pub vehicle: String,
//     pub direction: Direction,
// }
