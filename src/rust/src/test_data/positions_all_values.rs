use extendr_api::prelude::*;

use crate::{test_data::write_positions, transit_realtime::{Position, TripDescriptor, VehicleDescriptor, VehiclePosition, trip_descriptor, vehicle_descriptor::WheelchairAccessible, vehicle_position::{CongestionLevel, OccupancyStatus, VehicleStopStatus}}};

/// Make sure we can read all values in vehicle positions
/// For the other types this is tested incidentally by the unwrapping tests, but there's no unwrapping
/// test for positions since each vehicle position record becomes just one row.
/// @keywords internal
#[extendr]
pub fn test_data_positions_all_values(filename: &str) -> Result<()> {
    let positions = vec![
        VehiclePosition {
            trip: Some(TripDescriptor {
                trip_id: Some("trip".to_owned()),
                route_id: Some("route".to_owned()),
                direction_id: Some(1),
                start_time: Some("07:00:00".to_owned()),
                start_date: Some("20260401".to_owned()),
                schedule_relationship: Some(trip_descriptor::ScheduleRelationship::Added as i32),
                modified_trip: None // unused
            }),
            vehicle: Some(VehicleDescriptor {
                id: Some("42".to_owned()),
                label: Some("label".to_owned()),
                license_plate: Some("LIC-4242".to_owned()),
                wheelchair_accessible: Some(WheelchairAccessible::WheelchairAccessible as i32)
            }),
            position: Some(Position {
                latitude: 37.363,
                longitude: -122.123,
                bearing: Some(78.0),
                odometer: Some(8675809.0),
                speed: Some(45.0)
            }),
            current_stop_sequence: Some(10),
            stop_id: Some("stop".to_owned()),
            current_status: Some(VehicleStopStatus::StoppedAt as i32),
            timestamp: Some(1775076484),
            congestion_level: Some(CongestionLevel::SevereCongestion as i32),
            occupancy_status: Some(OccupancyStatus::CrushedStandingRoomOnly as i32),
            occupancy_percentage: Some(15),
            multi_carriage_details: vec![]
        },
        VehiclePosition {
            trip: Some(TripDescriptor {
                trip_id: None,
                route_id: None,
                direction_id: None,
                start_time: None,
                start_date: None,
                schedule_relationship: None,
                modified_trip: None // unused
            }),
            vehicle: Some(VehicleDescriptor {
                id: None,
                label: None,
                license_plate: None,
                wheelchair_accessible: None
            }),
            position: Some(Position {
                latitude: 37.363,
                longitude: -122.123,
                bearing:None,
                odometer:None,
                speed:None
            }),
            current_stop_sequence: None,
            stop_id: None,
            current_status: None,
            timestamp: None,
            congestion_level: None,
            occupancy_status: None,
            occupancy_percentage: None,
            multi_carriage_details: vec![]
        },

        VehiclePosition {
            trip: None,
            vehicle: None,
            position: None,
            current_stop_sequence: None,
            stop_id: None,
            current_status: None,
            timestamp: None,
            congestion_level: None,
            occupancy_status: None,
            occupancy_percentage: None,
            multi_carriage_details: vec![]
        }
    ];

    write_positions(filename, positions)?;

    Ok(())
}

extendr_module! {
    mod positions_all_values;
    fn test_data_positions_all_values;
}