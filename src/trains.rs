use chrono::{DateTime, Duration, Utc};
use alloc::{vec::Vec, string::{String, ToString}};

use crate::{
    irail::{Arrival, Connection, Connections, Departure, StationInfo},
    merge_relative_order::MergeAndMaintainRelativeOrder,
};

pub struct TrainSchedule {
    pub timestamp: DateTime<Utc>,
    pub stations: Vec<String>,
    pub connections: Vec<TrainConnection>,
}

impl TrainSchedule {
    pub fn get_stations(
        connections: &Vec<TrainConnectionInterim>,
        config_stations: &[&'static str],
    ) -> Vec<String> {
        let c_stations = config_stations.iter().map(ToString::to_string).collect();
        if config_stations.len() > 2 {
            return c_stations;
        }

        let conn_stops: Vec<Vec<String>> = connections
            .iter()
            .map(|c| c.stops.iter().map(|s| s.station.clone()).collect())
            .collect();

        conn_stops.merge_and_maintain_relative_order()
    }

    pub fn from_connections(
        connections: Connections,
        config_stations: &[&'static str],
        limit: Option<u8>,
    ) -> Self {
        let sub_stations = if config_stations.len() > 1 {
            &config_stations[1..config_stations.len() - 1]
        } else {
            &config_stations
        };

        let mut conns: Vec<TrainConnectionInterim> = connections
            .connections
            .iter()
            .map(|c| TrainConnectionInterim::from_connection(c, sub_stations))
            .collect();

        if conns.len() < 2 {
            return Self {
                timestamp: connections.timestamp,
                connections: vec![],
                stations: vec![],
            };
        }

        // first, list all places we have to display
        let stations = TrainSchedule::get_stations(&conns, config_stations);

        // now, make sure every connection has these stops:
        let mut new_conns: Vec<TrainConnection> = conns
            .iter_mut()
            .map(|conn| {
                if conn.stops.len() != stations.len() {
                    for (index, station) in stations.iter().enumerate() {
                        if !conn.stops[index].station.eq(station) {
                            conn.stops.insert(
                                index,
                                TrainStop {
                                    station: station.clone(),
                                    info: None,
                                },
                            )
                        }
                    }
                }
                TrainConnection {
                    duration: conn.duration,
                    stops: conn.stops.iter().map(|s| s.info.clone()).collect(),
                    canceled: conn.canceled
                }
            })
            .collect();

        new_conns.truncate(limit.unwrap_or(new_conns.len() as u8) as usize);

        Self {
            timestamp: connections.timestamp,
            connections: new_conns,
            stations,
        }
    }
}

pub struct TrainConnection {
    pub duration: Duration,
    pub stops: Vec<Option<TrainStopInfo>>,
    pub canceled: bool,
}
pub struct TrainConnectionInterim {
    pub duration: Duration,
    pub stops: Vec<TrainStop>,
    pub canceled: bool,
}

#[derive(Clone)]
pub struct TrainStopInfo {
    pub time: DateTime<Utc>,
    pub delay: Duration,
    pub platform: String,
    pub canceled: bool,
}

pub struct TrainStop {
    pub station: String,
    pub info: Option<TrainStopInfo>,
}

impl TrainConnectionInterim {
    fn get_station_name(info: &Option<StationInfo>) -> String {
        info.as_ref()
            .map_or("".to_string(), |f| f.standardname.clone())
    }

    fn get_duration(delay: &Option<Duration>) -> Duration {
        delay.unwrap_or(Duration::zero())
    }

    fn departure_to_stop(departure: &Departure) -> TrainStop {
        TrainStop {
            station: TrainConnectionInterim::get_station_name(&departure.station_info),
            info: Some(TrainStopInfo {
                time: departure.time.clone(),
                delay: TrainConnectionInterim::get_duration(&departure.delay),
                platform: departure.platform.clone(),
                canceled: false,
            }),
        }
    }

    fn arrival_to_stop(arrival: &Arrival) -> TrainStop {
        TrainStop {
            station: TrainConnectionInterim::get_station_name(&arrival.station_info),
            info: Some(TrainStopInfo {
                time: arrival.time.clone(),
                delay: TrainConnectionInterim::get_duration(&arrival.delay),
                platform: arrival.platform.clone(),
                canceled: false,
            }),
        }
    }

    fn stops_to_stops(connection: &Connection, stations_of_interest: &[&str]) -> Vec<TrainStop> {
        let mut train_stops = Vec::new();

        if let Some(stops) = &connection.departure.stops {
            for stop in stops.stops.iter() {
                let station_name = TrainConnectionInterim::get_station_name(&stop.station_info);
                if stations_of_interest.len() > 0
                    && stations_of_interest
                        .iter()
                        .find(|s| **s == station_name)
                        .is_none()
                {
                    continue;
                }

                train_stops.push(TrainStop {
                    station: TrainConnectionInterim::get_station_name(&stop.station_info),
                    info: Some(TrainStopInfo {
                        time: stop.scheduled_departure_time.clone(),
                        delay: TrainConnectionInterim::get_duration(&stop.departure_delay),
                        canceled: stop.departure_canceled,
                        platform: stop.platform.clone(),
                    }),
                })
            }
        }

        train_stops
    }

    pub fn from_connection(connection: &Connection, stations_of_interest: &[&str]) -> Self {
        let mut train_stops = Vec::new();

        train_stops.push(TrainConnectionInterim::departure_to_stop(
            &connection.departure,
        ));
        train_stops.append(&mut TrainConnectionInterim::stops_to_stops(
            connection,
            stations_of_interest,
        ));
        train_stops.push(TrainConnectionInterim::arrival_to_stop(&connection.arrival));

        Self {
            duration: connection.duration,
            stops: train_stops,
            canceled: connection.departure.canceled,
        }
    }
}
