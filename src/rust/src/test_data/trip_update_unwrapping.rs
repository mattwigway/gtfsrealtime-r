use extendr_api::prelude::*;

use crate::{
    test_data::write_updates,
    transit_realtime::{
        trip_descriptor,
        trip_update::{stop_time_update, StopTimeEvent, StopTimeUpdate},
        vehicle_descriptor::WheelchairAccessible,
        vehicle_position::OccupancyStatus,
        TripDescriptor, TripUpdate, VehicleDescriptor,
    },
};

/// Dataset that has two trip updates with
/// id 1: two stop times - should get expanded to two rows,
/// id 2: one stop time - should remain one row,
/// id 3: no stop times which should exist as a single row.
/// @keywords internal
#[extendr]
pub fn test_data_update_unwrapping(filename: &str) -> Result<()> {
    let values = vec![
        ///////////////////////////////////////////////////
        // id 1: two stop time updates
        TripUpdate {
            trip: TripDescriptor {
                trip_id: Some("one".to_owned()),
                route_id: Some("rte1".to_owned()),
                direction_id: Some(1),
                start_date: Some("20260401".to_owned()),
                start_time: Some("06:00:00".to_owned()),
                schedule_relationship: Some(
                    trip_descriptor::ScheduleRelationship::Scheduled as i32,
                ),
                modified_trip: None, // experimental, unused
            },
            vehicle: Some(VehicleDescriptor {
                id: Some("veh1".to_owned()),
                label: Some("lab1".to_owned()),
                license_plate: Some("PLA-0001".to_owned()),
                wheelchair_accessible: Some(WheelchairAccessible::WheelchairAccessible as i32),
            }),
            stop_time_update: vec![
                StopTimeUpdate {
                    stop_sequence: Some(2),
                    stop_id: Some("stop1".to_owned()),
                    arrival: Some(StopTimeEvent {
                        delay: Some(5),
                        time: Some(1775059604),
                        uncertainty: Some(35), // units are not documented...
                        scheduled_time: None,
                    }),
                    departure: Some(StopTimeEvent {
                        delay: Some(25),
                        time: Some(1775059624),
                        uncertainty: Some(25), // units are not documented...
                        scheduled_time: None,
                    }),
                    departure_occupancy_status: Some(OccupancyStatus::ManySeatsAvailable as i32),
                    schedule_relationship: Some(
                        stop_time_update::ScheduleRelationship::Scheduled as i32,
                    ),
                    stop_time_properties: None, // experimental
                },
                StopTimeUpdate {
                    stop_sequence: Some(4),
                    stop_id: Some("stop2".to_owned()),
                    arrival: Some(StopTimeEvent {
                        delay: Some(10),
                        time: Some(1775059704),
                        uncertainty: Some(37), // units are not documented...
                        scheduled_time: None,
                    }),
                    departure: Some(StopTimeEvent {
                        delay: Some(30),
                        time: Some(1775059724),
                        uncertainty: Some(27), // units are not documented...
                        scheduled_time: None,
                    }),
                    departure_occupancy_status: Some(OccupancyStatus::FewSeatsAvailable as i32),
                    schedule_relationship: Some(
                        stop_time_update::ScheduleRelationship::Skipped as i32,
                    ),
                    stop_time_properties: None, // experimental
                },
            ],
            timestamp: Some(1775059604),
            delay: Some(10),
            trip_properties: None, // experimental
        },
        ///////////////////////////////////////////////////
        // id 2: one stop time update
        TripUpdate {
            trip: TripDescriptor {
                trip_id: Some("two".to_owned()),
                route_id: Some("rte2".to_owned()),
                direction_id: Some(0),
                start_date: Some("20260402".to_owned()),
                start_time: Some("06:00:02".to_owned()),
                schedule_relationship: Some(trip_descriptor::ScheduleRelationship::Added as i32),
                modified_trip: None, // experimental, unused
            },
            vehicle: Some(VehicleDescriptor {
                id: Some("veh2".to_owned()),
                label: Some("lab2".to_owned()),
                license_plate: Some("PLA-0002".to_owned()),
                wheelchair_accessible: Some(WheelchairAccessible::NoValue as i32),
            }),
            stop_time_update: vec![StopTimeUpdate {
                stop_sequence: Some(1),
                stop_id: Some("stop1_2".to_owned()),
                arrival: Some(StopTimeEvent {
                    delay: Some(12),
                    time: Some(1775058604),
                    uncertainty: Some(30), // units are not documented...
                    scheduled_time: None,
                }),
                departure: Some(StopTimeEvent {
                    delay: Some(20),
                    time: Some(1775058624),
                    uncertainty: Some(24), // units are not documented...
                    scheduled_time: None,
                }),
                departure_occupancy_status: Some(OccupancyStatus::ManySeatsAvailable as i32),
                schedule_relationship: Some(stop_time_update::ScheduleRelationship::NoData as i32),
                stop_time_properties: None, // experimental
            }],
            timestamp: Some(1775059600),
            delay: Some(11),
            trip_properties: None, // experimental
        },
        ///////////////////////////////////////////////////
        // id 3: no stop time updates
        TripUpdate {
            trip: TripDescriptor {
                trip_id: Some("three".to_owned()),
                route_id: Some("rte3".to_owned()),
                direction_id: Some(0),
                start_date: Some("20260403".to_owned()),
                start_time: Some("06:00:03".to_owned()),
                schedule_relationship: Some(
                    trip_descriptor::ScheduleRelationship::Scheduled as i32,
                ),
                modified_trip: None, // experimental, unused
            },
            vehicle: None,
            stop_time_update: vec![],
            timestamp: Some(1775059610),
            delay: Some(2),
            trip_properties: None, // experimental
        },
    ];

    write_updates(filename, values)?;

    Ok(())
}

extendr_module! {
    mod trip_update_unwrapping;
    fn test_data_update_unwrapping;
}
