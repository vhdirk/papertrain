use chrono::{DateTime, Duration, Utc};
use defmt::*;
use serde::{self, de, Deserialize};
use serde_with::{serde_as, DisplayFromStr, DurationSeconds, TimestampSeconds};

use alloc::{string::String, vec::Vec};

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "1" | "true" | "yes" => Ok(true),
        "0" | "false" | "no" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["1", "0"])),
    }
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    pub version: String,

    #[defmt(Debug2Format)]
    #[serde_as(as = "TimestampSeconds<String>")]
    pub timestamp: DateTime<Utc>,

    #[defmt(Debug2Format)]
    #[serde(rename(deserialize = "connection"))]
    pub connections: Vec<Connection>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u32,
    pub departure: Departure,
    pub arrival: Arrival,

    #[defmt(Debug2Format)]
    #[serde_as(as = "DurationSeconds<String>")]
    pub duration: Duration,
    // pub alerts: Option<Alerts>,
    // pub vias: Option<Vias>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    #[defmt(Debug2Format)]
    #[serde_as(as = "Option<DurationSeconds<String>>")]
    pub delay: Option<Duration>,

    // pub station: Option<String>,

    #[serde(rename(deserialize = "stationinfo"))]
    pub station_info: Option<StationInfo>,

    #[defmt(Debug2Format)]
    #[serde_as(as = "TimestampSeconds<String>")]
    pub time: DateTime<Utc>,

    // pub vehicle: String,
    // #[serde(rename(deserialize = "vehicleinfo"))]
    // pub vehicle_info: VehicleInfo,

    pub platform: String,
    // #[serde(rename(deserialize = "platforminfo"))]
    // pub platform_info: PlatformInfo,

    #[serde(deserialize_with = "deserialize_bool")]
    pub left: bool,

    #[serde(deserialize_with = "deserialize_bool", default = "bool::default")]
    pub canceled: bool,

    // pub direction: Direction,
    pub stops: Option<Stops>,
    // pub alerts: Option<Alerts>,

    // #[defmt(Debug2Format)]
    // #[serde_as(as = "DurationSeconds<String>")]
    // pub walking: Duration,

    // pub departure_connection: Option<String>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationInfo {
    // pub id: String,
    // #[serde_as(as = "DisplayFromStr")]
    // pub location_x: f32,
    // #[serde_as(as = "DisplayFromStr")]
    // pub location_y: f32,
    pub standardname: String,
    // pub name: String,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleInfo {
    pub name: String,
    pub shortname: String,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    pub name: String,

    #[serde(deserialize_with = "deserialize_bool", default = "bool::default")]
    pub normal: bool,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Direction {
    pub name: String,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stops {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u8,

    #[serde(rename(deserialize = "stop"))]
    pub stops: Vec<Stop>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    // #[serde_as(as = "DisplayFromStr")]
    // pub id: u32,
    // pub station: Option<String>,
    #[serde(rename(deserialize = "stationinfo"))]
    pub station_info: Option<StationInfo>,

    #[defmt(Debug2Format)]
    #[serde_as(as = "Option<DurationSeconds<String>>")]
    pub delay: Option<Duration>,

    #[serde(deserialize_with = "deserialize_bool", default = "bool::default")]
    pub canceled: bool,

    #[defmt(Debug2Format)]
    #[serde_as(as = "Option<DurationSeconds<String>>")]
    pub departure_delay: Option<Duration>,

    #[serde(deserialize_with = "deserialize_bool", default = "bool::default")]
    pub departure_canceled: bool,

    #[defmt(Debug2Format)]
    #[serde_as(as = "TimestampSeconds<String>")]
    pub scheduled_departure_time: DateTime<Utc>,

    // #[defmt(Debug2Format)]
    // #[serde_as(as = "DurationSeconds<String>")]
    // pub arrival_delay: Duration,

    // #[serde(deserialize_with = "deserialize_bool", default = "bool::default")]
    // pub arrival_canceled: bool,
    // #[serde(deserialize_with = "deserialize_bool", default = "bool::default")]
    // pub is_extra_stop: bool,

    #[defmt(Debug2Format)]
    #[serde_as(as = "TimestampSeconds<String>")]
    pub scheduled_arrival_time: DateTime<Utc>,

    pub platform: String,
    #[serde(rename(deserialize = "platforminfo"))]
    pub platform_info: PlatformInfo,

    // pub departure_connection: Option<String>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alerts {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u8,
    pub alert: Vec<Alert>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u32,

    pub header: String,
    pub lead: String,
    pub link: String,

    #[defmt(Debug2Format)]
    #[serde_as(as = "TimestampSeconds<String>")]
    pub start_time: DateTime<Utc>,

    #[defmt(Debug2Format)]
    #[serde_as(as = "TimestampSeconds<String>")]
    pub end_time: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrival {
    #[defmt(Debug2Format)]
    #[serde_as(as = "Option<DurationSeconds<String>>")]
    pub delay: Option<Duration>,

    // pub station: Option<String>,
    #[serde(rename(deserialize = "stationinfo"))]
    pub station_info: Option<StationInfo>,

    #[defmt(Debug2Format)]
    #[serde_as(as = "TimestampSeconds<String>")]
    pub time: DateTime<Utc>,

    // pub vehicle: String,
    // #[serde(rename(deserialize = "vehicleinfo"))]
    // pub vehicle_info: VehicleInfo,
    pub platform: String,
    // #[serde(rename(deserialize = "platforminfo"))]
    // pub platform_info: PlatformInfo,
    #[serde(deserialize_with = "deserialize_bool")]
    pub arrived: bool,
    #[serde(deserialize_with = "deserialize_bool", default = "bool::default")]
    pub canceled: bool,
    // #[serde(deserialize_with = "deserialize_bool")]
    // pub walking: bool,
    // pub direction: Direction,

    // pub alerts: Option<Alerts>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vias {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u8,
    pub via: Vec<Vum>,
}

#[serde_as]
#[derive(Debug, Format, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vum {
    pub id: String,
    pub arrival: Arrival,
    pub departure: Departure,

    #[defmt(Debug2Format)]
    #[serde_as(as = "DurationSeconds<String>")]
    pub time_between: Duration,

    pub station: Option<String>,
    #[serde(rename(deserialize = "stationinfo"))]
    pub station_info: Option<StationInfo>,
    pub vehicle: String,
    pub direction: Direction,
}
