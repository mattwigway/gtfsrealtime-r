// code to create various GTFS-realtime datasets for use in tests

use bytes::BytesMut;
use extendr_api::extendr_module;
use prost::Message;
use std::{fs, string::ToString};
use extendr_api::prelude::*;

use crate::transit_realtime::{self, Alert, FeedEntity, FeedHeader, Position, TripDescriptor, TripUpdate, VehicleDescriptor, VehiclePosition, feed_header::Incrementality, trip_descriptor::{self, ModifiedTripSelector, ScheduleRelationship}, vehicle_descriptor, vehicle_position::{OccupancyStatus, VehicleStopStatus}};

fn write_msg(filename: &str, positions: Vec<VehiclePosition>, alerts: Vec<Alert>, trip_updates: Vec<TripUpdate>) -> Result<()> {
    let mut pos_id = 0;
    let mut alert_id = 0;
    let mut upd_id = 0;

    let mut msg= transit_realtime::FeedMessage {
        header: FeedHeader {
            gtfs_realtime_version: "2.0".to_owned(),
            incrementality: Some(Incrementality::FullDataset as i32),
            timestamp: Some(1774967578),
            feed_version: None,
        },
        entity: positions.into_iter().map(|x| {
            pos_id += 1;
            FeedEntity {
                    id: pos_id.to_string(),
                    is_deleted: None,
                    vehicle: Some(x),
                    trip_update: None,
                    alert: None,
                    shape: None,
                    stop: None,
                    trip_modifications: None
            }
            }).chain(
                alerts.into_iter().map(|x| {
                    alert_id += 1;
                    FeedEntity {
                        id: alert_id.to_string(),
                        is_deleted: None,
                        vehicle: None,
                        trip_update: None,
                        alert: Some(x),
                        shape: None,
                        stop: None,
                        trip_modifications: None
                    }
                })
            ).chain(
                trip_updates.into_iter().map(|x| {
                    upd_id += 1;
                    FeedEntity {
                        id: upd_id.to_string(),
                        is_deleted: None,
                        vehicle: None,
                        trip_update: Some(x),
                        alert: None,
                        shape: None,
                        stop: None,
                        trip_modifications: None
                    }
                })
            ).collect()
    };

    let mut bytes = BytesMut::new();
    msg.encode(&mut bytes).or(Err(Error::Other("encoding error".to_string())))?;
    fs::write(filename, &bytes).or(Err(Error::Other("fs error".to_string())))?;
    Ok(())
}

fn write_positions(filename: &str, positions: Vec<VehiclePosition>) -> Result<()> {
    write_msg(filename, positions, vec![], vec![])
}

// A vehicle positions feed with invalid enum values
#[extendr]
pub fn test_data_invalid_enum_positions(filename: &str) -> Result<()> {
    write_positions(filename, vec![
        VehiclePosition {
                    trip: Some(TripDescriptor {
                        trip_id: Some("8675309".to_owned()),
                        route_id: Some("16".to_owned()),
                        direction_id: Some(0),
                        start_time: Some("06:00:00".to_owned()),
                        start_date: Some("20260331".to_owned()),
                        schedule_relationship: Some(256), // invalid value
                        modified_trip: None
                    }),
                    vehicle: Some(VehicleDescriptor { id: None, label: None, license_plate: None, wheelchair_accessible: None }),
                    position: Some(Position { latitude: 37.363f32, longitude: -122.123f32, bearing: None, odometer: None, speed: None }),
                    current_stop_sequence: Some(5),
                    stop_id: Some("52".to_owned()),
                    current_status: Some(VehicleStopStatus::InTransitTo as i32),
                    timestamp: Some(1774967570),
                    congestion_level: None,
                    occupancy_status: Some(OccupancyStatus::StandingRoomOnly as i32),
                    occupancy_percentage: None,
                    multi_carriage_details: vec![] // not supported/todo
        },
                VehiclePosition {
                    trip: Some(TripDescriptor {
                        trip_id: Some("8675310".to_owned()),
                        route_id: Some("16".to_owned()),
                        direction_id: Some(0),
                        start_time: Some("06:05:00".to_owned()),
                        start_date: Some("20260331".to_owned()),
                        schedule_relationship: Some(trip_descriptor::ScheduleRelationship::Scheduled as i32), // valid value
                        modified_trip: None
                    }),
                    vehicle: Some(VehicleDescriptor { id: None, label: None, license_plate: None, wheelchair_accessible: None }),
                    position: Some(Position { latitude: 37.363f32, longitude: -122.123f32, bearing: None, odometer: None, speed: None }),
                    current_stop_sequence: Some(5),
                    stop_id: Some("52".to_owned()),
                    current_status: Some(VehicleStopStatus::InTransitTo as i32),
                    timestamp: Some(1774967570),
                    congestion_level: None,
                    occupancy_status: Some(OccupancyStatus::StandingRoomOnly as i32),
                    occupancy_percentage: None,
                    multi_carriage_details: vec![] // not supported/todo
        },
        ])
}

extendr_module! {
    mod test_data;
    fn test_data_invalid_enum_positions;
}