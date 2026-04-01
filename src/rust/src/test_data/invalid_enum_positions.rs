use extendr_api::extendr_module;
use extendr_api::prelude::*;
use std::string::ToString;

use crate::test_data::write_positions;
use crate::transit_realtime::{
    trip_descriptor,
    vehicle_position::{OccupancyStatus, VehicleStopStatus}, Position, TripDescriptor,
    VehicleDescriptor, VehiclePosition,
};

// A vehicle positions feed with invalid enum values
#[extendr]
pub fn test_data_invalid_enum_positions(filename: &str) -> Result<()> {
    write_positions(
        filename,
        vec![
            VehiclePosition {
                trip: Some(TripDescriptor {
                    trip_id: Some("8675309".to_owned()),
                    route_id: Some("16".to_owned()),
                    direction_id: Some(0),
                    start_time: Some("06:00:00".to_owned()),
                    start_date: Some("20260331".to_owned()),
                    schedule_relationship: Some(256), // invalid value
                    modified_trip: None,
                }),
                vehicle: Some(VehicleDescriptor {
                    id: None,
                    label: None,
                    license_plate: None,
                    wheelchair_accessible: None,
                }),
                position: Some(Position {
                    latitude: 37.363f32,
                    longitude: -122.123f32,
                    bearing: None,
                    odometer: None,
                    speed: None,
                }),
                current_stop_sequence: Some(5),
                stop_id: Some("52".to_owned()),
                current_status: Some(VehicleStopStatus::InTransitTo as i32),
                timestamp: Some(1774967570),
                congestion_level: None,
                occupancy_status: Some(OccupancyStatus::StandingRoomOnly as i32),
                occupancy_percentage: None,
                multi_carriage_details: vec![], // not supported/todo
            },
            VehiclePosition {
                trip: Some(TripDescriptor {
                    trip_id: Some("8675310".to_owned()),
                    route_id: Some("16".to_owned()),
                    direction_id: Some(0),
                    start_time: Some("06:05:00".to_owned()),
                    start_date: Some("20260331".to_owned()),
                    schedule_relationship: Some(
                        trip_descriptor::ScheduleRelationship::Scheduled as i32,
                    ), // valid value
                    modified_trip: None,
                }),
                vehicle: Some(VehicleDescriptor {
                    id: None,
                    label: None,
                    license_plate: None,
                    wheelchair_accessible: None,
                }),
                position: Some(Position {
                    latitude: 37.363f32,
                    longitude: -122.123f32,
                    bearing: None,
                    odometer: None,
                    speed: None,
                }),
                current_stop_sequence: Some(5),
                stop_id: Some("52".to_owned()),
                current_status: Some(VehicleStopStatus::InTransitTo as i32),
                timestamp: Some(1774967570),
                congestion_level: None,
                occupancy_status: Some(OccupancyStatus::StandingRoomOnly as i32),
                occupancy_percentage: None,
                multi_carriage_details: vec![], // not supported/todo
            },
        ],
    )
}

extendr_module! {
    mod invalid_enum_positions;
    fn test_data_invalid_enum_positions;
}
