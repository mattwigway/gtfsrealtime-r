use bytes::BytesMut;
use extendr_api::extendr_module;
use extendr_api::prelude::*;
use prost::Message;
use std::{fs, string::ToString};
use strum::VariantArray;

use crate::test_data::enum_roundtrip::get_or_none;
use crate::test_data::write_updates;
use crate::transit_realtime::{
    self,
    alert::{Cause, Effect, SeverityLevel},
    feed_header::Incrementality,
    trip_descriptor,
    trip_update::{
        stop_time_update::{self},
        StopTimeUpdate,
    },
    vehicle_descriptor::WheelchairAccessible,
    vehicle_position::{CongestionLevel, OccupancyStatus, VehicleStopStatus},
    Alert, EntitySelector, FeedEntity, FeedHeader, Position, TimeRange, TripDescriptor, TripUpdate,
    VehicleDescriptor, VehiclePosition,
};

#[extendr]
pub fn test_data_enum_roundtrip_updates(filename: &str) -> Result<List> {
    let n = [
        trip_descriptor::ScheduleRelationship::VARIANTS.len(),
        stop_time_update::ScheduleRelationship::VARIANTS.len(),
        OccupancyStatus::VARIANTS.len(),
        WheelchairAccessible::VARIANTS.len(),
    ]
    .into_iter()
    .max()
    .unwrap()
        + 1;

    let values: Vec<TripUpdate> = (0..n)
        .into_iter()
        .map(|i| {
            let mut update = TripUpdate {
                timestamp: Some(1774967578),
                trip: TripDescriptor {
                    trip_id: Some("42".to_owned()),
                    route_id: Some("42".to_owned()),
                    direction_id: Some(0),
                    start_time: Some("06:00:00".to_owned()),
                    start_date: Some("20250101".to_owned()),
                    schedule_relationship: None,
                    modified_trip: None,
                },
                vehicle: Some(VehicleDescriptor {
                    id: Some("42".to_owned()),
                    label: Some("42".to_owned()),
                    license_plate: Some("42".to_owned()),
                    wheelchair_accessible: None,
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
                        stop_time_properties: None,
                    },
                ],
                delay: None,
                trip_properties: None, //currently unused
            };

            match get_or_none::<trip_descriptor::ScheduleRelationship>(i) {
                Some(r) => update.trip.set_schedule_relationship(r),
                None => (),
            };

            match get_or_none::<WheelchairAccessible>(i) {
                Some(r) => update
                    .vehicle
                    .as_mut()
                    .unwrap()
                    .set_wheelchair_accessible(r),
                None => (),
            };

            match get_or_none::<OccupancyStatus>(i) {
                Some(r) => update.stop_time_update[0].set_departure_occupancy_status(r),
                None => (),
            };

            match get_or_none::<stop_time_update::ScheduleRelationship>(i) {
                Some(r) => update.stop_time_update[0].set_schedule_relationship(r),
                None => (),
            };

            update
        })
        .collect();

    // get the correct values as strings to pass to R separately for validation
    let l = list!(
        trip_schedule_relationship = values
            .iter()
            .map(|u| match u.trip.schedule_relationship {
                Some(_) => Some(u.trip.schedule_relationship().as_str_name()),
                None => None,
            })
            .collect::<Vec<Option<&str>>>(),
        wheelchair_accessible = values
            .iter()
            .map(
                |p| match p.vehicle.as_ref().unwrap().wheelchair_accessible {
                    Some(_) => Some(
                        p.vehicle
                            .as_ref()
                            .unwrap()
                            .wheelchair_accessible()
                            .as_str_name()
                    ),
                    None => None,
                }
            )
            .collect::<Vec<Option<&str>>>(),
        departure_occupancy_status = values
            .iter()
            .map(|p| match p.stop_time_update[0].departure_occupancy_status {
                Some(_) => Some(
                    p.stop_time_update[0]
                        .departure_occupancy_status()
                        .as_str_name()
                ),
                None => None,
            })
            .collect::<Vec<Option<&str>>>(),
        stop_schedule_relationship = values
            .iter()
            .map(|p| match p.stop_time_update[0].schedule_relationship {
                Some(_) => Some(p.stop_time_update[0].schedule_relationship().as_str_name()),
                None => None,
            })
            .collect::<Vec<Option<&str>>>()
    );

    write_updates(filename, values)?;

    Ok(l)
}

extendr_module! {
    mod enum_roundtrip_updates;
    fn test_data_enum_roundtrip_updates;
}
