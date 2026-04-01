// code to create various GTFS-realtime datasets for use in tests

use bytes::BytesMut;
use extendr_api::extendr_module;
use prost::Message;
use strum::VariantArray;
use std::{fs, string::ToString};
use extendr_api::prelude::*;

use crate::transit_realtime::{self, Alert, FeedEntity, FeedHeader, Position, TripDescriptor, TripUpdate, VehicleDescriptor, VehiclePosition, feed_header::Incrementality, trip_descriptor, trip_update::{self, StopTimeEvent, StopTimeUpdate, TripProperties, stop_time_update::{self, StopTimeProperties}}, vehicle_descriptor::{self, WheelchairAccessible}, vehicle_position::{CongestionLevel, OccupancyStatus, VehicleStopStatus}};

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

fn write_updates(filename: &str, updates: Vec<TripUpdate>) -> Result<()> {
    write_msg(filename, vec![], vec![], updates)
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

// test data to ensure enum roundtripping is correct
// each of these functions creates a file that has every enum value,
// and returns a list with the expected order.

fn get_or_none<T: Copy + VariantArray>(i: usize) -> Option<T> {
    if i >= T::VARIANTS.len() {
        None
    } else {
        Some(T::VARIANTS[i])
    }
}

#[extendr]
pub fn test_data_enum_roundtrip_positions(filename: &str) -> Result<List> {
    let n = [
        VehicleStopStatus::VARIANTS.len(),
        OccupancyStatus::VARIANTS.len(),
        CongestionLevel::VARIANTS.len(),
        trip_descriptor::ScheduleRelationship::VARIANTS.len(),
        WheelchairAccessible::VARIANTS.len()
    ].into_iter().max().unwrap() + 1;

    let values: Vec<VehiclePosition> = (0..n).into_iter().map(|i| {
        let mut p = VehiclePosition {
            trip: Some(TripDescriptor {
                trip_id: Some("42".to_owned()),
                route_id: Some("42".to_owned()),
                direction_id: Some(0),
                start_time: Some("06:00:00".to_owned()),
                start_date: Some("20250101".to_owned()),
                schedule_relationship: None,
                modified_trip: None
            }),
            vehicle: Some(VehicleDescriptor {
                id: Some("42".to_owned()),
                label: Some("42".to_owned()),
                license_plate: Some("42".to_owned()),
                wheelchair_accessible: None
            }),
            position: Some(Position { latitude: 37.363, longitude: -122.123, bearing: None, odometer: None, speed: None }),
            current_stop_sequence: Some(3),
            stop_id: Some("42".to_owned()),
            current_status: None,
            timestamp: Some(1774967570),
            congestion_level: None,
            occupancy_status: None,
            occupancy_percentage: None,
            multi_carriage_details: vec![]
        };

        // do it this way rather than specifying above so types are enforced by the rust type system.
        match get_or_none::<trip_descriptor::ScheduleRelationship>(i) {
            Some(r) => p.trip.as_mut().unwrap().set_schedule_relationship(r),
            None => {}
        };

        match get_or_none::<WheelchairAccessible>(i) {
            Some(r) => p.vehicle.as_mut().unwrap().set_wheelchair_accessible(r),
            None => {}
        };

        match get_or_none::<VehicleStopStatus>(i) {
            Some(r) => p.set_current_status(r),
            None => {}
        };

        match get_or_none::<CongestionLevel>(i) {
            Some(r) => p.set_congestion_level(r),
            None => {}
        };

        match get_or_none::<OccupancyStatus>(i) {
            Some(r) => p.set_occupancy_status(r),
            None => {}
        };

        p
    }).collect();

    // get the correct values as strings to pass to R separately for validation
    let l = list!(
        schedule_relationship = values.iter().map(|p| 
            match p.trip.as_ref().unwrap().schedule_relationship {
                Some(_) => Some(p.trip.as_ref().unwrap().schedule_relationship().as_str_name()),
                None => None
            }).collect::<Vec<Option<&str>>>(),


        wheelchair_accessible = values.iter().map(|p| 
            match p.vehicle.as_ref().unwrap().wheelchair_accessible {
                Some(_) => Some(p.vehicle.as_ref().unwrap().wheelchair_accessible().as_str_name()),
                None => None
            }).collect::<Vec<Option<&str>>>(),

        current_status = values.iter().map(|p| 
            match p.current_status {
                Some(_) => Some(p.current_status().as_str_name()),
                None => None
        }).collect::<Vec<Option<&str>>>(),

        congestion_level = values.iter().map(|p| 
            match p.congestion_level {
                Some(_) => Some(p.congestion_level().as_str_name()),
                None => None
        }).collect::<Vec<Option<&str>>>(),

        occupancy_status = values.iter().map(|p| 
            match p.occupancy_status {
                Some(_) => Some(p.occupancy_status().as_str_name()),
                None => None
        }).collect::<Vec<Option<&str>>>()
    );

    write_positions(filename, values)?;

    Ok(l)
}

#[extendr]
pub fn test_data_enum_roundtrip_updates (filename: &str) -> Result<List> {
    let n = [
        trip_descriptor::ScheduleRelationship::VARIANTS.len(),
        stop_time_update::ScheduleRelationship::VARIANTS.len(),
        OccupancyStatus::VARIANTS.len(),
        WheelchairAccessible::VARIANTS.len()
    ].into_iter().max().unwrap() + 1;

    let values: Vec<TripUpdate> = (0..n).into_iter().map(|i| {
        let mut update = TripUpdate {
            timestamp: Some(1774967578),
            trip: TripDescriptor {
                trip_id: Some("42".to_owned()),
                route_id: Some("42".to_owned()),
                direction_id: Some(0),
                start_time: Some("06:00:00".to_owned()),
                start_date: Some("20250101".to_owned()),
                schedule_relationship: None,
                modified_trip: None
            },
            vehicle: Some(VehicleDescriptor {
                id: Some("42".to_owned()),
                label: Some("42".to_owned()),
                license_plate: Some("42".to_owned()),
                wheelchair_accessible: None
            }),
            stop_time_update: vec![
                // Just one stop time update so this doesnt get expanded to multiple rows
                StopTimeUpdate {
                    stop_id: Some("fortytwo".to_owned()),
                    stop_sequence: Some(0),
                    arrival: None,
                    departure: None,
                    departure_occupancy_status: None,
                    schedule_relationship: None,
                    stop_time_properties: None
                }
            ],
            delay: None,
            trip_properties: None //currently unused
        };

        match get_or_none::<trip_descriptor::ScheduleRelationship>(i) {
            Some(r) => update.trip.set_schedule_relationship(r),
            None => ()
        };

        match get_or_none::<WheelchairAccessible>(i) {
            Some(r) => update.vehicle.as_mut().unwrap().set_wheelchair_accessible(r),
            None => ()
        };

        match get_or_none::<OccupancyStatus>(i) {
            Some(r) => update.stop_time_update[0].set_departure_occupancy_status(r),
            None => ()
        };

        match get_or_none::<stop_time_update::ScheduleRelationship>(i) {
            Some(r) => update.stop_time_update[0].set_schedule_relationship(r),
            None => ()
        };

        update
    }).collect();

    // get the correct values as strings to pass to R separately for validation
    let l = list!(
        trip_schedule_relationship = values.iter().map(|u| 
            match u.trip.schedule_relationship {
                Some(_) => Some(u.trip.schedule_relationship().as_str_name()),
                None => None
            }).collect::<Vec<Option<&str>>>(),

        wheelchair_accessible = values.iter().map(|p| 
            match p.vehicle.as_ref().unwrap().wheelchair_accessible {
                Some(_) => Some(p.vehicle.as_ref().unwrap().wheelchair_accessible().as_str_name()),
                None => None
            }).collect::<Vec<Option<&str>>>(),

        departure_occupancy_status = values.iter().map(|p| 
            match p.stop_time_update[0].departure_occupancy_status {
                Some(_) => Some(p.stop_time_update[0].departure_occupancy_status().as_str_name()),
                None => None
        }).collect::<Vec<Option<&str>>>(),

        stop_schedule_relationship = values.iter().map(|p| 
            match p.stop_time_update[0].schedule_relationship {
                Some(_) => Some(p.stop_time_update[0].schedule_relationship().as_str_name()),
                None => None
        }).collect::<Vec<Option<&str>>>()
    );

    write_updates(filename, values)?;

    Ok(l)
}

// #[extendr]
// fn test_data_enum_roundtrip_alerts

extendr_module! {
    mod test_data;
    fn test_data_invalid_enum_positions;
    fn test_data_enum_roundtrip_positions;
    fn test_data_enum_roundtrip_updates;
}